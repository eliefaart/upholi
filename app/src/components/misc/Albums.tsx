import * as React from "react";
import { FC } from "react";
import useAlbums from "../../hooks/useAlbums";
import { AlbumNew } from "../../models/Album";
import Album from "./Album";

interface Props {
	onClick: (album: AlbumNew) => void,
	//albums: AlbumNew[]
}

const Albums: FC<Props> = (props) => {
	const stateAlbums = useAlbums();
	const albums = stateAlbums.map((album) => (
		<Album key={album.id} album={album} onClick={props.onClick} />
	));

	return <div className="albums">
		{albums}
	</div>;
};

export default Albums;