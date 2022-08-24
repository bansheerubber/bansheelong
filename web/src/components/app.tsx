import * as React from "react";

import IO from "../api/io";
import Todos from "./todos/todos";

interface Props {};

enum IOState {
	Invalid,
	Valid,
};

interface State {
	ioState: IOState;
};

class Application extends React.Component<Props, State> {
	io: IO = new IO({
		resource: "http://bansheerubber.com:3000"
	});

	constructor(props: Props) {
		super(props);

		this.state = {
			ioState: IOState.Invalid,
		};

		this.refresh();
		setInterval(this.refresh, 3 * 60 * 1000);
	}

	refresh() {
		this.io.readDatabase().then(() => {
			this.setState({
				ioState: IOState.Valid,
			});
		}).catch(() => {
			this.setState({
				ioState: IOState.Invalid,
			});
		});
	}
	
	override render(): JSX.Element {
		return (<div className="application">
			<Todos database={this.state.ioState === IOState.Valid ? this.io : null} />
		</div>);
	}
};

export default Application;
