drop index Appear3_FK;

drop index Appear2_FK;

drop index Appear_FK;

drop index Appear_PK;

drop table Appear;

drop index Relationship_8_FK;

drop index Cat_PK;

drop table Cat;

drop index CatBreed_PK;

drop table CatBreed;

drop index Commit_PK;

drop table Commit;

drop index 用户_联系方式_FK;

drop index Contact_PK;

drop table Contact;

drop index Feed3_FK;

drop index Feed2_FK;

drop index Feed_FK;

drop index Feed_PK;

drop table Feed;

drop index Image_PK;

drop table Image;

drop index Place_PK;

drop table Place;

drop index Post5_FK;

drop index Post4_FK;

drop index Post3_FK;

drop index Post2_FK;

drop index Post_FK;

drop index Post_PK;

drop table Post;

drop index Treat3_FK;

drop index Treat2_FK;

drop index Treat_FK;

drop index Treat_PK;

drop table Treat;

drop index Relationship_9_FK;

drop index User_PK;

drop table "User";

create table Place (
placeId              NUMERIC                        not null,
details              VARCHAR(32)                    not null,
primary key (placeId)
);

create table Image (
imageId              NUMERIC                        not null,
url                  VARCHAR(256)                   not null,
primary key (imageId)
);

create table "User" (
userId               NUMERIC                        not null,
imageId              NUMERIC,
username             VARCHAR(20)                    not null,
passwd               CHAR(64)                       not null,
usernick             VARCHAR(20)                    not null,
motto                VARCHAR(32),
primary key (userId),
unique (username),
foreign key (imageId)
      references Image (imageId)
);

create table CatBreed (
breedId              NUMERIC                        not null,
breedName            VARCHAR(10)                    not null,
breedDesc            VARCHAR(32),
primary key (breedId)
);

create table Cat (
catId                NUMERIC                        not null,
breedId              NUMERIC,
name                 VARCHAR(32)                    not null,
foundTime            DATE,
source               VARCHAR(32),
atSchool             SMALLINT                       not null,
whereabouts          VARCHAR(32),
health               VARCHAR(32),
primary key (catId),
foreign key (breedId)
      references CatBreed (breedId)
);

create table Appear (
placeId              NUMERIC                        not null,
userId               NUMERIC                        not null,
catId                NUMERIC                        not null,
appearTime            TIMESTAMP,
primary key (placeId, userId, catId),
foreign key (placeId)
      references Place (placeId),
foreign key (userId)
      references "User" (userId),
foreign key (catId)
      references Cat (catId)
);

create unique index Appear_PK on Appear (
placeId ASC,
userId ASC,
catId ASC
);

create  index Appear_FK on Appear (
placeId ASC
);

create  index Appear2_FK on Appear (
userId ASC
);

create  index Appear3_FK on Appear (
catId ASC
);

create unique index Cat_PK on Cat (
catId ASC
);

create  index Relationship_8_FK on Cat (
breedId ASC
);

create unique index CatBreed_PK on CatBreed (
breedId ASC
);

create table Commit (
commentText          VARCHAR(128),
commentId            NUMERIC                        not null,
primary key (commentId)
);

create unique index Commit_PK on Commit (
commentId ASC
);

create table Contact (
contactType          VARCHAR(8)                     not null,
contactContent       VARCHAR(32)                    not null,
contactId            NUMERIC                        not null,
userId               NUMERIC                        not null,
primary key (contactId),
foreign key (userId)
      references "User" (userId)
);

create unique index Contact_PK on Contact (
contactId ASC
);

create  index 用户_联系方式_FK on Contact (
userId ASC
);

create table Feed (
catId                NUMERIC                        not null,
userId               NUMERIC                        not null,
placeId              NUMERIC                        not null,
feedTime             TIMESTAMP                      not null,
feedFood             VARCHAR(16),
feedAmount           VARCHAR(16),
primary key (catId, userId, placeId),
foreign key (catId)
      references Cat (catId),
foreign key (userId)
      references "User" (userId),
foreign key (placeId)
      references Place (placeId)
);

create unique index Feed_PK on Feed (
catId ASC,
userId ASC,
placeId ASC
);

create  index Feed_FK on Feed (
catId ASC
);

create  index Feed2_FK on Feed (
userId ASC
);

create  index Feed3_FK on Feed (
placeId ASC
);

create unique index Image_PK on Image (
imageId ASC
);

create unique index Place_PK on Place (
placeId ASC
);

create table Post (
userId               NUMERIC                        not null,
catId                NUMERIC                        not null,
imageId              NUMERIC                        not null,
placeId              NUMERIC                        not null,
commentId            NUMERIC                        not null,
postText             VARCHAR(128),
postTime             TIMESTAMP,
primary key (userId, catId, imageId, placeId, commentId),
foreign key (userId)
      references "User" (userId),
foreign key (catId)
      references Cat (catId),
foreign key (imageId)
      references Image (imageId),
foreign key (placeId)
      references Place (placeId),
foreign key (commentId)
      references Commit (commentId)
);

create unique index Post_PK on Post (
userId ASC,
catId ASC,
imageId ASC,
placeId ASC,
commentId ASC
);

create  index Post_FK on Post (
userId ASC
);

create  index Post2_FK on Post (
catId ASC
);

create  index Post3_FK on Post (
imageId ASC
);

create  index Post4_FK on Post (
placeId ASC
);

create  index Post5_FK on Post (
commentId ASC
);

create table Treat (
placeId              NUMERIC                        not null,
catId                NUMERIC                        not null,
userId               NUMERIC                        not null,
treatResults         VARCHAR(128),
treatTime            TIMESTAMP,
primary key (placeId, catId, userId),
foreign key (placeId)
      references Place (placeId),
foreign key (catId)
      references Cat (catId),
foreign key (userId)
      references "User" (userId)
);

create unique index Treat_PK on Treat (
placeId ASC,
catId ASC,
userId ASC
);

create  index Treat_FK on Treat (
placeId ASC
);

create  index Treat2_FK on Treat (
catId ASC
);

create  index Treat3_FK on Treat (
userId ASC
);

create unique index User_PK on "User" (
userId ASC
);

create  index Relationship_9_FK on "User" (
imageId ASC
);

