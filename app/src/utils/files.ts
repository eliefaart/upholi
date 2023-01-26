import upholiService from "../services/UpholiService";

/**
 * Copies the content of given HTML element to clipboard
 */
export function downloadPhoto(photoId: string): void {
	upholiService.getPhotoOriginalImageSrc(photoId)
		.then((src) => {
			const imageSrc = src;
			const aElement = document.createElement("a");
			aElement.href = imageSrc;
			aElement.download = `${photoId}.jpg`;
			aElement.click();
		})
		.catch(console.error);
}