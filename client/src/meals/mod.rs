pub(crate) mod image_utils;
pub(crate) mod planner;
pub(crate) mod render;
pub(crate) mod right_panel;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use bansheelong_types::{ Date, IO, PlannedMeal };
use iced::{ button, image, scrollable };

use crate::menu::{ Menu, BUTTON_COUNT };

pub(crate) use image_utils::download_image;
pub(crate) use image_utils::has_image;
pub(crate) use image_utils::is_valid_image_url;

#[derive(Debug, Clone)]
pub enum Message {
	APIAddPlannedMeal(PlannedMeal),
	APIRemovePlannedMeal(Date),
	APIUpdatePlannedMeal(PlannedMeal),
	MenuChange(Menu),
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
struct PlannedInfo {
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
struct PlannerInfo {
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
pub(crate) enum PlannerState {
	DaySelect,
	MealSelect,
}

#[derive(Debug)]
pub(crate) struct View {
	button_states: [button::State; BUTTON_COUNT as usize],
	database: Option<Arc<IO>>,
	last_interaction: Option<Instant>,
	planned: PlannedInfo,
	planner: PlannerInfo,
	showing_planner: bool,
}
