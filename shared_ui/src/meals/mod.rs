pub mod image_utils;
pub mod planned;
pub mod planner;
pub mod render;
pub mod right_panel;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use bansheelong_types::{ Date, IO, PlannedMeal };
use chrono::{ Datelike, Local };
use iced::{ button, image, scrollable };

use crate::constants;

pub use image_utils::download_image;
pub use image_utils::has_image;
pub use image_utils::is_valid_image_url;

#[derive(Debug, Clone)]
pub enum Message {
	APIAddPlannedMeal(PlannedMeal),
	APIRemovePlannedMeal(Date),
	APIUpdatePlannedMeal(PlannedMeal),
	MenuChange(constants::Menu),
	PlannedIngredientsScroll,
	PlannedMealsScroll(f32),
	PlannedMealSelect(Date),
	PlannerDaySelect(i8),
	PlannerMonthSelect(u8, u32),
	PlannerRecipeScroll,
	PlannerRecipeSelect(usize),
	RecipesScroll(f32),
	SwitchToPlanned,
	SwitchToPlanner,
	Tick,
	Update(Option<Arc<IO>>),
}

#[derive(Debug)]
pub struct PlannedInfo {
	image: image::Handle,
	image_state: image::viewer::State,
	ingredient_button_states: Vec<button::State>,
	ingredients_state: scrollable::State,
	mapping: HashMap<Date, PlannedMeal>,
	meal_button_states: Vec<button::State>,
	meal_index: Option<Date>,
	meals_position: f32,
	meals_state: scrollable::State,
	remove_meal_state: button::State,
	switch_planner_state: button::State,
}

#[derive(Debug)]
pub struct PlannerInfo {
	day_button_states: Vec<button::State>,
	day_index: Option<i8>,
	image: image::Handle,
	image_state: image::viewer::State,
	ingredients_state: scrollable::State,
	meal_add_state: button::State,
	month_index: u32, // starts from 0
	next_month_state: button::State,
	previous_month_state: button::State,
	recipe_button_states: Vec<button::State>,
	recipe_index: Option<usize>,
	recipes_position: f32,
	recipes_state: scrollable::State,
	state: PlannerState,
	year_index: u8, // starts at 0, 0 represents 2000
}

#[derive(Clone, Copy, Debug)]
pub enum PlannerState {
	DaySelect,
	MealSelect,
}

#[derive(Debug)]
pub struct View {
	button_states: Vec<button::State>,
	props: Props,
	database: Option<Arc<IO>>,
	empty_padding: iced::Padding,
	last_interaction: Option<Instant>,
	menu_state: constants::MenuState,
	planned: PlannedInfo,
	planner: PlannerInfo,
	showing_planner: bool,
	window_state: constants::WindowState,
}

#[derive(Clone, Copy, Debug)]
pub struct Props {
	pub calendar_day_size: u16,
	pub calendar_day_spacing: u16,
	pub calendar_day_text_size: u16,
	pub calendar_month_text_size: u16,
	pub ingredient_list_width: u16,
	pub text_size: u16,
}

impl Props {
	fn get_calendar_width(&self) -> u16 {
		7 * self.calendar_day_size + 6 * self.calendar_day_spacing
	}
}

pub fn get_scroll_position(menu_state: &constants::MenuState) -> f32 {
	let position = menu_state.get_area_size() + menu_state.button_height;

	if menu_state.button_count > 1 {
		(position + menu_state.button_spacing) as f32
	} else {
		position as f32
	}
}

pub fn get_current_month() -> u32 {
	Local::now().month() as u32 - 1
}

pub fn get_current_year() -> u8 {
	(Local::now().year() - 2000) as u8
}
