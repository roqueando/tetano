-- Your SQL goes here
create table posts (
  id uuid default uuid_generate_v4(),
  title varchar not null,
  body text not null,
  created_at timestamp default NOW(),
  updated_at timestamp default NOW(),
  PRIMARY KEY (id)
);
