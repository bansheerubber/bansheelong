use iced_native::alignment;
use iced_native::layout;
use iced_native::renderer;
use iced_native::text;
use iced_native::{ Color, Element, Layout, Length, Point, Rectangle, Size, Widget };

#[derive(Debug)]
pub struct Date<Renderer: text::Renderer> {
	content: String,
	size: Option<u16>,
	color: Option<Color>,
	font: Renderer::Font,
	width: Length,
	height: Length,
	horizontal_alignment: alignment::Horizontal,
	vertical_alignment: alignment::Vertical,
}

impl<Renderer: text::Renderer> Date<Renderer> {
	/// Create a new fragment of [`Date`] with the given contents.
	pub fn new<T: Into<String>>(label: T) -> Self {
		Date {
			content: label.into(),
			size: None,
			color: None,
			font: Default::default(),
			width: Length::Shrink,
			height: Length::Shrink,
			horizontal_alignment: alignment::Horizontal::Left,
			vertical_alignment: alignment::Vertical::Top,
		}
	}

	/// Sets the size of the [`Date`].
	pub fn size(mut self, size: u16) -> Self {
		self.size = Some(size);
		self
	}

	/// Sets the [`Color`] of the [`Date`].
	pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
		self.color = Some(color.into());
		self
	}

	/// Sets the [`Font`] of the [`Date`].
	///
	/// [`Font`]: crate::text::Renderer::Font
	pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
		self.font = font.into();
		self
	}

	/// Sets the width of the [`Date`] boundaries.
	pub fn width(mut self, width: Length) -> Self {
		self.width = width;
		self
	}

	/// Sets the height of the [`Date`] boundaries.
	pub fn height(mut self, height: Length) -> Self {
		self.height = height;
		self
	}

	/// Sets the [`alignment::Horizontal`] of the [`Date`].
	pub fn horizontal_alignment(
		mut self,
		alignment: alignment::Horizontal,
	) -> Self {
		self.horizontal_alignment = alignment;
		self
	}

	/// Sets the [`alignment::Vertical`] of the [`Date`].
	pub fn vertical_alignment(
		mut self,
		alignment: alignment::Vertical,
	) -> Self {
		self.vertical_alignment = alignment;
		self
	}
}

impl<Message, Renderer> Widget<Message, Renderer> for Date<Renderer>
where
	Renderer: text::Renderer,
{
	fn width(&self) -> Length {
		self.width
	}

	fn height(&self) -> Length {
		self.height
	}

	fn layout(
		&self,
		renderer: &Renderer,
		limits: &layout::Limits,
	) -> layout::Node {
		let limits = limits.width(self.width).height(self.height);

		let size = self.size.unwrap_or(renderer.default_size());

		let bounds = limits.max();

		let (width, height) =
			renderer.measure(&self.content, size, self.font.clone(), bounds);

		let size = limits.resolve(Size::new(width, height));

		layout::Node::new(size)
	}

	fn draw(
		&self,
		renderer: &mut Renderer,
		style: &renderer::Style,
		layout: Layout<'_>,
		_cursor_position: Point,
		_viewport: &Rectangle,
	) {
		iced_native::widget::text::draw(
			renderer,
			style,
			layout,
			&self.content,
			self.font.clone(),
			self.size,
			self.color,
			self.horizontal_alignment,
			self.vertical_alignment,
		);

		renderer.fill_quad(
			renderer::Quad {
				bounds: Rectangle {
					x: layout.bounds().x,
					y: layout.bounds().y + layout.bounds().height - 2.0,
					width: layout.bounds().width,
					height: 1.0,
				},
				border_radius: 0.0,
				border_width: 0.0,
				border_color: Color::TRANSPARENT,
			},
			self.color.unwrap_or(style.text_color),
		);
	}
}

impl<'a, Message, Renderer> From<Date<Renderer>>
	for Element<'a, Message, Renderer>
where
	Renderer: text::Renderer + 'a,
{
	fn from(text: Date<Renderer>) -> Element<'a, Message, Renderer> {
		Element::new(text)
	}
}

impl<Renderer: text::Renderer> Clone for Date<Renderer> {
	fn clone(&self) -> Self {
		Self {
			content: self.content.clone(),
			size: self.size,
			color: self.color,
			font: self.font.clone(),
			width: self.width,
			height: self.height,
			horizontal_alignment: self.horizontal_alignment,
			vertical_alignment: self.vertical_alignment,
		}
	}
}

