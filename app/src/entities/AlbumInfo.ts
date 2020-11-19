/**
 * A light/small version of Album, since many components don't need to know/load all photos contained in an album
 */
export default interface AlbumInfo {
	id: string,
	title: string,
	thumbPhotoId: string | null
}