import * as React from "react";
import { FC } from "react";
import { AlbumPlain } from "../../models/Album";
import Album from "./Album";

interface Props {
	albums: AlbumPlain[],
	onClick: (album: AlbumPlain) => void,
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