#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

use is_my_site_up::database::{make_pool, PgPool};
use is_my_site_up::models::{Url, UrlStatus};
use rocket_contrib::json::Json;

lazy_static! {
    pub static ref PG_POOL: PgPool = make_pool();
}

#[get("/")]
fn index() -> Json<Vec<UrlStatus>> {
    let db = PG_POOL.get().unwrap();
    Json(Url::get_status(&db, "http://dpbriggs.ca").unwrap_or_else(|| vec![]))
}

pub fn server() {
    rocket::ignite().mount("/", routes![index]).launch();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    server();
    Ok(())
}
