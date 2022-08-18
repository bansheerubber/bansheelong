pub(crate) mod image;
pub(crate) mod render;
pub(crate) mod right_panel;

use std::sync::Arc;

use bansheelong_types::{ Date, IO, PlannedMeal };

use crate::menu::Menu;

pub(crate) use image::download_image;
pub(crate) use image::has_image;
pub(crate) use image::is_valid_image_url;
pub(crate) use render::View;

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

#[derive(Clone, Copy, Debug)]
pub(crate) enum PlannerState {
	DaySelect,
	MealSelect,
}
