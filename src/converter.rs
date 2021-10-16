use std::str::FromStr;
use std::io::{Error, ErrorKind};

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

	pub fn get_from(&self) -> &Currency {
		&self.from
	}

	pub fn get_to(&self) -> &Currency {
		&self.to
	}

	pub fn get_rate(&self) -> f64 {
		self.rate
	}

	pub fn set_from(&mut self, from: Currency) {
		self.from = from;
	}

	pub fn set_to(&mut self, to: Currency) {
		self.to = to;
	}

	pub fn set_rate(&mut self, rate: f64) {
		self.rate = rate;
	}

	pub fn convert(&self, val: f64, reverse: bool) -> f64 {
		if reverse {
			val / self.rate
		} else {
			val * self.rate
		}
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
			"Cad"	=> Ok(Self::Cad),
			"Cny"	=> Ok(Self::Cny),
			"Aud"	=> Ok(Self::Aud),
			&_		=> Err(Error::new(ErrorKind::InvalidData, "invalid currency"))
		}
	}
}