import * as React from "react";
import Modal from "./Modal";
import PhotoService from "../../services/PhotoService";
import PhotoDetail from "../PhotoDetail";
import { IconDownload } from "../Icons";
import ModalPropsBase from "../../models/ModalPropsBase";
//import Photo from "../../models/Photo";
import upholiService, { Photo } from "../../services/UpholiService";

interface Props extends ModalPropsBase {
	photoId: string
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
			const fnOnPhotoBase64Received = (base64: string) => {
				this.setState({ src: `data:image/png;base64,${base64}` });
			};

			this.isRequestingPhotoId = this.props.photoId;

			const getInfoPromise = upholiService.getPhoto(this.props.photoId);
			getInfoPromise
				.then(fnOnPhotoDataReceived)
				.catch(console.error);

			const getImageSrcPromise = upholiService.getPhotoPreviewBase64(this.props.photoId);
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

		const photoBaseUrl = PhotoService.baseUrl() + "/photo/" + this.props.photoId;
		const downloadUrl = photoBaseUrl + "/original";

		const headerActions = <a className="iconOnly asButton" href={downloadUrl} download title="Download">
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