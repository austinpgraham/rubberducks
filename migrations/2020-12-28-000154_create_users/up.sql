create table if not exists users (
    id bigserial primary key,
    email text not null,
    signup_source text not null,
    access_token text not null,
    token_expire_time timestamp without time zone not null,
    profile_picture text,
    first_name text not null,
    last_name text not null,
    phone text,
    location text,
    last_logged_in_time timestamp without time zone,
    created_at timestamp without time zone not null default now(),
    updated_at timestamp without time zone not null default now()
);
create index if not exists user_email_index on users (email);
comment on table users is 'Guess what, it''s the users of the system.';
