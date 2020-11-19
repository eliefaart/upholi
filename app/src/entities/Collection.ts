import AlbumInfo from "./AlbumInfo";

interface CollectionSharingOptions {
	shared: boolean,
	requirePassword: boolean,
	token: string
}

export default interface Collection {
	id: string,
	title: string,
	albums: AlbumInfo[],
	sharing: CollectionSharingOptions
}