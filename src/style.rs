use iced::{text_input, Background, Color, container, scrollable};

pub struct Container;

const BACKGROUND_PURPLE: Color = Color::from_rgb(
	0x38 as f32 / 255.0,
	0x26 as f32 / 255.0,
	0x3F as f32 / 255.0,
);

const TEXT_COLOR: Color = Color::from_rgb(
	0xFF as f32 / 255.0,
	0xDD as f32 / 255.0,
	0xF3 as f32 / 255.0,
);

impl container::StyleSheet for Container {
	fn style(&self) -> container::Style {
		container::Style {
			background: Some(Background::Color(BACKGROUND_PURPLE)),
			text_color: Some(TEXT_COLOR),
			..container::Style::default()
		}
	}
}
