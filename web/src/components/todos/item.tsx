import * as React from "react";

interface Props {
	children: React.ReactNode;
	dash: '-' | '!' | '%' | 'z' | '#';
};

class Item extends React.Component<Props> {
	constructor(props: any) {
		super(props);
	}
	
	override render(): JSX.Element {
		return (<div className="item">
			<div className="dash">{this.props.dash}</div>
			<div>{this.props.children}</div>
		</div>);
	}
};

export default Item;
