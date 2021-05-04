import * as React from "react";
import Modal from "./Modal";
import PhotoService from "../../services/PhotoService";
import ModalPropsBase from "../../models/ModalPropsBase";
import Album from "../../models/Album";

interface Props extends ModalPropsBase {
	album: Album
}

interface State {
	title: string,
	tags: string[]
}

export default class ModalEditAlbum extends React.Component<Props, State> {

	titleInput: React.RefObject<HTMLInputElement>;
	tagsInput: React.RefObject<HTMLInputElement>;

	constructor(props: Props) {
		super(props);

		this.titleInput = React.createRef();
		this.tagsInput = React.createRef();

		this.saveChanges = this.saveChanges.bind(this);

		this.state = {
			title: this.props.album.title,
			tags: this.props.album.tags
		};
	}

	saveChanges(): void {
		if (this.titleInput.current && this.tagsInput.current) {
			const promise = PhotoService.updateAlbum(this.props.album.id, {
					title: this.titleInput.current.value,
					thumbPhotoId: null,
					photos: null,
					tags: this.tagsInput.current.value.trim().toLowerCase().split(" ").filter(tag => !!tag)
				});

			promise
				.then(() => this.props.onRequestClose())
				.catch(console.error);
		}
	}

	render(): React.ReactNode {
		return (
			<Modal
				title={this.props.album.title}
				isOpen={this.props.isOpen}
				onRequestClose={this.props.onRequestClose}
				className={this.props.className + " modal-update-album"}
				okButtonText={null}
			>
				<input type="text" placeholder="Title" ref={this.titleInput} defaultValue={this.props.album.title} />
				<input type="text" placeholder="Tags" ref={this.tagsInput} defaultValue={this.props.album.tags.join(" ")} />
				<button onClick={this.saveChanges}>Save</button>
			</Modal>
		);
	}
}