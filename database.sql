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

create table zorya.apis (
    id bigserial not null primary key,
    name varchar(100) not null,
    email varchar(200) not null,
    password varchar(100) not null,
    project varchar(30) not null,
    created_at timestamp default now(),
    updated_at timestamp default now()
);

create table zorya.tokens (
    id bigserial not null primary key,
    token varchar(200) not null,
    expiration bigint not null,
    email varchar(200) not null unique,
    created_at timestamp default now(),
    updated_at timestamp default now()
);

-- AUTO UPDATE "updated_at" column

CREATE OR REPLACE FUNCTION zorya.auto_update_at()
    RETURNS trigger
    LANGUAGE plpgsql
AS $function$
BEGIN
    new.updated_at = now();
    return new;
END;
$function$
;

create trigger auto_updated_at before update on zorya.users
    for each row execute function zorya.auto_update_at();
create trigger auto_updated_at before update on zorya.apis
    for each row execute function zorya.auto_update_at();
create trigger auto_updated_at before update on zorya.tokens
    for each row execute function zorya.auto_update_at();
