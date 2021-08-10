import * as wasm from "wasm";
import Exif from "../models/Exif";

export interface PhotoMinimal {
	id: string,
	width: number,
	height: number
}

export interface Photo {
	id: string,
	hash: string,
	width: number,
	height: number,
	contentType: string,
	exif: Exif
}

/**
 * This class exists mainly to assign types to the return values of functions within 'wasm.UpholiClient'
 */
class UpholiService {
	private _client: wasm.UpholiClient | null;
	private get client(): wasm.UpholiClient {
		if (!this._client) {
			this._client = new wasm.UpholiClient("http://localhost", "e0ca4c29d5504e8daa8c52e873e66f71");
		}
		return this._client;
	}

	constructor() {
		this._client = null;
	}

	async getPhoto(id: string): Promise<Photo> {
		const json = await this.client.getPhoto(id);
		const photo: Photo = JSON.parse(json);

		// wasm response doesn't id for now. I'll set it here manually.
		photo.id = id;

		return photo;
	}

	async getPhotos(): Promise<PhotoMinimal[]> {
		const photos = await this.client.getPhotos();

		return photos.map((photo: any) => {
			const typed: PhotoMinimal = {
				id: photo.id,
				width: photo.width,
				height: photo.height
			};

			return typed;
		});
	}

	// async getPhotoThumbnailBase64(id: string): Promise<string> {
	// 	return await this.client.getPhotoThumbnailBase64(id);
	// }

	// async getPhotoPreviewBase64(id: string): Promise<string> {
	// 	return await this.client.getPhotoPreviewBase64(id);
	// }

	// async getPhotoOriginalBase64(id: string): Promise<string> {
	// 	return await this.client.getPhotoOriginalBase64(id);
	// }

	async getPhotoThumbnailImageSrc(id: string): Promise<string> {
		return await this.client.getPhotoThumbnailImageSrc(id);
	}

	async getPhotoPreviewImageSrc(id: string): Promise<string> {
		return await this.client.getPhotoPreviewImageSrc(id);
	}

	async getPhotoOriginalImageSrc(id: string): Promise<string> {
		return await this.client.getPhotoOriginalImageSrc(id);
	}

	async deletePhoto(id: string): Promise<void> {
		return await this.client.deletePhoto(id);
	}
}

const upholiService = new UpholiService();
export default upholiService;