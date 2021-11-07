import * as React from "react";
import Modal from "./Modal";
import PhotoDetail from "../PhotoDetail";
import { IconDownload } from "../misc/Icons";
import ModalPropsBase from "../../models/ModalPropsBase";
import upholiService from "../../services/UpholiService";
import { Photo } from "../../models/Photo";
import { downloadPhoto } from "../../utils/files";

interface Props extends ModalPropsBase {
	photoId: string,
	photoKey?: string,
}

interface State {
	photo: Photo | null,
	src: string
}

class ModalPhotoDetail extends React.Component<Props, State> {

	isRequestingPhotoId: string | null;

	constructor(props: Props) {
		super(props);

		this.isRequestingPhotoId = null;

		this.state = {
			photo: null,
			src: ""
		};
	}

	componentDidMount(): void {
		this.getPhotoInfo();
	}

	componentDidUpdate(prevProps: Props): void {
		if (this.props.photoId && this.isRequestingPhotoId !== this.props.photoId && (this.state.photo == null || this.state.photo?.id !== prevProps.photoId)) {
			this.getPhotoInfo();
		}
	}

	getPhotoInfo(): void {
		if (this.props.photoId) {
			const fnOnPhotoDataReceived = (photo: Photo) => {
				this.setState({ photo });
			};
			const fnOnPhotoBase64Received = (src: string) => {
				this.setState({ src });
			};

			this.isRequestingPhotoId = this.props.photoId;

			const getInfoPromise = upholiService.getPhoto(this.props.photoId, this.props.photoKey);
			getInfoPromise
				.then(fnOnPhotoDataReceived)
				.catch(console.error);

			const getImageSrcPromise = upholiService.getPhotoPreviewImageSrc(this.props.photoId, this.props.photoKey);
			getImageSrcPromise
				.then(fnOnPhotoBase64Received)
				.catch(console.error);

			Promise.all([ getInfoPromise, getImageSrcPromise ])
				.finally(() => this.isRequestingPhotoId = null);
		}
	}

	render(): React.ReactNode {
		if (!this.props.photoId) {
			return null;
		}

		const headerActions = <a className="iconOnly asButton" title="Download" onClick={() => downloadPhoto(this.props.photoId, this.props.photoKey)}>
			<IconDownload/>
		</a>;

		return (
			<Modal
				title=""
				isOpen={this.props.isOpen}
				onRequestClose={this.props.onRequestClose}
				className={this.props.className + " modalPhotoDetail fullscreen"}
				headerActions={headerActions}
				okButtonText={null}
			>
				<PhotoDetail
					src={this.state.src}
					isVideo={!!this.state.photo && this.state.photo.contentType.startsWith("video/")}
					exif={this.state.photo ? this.state.photo.exif : null} />
			</Modal>
		);
	}
}

export default ModalPhotoDetail;