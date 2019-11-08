// fn index(info: web::Path<(u32, String)>) -> impl Responder {
//     format!("Hello {}! id:{}", info.1, info.0)
// }

// pub fn server() -> std::io::Result<()> {
//     let sys = actix::System::new("mystore");
//     HttpServer::new(|| {
//         App::new().service(web::resource("/products").route(web::get().to_async(index)))
//     })
//     .bind("127.0.0.1:8088")
//     .unwrap()
//     .start();
//     println!("Started http server: 127.0.0.1:8088");
//     let _ = sys.run();
//     Ok(())
// }

use crate::database::{make_pool, PgPool, PooledPg};
use crate::models::Url;
use std::sync::Arc;
// use warp::{self, reject, Filter};
use warp::{self, path, reject, Filter};

pub fn server() {
    let pool = make_pool();
    let hello = path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let pg = warp::any()
        .map(move || pool.clone())
        .and_then(|pool: PgPool| match pool.get() {
            Ok(conn) => Ok(conn),
            Err(_) => Err(reject::server_error()),
        });
    let hello_world = warp::get(
        warp::index()
            // use the pg connection on the route
            .and(pg.clone())
            .map(|db: PooledPg| {
                let statuses = Url::get_status(&db, "http://dpbriggs.ca");
                warp::reply::json(&statuses)
            }),
    );

    let routes = warp::get2().and(hello_world.or(hello));
    warp::serve(routes).run(([0, 0, 0, 0], 3030));
}
