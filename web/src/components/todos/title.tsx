import * as React from "react";

interface Props {
	children: React.ReactNode;
};

class Title extends React.Component<Props> {
	constructor(props: any) {
		super(props);
	}
	
	override render(): JSX.Element {
		return (<div className="title">
			{this.props.children}
		</div>);
	}
};

export default Title;
