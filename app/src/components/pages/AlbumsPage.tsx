import * as React from "react";
import { FC } from "react";
import Content from "../layout/Content";
import ModalCreateAlbum from "../modals/ModalCreateAlbum";
import appStateContext from "../../contexts/AppStateContext";
import Album from "../misc/Album";
import { IconCreate, IconHashTag } from "../misc/Icons";
import { AlbumNew } from "../../models/Album";
import { useTitle } from "../../hooks/useTitle";
import { setHeader } from "../../hooks/useHeader";
import useAlbums from "../../hooks/useAlbums";

const AlbumsPage: FC = () => {
	const [ newAlbumDialogOpen, setNewAlbumDialogOpen ] = React.useState(false);
	const albums = useAlbums();
	const context = React.useContext(appStateContext);
	useTitle("Albums");
	setHeader({
		visible: true,
		headerActions: <React.Fragment>
			{<button
				className="iconOnly"
				onClick={() => onCreateAlbumClick()}
				title="Create album">
				<IconCreate/>
			</button>}
		</React.Fragment>
	});

	const onCreateAlbumClick = (): void => {
		setNewAlbumDialogOpen(true);
	};

	const renderAlbumsInTagContainer = function(tag: string, albums: AlbumNew[]): React.ReactNode {
		return <div key={tag} className="album-tag">
			{tag && <h2><IconHashTag/>{tag}</h2>}
			{renderAlbums(albums)}
		</div>;
	};

	const renderAlbums = function(albums: AlbumNew[]): React.ReactNode {
		return <div className="albums">
			{albums.map(album => {
				return <Album key={album.id}
					album={album}
					onClick={album => context.history.push("/album/" + album.id)}
				/>;
			})}
		</div>;
	};

	const tags = albums.flatMap(a => a.tags)
		.filter((tag ,index, array) => array.indexOf(tag) === index)
		.sort();
	const albumsWithoutTag = albums.filter(album => album.tags.length === 0);

	return (
		<Content paddingTop={false}>
			{/* Render albums per tag. An album may appear in multiple tags. */}
			{tags.map(tag => {
				const albumsWithTag = albums
					.filter(album => album.tags.some(t => t === tag));

				return renderAlbumsInTagContainer(tag, albumsWithTag);
			})}

			{/* Also render all albums that do not have any tags */}
			{albumsWithoutTag.length > 0 && renderAlbumsInTagContainer(tags.length === 0 ? "" : "no-tag", albumsWithoutTag)}

			<ModalCreateAlbum
				isOpen={newAlbumDialogOpen}
				onRequestClose={() => setNewAlbumDialogOpen(false)}/>
		</Content>
	);
};

export default AlbumsPage;