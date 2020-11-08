interface Album {
	id: string,
	title: string,
	thumbPhotoId: string[]
}

interface SharingOptions {
	shared: boolean,
	require_password: boolean,
	token: string
}

export default interface Collection {
	id: string,
	title: string,
	albums: Album[],
	sharing: SharingOptions
}