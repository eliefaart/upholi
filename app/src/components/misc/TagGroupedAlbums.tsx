import * as React from "react";
import { FC } from "react";
import { AlbumPlain } from "../../models/Album";
import AlbumTag from "./AlbumTag";

interface Props {
	albums: AlbumPlain[],
	onAlbumClick: (album: AlbumPlain) => void,
}

/**
 * Renders given albums, grouped in tags. An album pay appear in multiple tags
 */
const TagGroupedAlbums: FC<Props> = (props: Props) => {
	const tags = props.albums.flatMap(a => a.tags)
		.filter((tag ,index, array) => array.indexOf(tag) === index)
		.sort();
	const albumsWithoutTag = props.albums.filter(album => album.tags.length === 0);

	return <>
		{/* Render albums per tag. An album may appear in multiple tags. */}
		{tags.map(tag => {
			const albumsWithTag = props.albums
				.filter(album => album.tags.some(t => t === tag));

			return <AlbumTag
				key={tag}
				tag={tag}
				albums={albumsWithTag}
				onAlbumClick={props.onAlbumClick}/>;
		})}

		{/* Also render all albums that do not have any tags */}
		{albumsWithoutTag.length > 0 && <AlbumTag
			tag="no-tag"
			albums={albumsWithoutTag}
			onAlbumClick={props.onAlbumClick}/>}
	</>;
};

export default TagGroupedAlbums;