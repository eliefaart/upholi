export default interface Exif {
	manufactorer: string | null;
	model: string | null;
	aperture: string | null;
	exposureTime: string | null;
	iso: number | null;
	focalLength: number | null;
	focalLength35mmEquiv: number | null;
	orientation: number | null;
	dateTaken: Date | null;
	gpsLatitude: number | null;
	gpsLongitude: number | null;
}