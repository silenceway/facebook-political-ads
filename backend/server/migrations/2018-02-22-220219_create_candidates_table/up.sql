-- Your SQL goes here
CREATE TABLE candidates (
  facebook_url text primary key not null, 
  name text not null, 
  state varchar(2),
  district varchar(2),
  race_type varchar(20) not null
  );