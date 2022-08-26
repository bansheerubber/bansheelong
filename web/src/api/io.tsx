import Cookies from "js-cookie";

interface Props {
	resource: string | null,
};

// todo database
export enum Weekday {
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday,
	Sunday,
};

export interface Date {
	day: number;
	month: number;
	year: number;
};

export function dateToString(date: Date | null) {
	if (date === null) {
		return "";
	} else {
		return `${date.month}/${date.day}/${date.year}`;
	}
}

export interface Time {
	day: Weekday | null;
	start_hour: number;
	start_minute: number;
	end_hour: number;
	end_minute: number;
};

export interface Item {
	description: String;
	time: Time | null;
};

export interface Day {
	date: Date | null;
	items: Item[];
};

export interface TodosDatabase {
	mapping: { [index: string]: Day }
};

// meals database
export interface Ingredient {
	name: string;
};

export interface Recipe {
	cookingSteps: string[];
	ingredients: Ingredient[];
	imageUrl: string | null
	minutes: number | null;
	name: string;
	preparationSteps: string[];
};

export interface PlannedIngredient {
	acquired: boolean;
	ingredient: Ingredient;
};

export interface PlannedMeal {
	date: Date,
	ingredients: PlannedIngredient[];
	recipe: Recipe;
};

export interface MealsDatabase {
	plannedMealMapping: { [index: string]: PlannedMeal };
	recipes: Recipe[];
};

class IO {
	mealsDatabase: MealsDatabase;
	resource: string | null = null;
	todosDatabase: TodosDatabase;

	constructor(props: Props) {
		const {
			resource,
		} = props;

		this.mealsDatabase = {
			plannedMealMapping: {},
			recipes: [],
		};
		this.resource = resource;
		this.todosDatabase = {
			mapping: {},
		};
	}

	readDatabase(): Promise<void> {
		return new Promise((resolve, reject) => {
			if (this.resource === null) {
				reject(null);
				return;
			}

			const request = new XMLHttpRequest();
			request.onreadystatechange = () => {
				if (request.readyState === XMLHttpRequest.DONE) {
					if (request.status == 200) {
						const [todos, meals] = JSON.parse(request.responseText);
	
						// handle todos
						for (const [date, day] of todos.mapping as [Date | null, Day][]) {
							this.todosDatabase.mapping[dateToString(date)] = day;
						}

						// handle meals
						const normalizeRecipe = (recipe: {
							cooking_steps: string[];
							ingredients: Ingredient[];
							image_url: string | null
							minutes: number | null;
							name: string;
							preparation_steps: string[];
						}) => ({
							cookingSteps: recipe.cooking_steps,
							ingredients: recipe.ingredients,
							imageUrl: recipe.image_url,
							minutes: recipe.minutes,
							name: recipe.name,
							preparationSteps: recipe.preparation_steps,
						});

						this.mealsDatabase.recipes = meals.recipes.map(normalizeRecipe);
						for (const [date, meal] of meals.planned_meal_mapping as [Date | null, PlannedMeal][]) {
							const normalizedMeal = {
								date: meal.date,
								ingredients: meal.ingredients,
								recipe: normalizeRecipe(meal.recipe as any),
							};

							this.mealsDatabase.plannedMealMapping[dateToString(date)] = normalizedMeal;
						}

						resolve();
					} else {
						reject(request.status);
					}
				}
			};

			const secret = Cookies.get("secret");
			request.onerror = reject;
			request.open("GET", `${this.resource}/get-database/`);
			request.setRequestHeader("Secret", secret ? secret : "");
			request.send();
		});
	}
};

export default IO;
