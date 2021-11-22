import * as React from "react";
import { FC } from "react";
import Modal from "./Modal";
import Switch from "react-switch";
import { SharingOptions } from "../../models/SharingOptions";
import { Share } from "../../models/Share";
import CopyUrl from "../misc/CopyUrl";

interface Props {
	share?: Share,
	isOpen: boolean,
	onRequestClose?: () => void,
	onOkButtonClick: (password: string) => void,
	onSharingOptionsUpdated: (options: SharingOptions) => void
}

const ModalSharingOptions: FC<Props> = (props) => {
	const [shared, setShared] = React.useState(!!props.share);
	const [password, setPassword] = React.useState(props.share?.password ?? "");

	const shareUrl = document.location.origin + "/s/" + props.share?.id;
	const inputValid = !shared || !!password;

	const updateSharingOptions = (): void => {
		const options: SharingOptions = {
			shared,
			password
		};

		const optionsChanged = shared !== !!props.share || password !== props.share?.password;
		if (inputValid && optionsChanged) {
			props.onSharingOptionsUpdated(options);
		}
	};

	return <Modal
		title="Sharing options"
		className="modalSharingOptions"
		isOpen={props.isOpen}
		onRequestClose={() => {
			setShared(!!props.share);
			setPassword(props.share?.password ?? "");
			!!props.onRequestClose && props.onRequestClose();
		}}
		okButtonText="Save"
		onOkButtonClick={() => updateSharingOptions()}
		okButtonDisabled={!inputValid}
		>
			<div>
				<label className="switch">
					<Switch checked={shared}
						width={40}
						height={15}
						handleDiameter={25}
						onColor="#20aced"
						offColor="#1c1c1c"
						checkedIcon={<span className="checkedIcon"/>}
						uncheckedIcon={<span className="uncheckedIcon"/>}
						onChange={setShared}
						/>
					<span>Share via URL</span>
				</label>
			</div>

			{shared && <label>
				Password
				<input type="text"
					defaultValue={props.share?.password}
					onChange={(e) => setPassword(e.currentTarget.value)}
					placeholder=""/>
			</label>}

			{shared && props.share && <>
				<hr/>
				<div className="url">
					Sharing URL
					<CopyUrl shareUrl={shareUrl}/>
				</div>
			</>}
	</Modal>;
};

export default ModalSharingOptions;