use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{MySql, MySqlPool, Pool};

mod database;
mod read_request_body;
mod read_response_body;
mod redirect;
mod simple;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();
    // std::env::set_var("DATABASE_URL", "mysql://tulipan:Okcpli$t94zxcv@localhost:3306/DiscordBot");
	std::env::set_var("RUST_LOG", "actix_web=debug");
    // let a = std::env::var("DATABASE_URL").unwrap();
    // println!("DATABASE_URL from std::env::var => {}", a);
    // let pool = MySqlPoolOptions::new()
    // .max_connections(10)
    // .after_connect(|_| Box::pin(async move {
    // 	println!("Connected to Discord Bot database");

    // 	Ok(())
    // }))
    // .connect(&std::env::var("DATABASE_URL")?)
    // .await?;

	let pool = MySqlPool::connect(&std::env::var("DATABASE_URL").
		expect("DATABASE_URL must be set")).await?;
    // Make a simple query to return the given parameter
    let _row: (i64,) = sqlx::query_as("SELECT COUNT(id) FROM `guilds` WHERE 1")
        .bind(150_i64)
        .fetch_one(&pool)
		.await?;
	#[derive(Debug)]
	struct User { id: String }

	// let mut account = sqlx::query_as::<_ , User>("
	// SELECT id
	// FROM guilds
	// ").fetch(&mut pool);
	
	let _id = sqlx::query_as!(User, "SELECT id from users")
	.fetch_all(&pool)
	.await?;
	
	println!("{:?}", _id[0]);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // .wrap(redirect::CheckLogin)
            // .wrap(read_request_body::Logging)
            // .wrap(read_response_body::Logging)
            // .wrap(simple::SayHi)
            // .wrap_fn(|req, srv| {
            //      println!("Hi from start. You requested: {}", req.path());
            //     srv.call(req).map(|res| {
            //         println!("Hi from response");
            //         res
            //     })
            // })
            .service(web::resource("/login").to(|| async {
                "You are on /login. Go to src/redirect.rs to change this behavior."
            }))
            .service(
                web::resource("/").to(|| async {
                    "Hello, middleware! Check the console where the server is run."
                }),
            )
            .service(discord_login)
    })
    .bind("127.0.0.1:3333")?
    .run()
    .await?;

    Ok(())
}
#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
}

#[derive(Deserialize)]
struct DisocrdCode {
    code: Option<String>,
}

#[get("/api/discord-login")]
async fn discord_login(
    _pool: web::Data<Pool<MySql>>,
    info: web::Query<DisocrdCode>,
    _req: HttpRequest,
) -> HttpResponse {
    println!("{:?} -------------------------", info.code);
    if info.code.is_some() {
        println!("YEP");
    } else {
        println!("NOP");
    }

    HttpResponse::Ok().json(MyObj {
        name: "dflkjlk".to_string(),
    })
}
#[get("/api/{guild_id}/users")]
async fn get_users_for_guild(web::Path(guild_id): web::Path<String>) -> HttpResponse {
    println!("Guild_id: {}", guild_id);

    HttpResponse::Ok().json(MyObj {
        name: "dflkjlk".to_string(),
    })
}

#[get("/api/users/@me")]
async fn get_user() -> HttpResponse {
    HttpResponse::Ok().json(MyObj {
        name: "dflkjlk".to_string(),
    })
}

#[get("/api/guilds/@channels")]
async fn get_guild_channels() -> HttpResponse {
    HttpResponse::Ok().json(MyObj {
        name: "dflkjlk".to_string(),
    })
}

// only path
#[get("/api/channels/{channel_id}/messages")]
async fn get_messages_for_channel(web::Path(channel_id): web::Path<String>) -> HttpResponse {
    println!("channel_id: {}", channel_id);

    HttpResponse::Ok().json(MyObj {
        name: "dflkjlk".to_string(),
    })
}

#[post("/interactions")]
async fn interactions() -> HttpResponse {
    HttpResponse::Ok().json(MyObj {
        name: "interactions".to_string(),
    })
}
