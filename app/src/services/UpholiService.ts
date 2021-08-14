import * as wasm from "wasm";
import { AlbumNew } from "../models/Album";
import { Photo, PhotoMinimal } from "../models/Photo";

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

	async getAlbums(): Promise<AlbumNew[]> {
		const albums = await this.client.getAlbums();

		return albums.map((album: any) => {
			const typed: AlbumNew = {
				id: album.id,
				title: album.title,
				thumbnailPhotoId: album.thumbnailPhotoId,
				tags: album.tags,
				photos: album.photos
			};

			return typed;
		});
	}

	async createAlbum(title: string): Promise<void> {
		return this.client.createAlbum(title);
	}
}

const upholiService = new UpholiService();
export default upholiService;