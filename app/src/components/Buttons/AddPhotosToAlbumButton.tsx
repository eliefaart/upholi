import * as React from "react";
import { toast } from "react-toastify";
import { AlbumNew } from "../../models/Album";
import upholiService from "../../services/UpholiService";
import { IconAddToAlbum } from "../Icons";
import ModalAddToAlbum from "../modals/ModalAddToAlbum";
import ModalCreateAlbum from "../modals/ModalCreateAlbum";

interface Props {
	selectedPhotoIds: string[];
	onSelectionAddedToAlbum?: () => void;
}

interface State {
	selectAlbumModelOpen: boolean;
	createAlbumModelOpen: boolean;
}

/**
 * A button that handles adding a selection of photos to an album
 */
export default class AddPhotosToAlbumButton extends React.Component<Props, State> {

	constructor(props: Props) {
		super(props);

		this.addSelectedPhotosToAlbum = this.addSelectedPhotosToAlbum.bind(this);
		this.openSelectAlbumModal = this.openSelectAlbumModal.bind(this);
		this.openCreateAlbumModal = this.openCreateAlbumModal.bind(this);

		this.state = {
			selectAlbumModelOpen: false,
			createAlbumModelOpen: false
		};
	}

	addSelectedPhotosToAlbum(album: AlbumNew): void {
		upholiService.addPhotosToAlbum(album.id, this.props.selectedPhotoIds)
			.then(() => {
				this.setState({
					selectAlbumModelOpen: false
				});

				toast.info("Photos added to album.");

				if (this.props.onSelectionAddedToAlbum) {
					this.props.onSelectionAddedToAlbum();
				}
			})
			.catch(console.error);
	}

	openSelectAlbumModal(): void {
		this.setState({
			selectAlbumModelOpen: true
		});
	}

	openCreateAlbumModal(): void {
		this.setState({
			createAlbumModelOpen: true
		});
	}

	render(): React.ReactNode {
		if (this.props.selectedPhotoIds.length === 0) {
			return null;
		}
		else {
			return <>
				<button className="iconOnly" onClick={this.openSelectAlbumModal} title="Add to album">
					<IconAddToAlbum/>
				</button>

				<ModalAddToAlbum
					isOpen={this.state.selectAlbumModelOpen}
					onRequestClose={() => this.setState({selectAlbumModelOpen: false})}
					onClickNewAlbum={this.openCreateAlbumModal}
					onClickExistingAlbum={this.addSelectedPhotosToAlbum}
					/>

				<ModalCreateAlbum
					isOpen={this.state.createAlbumModelOpen}
					onRequestClose={() => this.setState({createAlbumModelOpen: false})}
					createWithPhotoIds={this.props.selectedPhotoIds}
					/>
			</>;
		}
	}
}