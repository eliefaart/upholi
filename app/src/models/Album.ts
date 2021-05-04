interface AlbumPhoto {
	id: string,
	width: number,
	height: number
}

interface Album {
	id: string,
	title: string,
	thumbPhoto: AlbumPhoto | null,
	photos: AlbumPhoto[],
	tags: string[]
}

export default Album;