import * as React from "react";
import { FC } from "react";
import { AlbumPlain } from "../../models/Album";
import Albums from "./Albums";
import { IconHashTag } from "./Icons";

interface Props {
	tag: string,
	albums: AlbumPlain[],
	onAlbumClick: (album: AlbumPlain) => void,
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