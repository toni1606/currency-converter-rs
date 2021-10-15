use std::str::FromStr;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub enum Currency {
	Usd,
	Eur,
	All,
	Jpy,
	Gbp,
	Cad,
	Cny,
	Aud,
}

#[derive(Debug)]
pub struct Convert {
	from: Currency,
	to: Currency,
	rate: f64
}

impl Convert {
	pub fn new(from: Currency, to: Currency, rate: f64) -> Convert {
		Convert {
			from,
			to,
			rate
		}
	}

	fn get_from(&self) -> &Currency {
		&self.from
	}

	fn get_to(&self) -> &Currency {
		&self.to
	}

	fn get_rate(&self) -> f64 {
		self.rate
	}

	fn set_from(&mut self, from: Currency) {
		self.from = from;
	}

	fn set_to(&mut self, to: Currency) {
		self.to = to;
	}

	fn set_rate(&mut self, rate: f64) {
		self.rate = rate;
	}
}

impl FromStr for  Currency {
	type Err = std::io::Error;
	
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"Usd"	=> Ok(Self::Usd),
			"Eur"	=> Ok(Self::Eur),
			"All"	=> Ok(Self::All),
			"Jpy"	=> Ok(Self::Jpy),
			"Gbp"	=> Ok(Self::Gbp),
			"Cat"	=> Ok(Self::Cad),
			"Cny"	=> Ok(Self::Cny),
			"Aud"	=> Ok(Self::Aud),
			&_		=> Err(Error::new(ErrorKind::InvalidData, "invalid field in JSON object"))
		}
	}
}