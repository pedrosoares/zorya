create schema zorya;

create table zorya.users (
    id bigserial not null primary key,
    name varchar(100) not null,
    email varchar(200) not null,
    password varchar(100) not null,
    project varchar(30) not null,
    created_at timestamp default now(),
    updated_at timestamp default now()
);

create trigger auto_updated_at before update on zorya.users
    for each row execute function public.auto_update_at();

create table zorya.apis (
    id bigserial not null primary key,
    name varchar(100) not null,
    email varchar(200) not null,
    password varchar(100) not null,
    project varchar(30) not null,
    created_at timestamp default now(),
    updated_at timestamp default now()
);

create trigger auto_updated_at before update on zorya.apis
    for each row execute function public.auto_update_at();

create table zorya.tokens (
    id bigserial not null primary key,
    token varchar(200) not null,
    expiration bigint not null,
    email varchar(200) not null unique,
    created_at timestamp default now(),
    updated_at timestamp default now()
);

create trigger auto_updated_at before update on zorya.tokens
    for each row execute function public.auto_update_at();
