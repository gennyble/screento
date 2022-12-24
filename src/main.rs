use std::fs::File;

use gifed::{
	block::{extension::GraphicControl, Palette},
	writer::{ImageBuilder, Writer},
	Color,
};

fn main() {
	let file = File::create("screento.gif").unwrap();
	let mut write = Writer::new(file, 256, 144, Some(grayscale())).unwrap();

	let mut buffer = vec![0u8; 256 * 144];
	for x in 0u8..=255 {
		for y in 0..144 {
			buffer[x as usize + y as usize * 256] = x;
		}

		write
			.image(
				ImageBuilder::new(256, 144)
					.delay(2)
					.build(buffer.clone())
					.unwrap(),
			)
			.unwrap();
	}

	write.done().unwrap();
}

pub fn grayscale() -> Palette {
	let mut plt = Palette::new();

	for idx in 0..=255 {
		plt.push(Color::new(idx, idx, idx));
	}

	plt
}
