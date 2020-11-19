import Exif from "./Exif";

export default interface Photo {
	id: string;
	name: string;
	width: number;
	height: number;
	createdOn: Date;
	hash: string;
	pathThumbnail: string;
	pathPreview: string;
	pathOriginal: string;
	exif: Exif;
}

export { Photo }