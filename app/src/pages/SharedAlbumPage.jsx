import React from "react";
import PhotoGallerySelectable from "../components/PhotoGallerySelectable.jsx";
import PageLayout from "../components/PageLayout.jsx"
import AppStateContext from "../contexts/AppStateContext.jsx";
import PhotoService from "../services/PhotoService";
import { IconBack } from "../components/Icons.jsx";

class SharedAlbumPage extends React.Component {
	constructor(props) {
		super(props);

		this.state = {
			albumId: props.match.params.albumId,
			title: "",
			photos: [],
		};
	}

	componentDidMount() {
		this.refreshPhotos();
	}

	refreshPhotos() {
		let _this = this;
		PhotoService.getAlbum(this.state.albumId)
			.then((response) => {
				_this.setState({
					title: response.title,
					public: response.public,
					photos: response.photos.map((photo) => {
						return {
							id: photo.id,
							src: PhotoService.baseUrl() + "/photo/" + photo.id + "/thumb",
							width: photo.width,
							height: photo.height
						};
					}),
					selectedPhotos: []
				});
			});
	}

	onPhotoClicked(_, target) {
		let photo = this.state.photos[target.index];
		!!this.context.history && this.context.history.push("/photo/" + photo.id);
	}

	onBackToCollectionClicked() {
		!!this.context.history && this.context.history.push("/shared/collection/" + this.props.match.params.token);

	}

	render() {
		return (
			<PageLayout title={"Album - " + this.state.title} requiresAuthentication={false} onDrop={(event) => this.onFilesDropped(event)}>
				<div className="topBar">
					<IconBack className="iconOnly asButton" onClick={() => this.onBackToCollectionClicked()}/>
					<h1>{this.state.title}</h1>
				</div>

				{!!this.state.title && this.state.photos.length === 0 &&
					<span className="centerText">This album has no photos.</span>
				}

				{this.state.photos.length > 0 && <PhotoGallerySelectable
					onClick={(event, target) => this.onPhotoClicked(event, target)}
					photos={this.state.photos}
					selectedItems={[]}
					onPhotoSelectedChange={() => {}}/>
				}
			</PageLayout>
		);
	}
}

SharedAlbumPage.contextType = AppStateContext;
export default SharedAlbumPage;