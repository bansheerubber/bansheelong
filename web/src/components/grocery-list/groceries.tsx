import * as React from "react";

import IO, { PlannedIngredient, PlannedMeal } from "../../api/io";
import Ingredient from "./ingredient";

interface Props {
	database: IO | null;
};

const Groceries: React.FunctionComponent<Props> = (props: Props) => {
	if (props.database === null) {
		return <div className="groceries"></div>;
	}

	const meals = props.database.mealsDatabase.plannedMealMapping;
	const ingredientsToMeal: {
		[index: string]: PlannedMeal[]
	} = {};
	const ingredients: {
		[index: string]: [string, PlannedIngredient[]]
	} = {};

	for (const [_, meal] of Object.entries(meals)) {
		for (const ingredient of meal.ingredients) {
			ingredientsToMeal[ingredient.ingredient.name] = ingredientsToMeal[ingredient.ingredient.name] === undefined
				? [meal]
				: [meal, ...ingredientsToMeal[ingredient.ingredient.name]!];
			
			ingredients[ingredient.ingredient.name] = ingredients[ingredient.ingredient.name] === undefined
				? [ingredient.ingredient.name, [ingredient]]
				: [ingredient.ingredient.name, [ingredient, ...ingredients[ingredient.ingredient.name]![1]]];
		}
	}

	const items = [];
	for (const [_, [name, ings]] of Object.entries(ingredients)) {
		items.push(<Ingredient ingredients={ings} name={name} />);
	}

	return (<div className="groceries">
		<div className="inner">
			{items}
		</div>
	</div>);
};

export default Groceries;
