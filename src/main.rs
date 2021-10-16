use currency_converter::json_parsing;

fn main() {
	match json_parsing::run("./data/data.json") {
		Ok(v)	=> println!("value: {}", v),
		Err(e)	=> panic!("{}", e)
	}
}
