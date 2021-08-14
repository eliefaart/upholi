import { AlbumNew } from "./Album";

interface CollectionSharingOptions {
	requirePassword: boolean,
	token: string
}

interface Collection {
	id: string,
	title: string,
	albums: AlbumNew[],
	sharing: CollectionSharingOptions
}

export default Collection;