import { PhotoMinimal } from "./Photo";

export interface AlbumPhoto extends PhotoMinimal {
	key: string | null
}

export interface Album {
	id: string,
	title: string,
	thumbPhoto: AlbumPhoto | null,
	photos: AlbumPhoto[],
	tags: string[]
}


export interface AlbumNew {
	id: string,
	title: string,
	tags: string[],
	photos: string[],
	thumbnailPhotoId: string,
}