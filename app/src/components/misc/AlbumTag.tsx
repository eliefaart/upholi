import * as React from "react";
import { FC } from "react";
import { AlbumNew } from "../../models/Album";
import Albums from "./Albums";
import { IconHashTag } from "./Icons";

interface Props {
	tag: string,
	albums: AlbumNew[],
	onAlbumClick: (album: AlbumNew) => void,
}

/**
 * Renders one tag and its albums
 */
const AlbumTag: FC<Props> = (props: Props) => {
	return <div key={props.tag} className="album-tag">
		<h2>
			<IconHashTag/>
			{props.tag}
		</h2>
		<Albums onClick={props.onAlbumClick} albums={props.albums} />
	</div>;
};

export default AlbumTag;