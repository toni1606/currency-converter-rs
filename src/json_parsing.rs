use std::fs::File;
use std::io::Read;
use std::io::{Error, ErrorKind};
use crate::converter::Convert;

pub fn run() {
	let mut raw_file_data = match read_file(String::from("./data/data.json")) {
		Ok(v)	=> v,
		Err(e)	=> panic!("error while reading the file: {}", e)
	};

	let data = match parse_data(&mut raw_file_data) {
		Ok(v)	=> v,
		Err(e)	=> panic!("Error while parsing data: {}", e)
	};
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

	let array_identifier = "\"converts\"";
	let converts: Vec<Convert> = vec![];

	if json.chars().nth(0).unwrap() == '{' && json.contains(array_identifier) {
		let end_index = json.find(array_identifier).unwrap() + array_identifier.len();
		parse_array(&json[end_index + 1..json.len() - 1])
	} else {
		Err(Box::new(Error::new(ErrorKind::InvalidData, "invalid JSON data!")))
	}
}

fn parse_array(array: &str) -> Result<Vec<Convert>, Box<dyn std::error::Error>> {
	if array.chars().nth(0).unwrap() != '[' || array.chars().nth(array.len() - 1).unwrap() != ']' {
		return Err(Box::new(Error::new(ErrorKind::InvalidData, "invalid JSON array")));
	}
	let array: Vec<&str> = array[1..array.len() - 1].split("},").collect();
	let mut array: Vec<String> = array.iter().map(|c| String::from(*c)).collect();

	for element in &mut array {
		(*element).remove(0);

		if (*element).chars().nth(element.len() - 1).unwrap() == '}' {
			(*element).pop();
		}
	}

	Ok(vec![])
}