#[derive(Debug)]
pub struct MovingAverage {
	/// which index is the next to be replaced
	index: usize,
	list: [f32; Self::NUM],
	new: bool,
}

impl MovingAverage {
	/// how many numbers are kept in the moving average
	const NUM: usize = 10;

	pub fn new() -> Self {
		Self {
			index: 0,
			list: [0.0; Self::NUM],
			new: false
		}
	}

	pub fn insert(&mut self, x: f32) {
		if self.new {
			self.list = [x; Self::NUM];
		} else {
			self.list[self.index] = x;
			self.index += 1;
			if self.index >= self.list.len() {
				self.index = 0;
			}
		}
	}

	pub fn insert_i32(&mut self, x: i32) {
		self.insert(x as f32);
	}

	pub fn avg(&self) -> f32 {
		let sum: f32 = self.list.iter().sum();
		sum / (self.list.len() as f32)
	}
}