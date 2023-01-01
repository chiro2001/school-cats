drop table if exists Appear;

drop table if exists Cat;

drop table if exists CatBreed;

drop table if exists Commit;

drop table if exists Contact;

drop table if exists Feed;

drop table if exists Image;

drop table if exists Place;

drop table if exists Post;

drop table if exists Token;

drop table if exists Treat;

drop table if exists User;

create table Place
(
   placeId              numeric(8,0) not null,
   details              varchar(32) not null,
   primary key (placeId)
);

create table Image
(
   imageId              numeric(8,0) not null auto_increment,
   url                  varchar(256) not null,
   primary key (imageId)
);

create table User
(
   userId               numeric(8,0) not null,
   imageId              numeric(8,0),
   username             varchar(20) not null,
   passwd               char(64) not null,
   usernick             varchar(20) not null,
   motto                varchar(32),
   primary key (userId),
   key AK_Identifier_2 (username),
   constraint FK_Relationship_9 foreign key (imageId)
      references Image (imageId) on delete restrict on update restrict
);

create table CatBreed
(
   breedId              numeric(8,0) not null,
   breedName            varchar(10) not null,
   breedDesc            varchar(32),
   primary key (breedId)
);

create table Cat
(
   catId                numeric(8,0) not null,
   breedId              numeric(8,0),
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
   placeId              numeric(8,0) not null,
   userId               numeric(8,0) not null,
   catId                numeric(8,0) not null,
   appearTime           datetime,
   primary key (placeId, userId, catId),
   constraint FK_Appear foreign key (placeId)
      references Place (placeId) on delete restrict on update restrict,
   constraint FK_Appear2 foreign key (userId)
      references User (userId) on delete restrict on update restrict,
   constraint FK_Appear3 foreign key (catId)
      references Cat (catId) on delete restrict on update restrict
);

create table Commit
(
   commentText          varchar(128),
   commentId            numeric(8,0) not null,
   primary key (commentId)
);

create table Contact
(
   contactType          varchar(8) not null,
   contactContent       varchar(32) not null,
   contactId            numeric(8,0) not null,
   userId               numeric(8,0) not null,
   primary key (contactId),
   constraint FK_用户_联系方式 foreign key (userId)
      references User (userId) on delete restrict on update restrict
);

create table Feed
(
   catId                numeric(8,0) not null,
   userId               numeric(8,0) not null,
   placeId              numeric(8,0) not null,
   feedTime             datetime not null,
   feedFood             varchar(16),
   feedAmount           varchar(16),
   primary key (catId, userId, placeId),
   constraint FK_Feed foreign key (catId)
      references Cat (catId) on delete restrict on update restrict,
   constraint FK_Feed2 foreign key (userId)
      references User (userId) on delete restrict on update restrict,
   constraint FK_Feed3 foreign key (placeId)
      references Place (placeId) on delete restrict on update restrict
);

create table Post
(
   userId               numeric(8,0) not null,
   catId                numeric(8,0) not null,
   imageId              numeric(8,0) not null,
   placeId              numeric(8,0) not null,
   commentId            numeric(8,0) not null,
   postText             varchar(128),
   postTime             datetime,
   primary key (userId, catId, imageId, placeId, commentId),
   constraint FK_Post foreign key (userId)
      references User (userId) on delete restrict on update restrict,
   constraint FK_Post2 foreign key (catId)
      references Cat (catId) on delete restrict on update restrict,
   constraint FK_Post3 foreign key (imageId)
      references Image (imageId) on delete restrict on update restrict,
   constraint FK_Post4 foreign key (placeId)
      references Place (placeId) on delete restrict on update restrict,
   constraint FK_Post5 foreign key (commentId)
      references Commit (commentId) on delete restrict on update restrict
);

create table Token
(
   token                char(128) not null,
   uid                  numeric(8,0) not null,
   exp                  timestamp not null
);

create table Treat
(
   placeId              numeric(8,0) not null,
   catId                numeric(8,0) not null,
   userId               numeric(8,0) not null,
   treatResults         varchar(128),
   treatTime            datetime,
   primary key (placeId, catId, userId),
   constraint FK_Treat foreign key (placeId)
      references Place (placeId) on delete restrict on update restrict,
   constraint FK_Treat2 foreign key (catId)
      references Cat (catId) on delete restrict on update restrict,
   constraint FK_Treat3 foreign key (userId)
      references User (userId) on delete restrict on update restrict
);

