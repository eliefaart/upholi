import * as React from "react";
import { PageBaseComponent, PageBaseComponentProps } from "./PageBaseComponent";
import AllUserAlbums from "../AllUserAlbums";
import ContentContainer from "../ContentContainer";
import ModalCreateAlbum from "../modals/ModalCreateAlbum";
import AppStateContext from "../../contexts/AppStateContext";
import PhotoService from "../../services/PhotoService";
import AlbumInfo from "../../models/AlbumInfo";
import Album from "../Album";
import { IconHashTag } from "../Icons";

interface AlbumsPageState {
	newAlbumDialogOpen: boolean,
	albums: AlbumInfo[]
}

class AlbumsPage extends PageBaseComponent<AlbumsPageState> {

	constructor(props: PageBaseComponentProps) {
		super(props);

		this.state = {
			newAlbumDialogOpen: false,
			albums: []
		};
	}

	componentDidMount(): void {
		PhotoService.getAlbums()
			.then((albums) => {
				this.setState({
					albums: albums
				});
			})
			.catch(console.error);
	}

	getHeaderActions(): JSX.Element {
		return <React.Fragment>
			{<button onClick={() => this.onCreateAlbumClick()} title="Create album">
				New album
			</button>}
		</React.Fragment>;
	}

	getTitle(): string {
		return "Albums";
	}

	onCreateAlbumClick(): void {
		this.setState({
			newAlbumDialogOpen: true
		});
	}

	render(): React.ReactNode {
		const history = this.context.history;

		const renderAlbumsInTagContainer = function(tag: string, albums: AlbumInfo[]): React.ReactNode {
			return <div key={tag} className="album-tag">
				{tag && <h2><IconHashTag/>{tag}</h2>}
				{renderAlbums(albums)}
			</div>;
		};

		const renderAlbums = function(albums: AlbumInfo[]): React.ReactNode {
			return <div className="albums">
				{albums.map(album => {
					return <Album key={album.id}
						album={album}
						onClick={album => history.push("/album/" + album.id)}
					/>;
				})}
			</div>;
		};

		const tags = this.state.albums.flatMap(a => a.tags)
			.filter((tag ,index, array) => array.indexOf(tag) === index)
			.sort();
		const albumsWithoutTag = this.state.albums.filter(album => album.tags.length === 0);

		return (
			<ContentContainer paddingTop={false}>
				{/* Render albums per tag. An album may appear in multiple tags. */}
				{tags.map(tag => {
					const albumsWithTag = this.state.albums
						.filter(album => album.tags.some(t => t === tag));

					return renderAlbumsInTagContainer(tag, albumsWithTag);
				})}

				{/* Also render all albums that do not have any tags */}
				{albumsWithoutTag.length > 0 && renderAlbumsInTagContainer(tags.length === 0 ? "" : "no-tag", albumsWithoutTag)}

				<ModalCreateAlbum
					isOpen={this.state.newAlbumDialogOpen}
					onRequestClose={() => this.setState({newAlbumDialogOpen: false})}/>
			</ContentContainer>
		);
	}
}

AlbumsPage.contextType = AppStateContext;
export default AlbumsPage;