class PhotoService {
	static baseUrl() {
		return "/api";
	}

	static uploadPhotos(files, fnFileStatusUpdatedCallback) {
		const nConcurrentUploads = 3;

		// Create queue: reverse files array so we can use pop() which is faster than shift()
		let queue = [];
		for (let file of files) {
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

	static getPhotos() {
		return new Promise((ok, err) => {
			PhotoService.getJson("GET", PhotoService.baseUrl() + "/photos")
				.then((response) => {
					let photos = !response ? [] : response.map((photo) => {
						return {
							id: photo.id,
							src: PhotoService.baseUrl() + "/photo/" + photo.id + "/thumb",
							width: photo.width,
							height: photo.height
						};
					});

					ok(photos);
				})
				.catch(err);
		});
	}

	static deletePhotos(photoIds) {
		return PhotoService.sendRequest("DELETE", PhotoService.baseUrl() + "/photos", photoIds);
	}

	static createAlbum(title, photoIds, callback) {
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
		return PhotoService.getAlbumInfo(PhotoService.baseUrl() + "/album/" + albumId);
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

	static getSharedCollection(collectionId) {
		return PhotoService.getAlbumInfo(PhotoService.baseUrl() + "/pub/collection/" + collectionId);
	}

	static getPhotoInfo(url) {
		return PhotoService.getJson("GET", url);
	}

	static getAlbumInfo(url) {
		return PhotoService.getJson("GET", url);
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