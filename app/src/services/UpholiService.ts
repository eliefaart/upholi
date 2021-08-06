import * as wasm from "wasm";

export interface PhotoMinimal {
	id: string,
	width: number,
	height: number
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

	async getPhotoBase64(id: string): Promise<string> {
		return await this.client.getPhotoBase64(id);
	}

	async deletePhoto(id: string): Promise<void> {
		return await this.client.deletePhoto(id);
	}

	// async deletePhotos(ids: string[]): Promise<void> {
	// 	await this.client.deletePhotos(ids);
	// }
}

const upholiService = new UpholiService();
export default upholiService;