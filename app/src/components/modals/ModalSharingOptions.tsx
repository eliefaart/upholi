import * as React from "react";
import { FC } from "react";
import Modal from "./Modal";
import { SharingOptions } from "../../models/SharingOptions";
import { Share } from "../../models/Share";
import CopyUrl from "../misc/CopyUrl";
import Switch from "../misc/Switch";

interface Props {
	share?: Share,
	isOpen: boolean,
	onRequestClose?: () => void,
	onOkButtonClick: (password: string) => void,
	onSharingOptionsUpdated: (options: SharingOptions) => void
}

const ModalSharingOptions: FC<Props> = (props) => {
	const [shared, setShared] = React.useState(!!props.share);
	const [passwordProtected, setPasswordProtected] = React.useState(props.share?.password !== "");
	const [password, setPassword] = React.useState(props.share?.password ?? "");

	const shareUrl = document.location.origin + "/s/" + props.share?.id;
	const inputValid = !shared || (!passwordProtected || !!password);

	React.useEffect(() => {
		// Ensure password is empty if share is not password protected.
		if (!passwordProtected && password !== "") {
			setPassword("");
		}
	}, [passwordProtected]);

	const updateSharingOptions = (): void => {
		const options: SharingOptions = {
			shared,
			password: passwordProtected ? password : ""
		};

		const optionsChanged = shared !== !!props.share || password !== props.share?.password;
		if (inputValid && optionsChanged) {
			props.onSharingOptionsUpdated(options);
		}
	};

	return <Modal
		title="Sharing options"
		className="modal-sharing-options"
		isOpen={props.isOpen}
		onRequestClose={() => {
			setShared(!!props.share);
			setPasswordProtected(props.share?.password !== "");
			setPassword(props.share?.password ?? "");
			!!props.onRequestClose && props.onRequestClose();
		}}
		okButtonText="Save"
		onOkButtonClick={() => updateSharingOptions()}
		okButtonDisabled={!inputValid}
	>

		<Switch checked={shared}
			label="Share via URL"
			onChange={setShared} />

		{shared && <Switch checked={passwordProtected}
			label="Require password"
			onChange={setPasswordProtected} />}

		{shared && passwordProtected && <label>
			Password
			<input type="text"
				defaultValue={props.share?.password}
				onChange={(e) => setPassword(e.currentTarget.value)}
				placeholder="" />
		</label>}

		{shared && props.share && <>
			<hr />
			<div className="url">
				Sharing URL
				<CopyUrl shareUrl={shareUrl} />
			</div>
		</>}
	</Modal>;
};

export default ModalSharingOptions;