import AlbumInfo from "./AlbumInfo";

interface CollectionSharingOptions {
	requirePassword: boolean,
	token: string
}

interface Collection {
	id: string,
	title: string,
	albums: AlbumInfo[],
	sharing: CollectionSharingOptions
}

export default Collection;