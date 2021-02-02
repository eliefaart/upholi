import * as React from "react";

interface Props {
	className?: string,
	prompt?: string,
	onSubmitPassword: (password: string) => void,
	lastPasswordIncorrect?: boolean
}

class InputPassword extends React.Component<Props> {
	passwordInput: React.RefObject<HTMLInputElement>;

	constructor(props: Props) {
		super(props);

		this.passwordInput = React.createRef();
		this.submitPassword = this.submitPassword.bind(this);
	}

	submitPassword() {
		if (!!this.passwordInput.current) {
			const password = this.passwordInput.current.value;
			this.props.onSubmitPassword(password);
		}
	}

	render() {
		let className = "input-password";
		if (!!this.props.className) {
			className += ` ${this.props.className}`;
		}

		return <div className={className}>
			{this.props.lastPasswordIncorrect !== true && !!this.props.prompt && <label htmlFor="input-password">{this.props.prompt}</label>}
			{this.props.lastPasswordIncorrect === true && <label className="error-text">Incorrect password</label>}
			<input id="input-password" type="password" placeholder="Password" ref={this.passwordInput} />
			<button onClick={this.submitPassword}>Submit</button>
		</div>;
	}
}

export default InputPassword;