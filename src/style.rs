use iced::{ Background, Color, container, button, scrollable, };

const BACKGROUND_PURPLE: Color = Color::from_rgb(
	0x38 as f32 / 255.0,
	0x26 as f32 / 255.0,
	0x3F as f32 / 255.0,
);

const BACKGROUND_LIGHT_PURPLE: Color = Color::from_rgb(
	0x58 as f32 / 255.0,
	0x3C as f32 / 255.0,
	0x63 as f32 / 255.0,
);

const BACKGROUND_DARK_PURPLE: Color = Color::from_rgb(
	0x32 as f32 / 255.0,
	0x22 as f32 / 255.0,
	0x38 as f32 / 255.0,
);

const BACKGROUND_DARKER_PURPLE: Color = Color::from_rgb(
	0x26 as f32 / 255.0,
	0x1A as f32 / 255.0,
	0x2B as f32 / 255.0,
);

const TEXT_COLOR: Color = Color::from_rgb(
	0xFF as f32 / 255.0,
	0xDD as f32 / 255.0,
	0xF3 as f32 / 255.0,
);

pub struct Container;
impl container::StyleSheet for Container {
	fn style(&self) -> container::Style {
		container::Style {
			background: Some(Background::Color(BACKGROUND_PURPLE)),
			text_color: Some(TEXT_COLOR),
			..container::Style::default()
		}
	}
}

pub struct VerticalRule;
impl container::StyleSheet for VerticalRule {
	fn style(&self) -> container::Style {
		container::Style {
			background: Some(Background::Color(BACKGROUND_LIGHT_PURPLE)),
			text_color: Some(TEXT_COLOR),
			..container::Style::default()
		}
	}
}

pub struct WeatherContainer;
impl container::StyleSheet for WeatherContainer {
	fn style(&self) -> container::Style {
		container::Style {
			background: Some(Background::Color(BACKGROUND_PURPLE)),
			text_color: Some(TEXT_COLOR),
			..container::Style::default()
		}
	}
}

pub struct BlankWeatherContainer;
impl container::StyleSheet for BlankWeatherContainer {
	fn style(&self) -> container::Style {
		container::Style {
			background: Some(Background::Color(BACKGROUND_LIGHT_PURPLE)),
			text_color: Some(TEXT_COLOR),
			..container::Style::default()
		}
	}
}

pub struct WeatherButton;
impl button::StyleSheet for WeatherButton {
	fn active(&self) -> button::Style {
		button::Style {
			background: Some(Background::Color(BACKGROUND_PURPLE)),
			border_radius: 0.0,
			border_width: 0.0,
			text_color: TEXT_COLOR,
			..button::Style::default()
		}
	}

	fn hovered(&self) -> button::Style {
		button::Style {
			background: Some(Background::Color(BACKGROUND_PURPLE)),
			border_radius: 0.0,
			border_width: 0.0,
			text_color: TEXT_COLOR,
			..button::Style::default()
		}
	}

	fn pressed(&self) -> button::Style {
		button::Style {
			background: Some(Background::Color(BACKGROUND_PURPLE)),
			border_radius: 0.0,
			border_width: 0.0,
			text_color: TEXT_COLOR,
			..button::Style::default()
		}
	}

	fn disabled(&self) -> button::Style {
		button::Style {
			background: Some(Background::Color(BACKGROUND_PURPLE)),
			border_radius: 0.0,
			border_width: 0.0,
			text_color: TEXT_COLOR,
			..button::Style::default()
		}
	}
}

pub struct TodoItem;
impl container::StyleSheet for TodoItem {
	fn style(&self) -> container::Style {
		container::Style {
			background: Some(Background::Color(BACKGROUND_DARK_PURPLE)),
			text_color: Some(TEXT_COLOR),
			..container::Style::default()
		}
	}
}

pub struct TodoItemContainer;
impl container::StyleSheet for TodoItemContainer {
	fn style(&self) -> container::Style {
		container::Style {
			background: Some(Background::Color(BACKGROUND_PURPLE)),
			text_color: Some(TEXT_COLOR),
			..container::Style::default()
		}
	}
}

pub struct TodoScrollable;
impl scrollable::StyleSheet for TodoScrollable {
	fn active(&self) -> scrollable::Scrollbar {
		scrollable::Scrollbar {
			background: Some(Background::Color(Color::TRANSPARENT)),
			border_radius: 0.0,
			border_width: 0.0,
			border_color: Color::TRANSPARENT,
			scroller: scrollable::Scroller {
				color: BACKGROUND_DARKER_PURPLE,
				border_radius: 5.0,
				border_width: 0.0,
				border_color: Color::TRANSPARENT,
			},
		}
	}

	fn hovered(&self) -> scrollable::Scrollbar {
		let active = self.active();
		scrollable::Scrollbar {
			..active
		}
	}

	fn dragging(&self) -> scrollable::Scrollbar {
		let active = self.active();
		scrollable::Scrollbar {
			..active
		}
	}
}
