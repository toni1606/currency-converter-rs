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

pub struct Convert {
	from: Currency,
	to: Currency,
	rate: f64
}

impl Convert {
	fn new(from: Currency, to: Currency, rate: f64) -> Convert {
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