import * as React from "react";
import { FC } from "react";
import Modal from "./Modal";
import PhotoDetail from "../PhotoDetail";
import { IconDownload, IconInfo } from "../misc/Icons";
import ModalPropsBase from "../../models/ModalPropsBase";
import upholiService from "../../services/UpholiService";
import { Photo } from "../../models/Photo";
import { downloadPhoto } from "../../utils/files";

interface Props extends ModalPropsBase {
	photoId: string,
	photoKey?: string,
}

const ModalPhotoDetail: FC<Props> = (props) => {
	const [photo, setPhoto] = React.useState<Photo | null>(null);
	const [photoSrc, setPhotoSrc] = React.useState<string>("");
	const [showExif, setShowExif] = React.useState(false);

	React.useEffect(() => {
		upholiService.getPhoto(props.photoId, props.photoKey)
			.then(setPhoto)
			.catch(console.error);

		upholiService.getPhotoPreviewImageSrc(props.photoId, props.photoKey)
			.then(setPhotoSrc)
			.catch(console.error);
	}, [props.photoId]);

	const headerActions = <>
		<a className="with-icon as-button" title="Download" onClick={() => downloadPhoto(props.photoId, props.photoKey)}>
			<IconDownload />Download
		</a>
		{photo?.exif && <a className="with-icon as-button" title="Info" onClick={() => setShowExif(!showExif)}>
			<IconInfo />Info
		</a>}
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