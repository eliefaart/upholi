interface GalleryPhoto {
	id: string;
	width: number;
	height: number;
	/**
	 * Indicate wether this photo is allowed to load its thumbnail.
	 */
	mayLoad: boolean;
}

export default GalleryPhoto;