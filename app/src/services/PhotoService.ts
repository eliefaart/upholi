import Album from "../models/Album";
import Photo from "../models/Photo";
import Collection from "../models/Collection";
import AlbumInfo from "../models/AlbumInfo";

/**
 * Response from server on calls that create new entities such as albums
 */
export interface CreatedResult {
	id: string
}

export interface CreateAlbum {
	title: string
}

export interface UpdateAlbum {
	title: string | null,
	thumbPhotoId: string | null,
	photos: string[] | null
}

export interface CreateCollection {
	title: string
}

export interface UpdateCollection {
	title?: string | null,
	public?: boolean | null,
	albums?: string[] | null,
	sharing?: {
		shared: boolean
		requirePassword: boolean,
		password?: string
	}
}

/**
 * Handles all calls to back-end
 */
class PhotoService {
	public static baseUrl(): string {
		return "/api";
	}

	public static getThumbUrl(photoId: string): string {
		return this.getUrl(photoId, "thumb");
	}

	public static getPreviewUrl(photoId: string): string {
		return this.getUrl(photoId, "preview");
	}

	public static getOriginalUrl(photoId: string): string {
		return this.getUrl(photoId, "original");
	}

	private static getUrl(photoId: string, variant: string): string {
		const baseUrl = "/api";
		return baseUrl + "/photo/" + photoId + "/" + variant;
	}

	static uploadPhotos(fileList: FileList, fnFileStatusUpdatedCallback: (file: File, newState: string) => void): Promise<string[]> {
		const nConcurrentUploads = 3;

		// Create queue, and set initial status
		const queue: globalThis.File[] = [];
		for (let i = 0; i < fileList.length; i++) {
			const file = fileList.item(i);
			if (file) {
				// Reverse array using unshift,
				// so we can use pop() when consuming array, which is faster than shift()
				queue.unshift(file);
				fnFileStatusUpdatedCallback(file, "Waiting");
			}
		}

		const photoIdsUploaded: string[] = [];

		return new Promise<string[]>((ok, err) => {
			const uploadPromises: Promise<string>[] = [];

			const fnStartNextUpload = () => {
				if (queue.length > 0) {
					const file = queue.pop();
					if (file) {
						const uploadPromise = PhotoService.uploadPhoto(file);
						uploadPromises.push(uploadPromise);
						fnFileStatusUpdatedCallback(file, "Uploading");

						uploadPromise.then((photoId: string) => {
							uploadPromises.splice(uploadPromises.indexOf(uploadPromise), 1);

							photoIdsUploaded.push(photoId);
							fnFileStatusUpdatedCallback(file, "Done");
							fnStartNextUpload();

							if (queue.length === 0 && uploadPromises.length === 0) {
								ok(photoIdsUploaded);
							}
						}).catch((error) => {
							fnFileStatusUpdatedCallback(file, "Failed");
							err(error);
						});
					}
					else {
						err("Calling pop() on file upload queue returned no item");
					}
				}
			};

			for (let i = 0; i < nConcurrentUploads; i++) {
				fnStartNextUpload();
			}
		});
	}

	static uploadPhoto(file: globalThis.File): Promise<string> {
		return new Promise((ok, err) => {
			const xhr = new XMLHttpRequest();
			const formData = new FormData();
			formData.append(file.name, file);

			xhr.open("POST", PhotoService.baseUrl() + "/photo", true);
			xhr.onreadystatechange = function () {
				if (xhr.readyState == 4 && (xhr.status === 201)) {
					const response = JSON.parse(xhr.responseText);
					ok(response.id);
				} else if (xhr.readyState == 4 && xhr.status !== 201) {
					err();
				}
			};

			// Initiate a multipart/form-data upload
			xhr.send(formData);
		});
	}

	static getPhotoInfo(photoId: string): Promise<Photo> {
		return PhotoService.getJson<Photo>("GET", PhotoService.baseUrl() + "/photo/" + photoId, null);
	}

	static getPhotos(): Promise<Photo[]> {
		return PhotoService.getJson<Photo[]>("GET", PhotoService.baseUrl() + "/photos", null);
		// return new Promise((ok, err) => {
		// 	PhotoService.getJson<Photo[]>("GET", PhotoService.baseUrl() + "/photos", null)
		// 		.then((response) => {
		// 			let photos = !response ? [] : response.map((photo) => {
		// 				const ph: Photo = {
		// 					photo.id,
		// 					photo.width,
		// 					photo.height
		// 				};

		// 				return ph;
		// 			});

		// 			ok(photos);
		// 		})
		// 		.catch(err);
		// });
	}

	static deletePhotos(photoIds: string[]): Promise<Response> {
		return PhotoService.sendRequest("DELETE", PhotoService.baseUrl() + "/photos", photoIds);
	}

	static createAlbum(title: string, photoIds: string[]): Promise<string> {
		const createRequestData: CreateAlbum = {
			title
		};

		return new Promise((ok, err) => {
			PhotoService.getJson<CreatedResult>("POST", PhotoService.baseUrl() + "/album", createRequestData)
				.then((response) => {
					// TODO: allow setting photos and thumb in initial create call
					const albumId = response.id;

					if (photoIds) {
						const updateRequestData: UpdateAlbum = {
							title: title,
							thumbPhotoId: photoIds[0],
							photos: photoIds
						};

						PhotoService.updateAlbum(albumId, updateRequestData)
							.then(() => ok(albumId))
							.catch(err);
					} else {
						ok(albumId);
					}
				})
				.catch(err);
		});
	}

	static addPhotosToAlbum(albumId: string, photoIds: string[]): Promise<void> {
		return new Promise((ok, err) => {
			this.getAlbum(albumId)
				.then((album) => {
					const existingPhotoIds =  album.photos.map(photo => photo.id);
					const updatedAlbum: UpdateAlbum = {
						title: null,
						thumbPhotoId: null,
						photos: existingPhotoIds.concat(photoIds)
					};

					PhotoService.updateAlbum(albumId, updatedAlbum)
						.then(() => {
							ok();
						});
				})
				.catch(err);
		});
	}

	static getAlbums(): Promise<AlbumInfo[]> {
		return PhotoService.getJson<AlbumInfo[]>("GET", PhotoService.baseUrl() + "/albums", null);
	}

	static getAlbum(albumId: string): Promise<Album> {
		return PhotoService.getJson("GET", PhotoService.baseUrl() + "/album/" + albumId, null);
	}

	static deleteAlbum(albumId: string): Promise<Response> {
		return PhotoService.sendRequest("DELETE", PhotoService.baseUrl() + "/album/" + albumId, null);
	}

	static updateAlbumPhotos(albumId: string, newPhotoIds: string[]): Promise<Response> {
		return PhotoService.updateAlbum(albumId, {
			title: null,
			thumbPhotoId: null,
			photos: newPhotoIds
		});
	}

	static updateAlbumCover(albumId: string, newCoverPhotoId: string): Promise<Response> {
		return PhotoService.updateAlbum(albumId, {
			title: null,
			thumbPhotoId: newCoverPhotoId,
			photos: null
		});
	}

	static updateAlbum(albumId: string, albumObjectWithModifiedProperties: UpdateAlbum): Promise<Response> {
		return PhotoService.sendRequest("PUT", PhotoService.baseUrl() + "/album/" + albumId, albumObjectWithModifiedProperties);
	}

	static getCollections(): Promise<Collection[]> {
		return PhotoService.getJson<Collection[]>("GET", PhotoService.baseUrl() + "/collections", null);
	}

	static getCollection(collectionId: string): Promise<Collection> {
		return PhotoService.getJson<Collection>("GET", PhotoService.baseUrl() + "/collection/" + collectionId, null);
	}

	static getCollectionByShareToken(shareToken: string): Promise<Collection> {
		return PhotoService.getJson<Collection>("GET", PhotoService.baseUrl() + "/collection/shared/" + shareToken, null);
	}

	static authenticateToCollectionByShareToken(shareToken: string, password: string): Promise<Response> {
		const requestData = {
			password
		};

		return PhotoService.sendRequest("POST", `${PhotoService.baseUrl()}/collection/shared/${shareToken}/authenticate`, requestData);
	}

	static createCollection(title: string): Promise<string> {
		const requestData: CreateCollection = {
			title
		};

		return new Promise((ok, err) => {
			PhotoService.getJson<CreatedResult>("POST", PhotoService.baseUrl() + "/collection", requestData)
				.then((response) => {
					const collectionId = response.id;
					ok(collectionId);
				})
				.catch(err);
		});
	}

	static deleteCollection(collectionId: string): Promise<Response> {
		return PhotoService.sendRequest("DELETE", PhotoService.baseUrl() + "/collection/" + collectionId, null);
	}

	static addAlbumToCollection(collectionId: string, albumId: string): Promise<Response> {
		return new Promise((ok, err) => {
			PhotoService.getCollection(collectionId)
				.then(collection => {
					const albumIds = collection.albums.map(a => a.id);
					if (albumIds.indexOf(albumId) === -1) {
						const updatedAlbums = albumIds.concat(albumId);
						PhotoService.updateCollection(collectionId, {
							title: null,
							public: null,
							albums: updatedAlbums
						}).then(ok).catch(err);
					}
					else {
						err("Album already exists in collection");
					}
				})
				.catch(console.error);
		});
	}

	static removeAlbumFromCollection(collectionId: string, albumId: string): Promise<Response> {
		return new Promise((ok, err) => {
			PhotoService.getCollection(collectionId)
				.then(collection => {
					const albumIds = collection.albums.map(a => a.id);
					const albumIndex = albumIds.indexOf(albumId);
					if (albumIndex !== -1) {
						albumIds.splice(albumIndex, 1);
						PhotoService.updateCollection(collectionId, {
							title: null,
							public: null,
							albums: albumIds
						}).then(ok).catch(err);
					}
					else {
						err("Album does not exist in collection");
					}
				})
				.catch(console.error);
		});
	}

	static updateCollectionPublic(collectionId: string, bPublic: boolean): Promise<Response> {
		return PhotoService.updateCollection(collectionId, {
			title: null,
			public: bPublic,
			albums: null
		});
	}

	static updateCollection(collectionId: string, collectionObjectWithModifiedProperties: UpdateCollection): Promise<Response> {
		return PhotoService.sendRequest("PUT", PhotoService.baseUrl() + "/collection/" + collectionId, collectionObjectWithModifiedProperties);
	}

	static rotateCollectionShareToken(collectionId: string): Promise<Response> {
		return PhotoService.getJson("POST", PhotoService.baseUrl() + "/collection/" + collectionId + "/rotate-token", null);
	}

	/// Send a web request and gets json from the response body, returns a promise.
	static getJson<T>(method: string, url: string, data: unknown): Promise<T> {
		return new Promise((ok, err) => {
			this.sendRequest(method, url, data)
				.then((response) => {
					response.json()
						.then(ok)
						.catch(err);
				})
				.catch(err);
		});
	}

	/// Send a web request, returns a promise.
	static sendRequest(method: string, url: string, data: unknown): Promise<Response> {
		const options: RequestInit = {
			method,
			credentials: "include",
			body: data ? JSON.stringify(data) : null,
		};

		if (data) {
			options.headers = {
				"Content-Type": "application/json"
			};
			options.body = JSON.stringify(data);
		}

		return new Promise((ok, err) => {
			fetch(url, options)
				.then(response => {
					if (response.ok) {
						ok(response);
					}
					else{
						err(response);
					}
				})
				.catch(err);
		});
	}
}

export default PhotoService;