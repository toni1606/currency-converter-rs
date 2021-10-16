use std::fs::File;
use std::io::Read;
use std::io::{Error, ErrorKind, stdin};

use crate::converter::{Convert, Currency};
use substring::Substring;

pub fn run() {
    let mut raw_file_data = match read_file(String::from("./data/data.json")) {
        Ok(v)	=> v,
        Err(e)	=> panic!("Error while reading the file: {}", e)
    };

    let data = match parse_data(&mut raw_file_data) {
        Ok(v)	=> v,
        Err(e)	=> panic!("Error while parsing data: {}", e)
    };

    // TODO: read data from stdin -> find neeeded Convert and convert into currency if not return Err()
    let convert = match read_data() {
        Ok(v)   => v,
        Err(e)  => panic!("Error while reading from stdin: {}", e)
    };

    println!("{:?}", convert);
}

fn read_data() -> Result<Convert, Box<dyn std::error::Error>> {
    let mut convert = Convert::new(Currency::All, Currency::All, 1.0);
    
    let mut raw_data = String::new();
    println!("To: ");
    stdin().read_line(&mut raw_data)?;
    convert.set_to(raw_data.trim().parse()?);

    raw_data = String::new();
    println!("From: ");
    stdin().read_line(&mut raw_data)?;
    convert.set_from(raw_data.trim().parse()?);

    Ok(convert)
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

    // remove '[' and ']' + split objects into a Vec<String>
    let mut array: Vec<String> = array[1..array.len() - 1].split("},")
                                                          .collect::<Vec<&str>>()
                                                          .iter()
                                                          .map(|c| String::from(*c))
                                                          .collect();

    // remove '{' and '}' if present
    for element in &mut array {
        (*element).remove(0);

        if (*element).chars().nth(element.len() - 1).unwrap() == '}' {
            (*element).pop();
        }
    }

    let mut converters: Vec<Convert> = vec![];
    for element in array {
        // spltit the associative array into fields
        let fields = element.split(",");

        // temporary storage for the Convert::new() method
        let mut to = Currency::All;
        let mut from = Currency::All;
        let mut rate = 0f64;

        for field in fields {
            // find the delimiter like "example":
            let end_of_name = field[1..].find(":").unwrap();
            let identifier = field.substring(
                                                field.find("\"").unwrap(),
                                                end_of_name + 1
                                            );	// identifier like "example" 
            let mut value = field.substring(
                                                field.find("\"").unwrap() + end_of_name + 2,
                                                field.len()
                                           );	// value like "All" or 125

            // remove '"' at beginning and end if present in value 
            if value.chars().nth(0).unwrap() == '\"' {
                value = &value[1..value.len() - 1];
            }

            // call parsing methods
            match identifier {
                "\"to\""	=> to = String::from(value).parse()?,
                "\"from\""	=> from = String::from(value).parse()?,
                "\"rate\""	=> rate = String::from(value).parse()?,
                &_			=> return Err(Box::new(Error::new(ErrorKind::InvalidData, "invalid field in JSON object"))) 
            }
        }

        converters.push(Convert::new(from, to, rate));
    }

    Ok(converters)
}