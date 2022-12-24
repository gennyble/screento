use std::{borrow::Borrow, fs::File};

use fontster::{
	parse_font_file, Font, HorizontalAlign, Layout, LayoutSettings, LineHeight, StyledText,
};
use gifed::{
	block::Palette,
	writer::{ImageBuilder, Writer},
	Color,
};

fn main() {
	let font = parse_font_file("Instruction.otf").unwrap();
	let mut layout: Layout<()> = Layout::new(LayoutSettings {
		horizontal_align: HorizontalAlign::Center,
		line_height: LineHeight::Smallest(0.0),
	});

	let size = 64.0;
	let widest = widest(&font, size);

	let x_metrics = font.metrics('X', size);
	let m_metrics = font.metrics('M', size);

	println!(
		"Widest was {} at {}px\nTallest was {} at {}px\nX is {}x{}\nM is {}x{}",
		widest.wide_number,
		widest.width,
		widest.tall_number,
		widest.height,
		x_metrics.width,
		x_metrics.height,
		m_metrics.width,
		m_metrics.height
	);

	// The spacing between letters
	let spacing = widest.width as f32 / 4.0;
	// The spacing between groups. Like in 1 000 000 the groups are 1, 000, and 000
	let group_padding = spacing * 2.0;
	// The padding we put around the edges of the image
	let edge_padding = spacing / 2.0;

	// In the number 1_000_000 there are three groups.
	// So we only beed 2 group_padding.
	// In all the groups, there is only ever two characters directly next
	// to one another four times. 4 spacing.
	let width = widest.width as f32 * 10.0; // + spacing * 4.0 + group_padding * 2.0 + edge_padding;
	let height = widest.height as f32 + edge_padding;

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

	let file = File::create("million.gif").unwrap();
	let mut write = Writer::new(file, width as u16, height as u16, Some(grayscale())).unwrap();

	for (idx, img) in numbers.into_iter().enumerate() {
		write
			.image(
				ImageBuilder::new(widest.width as u16, widest.height as u16)
					.offset((widest.width * idx) as u16, 0)
					.build(img)
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
