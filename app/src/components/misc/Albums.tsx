import * as React from "react";
import { FC } from "react";
import { Album as AlbumModel } from "../../models/Album";
import Album from "./Album";

interface Props {
	albums: AlbumModel[],
	onClick: (album: AlbumModel) => void,
}

const Albums: FC<Props> = (props) => {
	const albumElements = props.albums.map((album) => (
		<Album key={album.id} album={album} onClick={props.onClick} />
	));

	return <div className="albums">
		{albumElements}
	</div>;
};

export default Albums;