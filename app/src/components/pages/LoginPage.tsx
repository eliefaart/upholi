import * as React from "react";
import { FC } from "react";
import appStateContext from "../../contexts/AppStateContext";
import useAuthenticationStatus from "../../hooks/useAuthenticationStatus";
import { useTitle } from "../../hooks/useTitle";
import upholiService from "../../services/UpholiService";
import Content from "../layout/Content";
import ButtonBar from "../misc/ButtonBar";
import Errors from "../misc/Errors";
import { IconChevronRight } from "../misc/Icons";


const LoginPage: FC = () => {
	useTitle("Login");
	const [_, resetAuthenticationStatus] = useAuthenticationStatus();
	const context = React.useContext(appStateContext);
	const usernameInput = React.createRef<HTMLInputElement>();
	const passwordInput = React.createRef<HTMLInputElement>();
	const [errors, setErrors] = React.useState<string[]>([]);

	const createNewUser = (): void => {
		context.history.push("/register");
	};

	const login = (): void => {
		if (usernameInput.current && passwordInput.current) {
			const username = usernameInput.current.value;
			const password = passwordInput.current.value;

			if (username && password) {
				resetAuthenticationStatus();
				upholiService.login(username, password)
					.then(() => {
						context.history.push("/");
					})
					.catch(error => {
						setErrors([error ?? "Invalid credentials"]);
					});
			}
			else {
				setErrors([]);
			}
		}
	};

	return (
		<Content className="form-small">
			<h1>Log in</h1>

			<Errors errors={errors} />

			<input type="text" placeholder="username" ref={usernameInput} />
			<input type="password" placeholder="password" ref={passwordInput} />

			<ButtonBar
				right={<button className="primary" onClick={login}>Login</button>}
			/>

			<div className="register-prompt">
				<span>Don&apos;t have an account?</span>
				<button onClick={createNewUser}>Create new user<IconChevronRight /></button>
			</div>
		</Content>
	);
};

export default LoginPage;