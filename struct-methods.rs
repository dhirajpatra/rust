#[derive(Debug)]
struct Race {
	name: String,
	laps: Vec<i32>,
}

impl Race {
	fn new(name: &str) -> Race { // No receiver, a static method
		Race { name: String::new(name), laps: Vec::new() }
	}

	fn add_laps(&mut self, lap: i32) {
		self.laps.push(lap);
	}

	fn print_books(&self) { // Shared and read-only borrowed access to self
		println!("Recorded {} laps for {}:", self.laps.len(), self.name );
		for (idx, lap) in self.laps.iter().enumerate() {
			println!("Lap {idx}: {lap} sec");
		}
	}

	fn finish(self) { // Exclusive ownership of self
		let total = self.laps.iter().sum::<i32>();
		println!("Race {} is finished, total lap time: {}", self.name, total);
	}
}