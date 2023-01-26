import * as wasm from "wasm";
import { Album, AlbumPlain } from "../models/Album";
import { Photo, PhotoMinimal } from "../models/Photo";
import { LibraryShare } from "../models/Share";

interface UploadResult {
	skipped: boolean,
	photoId: string
}

/**
 * This class exists mainly to assign types to the return values of functions within 'wasm.UpholiClient'
 */
class UpholiService {
	private _client: wasm.UpholiClient | null;
	private get client(): wasm.UpholiClient {
		if (!this._client) {
			this._client = new wasm.UpholiClient();
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
		await await this.client.login(username, password);
	}

	async uploadPhoto(bytes: Uint8Array): Promise<UploadResult> {
		const result: UploadResult = await this.client.uploadPhoto(bytes);
		return result;
	}

	async getPhotos(): Promise<PhotoMinimal[]> {
		return this.client.getPhotos();
	}

	async getPhoto(id: string): Promise<Photo> {
		return this.client.getPhoto(id);
	}

	async getPhotoThumbnailImageSrc(id: string): Promise<string> {
		return this.client.getPhotoThumbnailImageSrc(id);
	}

	async getPhotoPreviewImageSrc(id: string): Promise<string> {
		return await this.client.getPhotoPreviewImageSrc(id);
	}

	async getPhotoOriginalImageSrc(id: string): Promise<string> {
		return await this.client.getPhotoOriginalImageSrc(id);
	}

	async deletePhotos(ids: string[]): Promise<void> {
		await this.client.deletePhotos(ids);
	}

	async getAlbums(): Promise<AlbumPlain[]> {
		return this.client.getAlbums();
	}

	async getAlbum(id: string): Promise<Album> {
		const album: Album = await this.client.getAlbum(id);
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

	async upsertAlbumShare(albumId: string, password: string): Promise<string> {
		return await this.client.upsertAlbumShare(albumId, password) as string;
	}

	async getShares(): Promise<LibraryShare[]> {
		return this.client.getShares();
	}

	async isAuthorizedForShare(id: string): Promise<boolean> {
		return await this.client.isAuthorizedForShare(id);
	}

	async authorizeShare(id: string, password: string): Promise<boolean> {
		return await this.client.authorizeShare(id, password);
	}

	async getAlbumShare(id: string): Promise<LibraryShare> {
		return this.client.getAlbumShare(id);
	}

	async getShareAlbum(id: string): Promise<Album> {
		return this.client.getShareAlbum(id);
	}

	async deleteShare(id: string): Promise<void> {
		await this.client.deleteShare(id);
	}
}

const upholiService = new UpholiService();
export default upholiService;