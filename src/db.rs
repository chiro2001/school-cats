#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use encoding::{DecoderTrap, Encoding};
use log::*;
use mysql::*;
use mysql::prelude::*;
use cats_api::cats::{CatDB, CatPlacesResponse};
use cats_api::jwt::{EXP_REFRESH, EXP_TOKEN, jwt_encode, TokenDB};
use cats_api::posts::{CommentDisp, PostDisp, PostsContentDB, PostsPost};
use cats_api::user::{User, UserDB};
use crate::utils::chrono2sys;

pub const SQL_FILE: &'static str = "database/crebas.sql";

pub fn sql_url() -> String {
    std::env::var("DATABASE_URL").unwrap().as_str().to_string()
}

pub async fn db_init(pool: &Pool) -> Result<()> {
    let content_utf8 = std::fs::read_to_string(SQL_FILE);
    let split = match content_utf8 {
        Ok(content) => content.split(";").map(|s| s.to_string()).collect::<Vec<String>>(),
        Err(_) => {
            warn!("retry reading using gbk");
            let file = File::open(SQL_FILE)?;
            let reader = BufReader::new(&file);
            let queries = reader.split(b';').map(|q| q.unwrap());
            queries.map(|q| encoding::all::GBK.decode(&q, DecoderTrap::Strict).unwrap())
                .collect::<Vec<String>>()
        }
    };
    let mut conn = pool.get_conn().unwrap().unwrap();
    info!("SET FOREIGN_KEY_CHECKS=0;");
    let _: Vec<String> = conn.exec("SET FOREIGN_KEY_CHECKS=0;", Params::Empty)?;
    for s in split {
        let t = s.trim();
        if t.is_empty() {
            continue;
        }
        let exec = format!("{};", t);
        info!("exec: {}", exec);
        let r: Result<Vec<String>> = match conn.exec(exec, Params::Empty) {
            Ok(r) => Ok(r),
            Err(e) => Err(anyhow!("{:?}", e)),
        };
        match std::env::var("IGNORE_INIT_ERRORS") {
            Ok(s) => match s.as_str() {
                "1" => {}
                _ => {
                    match r {
                        Ok(_) => {}
                        Err(e) => {
                            error!("{:?}", e);
                            return Err(e);
                        }
                    };
                }
            }
            Err(_e) => {}
        };
    }
    info!("SET FOREIGN_KEY_CHECKS=1;");
    conn.exec_drop("SET FOREIGN_KEY_CHECKS=1;", Params::Empty)?;
    // insert default image
    conn.query_drop("INSERT INTO Image (url) VALUES (\"https://yew.rs/img/logo.svg\")")?;
    assert_eq!(1, conn.last_insert_id());
    // insert default user
    conn.exec_drop("INSERT INTO User (username,passwd,imageId,usernick,motto) VALUES (?,?,?,?,?);",
                   ("", "", 1, "", "", ))?;
    assert_eq!(1, conn.last_insert_id());
    // insert default place
    conn.query_drop("INSERT INTO Place (details) VALUES (\"未知\")")?;
    assert_eq!(1, conn.last_insert_id());
    conn.query_drop("INSERT INTO Place (details) VALUES (\"未知2\")")?;
    assert_eq!(2, conn.last_insert_id());
    // insert default breed
    conn.query_drop("INSERT INTO CatBreed (breedName,breedDesc) VALUES (\"未知\",\"\")")?;
    assert_eq!(1, conn.last_insert_id());
    // insert default cat
    let cat = CatDB::default();
    conn.exec_drop("INSERT INTO Cat (breedId,name,foundTime,source,atSchool,whereabouts,health) \
        VALUES (?,?,?,?,?,?,?)", (cat.breedId, cat.name, cat.foundTime.duration_since(UNIX_EPOCH)?, cat.source, cat.atSchool, cat.whereabouts, cat.health))?;
    assert_eq!(1, conn.last_insert_id());
    let cat = CatDB::default();
    conn.exec_drop("INSERT INTO Cat (breedId,name,foundTime,source,atSchool,whereabouts,health) \
        VALUES (?,?,?,?,?,?,?)", (cat.breedId, cat.name, cat.foundTime.duration_since(UNIX_EPOCH)?, cat.source, cat.atSchool, cat.whereabouts, cat.health))?;
    assert_eq!(2, conn.last_insert_id());
    // insert test post
    conn.exec_drop("INSERT INTO PostContent (userId,postTime,postText) VALUES (?,?,?)",
                   (1, Utc::now().naive_utc(), "Text"))?;
    assert_eq!(1, conn.last_insert_id());
    conn.exec_drop("INSERT INTO PostPlace (postId,placeId) VALUES (?,?)",
                   (1, 2))?;
    conn.exec_drop("INSERT INTO PostCat (postId,catId) VALUES (?,?)",
                   (1, 2))?;
    // insert test comment
    conn.exec_drop("INSERT INTO PostComment (postId,userId,commentText) VALUES (?,?,?)",
                   (1, 1, "CommentText"))?;
    Ok(())
}

pub async fn db_get_pool() -> Result<Pool> {
    let pool = Pool::new(sql_url().as_str()).unwrap();
    match std::env::var("DB_FORCE_INIT") {
        Ok(s) => match s.as_str() {
            "1" => {
                db_init(&pool).await?;
            }
            _ => {}
        }
        Err(_e) => {}
    };
    Ok(pool)
}

#[derive(Clone)]
pub struct Database {
    pub pool: Arc<Pool>,
}

impl Database {
    pub fn new(pool: Pool) -> Self {
        Self { pool: Arc::new(pool) }
    }
    pub fn conn(&self) -> Result<PooledConn> {
        match self.pool.get_conn() {
            Ok(conn) => Ok(conn),
            Err(e) => Err(anyhow!("cannot get conn: {:?}", e))
        }
    }
    pub fn token_check(&self, token: &str) -> Result<TokenDB> {
        let mut conn = self.conn()?;
        let r = conn.exec_first("SELECT token,uid FROM Token WHERE token = :token",
                                params! { "token" => token })
            .map(|row| {
                row.map(|(token, uid)| TokenDB {
                    token,
                    exp: SystemTime::now(),
                    uid,
                })
            })?;
        match r {
            Some(t) => Ok(t),
            None => Err(anyhow!("no token found for {}", token))
        }
    }
    pub fn user_check(&self, username: &str, passwd: &str) -> Result<u32> {
        let mut conn = self.conn()?;
        let r: Option<u32> = conn.exec_first("SELECT userId FROM User WHERE username=? AND passwd=?", (username, passwd))?;
        match r {
            Some(uid) => Ok(uid),
            None => Err(anyhow!("username or password error"))
        }
    }
    fn create_token_exp(&self, uid: u32, exp_secs: u64) -> Result<(String, SystemTime)> {
        let exp = SystemTime::now().add(Duration::from_secs(exp_secs));
        let token = jwt_encode(uid, exp)?;
        // add token to database
        let mut conn = self.conn()?;
        let datetime: DateTime<Utc> = exp.into();
        let time = datetime.naive_utc();
        info!("create token with exp {:?}, {:?}", datetime, time);
        conn.exec_drop("INSERT INTO Token (token,exp,uid) VALUES (?,?,?)",
                       (&token, time, uid))?;
        Ok((token, exp))
    }
    pub fn create_token(&self, uid: u32) -> Result<(String, SystemTime)> { self.create_token_exp(uid, EXP_TOKEN) }
    pub fn create_refresh_token(&self, uid: u32) -> Result<(String, SystemTime)> { self.create_token_exp(uid, EXP_REFRESH) }
    pub fn user_db(&self, uid: u32) -> Result<UserDB> {
        let mut conn = self.conn()?;
        let r = conn.exec_first("SELECT userId,username,imageId,usernick,motto FROM User WHERE userId=?",
                                (uid, ))
            .map(|row| {
                row.map(|(userId, username, imageId, usernick, motto)| UserDB { userId, username, imageId, usernick, motto, passwd: "".to_string() })
            })?;
        match r {
            Some(t) => Ok(t),
            None => Err(anyhow!("no uid found for {}", uid))
        }
    }
    pub fn user(&self, uid: u32) -> Result<User> {
        let u = self.user_db(uid)?;
        let head = self.user_head(uid)?;
        Ok(User::from_db(u, head))
    }
    pub fn user_head(&self, uid: u32) -> Result<String> {
        let mut conn = self.conn()?;
        let r = conn.exec_first("SELECT Image.url FROM User \
            JOIN Image ON Image.imageId=User.imageId WHERE User.userId=?", (uid, ))?;
        match r {
            Some(r) => Ok(r),
            None => Err(anyhow!("not found"))
        }
    }
    pub fn image_insert(&self, url: &str) -> Result<u32> {
        let mut conn = self.conn()?;
        conn.exec_drop("INSERT INTO Image (url) VALUES (:url);", params! { "url" => url })?;
        Ok(conn.last_insert_id() as u32)
    }
    pub fn place_insert(&self, details: &str) -> Result<u32> {
        let mut conn = self.conn()?;
        conn.exec_drop("INSERT INTO Place (details) VALUES (?)", (details, ))?;
        Ok(conn.last_insert_id() as u32)
    }
    pub fn user_insert(&self, user: UserDB) -> Result<u32> {
        let mut conn = self.conn()?;
        info!("insert user: {:?}", user);
        conn.exec_drop("INSERT INTO User (username,passwd,imageId,usernick,motto) VALUES (?,?,?,?,?);", (
            user.username,
            user.passwd,
            user.imageId,
            user.usernick,
            user.motto
        ))?;
        Ok(conn.last_insert_id() as u32)
    }
    // pub fn post_relative_insert(&self, p: PostsDB) -> Result<()> {
    //     let mut conn = self.conn()?;
    //     conn.exec_drop("INSERT INTO Post (postId,userId,catId,imageId,placeId,commentId) VALUES (?,?,?,?,?,?)",
    //                    (p.postId, p.userId, p.catId, p.placeId, p.commentId))?;
    //     Ok(())
    // }
    pub fn post_insert(&self, uid: u32, post: PostsPost) -> Result<u32> {
        let mut conn = self.conn()?;
        info!("insert post: {:?}", post);
        // insert text
        conn.exec_drop("INSERT INTO PostContent (userId,postText,postTime) VALUES (?,?,?)",
                       (uid, post.text, Utc::now().naive_utc()))?;
        let id_post = conn.last_insert_id() as u32;
        // insert images
        for image in post.images {
            let id_image = self.image_insert(&image)?;
            conn.exec_drop("INSERT INTO PostImage (postId,imageId) VALUES (?,?)", (id_post, id_image))?;
        }
        // insert places
        for place in post.places {
            conn.exec_drop("INSERT INTO PostPlace (postId,placeId) VALUES (?,?)", (id_post, place))?;
        }
        Ok(id_post)
    }
    fn select_cat_mapper(s: (u32, u32, String, NaiveDateTime, String, bool, String, String)) -> CatDB {
        let (catId, breedId, name, foundTime, source, atSchool, whereabouts, health) = s;
        let foundTime = chrono2sys(foundTime);
        CatDB { catId, breedId, name, foundTime, source, atSchool, whereabouts, health }
    }
    pub fn post_data(&self, id: u32) -> Result<PostDisp> {
        let mut conn = self.conn()?;
        let cats = conn.exec_map("SELECT Cat.catId,Cat.breedId,Cat.name,Cat.foundTime,Cat.source,Cat.atSchool,Cat.whereabouts,Cat.health  \
            FROM PostContent JOIN PostCat ON PostContent.postId=PostCat.postId \
            JOIN Cat ON PostCat.catId=Cat.catId WHERE PostContent.postId=?", (id, ), Self::select_cat_mapper)?;
        info!("[{}] cats: {:?}", id, cats);
        #[allow(unused_parens)]
            let f = |(x)| x;
        let images: Vec<String> = conn.exec_map("SELECT Image.url \
            FROM PostContent JOIN PostImage ON PostContent.postId=PostImage.postId \
            JOIN Image ON Image.imageId=PostImage.imageId WHERE PostContent.postId=?", (id, ), f)?;
        info!("[{}] images: {:?}", id, images);
        let places: Vec<String> = conn.exec_map("SELECT Place.details \
            FROM PostContent JOIN PostPlace ON PostContent.postId=PostPlace.postId \
            JOIN Place ON PostPlace.placeId=Place.placeId WHERE PostContent.postId=?", (id, ), f)?;
        info!("[{}] places: {:?}", id, places);
        let f = |(text, uid, username, head, usernick, motto)| {
            CommentDisp { text, user: User { username, uid, head, usernick, motto } }
        };
        let comments = conn.exec_map("SELECT PostComment.commentText,PostComment.userId,User.username,Image.url,User.usernick,User.motto \
	        FROM PostContent JOIN PostComment ON PostContent.postId=PostComment.postId \
	        JOIN User ON PostComment.postId=User.userId \
	        JOIN Image ON User.imageId=Image.imageId WHERE PostContent.postId=?", (id, ), f)?;
        info!("[{}] comments: {:?}", id, comments);
        let post = match conn.exec_first("SELECT postId,userId,postTime,postText FROM PostContent \
            WHERE postId=?", (id, )).map(|row: Option<(u32, u32, NaiveDateTime, String)>| {
            row.map(|(postId, userId, postTime, postText)| PostsContentDB { postId, userId, postTime: chrono2sys(postTime), postText })
        })? {
            Some(p) => Ok(p),
            None => Err(anyhow!("no post found"))
        }?;
        info!("[{}] post: {:?}", id, post);
        let user = self.user(post.userId)?;
        Ok(PostDisp { postId: id, user, images, comments, places, cats, text: post.postText, time: post.postTime })
    }
    pub fn post_list(&self) -> Result<Vec<PostDisp>> {
        let mut conn = self.conn()?;
        let post_ids: Vec<u32> = conn.query_map("SELECT postId FROM PostContent ORDER BY postId DESC", |i| i)?;
        let posts = post_ids.into_iter().map(|id| self.post_data(id)).map(|x| match x {
            Ok(x) => x,
            Err(e) => {
                error!("{:?}", e);
                PostDisp::default()
            }
        }).filter(|p| p.postId != 0).collect::<Vec<PostDisp>>();
        info!("post_list: {:?}", posts);
        Ok(posts)
    }
    #[allow(dead_code)]
    pub fn cat_insert(&self, cat: CatDB) -> Result<u32> {
        let mut conn = self.conn()?;
        conn.exec_drop("INSERT INTO Cat (breedId,name,foundTime,source,atSchool,whereabouts,health) \
        VALUES (?,?,?,?,?,?,?)", (cat.breedId, cat.name, cat.foundTime.duration_since(UNIX_EPOCH)?, cat.source, cat.atSchool, cat.whereabouts, cat.health))?;
        Ok(conn.last_insert_id() as u32)
    }
    pub fn cats(&self) -> Result<Vec<CatDB>> {
        let mut conn = self.conn()?;
        Ok(conn.query_map("SELECT catId, breedId, name, foundTime, source, atSchool, whereabouts, health FROM Cat ORDER BY catId DESC", Self::select_cat_mapper)?)
    }
    pub fn cats_places(&self) -> Result<Vec<CatPlacesResponse>> {
        let mut conn = self.conn()?;
        let cats = self.cats()?;
        #[allow(unused_parens)]
        Ok(cats.into_iter().map(|cat| {
            let places: Vec<String> = match conn.exec_map("SELECT Place.details FROM PostCat \
                JOIN PostPlace ON PostPlace.postId=PostCat.postId \
                JOIN Place ON Place.placeId=PostPlace.postId \
                WHERE PostCat.catId=?", (cat.catId, ), |(s)| s) {
                Ok(s) => s,
                Err(_) => vec![],
            };
            CatPlacesResponse { cat, places }
        }).filter(|c| !c.places.is_empty()).collect())
    }
}