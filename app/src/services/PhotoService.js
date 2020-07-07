import $ from 'jquery';

class PhotoService {

	constructor(props) {
	}
	
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

	static getPhotos(callback) {
		$.get(PhotoService.baseUrl() + "/photos")
			.done((response) => {
				let photos = !response ? [] : response.map((photo) => {
					return {
						id: photo.id,
						src: PhotoService.baseUrl() + "/photo/" + photo.id + "/thumb",
						width: photo.width,
						height: photo.height
					};
				});

				if (callback)
					callback(photos);
			})
			.fail((error) => console.log("Get failed", error));
	}

	static getPhoto(photoId) {
		let url = PhotoService.baseUrl() + "/photo/" + photoId;
		
		return new Promise((ok, err) => {
			$.get(url).done(ok).fail(err);
		});
	}

	static deletePhotos(photoIds, callback) {
		$.ajax({
			url: PhotoService.baseUrl() + "/photos",
			type: "DELETE",
			data: JSON.stringify(photoIds),
			contentType: "application/json"
		})
		.done(() => {
			if (callback)
				callback();
		})
		.fail((error) => {
			console.log("Delete failed", error);
		});
	}

	static createAlbum(title, photoIds, callback) {
		let requestData = {
			title
		};

		$.ajax({
			url: PhotoService.baseUrl() + "/album",
			type: "POST",
			data: JSON.stringify(requestData),
			contentType: "application/json"
		}).done((response) => {
			// TODO: allow setting photos and thumb in initial create call
			let albumId = response.id;

			if (!!photoIds) {
				requestData.thumbPhotoId = photoIds[0];
				requestData.photos = photoIds;

				PhotoService.updateAlbum(albumId, requestData)
					.then(() => {
						if (callback)
							callback(albumId);
					});
			} else {
				if (callback)
					callback(albumId);
			}
		}).fail((response) => console.error(response.responseJSON));
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
				.catch(error => err(error));
		});
	}

	static getAlbums() {
		return new Promise((ok, err) => {
			$.get(PhotoService.baseUrl() + "/albums")
				.done((response) => ok(response))
				.fail((error) => err(error));
		});
	}

	static getAlbum(albumId) {
		return PhotoService.getAlbumInfo(PhotoService.baseUrl() + "/album/" + albumId);
	}

	static deleteAlbum(albumId) {
		return new Promise((ok, err) => {
			$.ajax({
				url: PhotoService.baseUrl() + "/album/" + albumId,
				type: "DELETE"
			})
			.done(() => ok())
			.fail((error) => err(error));
		});
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
		return new Promise((ok, err) => {
			$.ajax({
				url: PhotoService.baseUrl() + "/album/" + albumId,
				type: "PUT",
				data: JSON.stringify(albumObjectWithModifiedProperties),
				contentType: "application/json"
			})
			.done(() => ok())
			.fail((error) => err(error));
		});
	}

	static getSharedCollection(collectionId) {
		return PhotoService.getAlbumInfo(PhotoService.baseUrl() + "/pub/collection/" + collectionId);
	}

	static getSharedCollectionPhoto(collectionId, photoId) {
		let url = PhotoService.baseUrl() + "/pub/collection/" + collectionId + "/photo/" + photoId;
		
		return new Promise((ok, err) => {
			$.get(url).done(ok).fail(err);
		});
	}

	static getAlbumInfo(url) {
		return new Promise((ok, err) => {
			$.get(url)
				.done((response) => {
					ok(response);
				})
				.fail((error) => err(error));
		});
	}
}

export default PhotoService;