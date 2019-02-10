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
