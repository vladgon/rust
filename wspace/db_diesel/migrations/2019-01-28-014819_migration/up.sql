-- Your SQL goes here
create table customer
(
  id int auto_increment
    primary key,
  name varchar(255) null,
  title varchar(255) null,
  lastName varchar(255) null
);

create table posts
(
  id bigint unsigned auto_increment primary key,
  title varchar(255) null,
  body text null,
  published tinyint(1) null,
  constraint id
    unique (id)
);

INSERT INTO sample.posts (id, title, body, published) VALUES (1, 'vlad', 'Post body', null);
INSERT INTO sample.posts (id, title, body, published) VALUES (2, 'vlad', 'Post body', null);
INSERT INTO sample.posts (id, title, body, published) VALUES (3, 'vlad', 'Post body', null);
INSERT INTO sample.posts (id, title, body, published) VALUES (4, 'vlad', 'Post body', null);
INSERT INTO sample.posts (id, title, body, published) VALUES (5, 'vlad', 'Post body', null);
INSERT INTO sample.posts (id, title, body, published) VALUES (6, 'vlad', 'Post body', null);
INSERT INTO sample.posts (id, title, body, published) VALUES (7, 'vlad', 'Post body', null);
INSERT INTO sample.posts (id, title, body, published) VALUES (8, 'vlad', 'Post body', null);
INSERT INTO sample.posts (id, title, body, published) VALUES (9, 'vlad', 'Post body', null);
INSERT INTO sample.posts (id, title, body, published) VALUES (10, 'vlad', 'Post body', null);
INSERT INTO sample.posts (id, title, body, published) VALUES (11, 'vlad', 'Post body', null);
INSERT INTO sample.posts (id, title, body, published) VALUES (12, 'vlad', 'Post body', null);
INSERT INTO sample.posts (id, title, body, published) VALUES (13, 'vlad', 'Post body', null);
INSERT INTO sample.posts (id, title, body, published) VALUES (14, 'vlad', 'Post body', null);
INSERT INTO sample.posts (id, title, body, published) VALUES (15, 'vlad', 'Post body', null);