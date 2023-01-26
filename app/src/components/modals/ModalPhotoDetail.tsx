import * as React from "react";
import { FC } from "react";
import Modal from "./Modal";
import PhotoDetail from "../PhotoDetail";
import { IconDownload, IconInfo } from "../misc/Icons";
import ModalPropsBase from "../../models/ModalPropsBase";
import upholiService from "../../services/UpholiService";
import { Photo } from "../../models/Photo";
import { downloadPhoto } from "../../utils/files";
import Button from "../misc/Button";

interface Props extends ModalPropsBase {
	photoId: string,
}

const ModalPhotoDetail: FC<Props> = (props) => {
	const [photo, setPhoto] = React.useState<Photo | null>(null);
	const [photoSrc, setPhotoSrc] = React.useState<string>("");
	const [showExif, setShowExif] = React.useState(false);

	React.useEffect(() => {
		upholiService.getPhoto(props.photoId)
			.then(setPhoto)
			.catch(console.error);

		upholiService.getPhotoPreviewImageSrc(props.photoId)
			.then(setPhotoSrc)
			.catch(console.error);
	}, [props.photoId]);

	const headerActions = <>
		{photo?.exif && <Button onClick={() => setShowExif(!showExif)}
			label="Info"
			icon={<IconInfo />}
		/>}
		<Button onClick={() => downloadPhoto(props.photoId)}
			label="Download"
			icon={<IconDownload />}
		/>
	</>;

	return (
		<Modal
			title=""
			isOpen={props.isOpen}
			onRequestClose={props.onRequestClose}
			className={props.className + " modalPhotoDetail fullscreen"}
			headerActions={headerActions}
			okButtonText={null}
		>
			<PhotoDetail
				src={photoSrc}
				isVideo={!!photo && photo.contentType.startsWith("video/")}
				exif={photo && showExif ? photo.exif : null} />
		</Modal>
	);
};

export default ModalPhotoDetail;