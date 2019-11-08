use std::collections::HashMap;
mod fetching;
use is_my_site_up::database::make_pool;
use is_my_site_up::fetching::{head_req, init_client};
use is_my_site_up::models::{NewUrl, NewUrlStatus};
use is_my_site_up::server::server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // server().unwrap();
    server();
    // let pgpool = make_pool();
    // let conn = pgpool.get().unwrap();
    // let url = NewUrl::create_or_get(&conn, "http://dpbriggs.ca");
    // let client = init_client();
    // for _ in 0..5 {
    //     let resp = head_req(&client, &url.url).unwrap();
    //     let url_status = NewUrlStatus::new(&url, resp.status().as_u16() as i32).save(&conn);
    //     dbg!(url_status);
    // }
    Ok(())
}
