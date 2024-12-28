//! server functionalities

use server_fn::codec::GetUrl;

use crate::*;

#[server]
pub async fn get_article(id: u64,) -> Result<u64, ServerFnError,> {
	tracing::debug!("article id is {id}");

	Ok(id,)
}

#[server(endpoint = "example",input=GetUrl)]
async fn example() -> Result<i32, ServerFnError,> {
	tracing::debug!("enter example");
	Ok(6666,)
}
