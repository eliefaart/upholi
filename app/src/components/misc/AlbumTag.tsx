import * as React from "react";
import { FC } from "react";
import { Album } from "../../models/Album";
import AlbumCollection from "./Albums";
import { IconHashTag } from "./Icons";

interface Props {
  tag: string;
  albums: Album[];
  onAlbumClick: (album: Album) => void;
}

/**
 * Renders one tag and its albums
 */
const AlbumTag: FC<Props> = (props: Props) => {
  return (
    <div key={props.tag} className="album-tag">
      <h2>
        <IconHashTag />
        {props.tag}
      </h2>
      <AlbumCollection onClick={props.onAlbumClick} albums={props.albums} />
    </div>
  );
};

export default AlbumTag;
