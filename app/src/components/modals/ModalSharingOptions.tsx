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
	const passwordInput = React.createRef<HTMLInputElement>();
	const shareUrl = document.location.origin + "/s/" + props.share?.id;

	const updateSharingOptions = (): void => {
		const options: SharingOptions = {
			shared,
			password: passwordInput.current ? passwordInput.current.value : ""
		};

		props.onSharingOptionsUpdated(options);
	};

	return <Modal
		title="Sharing options"
		className="modalSharingOptions"
		isOpen={props.isOpen}
		onRequestClose={() => {!!props.onRequestClose && props.onRequestClose();}}
		okButtonText="Save"
		onOkButtonClick={() => updateSharingOptions()}
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

			{shared && <div className="url">
				Sharing URL
				<CopyUrl shareUrl={shareUrl}/>
			</div>}

			{shared && <label>
				Password
				<input type="text" defaultValue={props.share?.password} ref={passwordInput} placeholder=""/>
			</label>}
	</Modal>;
};

export default ModalSharingOptions;