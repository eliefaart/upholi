interface Exif {
	manufactorer: string | null;
	model: string | null;
	aperture: string | null;
	exposure_time: string | null;
	iso: number | null;
	focal_length: number | null;
	focal_length_35mm_equiv: number | null;
	orientation: number | null;
	date_taken: Date | null;
	gps_latitude: number | null;
	gps_longitude: number | null;
}

export default interface Photo {
	id: String;
	name: String;
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