import * as React from "react";

import Block from "./block";
import IO, { Item as IOItem } from "../../api/io";
import Item from "./item";
import Title from "./title";

interface Props {
	database: IO | null;
};

const Todos: React.FunctionComponent<Props> = (props: Props) => {
	if (props.database === null) {
		return <div className="todos"></div>;
	}

	const blocks: JSX.Element[] = [];
	for (const date in props.database.todosDatabase.mapping) { // logic stolen from rust
		const day = props.database.todosDatabase.mapping[date]!;

		const meal = props.database.mealsDatabase.plannedMealMapping[date];

		const hasTimeDay = (item: IOItem) => 
			item.time !== null && item.time.day !== null;

		let lastIndex = -1;
		let index = 0;
		for (const item of day.items) {
			if (item.description != "" && !hasTimeDay(item)) {
				lastIndex = index;
			}
			index += 1;
		}

		if (lastIndex == -1 && meal === undefined) {
			continue;
		}

		index = 0;
		blocks.push(
			<Block key={date}>
				<Title>{day.date
					? `${day.date.month}/${day.date.day}/${day.date.year}`
					: "General list"
				}</Title>
				{
					day.items.map((item, index) => {
						index += 1;

						if (index - 1 > lastIndex) {
							return null;
						}

						if (item.description.length == 0) {
							return <Item dash="-" key={index}> </Item>;
						}
						
						const startingCharacter = item.description[0]!;
						if (["-", "!", "%", "z"].includes(startingCharacter)) {
							const rest = item.description.split(/[\-!%z]/);
							rest.shift();
							return <Item dash={startingCharacter as '-' | '!' | '%' | 'z'} key={index}>{rest.join(startingCharacter).trim()}</Item>;
						} else {
							return <Item dash="-" key={index}>{item.description}</Item>;
						}
					})
				}
				{
					meal ? <Item dash="#">{meal.recipe.name}</Item> : null
				}
			</Block>
		);
	}
	
	return <div className="todos">{blocks}</div>;
};

export default Todos;
