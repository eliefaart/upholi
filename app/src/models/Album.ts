interface AlbumPhoto {
	id: string,
	width: number,
	height: number
}

export default interface Album {
	id: string,
	title: string,
	thumbPhoto: AlbumPhoto | null,
	photos: AlbumPhoto[]
}