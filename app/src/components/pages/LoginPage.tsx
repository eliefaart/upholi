import * as React from "react";
import { FC } from "react";
import appStateContext from "../../contexts/AppStateContext";
import { useTitle } from "../../hooks/useTitle";
import upholiService from "../../services/UpholiService";
import Content from "../layout/Content";


const LoginPage: FC = () => {
	useTitle("Login");

	const context = React.useContext(appStateContext);
	const usernameInput = React.createRef<HTMLInputElement>();
	const passwordInput = React.createRef<HTMLInputElement>();

	const register = (): void => {
		if (usernameInput.current && passwordInput.current) {
			const username = usernameInput.current.value;
			const password = passwordInput.current.value;
			upholiService.register(username, password);
		}
	};

	const login = (): void => {
		if (usernameInput.current && passwordInput.current) {
			const username = usernameInput.current.value;
			const password = passwordInput.current.value;
			upholiService.login(username, password)
				.then(() => {
					context.history.push("/");
				});
		}
	};

	return (
		<Content>
			<input type="text" placeholder="username" ref={usernameInput}/>
			<input type="password" placeholder="password" ref={passwordInput}/>
			<button onClick={register}>Register</button>
			<button onClick={login}>Login</button>
		</Content>
	);
};

export default LoginPage;