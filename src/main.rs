extern crate clap;

use clap::{Arg, App};

mod util;
mod day1;

fn main() {
	let matches = App::new("Advent of Rust 2018")
		.author("Mitchell Johnson <ehntoo@gmail.com>")
		.arg(Arg::with_name("day")
			.required(true)
			.help("Day of the advent calendar")
			.validator(|str|
				str.parse::<u32>()
					.or(Err("day must be an integer".to_owned()))
					.and_then(|v| match v {
						1...25 => Ok(()),
						_ => Err("day must be between 1 and 25".to_owned())
					})))
		.get_matches();

	match matches.value_of("day").unwrap().parse::<u32>().unwrap() {
		1 => day1::solve(),
		_ => ()
	}
}
