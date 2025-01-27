use anyhow::Result as Rslt;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

pub fn run() -> Rslt<(),> {
	let listen = TcpListener::bind("127.0.0.1:8888",)?;

	while let Ok(request,) = listen.incoming().next().unwrap() {
		hndl_connect(request,)?;
	}
	Ok((),)
}

fn hndl_connect(mut stream: TcpStream,) -> Rslt<(),> {
	let reader = BufReader::new(&stream,);
	let buf = reader
		.lines()
		.map(|l| l.unwrap(),)
		.take_while(|s| !s.is_empty(),)
		.map(|s| format!("<p>{s}</p>"),)
		.collect::<Vec<String,>>()
		.join("\n",);
	crate::test_eprintln!("🫠 {buf}");

	let response_body = format!("<!doctype html><html><body>{buf}<p>日本語</p></body></html>");
	stream.write_all(response(response_body,).as_bytes(),)?;
	Ok((),)
}

fn response(body: String,) -> String {
	let header = "HTTP/1.1 200 OK";
	let len = body.len();
	crate::test_eprintln!("len is {len}");
	format!("{header}\r\nContent-Length: {len}\r\n\r\n{body}")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_server() -> Rslt<(),> {
		run()
	}

	#[test]
	fn utf8_to_hex() {
		let jp = "日本語";
		assert_eq!(jp.as_bytes(), &[0xe6, 0x97, 0xa5, 0xe6, 0x9c, 0xac, 0xe8, 0xaa, 0x9e])
	}
}
