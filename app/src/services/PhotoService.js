import Photo from "../entities/Photo.js";

class PhotoService {
	static baseUrl() {
		return "/api";
	}

	static uploadPhotos(files, fnFileStatusUpdatedCallback) {
		const nConcurrentUploads = 3;

		// Create queue, and set initial status
		let queue = [];
		for (let file of files) {
			// Reverse array using unshift,
			// so we can use pop() when consuming array, which is faster than shift()
			queue.unshift(file);
			fnFileStatusUpdatedCallback(file, "Waiting");
		}

		let photoIdsUploaded = [];

		return new Promise((ok, err) => {
			let uploadPromises = [];

			let fnStartNextUpload = () => {
				if (queue.length > 0) {
					let file = queue.pop();
					let uploadPromise = PhotoService.uploadPhoto(file);
					uploadPromises.push(uploadPromise);
					fnFileStatusUpdatedCallback(file, "Uploading");

					uploadPromise.then((photoId) => {
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
			}

			for (let i = 0; i < nConcurrentUploads; i++) {
				fnStartNextUpload();
			}
		});
	}

	static uploadPhoto(file) {
		return new Promise((ok, err) => {
			const xhr = new XMLHttpRequest();
			const formData = new FormData();
			formData.append('file', file);

			xhr.open("POST", PhotoService.baseUrl() + "/photo", true);
			xhr.onreadystatechange = function (event) {
				if (xhr.readyState == 4 && (xhr.status === 201)) {
					let response = JSON.parse(xhr.responseText);
					ok(response.id);
				} else if (xhr.readyState == 4 && xhr.status !== 201) {
					err();
				}
			};

			// Initiate a multipart/form-data upload
			xhr.send(formData);
		});
	}

	static getPhotoInfo(photoId) {
		return PhotoService.getJson("GET", PhotoService.baseUrl() + "/photo/" + photoId);
	}

	static getPhotos() {
		return new Promise((ok, err) => {
			PhotoService.getJson("GET", PhotoService.baseUrl() + "/photos")
				.then((response) => {
					let photos = !response ? [] : response.map((photo) =>
						new Photo(photo.id, photo.width, photo.height)
					);

					ok(photos);
				})
				.catch(err);
		});
	}

	static deletePhotos(photoIds) {
		return PhotoService.sendRequest("DELETE", PhotoService.baseUrl() + "/photos", photoIds);
	}

	static createAlbum(title, photoIds) {
		let requestData = {
			title
		};

		return new Promise((ok, err) => {
			PhotoService.getJson("POST", PhotoService.baseUrl() + "/album", requestData)
				.then((response) => {
					// TODO: allow setting photos and thumb in initial create call
					let albumId = response.id;

					if (!!photoIds) {
						requestData.thumbPhotoId = photoIds[0];
						requestData.photos = photoIds;

						PhotoService.updateAlbum(albumId, requestData)
							.then(() => ok(albumId))
							.catch(err);
					} else {
						ok(albumId);
					}
				})
				.catch(err);
		});
	}

	static addPhotosToAlbum(albumId, photoIds) {
		return new Promise((ok, err) => {
			this.getAlbum(albumId)
				.then((album) => {
					let existingPhotoIds =  album.photos.map(photo => photo.id);
					let updatedAlbum = {
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

	static getAlbums() {
		return PhotoService.getJson("GET", PhotoService.baseUrl() + "/albums");
	}

	static getAlbum(albumId) {
		return PhotoService.getJson("GET", PhotoService.baseUrl() + "/album/" + albumId);
	}

	static deleteAlbum(albumId) {
		return PhotoService.sendRequest("DELETE", PhotoService.baseUrl() + "/album/" + albumId);
	}

	static updateAlbumPhotos(albumId, newPhotoIds) {
		return PhotoService.updateAlbum(albumId, {
			photos: newPhotoIds
		});
	}

	static updateAlbumCover(albumId, newCoverPhotoId) {
		return PhotoService.updateAlbum(albumId, {
			thumbPhotoId: newCoverPhotoId
		});
	}

	static updateAlbumPublic(albumId, bPublic) {
		return PhotoService.updateAlbum(albumId, {
			public: bPublic
		});
	}

	static updateAlbum(albumId, albumObjectWithModifiedProperties) {
		return PhotoService.sendRequest("PUT", PhotoService.baseUrl() + "/album/" + albumId, albumObjectWithModifiedProperties);
	}

	static getCollections() {
		return PhotoService.getJson("GET", PhotoService.baseUrl() + "/collections");
	}

	static getCollection(collectionId) {
		return PhotoService.getJson("GET", PhotoService.baseUrl() + "/collection/" + collectionId);
	}

	static getCollectionByShareToken(shareToken) {
		return PhotoService.getJson("GET", PhotoService.baseUrl() + "/collection/shared/" + shareToken);
	}

	static createCollection(title) {
		let requestData = {
			title
		};

		return new Promise((ok, err) => {
			PhotoService.getJson("POST", PhotoService.baseUrl() + "/collection", requestData)
				.then((response) => {
					let collectionId = response.id;
					ok(collectionId);
				})
				.catch(err);
		});
	}

	static deleteCollection(collectionId) {
		return PhotoService.sendRequest("DELETE", PhotoService.baseUrl() + "/collection/" + collectionId);
	}

	static addAlbumToCollection(collectionId, albumId) {
		return new Promise((ok, err) => {
			PhotoService.getCollection(collectionId)
				.then(collection => {
					let albumIds = collection.albums.map(a => a.id);
					if (albumIds.indexOf(albumId) === -1) {
						var updatedAlbums = albumIds.concat(albumId);
						PhotoService.updateCollection(collectionId, {
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

	static removeAlbumFromCollection(collectionId, albumId) {
		return new Promise((ok, err) => {
			PhotoService.getCollection(collectionId)
				.then(collection => {
					let albumIds = collection.albums.map(a => a.id);
					const albumIndex = albumIds.indexOf(albumId);
					if (albumIndex !== -1) {
						albumIds.splice(albumIndex, 1);
						PhotoService.updateCollection(collectionId, {
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

	static updateCollectionPublic(collectionId, bPublic) {
		return PhotoService.updateCollection(collectionId, {
			public: bPublic
		});
	}

	static updateCollection(collectionId, collectionObjectWithModifiedProperties) {
		return PhotoService.sendRequest("PUT", PhotoService.baseUrl() + "/collection/" + collectionId, collectionObjectWithModifiedProperties);
	}

	static rotateCollectionShareToken(collectionId) {
		return PhotoService.getJson("POST", PhotoService.baseUrl() + "/collection/" + collectionId + "/rotate-token");
	}

	/// Send a web request and gets json from the response body, returns a promise.
	static getJson(method, url, data) {
		return new Promise((ok, err) => {
			this.sendRequest(method, url, data)
			.then(response => {
				response.json()
					.then(ok)
					.catch(err);
			})
			.catch(err);
		});
	}

	/// Send a web request, returns a promise.
	static sendRequest(method, url, data) {
		const options = {
			method,
			credentials: "include",
			body: JSON.stringify(data)
		};

		if (!!data) {
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