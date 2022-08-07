use iced::{ Background, Color, container, button, scrollable, };

pub const BACKGROUND_PURPLE: Color = Color::from_rgb(
	0x38 as f32 / 255.0,
	0x26 as f32 / 255.0,
	0x3F as f32 / 255.0,
);

pub const BACKGROUND_LIGHT_PURPLE: Color = Color::from_rgb(
	0x58 as f32 / 255.0,
	0x3C as f32 / 255.0,
	0x63 as f32 / 255.0,
);

pub const BACKGROUND_DARK_PURPLE: Color = Color::from_rgb(
	0x32 as f32 / 255.0,
	0x22 as f32 / 255.0,
	0x38 as f32 / 255.0,
);

pub const BACKGROUND_DARKER_PURPLE: Color = Color::from_rgb(
	0x26 as f32 / 255.0,
	0x1A as f32 / 255.0,
	0x2B as f32 / 255.0,
);

pub const TEXT_COLOR: Color = Color::from_rgb(
	0xFF as f32 / 255.0,
	0xDD as f32 / 255.0,
	0xF3 as f32 / 255.0,
);

pub const BLUE_COLOR: Color = Color::from_rgb(
	0x8A as f32 / 255.0,
	0x76 as f32 / 255.0,
	0xE0 as f32 / 255.0,
);

pub const MAGENTA_COLOR: Color = Color::from_rgb(
	0xE0 as f32 / 255.0,
	0x59 as f32 / 255.0,
	0xE0 as f32 / 255.0,
);

pub const GREEN_COLOR: Color = Color::from_rgb(
	0x2C as f32 / 255.0,
	0xBA as f32 / 255.0,
	0x60 as f32 / 255.0,
);

pub const CYAN_COLOR: Color = Color::from_rgb(
	0x58 as f32 / 255.0,
	0xB7 as f32 / 255.0,
	0xCE as f32 / 255.0,
);

pub const TODO_COLORS: [Color; 4] = [
	BLUE_COLOR,
	MAGENTA_COLOR,
	GREEN_COLOR,
	CYAN_COLOR,
];

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
			background: None,
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

pub struct TodoCircleBlue;
impl container::StyleSheet for TodoCircleBlue {
	fn style(&self) -> container::Style {
		container::Style {
			background: Some(Background::Color(BLUE_COLOR)),
			border_radius: 100.0,
			..container::Style::default()
		}
	}
}

pub struct TodoCircleMagenta;
impl container::StyleSheet for TodoCircleMagenta {
	fn style(&self) -> container::Style {
		container::Style {
			background: Some(Background::Color(MAGENTA_COLOR)),
			border_radius: 100.0,
			..container::Style::default()
		}
	}
}

pub struct TodoCircleGreen;
impl container::StyleSheet for TodoCircleGreen {
	fn style(&self) -> container::Style {
		container::Style {
			background: Some(Background::Color(GREEN_COLOR)),
			border_radius: 100.0,
			..container::Style::default()
		}
	}
}

pub struct TodoCircleCyan;
impl container::StyleSheet for TodoCircleCyan {
	fn style(&self) -> container::Style {
		container::Style {
			background: Some(Background::Color(CYAN_COLOR)),
			border_radius: 100.0,
			..container::Style::default()
		}
	}
}

pub struct MealsDayContainer;
impl container::StyleSheet for MealsDayContainer {
	fn style(&self) -> container::Style {
		container::Style {
			background: Some(Background::Color(BACKGROUND_LIGHT_PURPLE)),
			text_color: Some(TEXT_COLOR),
			..container::Style::default()
		}
	}
}

pub struct MealsCalendarContainer;
impl container::StyleSheet for MealsCalendarContainer {
	fn style(&self) -> container::Style {
		container::Style {
			background: Some(Background::Color(BACKGROUND_DARK_PURPLE)),
			text_color: Some(TEXT_COLOR),
			..container::Style::default()
		}
	}
}
