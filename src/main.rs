use std::{borrow::Borrow, fs::File};

use fontster::{
	parse_font_file, Font, HorizontalAlign, Layout, LayoutSettings, LineHeight, StyledText,
};
use gifed::{
	block::{LoopCount, Palette},
	writer::{ImageBuilder, Writer},
	Color,
};

fn main() {
	let font = parse_font_file("Instruction.otf").unwrap();

	let size = 64.0;
	let widest = widest(&font, size);

	let width = widest.width as f32 * 10.0;
	let height = widest.height as f32;

	let width = width.ceil() as usize;
	let height = height.ceil() as usize;

	let mut numbers = vec![];
	for n in 0u8..=9 {
		let (metrics, raster) = font.rasterize((48 + n) as char, size);
		let dx = widest.width - metrics.width;
		let dy = widest.height - metrics.height;

		let mut n_buffer = vec![0u8; widest.width * widest.height];
		for y in dy..widest.height {
			let line_start = dx / 2 + (y * widest.width);
			let line = &mut n_buffer[line_start..line_start + metrics.width];
			let raster_line_start = (y - dy) * metrics.width;
			line.copy_from_slice(&raster[raster_line_start..raster_line_start + metrics.width]);
		}
		numbers.push(n_buffer);
	}

	let file = File::create("0123456789.gif").unwrap();
	let mut write = Writer::new(file, width as u16, height as u16, Some(grayscale())).unwrap();

	write.repeat(LoopCount::Forever).unwrap();

	write
		.image(
			ImageBuilder::new(width as u16, height as u16)
				.build(vec![0; width * height])
				.unwrap(),
		)
		.unwrap();

	for (idx, img) in numbers.into_iter().enumerate() {
		write
			.image(
				ImageBuilder::new(widest.width as u16, widest.height as u16)
					.offset((widest.width * idx) as u16, 0)
					.delay(25)
					.build(img)
					.unwrap(),
			)
			.unwrap();
	}

	for idx in (0..=9).rev() {
		write
			.image(
				ImageBuilder::new(widest.width as u16, widest.height as u16)
					.offset((widest.width * idx) as u16, 0)
					.delay(25)
					.build(vec![0; widest.width * widest.height])
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

fn widest(font: &Font, size: f32) -> Widest {
	let mut widest = Widest {
		wide_number: 0,
		width: 0,
		tall_number: 0,
		height: 0,
	};

	for idx in 0u8..=9 {
		let metrics = font.metrics((48 + idx) as char, size);

		if metrics.width > widest.width {
			widest.wide_number = idx;
			widest.width = metrics.width;
		}
		if metrics.height > widest.height {
			widest.tall_number = idx;
			widest.height = metrics.height;
		}
	}

	widest
}

pub struct Widest {
	pub wide_number: u8,
	pub width: usize,
	pub tall_number: u8,
	pub height: usize,
}
