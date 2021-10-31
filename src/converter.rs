use std::str::FromStr;
use std::io::{Error, ErrorKind};

#[derive(Debug, PartialEq)]
pub enum Currency {
	USD,
	EUR,
	ALL,
	JPY,
	GBP,
	CAD,
	CNY,
	AUD,
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
			"USD"	=> Ok(Self::USD),
			"EUR"	=> Ok(Self::EUR),
			"ALL"	=> Ok(Self::ALL),
			"JPY"	=> Ok(Self::JPY),
			"GBP"	=> Ok(Self::GBP),
			"CAD"	=> Ok(Self::CAD),
			"CNY"	=> Ok(Self::CNY),
			"AUD"	=> Ok(Self::AUD),
			&_		=> Err(Error::new(ErrorKind::InvalidData, "invalid currency"))
		}
	}
}