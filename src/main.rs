use std::fs::File;

use fontster::{parse_font_file, HorizontalAlign, Layout, LayoutSettings, LineHeight, StyledText};
use gifed::{
	block::Palette,
	writer::{ImageBuilder, Writer},
	Color,
};

fn main() {
	let font = parse_font_file("LTCarpet.ttf").unwrap();
	let mut layout: Layout<()> = Layout::new(LayoutSettings {
		horizontal_align: HorizontalAlign::Center,
		line_height: LineHeight::Smallest(0.0),
	});
	layout.append(
		&[&font],
		StyledText {
			text: "screen to\nscreen to\nscreen to\nscreen to",
			font_size: 64.0,
			font_index: 0,
			user: (),
		},
	);

	let padding = layout.height() / 2.0;
	let half_pad = (padding / 2.0) as u16;
	let width = layout.width() + padding;
	let height = layout.height() + padding;

	let file = File::create("screento.gif").unwrap();
	let mut write = Writer::new(file, width as u16, height as u16, Some(grayscale())).unwrap();

	let glyphs = layout.glyphs();
	let mut last_space = false;
	for glyph in glyphs {
		if glyph.c.is_whitespace() {
			last_space = true;
			continue;
		}

		let delay = if last_space { 20 } else { 10 };

		let (_, raster) = font.rasterize(glyph.c, glyph.font_size);
		write
			.image(
				ImageBuilder::new(glyph.width as u16, glyph.height as u16)
					.offset(
						glyph.x.round() as u16 + half_pad,
						glyph.y.round() as u16 + half_pad,
					)
					.delay(delay)
					.build(raster)
					.unwrap(),
			)
			.unwrap();

		last_space = false;
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
