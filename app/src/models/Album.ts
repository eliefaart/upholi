export interface AlbumPhoto {
	id: string,
	width: number,
	height: number,
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