import * as React from "react";
import { FC } from "react";
import { toast } from "react-toastify";
import appStateContext from "../../contexts/AppStateContext";
import { useTitle } from "../../hooks/useTitle";
import upholiService from "../../services/UpholiService";
import Content from "../layout/Content";
import ButtonBar from "../misc/ButtonBar";
import Errors from "../misc/Errors";
import { IconChevronLeft } from "../misc/Icons";

const RegisterPage: FC = () => {
	useTitle("Register");

	const context = React.useContext(appStateContext);
	const usernameInput = React.createRef<HTMLInputElement>();
	const passwordInput = React.createRef<HTMLInputElement>();
	const confirmPasswordInput = React.createRef<HTMLInputElement>();
	const [errors, setErrors] = React.useState<string[]>([]);

	const toLoginPage = (): void => {
		context.history.push("/login");
	};

	const create = (): void => {
		setErrors([]);

		if (usernameInput.current && passwordInput.current && confirmPasswordInput.current) {
			const username = usernameInput.current.value;
			const password = passwordInput.current.value;
			const confirmPassword = confirmPasswordInput.current.value;

			if (password !== confirmPassword) {
				setErrors([
					"Passwords do not match"
				]);
			}
			else {
				upholiService.register(username, password)
					.then(() => {
						toast.info("User created, you may now log in.");
						toLoginPage();
					})
					.catch((error) => {
						console.log(error);
						setErrors([
							"Error creating user"
						]);
					});
			}
		}
	};

	return (
		<Content className="form-small">
			<h1>Create new user</h1>

			<Errors errors={errors} />

			<input type="text" placeholder="username" ref={usernameInput} />
			<input type="password" placeholder="password" ref={passwordInput} />
			<input type="password" placeholder="confirm password" ref={confirmPasswordInput} />

			<ButtonBar
				left={<button onClick={toLoginPage}><IconChevronLeft />Sign in instead</button>}
				right={<button className="primary" onClick={create}>Create</button>}
			/>
		</Content>
	);
};

export default RegisterPage;