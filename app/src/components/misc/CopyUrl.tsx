import * as React from "react";
import { FC } from "react";
import { toast } from "react-toastify";
import { copyElementContentToClipboard } from "../../utils/elements";
import { IconCopy } from "../Icons";

interface Props {
	shareUrl: string
}

const CopyUrl: FC<Props> = (props) => {
	const copyUrlToClipboard = () => {
		const publicUrlElement = document.getElementsByClassName("urlToCopy")[0] as HTMLInputElement;
		copyElementContentToClipboard(publicUrlElement);
		toast.info("URL copied to clipboard.");
	};

	return <div className="copyUrl">
		<input className="urlToCopy" type="text" value={props.shareUrl}
			// Prevent changes to the value of this input by resetting value in onchange event.
			// I cannot make it 'disabled', because then I cannot copy the text using JS
			onChange={(event) => event.target.value = props.shareUrl}/>
		<button className="iconOnly" onClick={copyUrlToClipboard} title="Copy URL">
			<IconCopy/>
		</button>
	</div>;
};

export default CopyUrl;