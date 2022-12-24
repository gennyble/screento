pub struct Count {
	// The center of
	pub positions: Vec<f32>,
	pub current: usize,
	pub digits: Vec<u8>,
}

impl Count {
	pub fn new(positions: Vec<f32>) -> Self {
		Self {
			positions,
			current: 0,
			digits: vec![0],
		}
	}

	pub fn next(&mut self) -> Vec<Number> {
		let next = self.current + 1;

		let mut numbers = vec![];
		let digits = Self::digits(next);
		for (idx, digit) in digits.iter().enumerate() {
			let position = self.positions[idx];

			match self.digits.get(idx) {
				None => {
					numbers.push(Number {
						number: *digit,
						position,
					});
				}
				Some(num) if *num != *digit => {
					numbers.push(Number {
						number: *digit,
						position,
					});
				}
				_ => (),
			}
		}

		self.current = next;
		self.digits = digits;

		numbers
	}

	pub fn digits(mut n: usize) -> Vec<u8> {
		if n == 0 {
			return vec![0];
		}

		let mut ret = vec![];
		loop {
			if n == 0 {
				break ret;
			} else {
				ret.push((n % 10) as u8);
				n = n / 10;
			}
		}
	}
}

pub struct Number {
	pub number: u8,
	pub position: f32,
}
