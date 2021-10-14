use std::fs::File;
use std::io::Read;

pub fn run() {
	let raw_file_data = match read_file(String::from("./data/data.json")) {
		Ok(v)	=> v,
		Err(e)		=> panic!("error while reading the file: {}", e)
	};


}

fn read_file(filename: String) -> Result<String, Box<dyn std::error::Error>> {
	let mut file_handler = File::open(filename)?;
	let mut out = String::new();

	file_handler.read_to_string(&mut out);
	
	Ok(out)
}

fn parse_data(json: String) {
}