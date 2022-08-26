import Cookies from "js-cookie";
import * as React from "react";

import Groceries from "./grocery-list/groceries";
import IO from "../api/io";
import Todos from "./todos/todos";

interface Props {};

type MenuOption = "Todo manager" | "Grocery Manager";

enum IOState {
	Invalid,
	Unauthorized,
	Valid,
};

interface State {
	ioState: IOState;
	password: string;
	selectedMenu: MenuOption;
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
			selectedMenu: "Todo manager",
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

		const menuItems: [MenuOption, JSX.Element][] = [
			["Todo manager", <Todos database={this.state.ioState === IOState.Valid ? this.io : null} />],
			["Grocery Manager", <Groceries database={this.state.ioState === IOState.Valid ? this.io : null} />],
		];

		const element = menuItems.find(([menuName, _]) => {
			if (this.state.selectedMenu === menuName) {
				return true;
			} else {
				return false;
			}
		})![1];

		const buttons = menuItems.map(([menuName, _]) => {
			if (this.state.selectedMenu === menuName) {
				return null;
			} else {
				return (
					<button
						className="menu-button"
						onClick={() => {
							this.setState({
								selectedMenu: menuName,
							});
						}}
					>
						{menuName}
					</button>
				);
			}
		});

		return (<div className="application">
			<div className="menu">
				{buttons}
			</div>
			{element}
		</div>);
	}
};

export default Application;
