drop table if exists Appear;

drop table if exists Cat;

drop table if exists CatBreed;

drop table if exists Commit;

drop table if exists Contact;

drop table if exists Feed;

drop table if exists Image;

drop table if exists Place;

drop table if exists Post;

drop table if exists PostContent;

drop table if exists Token;

drop table if exists Treat;

drop table if exists User;

create table Place
(
   placeId              int not null auto_increment,
   details              varchar(32) not null,
   primary key (placeId)
);

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
   key AK_Identifier_2 (username),
   constraint FK_Relationship_9 foreign key (imageId)
      references Image (imageId) on delete restrict on update restrict
);

create table CatBreed
(
   breedId              int not null auto_increment,
   breedName            varchar(10) not null,
   breedDesc            varchar(32),
   primary key (breedId)
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
   constraint FK_Appear foreign key (placeId)
      references Place (placeId) on delete restrict on update restrict,
   constraint FK_Appear2 foreign key (userId)
      references User (userId) on delete restrict on update restrict,
   constraint FK_Appear3 foreign key (catId)
      references Cat (catId) on delete restrict on update restrict
);

create table Commit
(
   commentId            int not null auto_increment,
   userId               int not null,
   commentText          varchar(128),
   primary key (commentId)
       constraint FK_Comment foreign key (userId)
       references User (userId) on delete restrict on update restrict
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
   constraint FK_Feed foreign key (catId)
      references Cat (catId) on delete restrict on update restrict,
   constraint FK_Feed2 foreign key (userId)
      references User (userId) on delete restrict on update restrict,
   constraint FK_Feed3 foreign key (placeId)
      references Place (placeId) on delete restrict on update restrict
);

create table PostContent
(
   postId               int not null,
   postTime             datetime not null,
   postText             varchar(128) not null,
   primary key (postId)
);

create table Post
(
   userId               int not null,
   catId                int not null,
   imageId              int not null,
   placeId              int not null,
   commentId            int not null,
   postId               int not null,
   primary key (userId, catId, imageId, placeId, commentId, postId),
   constraint FK_Post foreign key (userId)
      references User (userId) on delete restrict on update restrict,
   constraint FK_Post2 foreign key (catId)
      references Cat (catId) on delete restrict on update restrict,
   constraint FK_Post3 foreign key (imageId)
      references Image (imageId) on delete restrict on update restrict,
   constraint FK_Post4 foreign key (placeId)
      references Place (placeId) on delete restrict on update restrict,
   constraint FK_Post5 foreign key (commentId)
      references Commit (commentId) on delete restrict on update restrict,
   constraint FK_Post6 foreign key (postId)
      references PostContent (postId) on delete restrict on update restrict
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
   constraint FK_Treat foreign key (placeId)
      references Place (placeId) on delete restrict on update restrict,
   constraint FK_Treat2 foreign key (catId)
      references Cat (catId) on delete restrict on update restrict,
   constraint FK_Treat3 foreign key (userId)
      references User (userId) on delete restrict on update restrict
);

