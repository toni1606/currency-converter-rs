use std::fs::File;
use std::io::Read;
use std::io::{Error, ErrorKind};
use crate::converter::Convert;

pub fn run() {
	let mut raw_file_data = match read_file(String::from("./data/data.json")) {
		Ok(v)	=> v,
		Err(e)		=> panic!("error while reading the file: {}", e)
	};

	parse_data(&mut raw_file_data);
}

fn read_file(filename: String) -> Result<String, Box<dyn std::error::Error>> {
	let mut file_handler = File::open(filename)?;
	let mut out = String::new();

	file_handler.read_to_string(&mut out)?;
	
	Ok(out)
}

fn parse_data(json: &mut String) -> Result<Vec<Convert>, Box<dyn std::error::Error>>{
	// remove whitespaces from String
	json.retain(|c| !c.is_whitespace());
	println!("{}", json);

	let converts: Vec<Convert> = vec![];

	if json.chars().nth(0).unwrap() != '{' {
		return Err(Box::new(Error::new(ErrorKind::InvalidData, "invalid JSON data!")));
	}

	Ok(converts)
}