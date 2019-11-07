use super::schema::{url_status, urls};
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Queryable)]
pub struct Url {
    pub id: i32,
    pub url: String,
    pub active: bool,
}

#[derive(Insertable)]
#[table_name = "urls"]
pub struct NewUrl<'a> {
    pub url: &'a str,
}

impl<'a> NewUrl<'a> {
    pub fn create_or_get(conn: &PgConnection, url_str: &'a str) -> Url {
        use super::schema::urls::dsl::*;
        let mut existing: Vec<_> = urls
            .filter(url.eq(url_str))
            .load::<Url>(conn)
            .expect("Error loading posts");
        match existing.len() {
            0 => NewUrl::make_url(conn, url_str),
            1 => existing.pop().unwrap(),
            _ => unreachable!(),
        }
    }
    pub fn make_url(conn: &PgConnection, url: &'a str) -> Url {
        diesel::insert_into(urls::table)
            .values(&NewUrl { url })
            .get_result(conn)
            .expect("Error saving new post")
    }
    pub fn get_urls(conn: &PgConnection) -> Vec<Url> {
        use super::schema::urls::dsl::*;
        urls.filter(active.eq(true))
            .load::<Url>(conn)
            .expect("Error loading posts")
    }
}

#[derive(Queryable, Associations, Debug, Serialize, Deserialize)]
#[belongs_to(Url)]
#[table_name = "url_status"]
pub struct UrlStatus {
    pub id: i32,
    pub url_id: i32,
    pub created_at: NaiveDateTime,
    pub http_status: i32,
}

#[derive(Insertable, Queryable)]
#[table_name = "url_status"]
pub struct NewUrlStatus {
    url_id: i32,
    http_status: i32,
}

impl NewUrlStatus {
    pub fn new(url: &Url, http_status: i32) -> NewUrlStatus {
        NewUrlStatus {
            url_id: url.id,
            http_status,
        }
    }

    pub fn save(&self, conn: &PgConnection) -> UrlStatus {
        diesel::insert_into(url_status::table)
            .values(self)
            .get_result(conn)
            .expect("Error saving new post")
    }

    pub fn fetch_all(url: &Url, conn: &PgConnection) -> Vec<UrlStatus> {
        use super::schema::url_status::dsl::*;
        url_status
            .filter(url_id.eq(url.id))
            .load(conn)
            .expect("Error loading posts")
    }
}
