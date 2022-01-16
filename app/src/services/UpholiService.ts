import * as wasm from "wasm";
import UpholiServiceLocalStorageHelper from "../helpers/UpholiServiceLocalStorageHelper";
import { Album, AlbumPlain } from "../models/Album";
import { Photo, PhotoMinimal } from "../models/Photo";
import { Share } from "../models/Share";

/**
 * This class exists mainly to assign types to the return values of functions within 'wasm.UpholiClient'
 */
class UpholiService {
	private baseUrl: string;
	private _client: wasm.UpholiClient | null;
	private get client(): wasm.UpholiClient {
		if (!this._client) {
			// Init from stored key
			const key = UpholiServiceLocalStorageHelper.getKey();
			if (key) {
				this._client = new wasm.UpholiClient(this.baseUrl, key);
			}
			else {
				//throw Error("WASM client not initialized. Is user logged in?");
				// For anonymous calls where user's private key is not needed.
				// TODO: I am not happy with this constructor now, need to rethink it in future.
				this._client = new wasm.UpholiClient(this.baseUrl, "");
			}
		}

		return this._client;
	}

	constructor() {
		this.baseUrl = document.location.origin;
		this._client = null;
	}

	async register(username: string, password: string): Promise<void> {
		return await this.client.register(username, password);
	}

	async login(username: string, password: string): Promise<void> {
		const key = await await this.client.login(username, password);

		// Write key to localStorage
		UpholiServiceLocalStorageHelper.storeKey(key);

		// Init innter client
		this._client = new wasm.UpholiClient(this.baseUrl, key);
	}

	async logout(): Promise<void> {
		UpholiServiceLocalStorageHelper.clear();
	}

	async getUserInfo(): Promise<void> {
		return await this.client.getUserInfo();
	}

	async uploadPhoto(bytes: Uint8Array): Promise<string> {
		const id: string = await this.client.uploadPhoto(bytes);
		return id;
	}

	async getPhotos(): Promise<PhotoMinimal[]> {
		const photos: PhotoMinimal[] = await this.client.getPhotos();
		return photos;
	}

	async getPhoto(id: string, key?: string): Promise<Photo> {
		const photo: Photo = key === undefined
			? await this.client.getPhoto(id)
			: await this.client.getPhotoWithProof(id, key);
		return photo;
	}

	async getPhotoThumbnailImageSrc(id: string, key?: string): Promise<string> {
		return key === undefined
			? await this.client.getPhotoThumbnailImageSrc(id)
			: await this.client.getPhotoThumbnailImageSrcWithProof(id, key);
	}

	async getPhotoPreviewImageSrc(id: string, key?: string): Promise<string> {
		return key === undefined
			? await this.client.getPhotoPreviewImageSrc(id)
			: await this.client.getPhotoPreviewImageSrcWithProof(id, key);
	}

	async getPhotoOriginalImageSrc(id: string, key?: string): Promise<string> {
		return key === undefined
			? await this.client.getPhotoOriginalImageSrc(id)
			: await this.client.getPhotoOriginalImageSrcWithProof(id, key);
	}

	async deletePhotos(ids: string[]): Promise<void> {
		return await this.client.deletePhotos(ids);
	}

	async getAlbums(): Promise<AlbumPlain[]> {
		const albums: AlbumPlain[] = await this.client.getAlbums();
		return albums;
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

	async getShares(): Promise<Share[]> {
		return this.client.getShares();
	}

	async getShare(id: string): Promise<void> {
		return this.client.getShare(id);
	}

	async getAlbumFromShare(id: string, password: string): Promise<Album> {
		return await this.client.getAlbumFromShare(id, password);
	}

	async getShareUsingPassword(id: string, password: string): Promise<void> {
		return this.client.getShareUsingPassword(id, password);
	}

	async findAlbumShare(id: string): Promise<Share> {
		return this.client.findAlbumShare(id);
	}

	async deleteShare(id: string): Promise<void> {
		await this.client.deleteShare(id);
	}
}

const upholiService = new UpholiService();
export default upholiService;