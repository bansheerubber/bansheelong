import RadioButtonUncheckedIcon from '@material-ui/icons/RadioButtonUnchecked';
import * as React from "react";

import { PlannedIngredient } from "../../api/io";

interface Props {
	ingredients: PlannedIngredient[];
	name: string;
};

class Ingredient extends React.Component<Props> {
	constructor(props: any) {
		super(props);
	}
	
	override render(): JSX.Element {
		const itemCount = this.props.ingredients.length == 1
			? ""
			: `(${this.props.ingredients.length})`;
		
		return (<div className="ingredient">
			<RadioButtonUncheckedIcon style={{ fontSize: 70, }} />
			<span>{this.props.name} {itemCount}</span>
		</div>);
	}
};

export default Ingredient;
