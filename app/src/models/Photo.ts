import Exif from "./Exif";

export default interface Photo {
	id: string;
	name: string;
	width: number;
	height: number;
	contentType: string;
	createdOn: Date;
	hash: string;
	pathThumbnail: string;
	pathPreview: string;
	pathOriginal: string;
	exif: Exif;
}

export { Photo }