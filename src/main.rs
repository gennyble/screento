mod count;

use std::{borrow::Borrow, fs::File, time::Instant};

use fontster::{
	parse_font_file, Font, HorizontalAlign, Layout, LayoutSettings, LineHeight, StyledText,
};
use gifed::{
	block::{
		extension::{DisposalMethod, GraphicControl},
		Block, Palette,
	},
	writer::{ImageBuilder, Writer},
	Color,
};

use crate::count::{Count, Number};

fn main() {
	let font = parse_font_file("Instruction.otf").unwrap();
	let mut layout: Layout<()> = Layout::new(LayoutSettings {
		horizontal_align: HorizontalAlign::Center,
		line_height: LineHeight::Smallest(0.0),
	});

	let all_start = Instant::now();

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
	// The padding we put around the edges of the image
	let edge_padding = spacing * 2.0;
	// The spacing between groups. Like in 1 000 000 the groups are 1, 000, and 000
	let group_padding = spacing * 2.0;

	// In the number 1_000_000 there are three groups.
	// So we only beed 2 group_padding.
	// In all the groups, there is only ever two characters directly next
	// to one another four times. 4 spacing.
	let width =
		widest.width as f32 * 7.0 + spacing * 4.0 + group_padding * 2.0 + edge_padding * 2.0;
	let height = widest.height as f32 + edge_padding * 2.0;

	// Could I make this automated and work with infinetly large numbers? Probably.
	// Do I want to do that right now? No.

	// Width from the left, no padding
	let wnp = width - edge_padding;
	let gw = widest.width as f32;
	// Half glyph width
	let hgw = widest.width as f32 / 2.0;

	// Positions ones, tenths, hundreds
	let positions = vec![
		wnp - hgw,
		wnp - gw - spacing - hgw,
		wnp - gw * 2.0 - spacing * 2.0 - hgw,
		wnp - gw * 3.0 - spacing * 2.0 - group_padding - hgw,
		wnp - gw * 4.0 - spacing * 3.0 - group_padding - hgw,
		wnp - gw * 5.0 - spacing * 4.0 - group_padding - hgw,
		wnp - gw * 6.0 - spacing * 4.0 - group_padding * 2.0 - hgw,
	];

	let mut count = Count::new(positions);

	let width = width.ceil() as usize;
	let height = height.ceil() as usize;

	// The below code- the very, very messy code- does this:
	// get the raster from fontster (really from fontdue, but fontster rexports) and stuff
	// it into a WidestXTallest buffer. The glyph is horizontally centered and vertically
	// bottom-anchored
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

	let file = File::create("1000.gif").unwrap();
	let mut write = Writer::new(file, width as u16, height as u16, Some(grayscale())).unwrap();

	/*write
	.block(Block::CommentExtension(
		String::from("by gennyble").as_bytes().to_vec(),
	))
	.unwrap();*/

	let gif_start = Instant::now();
	for _ in 0..1_000_000 {
		let count_numbers = count.next();

		let (last, count_numbers) = count_numbers.split_last().unwrap();
		for number in count_numbers {
			let left = (number.position - hgw) as u16;
			let raster = numbers[number.number as usize].clone();
			write
				.image(
					ImageBuilder::new(widest.width as u16, widest.height as u16)
						.offset(left, edge_padding as u16)
						.build(raster)
						.unwrap(),
				)
				.unwrap()
		}

		let left = (last.position - hgw) as u16;
		let raster = numbers[last.number as usize].clone();
		write
			.image(
				ImageBuilder::new(widest.width as u16, widest.height as u16)
					.delay(100)
					.offset(left, edge_padding as u16)
					.build(raster)
					.unwrap(),
			)
			.unwrap()
	}
	println!(
		"Gif took {}ms\nEverything took {}ms",
		gif_start.elapsed().as_millis(),
		all_start.elapsed().as_millis()
	);

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
