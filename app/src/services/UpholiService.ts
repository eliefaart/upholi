import * as wasm from "wasm";
import Album, { AlbumNew } from "../models/Album";
import { Photo, PhotoMinimal } from "../models/Photo";

/**
 * This class exists mainly to assign types to the return values of functions within 'wasm.UpholiClient'
 */
class UpholiService {
	private _client: wasm.UpholiClient | null;
	private get client(): wasm.UpholiClient {
		if (!this._client) {
			//this._client = new wasm.UpholiClient("http://localhost", "e0ca4c29d5504e8daa8c52e873e66f71");
			this._client = new wasm.UpholiClient("http://localhost", "e0ca4c29d5504e8d");
		}
		return this._client;
	}

	constructor() {
		this._client = null;
	}

	async register(username: string, password: string): Promise<void> {
		return await this.client.register(username, password);
	}

	async login(username: string, password: string): Promise<void> {
		return await this.client.login(username, password);
	}

	async getUserInfo(): Promise<void> {
		return await this.client.getUserInfo();
	}

	async uploadPhoto(bytes: Uint8Array): Promise<void> {
		return await this.client.uploadPhoto(bytes);
	}

	async getPhoto(id: string): Promise<Photo> {
		const json = await this.client.getPhoto(id);
		const photo: Photo = JSON.parse(json);

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

	async getAlbum(id: string): Promise<Album> {
		const json = await this.client.getAlbum(id);
		const album: Album = JSON.parse(json);

		return album;
	}

	async createAlbum(title: string, initialPhotoIds?: string[]): Promise<string> {
		const photoIds = initialPhotoIds ?? [];
		return this.client.createAlbum(title, photoIds);
	}

	async deleteAlbum(id: string): Promise<void> {
		return this.client.deleteAlbum(id);
	}

	async updateAlbumTitleTags(id: string, title: string, tags: string[]): Promise<void> {
		return this.client.updateAlbumTitleTags(id, title, tags);
	}

	async updateAlbumCover(id: string, coverPhotoId: string): Promise<void> {
		return this.client.updateAlbumCover(id, coverPhotoId);
	}

	async addPhotosToAlbum(id: string, photoIds: string[]): Promise<void> {
		return this.client.addPhotosToAlbum(id, photoIds);
	}

	async removePhotosFromAlbum(id: string, photoIds: string[]): Promise<void> {
		return this.client.removePhotosFromAlbum(id, photoIds);
	}
}

const upholiService = new UpholiService();
export default upholiService;