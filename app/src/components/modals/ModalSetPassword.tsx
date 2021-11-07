import * as React from "react";
import { FC } from "react";
import Modal from "./Modal";

interface Props {
	onRequestClose?: () => void,
	onOkButtonClick: (password: string) => void
}

const ModalSetPassword: FC<Props> = (props) => {
	const passwordInput = React.createRef<HTMLInputElement>();

	return <Modal
		title="Set password"
		className="modalSetPassword"
		isOpen={true}
		onRequestClose={() => !!props.onRequestClose && props.onRequestClose()}
		okButtonText="Save"
		onOkButtonClick={() => passwordInput.current && props.onOkButtonClick(passwordInput.current.value)}
		>
			<label>
				Password
				<input type="password" ref={passwordInput} placeholder="password"/>
			</label>
	</Modal>;
};

export default ModalSetPassword;