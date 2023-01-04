drop trigger IF EXISTS CleanToken;

drop VIEW IF EXISTS FindPostPlaces;

drop VIEW IF EXISTS FindPostImages;

drop VIEW IF EXISTS FindPostComments;

drop VIEW IF EXISTS FindPostCats;

drop VIEW IF EXISTS FindCatPlaces;

drop table if exists Appear;

drop table if exists Cat;

drop table if exists CatBreed;

drop table if exists CommentContent;

drop table if exists Contact;

drop table if exists Feed;

drop table if exists Image;

drop table if exists Place;

drop table if exists PostCat;

drop table if exists PostComment;

drop table if exists PostContent;

drop table if exists PostImage;

drop table if exists PostPlace;

drop table if exists Token;

drop table if exists Treat;

drop table if exists User;

create table Image
(
   imageId              int not null auto_increment,
   url                  varchar(256) not null,
   primary key (imageId)
);

create table User
(
   userId               int not null auto_increment,
   imageId              int,
   username             varchar(20) not null,
   passwd               char(64) not null,
   usernick             varchar(20) not null,
   motto                varchar(32),
   primary key (userId),
   unique key AK_Identifier_2 (username),
   constraint FK_Relationship_9 foreign key (imageId)
      references Image (imageId) on delete restrict on update restrict
);

create table Place
(
   placeId              int not null auto_increment,
   details              varchar(32) not null,
   primary key (placeId)
);

create table CatBreed
(
   breedId              int not null auto_increment,
   breedName            varchar(10) not null,
   breedDesc            varchar(32),
   primary key (breedId),
   unique key AK_Identifier_2 (breedName)
);

create table Cat
(
   catId                int not null auto_increment,
   breedId              int,
   name                 varchar(32) not null,
   foundTime            date,
   source               varchar(32),
   atSchool             bool not null,
   whereabouts          varchar(32),
   health               varchar(32),
   primary key (catId),
   constraint FK_Relationship_8 foreign key (breedId)
      references CatBreed (breedId) on delete restrict on update restrict
);

create table Appear
(
   placeId              int not null,
   userId               int not null,
   catId                int not null,
   appearTime           datetime,
   primary key (placeId, userId, catId),
   constraint FK_Appear2 foreign key (userId)
      references User (userId) on delete restrict on update restrict,
   constraint FK_Appear foreign key (placeId)
      references Place (placeId) on delete restrict on update restrict,
   constraint FK_Appear3 foreign key (catId)
      references Cat (catId) on delete restrict on update restrict
);

create table CommentContent
(
   commentId            int not null auto_increment,
   commentText          varchar(128),
   primary key (commentId)
);

create table Contact
(
   contactId            int not null auto_increment,
   userId               int not null,
   contactType          varchar(8) not null,
   contactContent       varchar(32) not null,
   primary key (contactId),
   constraint FK_用户_联系方式 foreign key (userId)
      references User (userId) on delete restrict on update restrict
);

create table Feed
(
   catId                int not null,
   userId               int not null,
   placeId              int not null,
   feedTime             datetime not null,
   feedFood             varchar(16),
   feedAmount           varchar(16),
   primary key (catId, userId, placeId),
   constraint FK_Feed2 foreign key (userId)
      references User (userId) on delete restrict on update restrict,
   constraint FK_Feed foreign key (catId)
      references Cat (catId) on delete restrict on update restrict,
   constraint FK_Feed3 foreign key (placeId)
      references Place (placeId) on delete restrict on update restrict
);

create table PostContent
(
   postId               int not null auto_increment,
   userId               int,
   postTime             datetime not null,
   postText             varchar(128),
   primary key (postId),
   constraint FK_Relationship_4 foreign key (userId)
      references User (userId) on delete restrict on update restrict
);

create table PostCat
(
   postId               int not null,
   catId                int not null,
   primary key (postId, catId),
   constraint FK_PostCat foreign key (postId)
      references PostContent (postId) on delete restrict on update restrict,
   constraint FK_PostCat2 foreign key (catId)
      references Cat (catId) on delete restrict on update restrict
);

create table PostComment
(
   postId               int not null,
   userId               int not null,
   commentId            int not null,
   primary key (postId, userId, commentId),
   constraint FK_PostComment2 foreign key (userId)
      references User (userId) on delete restrict on update restrict,
   constraint FK_PostComment foreign key (postId)
      references PostContent (postId) on delete restrict on update restrict,
   constraint FK_PostComment3 foreign key (commentId)
      references CommentContent (commentId) on delete restrict on update restrict
);

create table PostImage
(
   postId               int not null,
   imageId              int not null,
   primary key (postId, imageId),
   constraint FK_PostImage foreign key (postId)
      references PostContent (postId) on delete restrict on update restrict,
   constraint FK_PostImage2 foreign key (imageId)
      references Image (imageId) on delete restrict on update restrict
);

create table PostPlace
(
   postId               int not null,
   placeId              int not null,
   primary key (postId, placeId),
   constraint FK_PostPlace foreign key (postId)
      references PostContent (postId) on delete restrict on update restrict,
   constraint FK_PostPlace2 foreign key (placeId)
      references Place (placeId) on delete restrict on update restrict
);

create table Token
(
   token                char(128) not null,
   uid                  numeric(8,0) not null,
   exp                  timestamp not null
);

create table Treat
(
   placeId              int not null,
   catId                int not null,
   userId               int not null,
   treatResults         varchar(128),
   treatTime            datetime,
   primary key (placeId, catId, userId),
   constraint FK_Treat3 foreign key (userId)
      references User (userId) on delete restrict on update restrict,
   constraint FK_Treat foreign key (placeId)
      references Place (placeId) on delete restrict on update restrict,
   constraint FK_Treat2 foreign key (catId)
      references Cat (catId) on delete restrict on update restrict
);

create VIEW  FindCatPlaces
 as
SELECT PostCat.catId,Place.details FROM PostCat
                JOIN PostPlace ON PostPlace.postId=PostCat.postId
                JOIN Place ON Place.placeId=PostPlace.postId;

create VIEW  FindPostCats
 as
SELECT PostContent.postId,Cat.catId,Cat.breedId,Cat.name,Cat.foundTime,Cat.source,Cat.atSchool,Cat.whereabouts,Cat.health
            FROM PostContent JOIN PostCat ON PostContent.postId=PostCat.postId
            JOIN Cat ON PostCat.catId=Cat.catId;

create VIEW  FindPostComments
 as
SELECT PostContent.postId,CommentContent.commentText,PostComment.userId,User.username,Image.url,User.usernick,User.motto
	        FROM PostContent
	        JOIN PostComment ON PostContent.postId=PostComment.postId
	        JOIN User ON PostComment.postId=User.userId
	        JOIN Image ON User.imageId=Image.imageId
	        JOIN CommentContent ON CommentContent.commentId=PostComment.commentId;

create VIEW  FindPostImages
 as
SELECT PostContent.postId,Image.url
            FROM PostContent JOIN PostImage ON PostContent.postId=PostImage.postId
            JOIN Image ON Image.imageId=PostImage.imageId;

create VIEW  FindPostPlaces
 as
SELECT PostContent.postId,Place.details
            FROM PostContent JOIN PostPlace ON PostContent.postId=PostPlace.postId
            JOIN Place ON PostPlace.placeId=Place.placeId;


CREATE TRIGGER CleanToken 
BEFORE INSERT ON PostContent 
FOR EACH ROW BEGIN
    SET time_zone='+00:00';
	DELETE FROM Token WHERE exp < NOW();
END;

