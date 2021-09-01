import * as React from "react";
import AppStateContext from "../../contexts/AppStateContext";
import upholiService from "../../services/UpholiService";
import ContentContainer from "../ContentContainer";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";

interface State { }

class LoginPage extends PageBaseComponent<State> {

	usernameInput: React.RefObject<HTMLInputElement>;
	passwordInput: React.RefObject<HTMLInputElement>;

	constructor(props: PageBaseComponentProps) {
		super(props);

		this.register = this.register.bind(this);
		this.login = this.login.bind(this);

		this.usernameInput = React.createRef();
		this.passwordInput = React.createRef();

		this.state = { };
	}

	getHeaderActions(): JSX.Element | null {
		return null;
	}

	getTitle(): string {
		return "Login";
	}

	register(): void {
		if (this.usernameInput.current && this.passwordInput.current) {
			const username = this.usernameInput.current.value;
			const password = this.passwordInput.current.value;
			upholiService.register(username, password);
		}
	}

	login(): void {
		if (this.usernameInput.current && this.passwordInput.current) {
			const username = this.usernameInput.current.value;
			const password = this.passwordInput.current.value;
			upholiService.login(username, password)
				.then(() => document.location.pathname = "/");
		}
	}

	render(): React.ReactNode {
		return (
			<ContentContainer>
				<input type="text" placeholder="username" ref={this.usernameInput}/>
				<input type="password" placeholder="password" ref={this.passwordInput}/>
				<button onClick={this.register}>Register</button>
				<button onClick={this.login}>Login</button>
			</ContentContainer>
		);
	}
}

LoginPage.contextType = AppStateContext;
export default LoginPage;