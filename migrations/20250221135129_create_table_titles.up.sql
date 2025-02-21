-- Add up migration script here
create table titles 
(
    id serial primary key,
    title varchar(100) not null,
    description text,
    created_att timestamp not null default current_timestamp,
    updated_att timestamp not null default current_timestamp
);