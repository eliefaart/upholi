import * as React from "react";
import { IconMenu } from "./misc/Icons";

interface Props {
	items: MenuItem[]
}

interface State {
	opened: boolean;
}

interface MenuItem {
	title: string;
	active: boolean,
	onClick: () => void;
}

class Menu extends React.Component<Props, State> {

	constructor(props: Props) {
		super(props);

		this.toggleMenu = this.toggleMenu.bind(this);

		this.state = {
			opened: true
		};
	}

	toggleMenu(): void {
		this.setState(prevState => {
			return {
				opened: !prevState.opened
			};
		});
	}

	render(): React.ReactNode {
		return (
			<div className={"menu " + (this.state.opened ? "opened" : "closed")}>
				<span className="toggle" onClick={this.toggleMenu}>
					<IconMenu/>
				</span>
				<menu >
					{this.props.items.map(item => {
						return <span
							key={item.title}
							title={item.title}
							className={item.active ? "active" : ""}
							onClick={item.onClick}>
							{item.title}
						</span>;
					})}
				</menu>
			</div>
		);
	}
}

export default Menu;