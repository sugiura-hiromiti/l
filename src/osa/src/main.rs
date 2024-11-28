use mylibrary::sh_cmd;
const OSA_BIN: &str = "$HOME/.local/bin/";

fn main() {
	let mut arg = std::env::args();
	arg.next();
	let xec_nam = arg.next().unwrap();
	let mut args = vec![];
	arg.for_each(|a| args.push(a,),);
	sh_cmd!(format!("{OSA_BIN}{xec_nam}"), args).unwrap();
}
