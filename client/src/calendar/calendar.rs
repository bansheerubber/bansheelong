use std::sync::Arc;

use chrono::{ Datelike, Local, Timelike };

use iced_native::alignment;
use iced_native::layout;
use iced_native::renderer;
use iced_native::text;
use iced_native::{ Background, Color, Element, Layout, Length, Point, Rectangle, Size, Vector, Widget };

use bansheelong_types::{ Date, IO };

use crate::style::{ BACKGROUND_DARK_PURPLE, BACKGROUND_LIGHT_PURPLE, TODO_COLORS };

type HourMinute = i32;

pub static START_TIME: HourMinute = 1; // time range: 1-24 hours
pub static END_TIME: HourMinute = 24;
pub static TEXT_MARGIN: Vector = Vector::new(5.0, -4.0);
pub static TEXT_SPACING: Vector = Vector::new(0.0, 50.0);
pub static ITEM_MARGIN_LEFT: Vector = Vector::new(30.0, 0.0);
pub static ITEM_MARGIN_RIGHT: Vector = Vector::new(15.0, 0.0);
pub static ITEM_PADDING_LEFT_BOTTOM: Vector = Vector::new(5.0, 0.0);
pub static ITEM_PADDING_RIGHT_TOP: Vector = Vector::new(5.0, 0.0);
pub static Y_OFFSET: f32 = 5.0;

#[derive(Debug)]
pub struct Calendar<Renderer: text::Renderer> {
	font: Renderer::Font,
	item_size: Option<u16>,
	size: Option<u16>,
	todos: Option<Arc<IO>>,
	width: Length,
}

impl<Renderer: text::Renderer> Calendar<Renderer> {
	pub fn new(io: Option<Arc<IO>>) -> Self {
		Self {
			font: Default::default(),
			item_size: None,
			size: None,
			todos: io,
			width: Length::Shrink,
		}
	}

	pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
		self.font = font.into();
		self
	}

	pub fn font_size(mut self, size: u16) -> Self {
		self.size = Some(size);
		self
	}

	pub fn item_font_size(mut self, size: u16) -> Self {
		self.item_size = Some(size);
		self
	}

	pub fn width(mut self, width: Length) -> Self {
		self.width = width;
		self
	}
}

impl<Message, Renderer> Widget<Message, Renderer> for Calendar<Renderer>
where
	Renderer: text::Renderer,
{
	fn width(&self) -> Length {
		self.width
	}

	fn height(&self) -> Length {
		Length::Units((END_TIME - START_TIME + 1) as u16 * TEXT_SPACING.y as u16 + Y_OFFSET as u16)
	}

	fn layout(
		&self,
		_renderer: &Renderer,
		limits: &layout::Limits
	) -> layout::Node {
		let height = (END_TIME - START_TIME + 1) as u16 * TEXT_SPACING.y as u16 + Y_OFFSET as u16;
		let size = limits.width(self.width).height(Length::Units(height)).resolve(Size::ZERO);
		layout::Node::new(Size::new(size.width, size.height))
	}

	fn draw(
		&self,
		renderer: &mut Renderer,
		style: &renderer::Style,
		layout: Layout<'_>,
		_cursor_position: Point,
		_viewport: &Rectangle
	) {
		let time_to_position = |hour: HourMinute, minute: HourMinute| {
			Vector::new(0.0, TEXT_SPACING.y * (hour - START_TIME) as f32 + TEXT_SPACING.y * (minute as f32 / 60.0))
		};

		let time = Local::now();
		let current_hour = time.hour() as HourMinute;
		let current_minute = time.minute() as HourMinute;
		let day = time.day() as u8;
		let month = time.month() as u8;
		let year = (time.year() % 100) as u8;

		// draw background
		renderer.fill_quad(
			iced_native::renderer::Quad {
				bounds: Rectangle {
					x: layout.bounds().x,
					y: layout.bounds().y,
					width: layout.bounds().width,
					height: layout.bounds().height,
				},
				border_radius: 0.0,
				border_width: 0.0,
				border_color: Color::TRANSPARENT,
			},
			Background::Color(BACKGROUND_DARK_PURPLE)
		);

		// draw text
		{
			for i in START_TIME..=END_TIME {
				let string = if i == 24 {
					String::from("12")
				} else if i > 12 {
					(i % 12).to_string()
				} else {
					i.to_string()
				};

				let size = self.size.unwrap_or(renderer.default_size());
				let (width, height) =
					renderer.measure(&string, size, self.font.clone(), Size::new(50.0, 50.0));
				
				let text_node = &layout::Node::new(Size::new(width, height));
				let text_layout = layout::Layout::with_offset(
					Vector::new(
						layout.position().x + TEXT_MARGIN.x,
						layout.position().y + TEXT_MARGIN.y + Y_OFFSET
					) + time_to_position(i, 0),
					&text_node
				);

				iced_native::widget::text::draw(
					renderer,
					style,
					text_layout,
					&string,
					self.font.clone(),
					self.size,
					None,
					alignment::Horizontal::Left,
					alignment::Vertical::Bottom
				);
			}
		}

		let current_date = Date {
			day,
			month,
			year
		};

		if self.todos.is_some() && self.todos.as_ref().unwrap().database.mapping.get(&Some(current_date)).is_some() {
			let items = &self.todos.as_ref().unwrap().database.mapping.get(&Some(current_date)).unwrap().items;
			
			for item in items {
				let time = if item.time.is_none() {
					continue;
				} else {
					item.time.unwrap()
				};

				let start_time = time_to_position(time.start_hour as HourMinute, time.start_minute as HourMinute) + ITEM_MARGIN_LEFT;
				let end_time = time_to_position(time.end_hour as HourMinute, time.end_minute as HourMinute) + ITEM_MARGIN_LEFT;
				let item_width = layout.bounds().width - ITEM_MARGIN_LEFT.x - ITEM_MARGIN_RIGHT.x;
				let item_height = end_time.y - start_time.y;
				renderer.fill_quad(
					iced_native::renderer::Quad {
						bounds: Rectangle {
							x: layout.bounds().x + start_time.x,
							y: layout.bounds().y + start_time.y + Y_OFFSET,
							width: item_width,
							height: item_height,
						},
						border_radius: 0.0,
						border_width: 0.0,
						border_color: Color::TRANSPARENT,
					},
					Background::Color(TODO_COLORS[0])
				);

				// figure out string truncation
				let size = self.item_size.unwrap_or(renderer.default_size());
				let mut string = item.description.clone().replace("- ", "");
				let mut string_width = 0.0;
				let mut string_height = 0.0;

				let mut collector = String::from(string.chars().nth(0).unwrap());
				for i in 1..string.len() {
					let truncated = collector.clone() + &String::from("...");
					let (width, height) = renderer.measure(
						&truncated,
						size,
						self.font.clone(),
						Size::new(
							item_width - ITEM_PADDING_RIGHT_TOP.x,
							item_height * 2.0
						)
					);
					
					if height > item_height - ITEM_PADDING_LEFT_BOTTOM.y {
						collector.pop();
						string = collector.clone() + &String::from("...");
						break;
					}

					string_width = width;
					string_height = height;
					
					collector.extend(string.chars().nth(i));
				}
				
				// draw the truncated text
				let text_node = &layout::Node::new(Size::new(string_width, string_height));
				let text_layout = layout::Layout::with_offset(
					Vector::new(
						layout.bounds().x + start_time.x + ITEM_PADDING_LEFT_BOTTOM.x,
						layout.bounds().y + start_time.y + ITEM_PADDING_RIGHT_TOP.y + Y_OFFSET
					),
					&text_node
				);

				iced_native::widget::text::draw(
					renderer,
					style,
					text_layout,
					&string,
					self.font.clone(),
					self.item_size,
					None,
					alignment::Horizontal::Left,
					alignment::Vertical::Top
				);
			}
		}

		// time line
		renderer.fill_quad(
			iced_native::renderer::Quad {
				bounds: Rectangle {
					x: layout.bounds().x + time_to_position(current_hour, current_minute).x,
					y: layout.bounds().y + time_to_position(current_hour, current_minute).y + Y_OFFSET,
					width: layout.bounds().width,
					height: 1.0,
				},
				border_radius: 0.0,
				border_width: 0.0,
				border_color: Color::TRANSPARENT,
			},
			Background::Color(BACKGROUND_LIGHT_PURPLE)
		);
	}
}

impl<'a, Message, Renderer> From<Calendar<Renderer>>
	for Element<'a, Message, Renderer>
where
	Renderer: text::Renderer + 'a,
{
	fn from(
		column: Calendar<Renderer>,
	) -> Element<'a, Message, Renderer> {
		Element::new(column)
	}
}

