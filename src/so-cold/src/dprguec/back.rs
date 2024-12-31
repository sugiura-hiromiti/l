//! server functionalities

use crate::*;
use fallible_streaming_iterator::FallibleStreamingIterator;
use server_fn::codec::GetUrl;

//  TODO: use supabase
#[cfg(feature = "server")]
thread_local! {
pub static CONNECTION: rusqlite::Connection = {
	let con = rusqlite::Connection::open("so_cold.db",).expect("failed to open so_cold.db",);

	// create article table if not exist
	con.execute_batch(
		"CREATE TABLE IF NOT EXISTS article (
				id INTEGER PRIMARY KEY,
				date TEXT NOT NULL
		);",
	)
	.unwrap();

	// create user table if not exist
	con.execute_batch(
		"CREATE TABLE IF NOT EXISTS user (
				id INTEGER PRIMARY KEY,
				name VARCHAR(30) NOT NULL,
				email VARCHAR(30) NOT NULL,
				password VARCHAR(30) NOT NULL,
				created_date TEXT NOT NULL,
				updated_date TEXT NOT NULL,
		);",
	)
	.unwrap();

	con
};
}

#[server]
pub async fn verify_user(user: entity::User, way: String,) -> Result<(), ServerFnError,> {
	tracing::debug!("server: {way}");
	let mut user = user;
	if user.verify(way.as_str() == "sign_up",) {
		if way.as_str() == "sign_up" {
			CONNECTION.with(|db| {
				let new_email = db.prepare("SELECT email FROM user",)?.query([],)?.count()? == 0;
				if new_email {
					Ok((),)
				} else {
					Err(ServerFnError::ServerError(format!("email is already registered"),),)
				}
			},)
		} else {
			todo!()
		}
		//		CONNECTION.with(|db| db.query_row("SELECT (name, email, password)",),)
		// todo!(
		// 	"
		// 1. if user sign upped, register
		// 2. if sign inned, verify
		// "
		// )
	} else {
		Err(ServerFnError::ServerError(format!("user input is invalid"),),)
	}
}

#[server]
pub async fn get_article(id: u64,) -> Result<u64, ServerFnError,> {
	tracing::debug!("article id is {id}");

	let cur = chrono::Local::now().to_string();
	CONNECTION.with(|db| db.execute("INSERT INTO article (date) VALUES (?1)", &[&cur,],),)?;
	Ok(id,)
}

#[server(endpoint = "example",input=GetUrl)]
async fn example() -> Result<i32, ServerFnError,> {
	tracing::debug!("enter example");
	Ok(6666,)
}

#[server]
async fn register_user(user: entity::User,) -> Result<(), ServerFnError,> {
	CONNECTION.with(|db| {
		db.execute(
			"INSERT INTO user (name, email, password, created_date, updated_date) VALUES (?1, ?2, \
			 ?3, ?4, ?5)",
			user.db(),
		)
	},)
}
