-- Add up migration script here
create table categories 
(
    id serial primary key,
    name varchar(100) not null
);