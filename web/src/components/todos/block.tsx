import * as React from "react";

interface Props {
	children: React.ReactNode[];
};

const Block: React.FunctionComponent<Props> = (props) => (
	<div className="block">
		{props.children}
	</div>
);

export default Block;
