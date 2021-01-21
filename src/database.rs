use sqlx::{MySql, Pool};

#[derive(Debug)]
pub struct Database {
	pool: Result<Pool<MySql>, sqlx::Error>
}

// impl Database {
// 	pub async fn new() -> Self {
// 		Self {
// 			pool: MySqlPoolOptions::new()
// 			.max_connections(10)
// 			.after_connect(|_| Box::pin(async move {
// 				print!("Connected to Discord Bot database");
		
// 				Ok(())
// 			}))
// 			.connect("mysql://tulipan:Okcpli$t94zxcv@localhost:3307/DiscordBot")
// 			.await
// 		}
// 	}
// }

// const DB: Database = Database::new();

// let url = "mysql://tulipan:Okcpli$t94zxcv@localhost:3307/DiscordBot";
// impl database {
// 	async fn new() -> database {
// 		database {
// 			pool: MySqlPoolOptions::new()
// 				.max_connections(10)
// 				.connect("mysql://tulipan:Okcpli$t94zxcv@localhost:3307/DiscordBot")
// 				.await?
// 		}
// 	}
// }

// async fn a() -> Result<(), sqlx::Error> {

// 	// Make a simple query to return the given parameter

// 	Ok(())
// }