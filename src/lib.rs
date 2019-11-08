pub mod database;
pub mod fetching;
pub mod models;
pub mod schema;
pub mod server;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate serde_derive;
