import * as wasm from "wasm";
import Album, { AlbumNew } from "../models/Album";
import { Photo, PhotoMinimal } from "../models/Photo";
import { SharingOptions } from "../models/SharingOptions";

const LOCAL_STORAGE_KEY = "upholiService";

/**
 * Data this service stores in local storage
 */
interface LocalStorageData {
	key: string
}

/**
 * Helper class that takes care of storing data for UpholiService in localStorage.
 */
class UpholiServiceLocalStorageHelper {
	/**
	 * Gets currently stored master encryption key
	 */
	static getKey(): string | null {
		const localStorageDataJson = localStorage.getItem(LOCAL_STORAGE_KEY);
		if (localStorageDataJson) {
			const localStorageData: LocalStorageData = JSON.parse(localStorageDataJson);
			return localStorageData.key;
		}
		else {
			return null;
		}
	}

	/**
	 * Store a master encryption key
	 */
	static storeKey(key: string) {
		// TODO: How to invalidate this when session changes,
		// how to keep it in sync with session cookie?
		// if session cookie expires.. the localStorage will still be there.

		const localStorageData: LocalStorageData = {
			key
		};
		const localStorageDataJson = JSON.stringify(localStorageData);
		localStorage.setItem(LOCAL_STORAGE_KEY, localStorageDataJson);
	}

	/**
	 * Delete all stored localStorage data managed by this class
	 */
	static clear() {
		localStorage.removeItem(LOCAL_STORAGE_KEY);
	}
}

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
		const key = await wasm.UpholiClient.login(this.baseUrl, username, password);

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

	async deletePhotos(ids: string[]): Promise<void> {
		return await this.client.deletePhotos(ids);
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

	async getAlbumByShareToken(token: string, password: string): Promise<Album> {
		const json = await this.client.getAlbumFromToken(token, password);
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

	async updateAlbumSharingOptions(id: string, options: SharingOptions): Promise<string> {
		const token = await this.client.updateAlbumSharingOptions(id, options.shared, options.password) as string;
		return token;
	}
}

const upholiService = new UpholiService();
export default upholiService;