import AlbumInfo from "./AlbumInfo";

interface CollectionSharingOptions {
	requirePassword: boolean,
	token: string
}

export default interface Collection {
	id: string,
	title: string,
	albums: AlbumInfo[],
	sharing: CollectionSharingOptions
}