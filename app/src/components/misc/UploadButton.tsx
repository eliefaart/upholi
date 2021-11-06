import * as React from "react";
import { FC } from "react";
import { IconUpload } from "../Icons";

interface Props {
	className: string,
	onSubmit: (fileList: FileList) => void
}

const UploadButton: FC<Props> = (props) => {
	const onSubmitButtonClick = (event: React.ChangeEvent<HTMLInputElement>): void => {
		if (props.onSubmit && event.target.files) {
			props.onSubmit(event.target.files);
		}
	};

	return <form id="form-select-photos" className={props.className}>
		<label htmlFor="select-photos" className="asButton"><IconUpload/></label>
		<input id="select-photos" type="file" name="photos" accept=".jpg,.jpeg" onChange={onSubmitButtonClick} multiple/>
	</form>;
};

export default UploadButton;