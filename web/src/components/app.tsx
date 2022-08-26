import Cookies from "js-cookie";
import * as React from "react";

import IO from "../api/io";
import Todos from "./todos/todos";

interface Props {};

enum IOState {
	Invalid,
	Unauthorized,
	Valid,
};

interface State {
	ioState: IOState;
	password: string;
};

class Application extends React.Component<Props, State> {
	io: IO = new IO({
		resource: "https://bansheerubber.com/manager"
	});

	constructor(props: Props) {
		super(props);

		this.state = {
			ioState: IOState.Invalid,
			password: "",
		};

		this.refresh();
		setInterval(this.refresh, 3 * 60 * 1000);
	}

	refresh() {
		this.io.readDatabase().then(() => {
			this.setState({
				ioState: IOState.Valid,
			});
		}).catch((code: number | null) => {
			if (code === 403) {
				this.setState({
					ioState: IOState.Unauthorized,
				});
			} else {
				this.setState({
					ioState: IOState.Invalid,
				});
			}
		});
	}
	
	override render(): JSX.Element {
		const secret = Cookies.get("secret");

		if (!secret || this.state.ioState === IOState.Unauthorized) {
			return (<div className="application">
				<div className="password">
					<input
						onChange={(event) => {
							this.setState({
								password: event.target.value,
							});
						}}
						type="text"
						value={this.state.password}
					/>
					<button
						onClick={() => {
							Cookies.set("secret", this.state.password, {
								expires: 365,
								path: '',
								secure: true,
							});
							location.reload();
						}}
					>
						Enter
					</button>
				</div>
			</div>);
		}
		
		return (<div className="application">
			<Todos database={this.state.ioState === IOState.Valid ? this.io : null} />
		</div>);
	}
};

export default Application;
