#[cfg(test)]
mod tests {
	//use super::*;
	use anyhow::Result as Rslt;

	#[test]
	fn moku() -> Rslt<(),> {
		let moku = String::from_utf8(vec![
			227, 130, 130, 227, 129, 143, 227, 130, 130, 227, 129, 143, 227, 129, 151, 227, 129,
			190, 227, 129, 153,
		],)?;
		assert_eq!("もくもくします", moku);

		let my_fav_lang = unsafe {
			String::from_utf8_unchecked(vec![
				82, 117, 115, 116, 227, 129, 168, 108, 117, 97, 227, 129, 140, 229, 165, 189, 227,
				129, 141, 227, 129, 167, 227, 130, 136, 227, 129, 143, 232, 167, 166, 227, 129,
				163, 227, 129, 166, 227, 129, 132, 227, 129, 190, 227, 129, 153, 32, 229, 184, 131,
				230, 149, 153, 227, 129, 151, 227, 129, 159, 227, 129, 132,
			],)
		};
		assert_eq!("Rustとluaが好きでよく触っています 布教したい", my_fav_lang);
		Ok((),)
	}

	#[test]
	fn sqlite() -> Rslt<(),> {
		let connect = rusqlite::Connection::open_in_memory()?;
		connect.execute_batch(
			"create table if not exists articles (
				id integer primary key,
				date text not null
		);",
		)?;

		let should_one = connect.execute(
			"insert into articles (date) values (?1)",
			&[&chrono::Local::now().timestamp(),],
		)?;

		assert_eq!(1, should_one);

		Ok((),)
	}
}
