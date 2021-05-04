/**
 * A light/small version of Album, which does not contain detailed information about the photos contained in it.
 */
interface AlbumInfo {
	id: string,
	title: string,
	thumbPhotoId: string | null,
	tags: string[]
}

export default AlbumInfo;