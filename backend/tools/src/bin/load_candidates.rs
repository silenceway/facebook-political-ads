extern crate chrono;
extern crate csv;
extern crate dotenv;
extern crate serde_json;
extern crate server;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use server::start_logging;
use std::env;
use std::io;
use std::process;
use std::error::Error;

use server::models::Candidate;
use server::schema::candidates;

fn load_candidates() -> Result<(), Box<Error>>  {
  dotenv().ok();
  start_logging();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let conn = PgConnection::establish(&database_url).unwrap();

  let mut rdr = csv::ReaderBuilder::new()
      .has_headers(false)
      .from_reader(io::stdin());


  for result in rdr.deserialize() {
      let new_candidate: Candidate = result?;
      println!("{:?}", new_candidate);
      diesel::insert_into(candidates::table)
        .values(&new_candidate)
        .execute(&conn)
        .expect("Error saving new post");
  }
  Ok(())
}


fn main() {
    if let Err(err) = load_candidates() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
