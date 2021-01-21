// use actix_web::{App, HttpServer, Responder, get, web};



// struct COOKIES {
//     cookie_access: i32,
//     cookie_refresh: i32,
// }

// const REFRESH: COOKIES = COOKIES {
//     // 15 min
//     cookie_access: 60 * 15 * 1000,
//     // 30 days
//     cookie_refresh: 60 * 60 * 15 * 1000,
// };

// #[get("/api/{name}")]
// async fn index(web::Path(name): web::Path<String>) -> impl Responder {
// 	let [a,b] = [REFRESH.cookie_access, REFRESH.cookie_refresh];
//     println!("jfdkdfj {}!", name);
//     format!("Hello {}!", name)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(index))
//         .bind("localhost:3333")?
//         .run()
//         .await
// }
