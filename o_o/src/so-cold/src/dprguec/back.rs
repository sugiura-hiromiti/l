//! server functionalities

use crate::*;
use fallible_streaming_iterator::FallibleStreamingIterator;

#[cfg(feature = "server")] use rusqlite::Params;
#[cfg(feature = "server")] use rusqlite::Rows;
#[cfg(feature = "server")] use rusqlite::params_from_iter;
#[cfg(feature = "server")] use rusqlite::types::ToSql;

use server_fn::ServerFnError;
use server_fn::codec::GetUrl;

macro_rules! pass_or_exit {
	($e:expr) => {
		match $e {
			Ok(r,) => r,
			Err(e,) => return Err(ServerFnError::ServerError(e.to_string(),),),
		}
	};
	($var:ident, $e:expr) => {
		let $var = pass_or_exit!($e);
	};
	(mut $var:ident, $e:expr) => {
		let mut $var = pass_or_exit!($e);
	};
}

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
				email VARCHAR(30) NOT NULL UNIQUE,
				password VARCHAR(30) NOT NULL,
				created_date TEXT NOT NULL,
				updated_date TEXT NOT NULL,
		);",
	)
	.unwrap();

	con
};
}

//trait SerErr: std::error::Error + Sized {}
//#[cfg(feature = "server")]
//type SerErr = ServerFnError<rusqlite::Error,>;

#[cfg(feature = "server")]
mod helper {
	use super::*;
	struct ErrIR(String,);

	impl From<rusqlite::Error,> for ErrIR {
		fn from(value: rusqlite::Error,) -> Self {
			Self(value.to_string(),)
		}
	}

	impl From<ErrIR,> for ServerFnError<String,> {
		fn from(value: ErrIR,) -> Self {
			ServerFnError::WrappedServerError(value.0,)
		}
	}
}

#[server]
pub async fn verify_user(user: entity::User, way: String,) -> Result<(), ServerFnError,> {
	tracing::debug!("server: {way}");
	let mut user = user;
	if user.verify(way.as_str() == "sign_up",) {
		if way.as_str() == "sign_up" {
			let new_email = new_email(user.email.clone().unwrap(),).await?;
			CONNECTION.with(|db| {
				if new_email {
					register_user(user,)
				} else {
					Err(ServerFnError::ServerError(format!("email is already registered"),),)
				}
			},)
		} else {
			let candidates = find_user(user,)?;
			todo!("this is the case sign in")
		}
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
async fn new_email(email: String,) -> Result<bool, ServerFnError,> {
	CONNECTION.with(|db| {
		pass_or_exit!(mut stmt, db.prepare("SELECT email FROM user WHERE email = ?1"));
		pass_or_exit!(rows, stmt.query([email]));
		pass_or_exit!(count, rows.count());
		Ok(count == 0,)
	},)
}

#[server	(endpoint="all_user",input=GetUrl)]

async fn all_user() -> Result<Vec<String,>, ServerFnError,> {
	todo!()
}

#[cfg(feature = "server")]
fn register_user(user: entity::User,) -> Result<(), ServerFnError,> {
	CONNECTION.with(|db| {
		let exec_rslt = db.execute(
			"INSERT INTO user (name, email, password, created_date, updated_date) VALUES (?1, ?2, \
			 ?3, ?4, ?5)",
			user.db_insert(),
		);
		match exec_rslt {
			Ok(row_count,) => {
				assert_eq!(row_count, 1);
				Ok((),)
			},
			Err(e,) => Err(ServerFnError::ServerError(e.to_string(),),),
		}
	},)
}

#[cfg(feature = "server")]
fn find_user(user: entity::User,) -> Result<String, ServerFnError,> {
	let params: Vec<String,> = ["name", "email", "password",]
		.into_iter()
		.flat_map(|v| [v.to_string(), user.get_or_dflt(v,),],)
		.collect();
	let query = format!(
		"SELECT (?1, ?3, ?5) FROM user WHERE {}?3 = ?4 AND ?5 = ?6",
		if params[2] == "".to_string() { "" } else { "?1 = ?2 AND " }
	);

	if query_count(&query, params.as_slice().iter(),)? == 1 {
		query_then(query, params.as_slice().iter(), |mut rows| {
			pass_or_exit!(wrapped_row, rows.next());
			let row = wrapped_row.expect("all rows have been retrieved",);

			pass_or_exit!(name, row.get("name"));
			Ok(name,)
		},)
	} else {
		Err(ServerFnError::ServerError(format!("expected one user, found more"),),)
	}
}

#[cfg(feature = "server")]
fn query_then<T, P,>(
	q: impl AsRef<str,>,
	params: P,
	data_op: impl Fn(Rows,) -> Result<T, ServerFnError,>,
) -> Result<T, ServerFnError,>
where
	P: IntoIterator,
	P::Item: ToSql,
{
	CONNECTION.with(|db| {
		pass_or_exit!(mut stmt, db.prepare(q.as_ref()));
		pass_or_exit!(rows, stmt.query(params_from_iter(params)));
		data_op(rows,)
	},)
}

#[cfg(feature = "server")]
fn query_count<P,>(q: impl AsRef<str,>, params: P,) -> Result<usize, ServerFnError,>
where
	P: IntoIterator,
	P::Item: ToSql,
{
	query_then(q, params, |rows| {
		pass_or_exit!(count, rows.count());
		Ok(count,)
	},)
}
