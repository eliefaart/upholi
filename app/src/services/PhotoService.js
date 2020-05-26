import $ from 'jquery';

class PhotoService {

	constructor(props) {
	}
	
	static baseUrl() {
		return "http://127.0.0.1:8000";
	}

	static uploadPhotos(files) {
		let uploadPromises = [];
		for (let file of files) {
			let uploadPromise = PhotoService.uploadPhoto(file);
			uploadPromises.push(uploadPromise);
		}

		return Promise.all(uploadPromises);
	}

	static uploadPhotos2(files, fnFileUploadedCallback) {
		const nConcurrentUploads = 5;

		// Create queue: reverse files array so we can use pop() which is faster than shift()
		let queue = [];
		for (let file of files) 
			queue.push(file);

		return new Promise((ok, err) => {
			let uploadPromises = [];

			let fnStartNextUpload = () => {
				if (queue.length > 0) {
					let file = queue.pop();
					let uploadPromise = PhotoService.uploadPhoto(file);
					uploadPromises.push(uploadPromise);
	
					uploadPromise.then(() => {
						uploadPromises.splice(uploadPromises.indexOf(uploadPromise), 1);
	
						fnFileUploadedCallback(file, true);
						fnStartNextUpload();
	
						if (queue.length === 0 && uploadPromises.length === 0) {
							ok();
						}
					}).catch((error) => {
						fnFileUploadedCallback(file, false);
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
				if (xhr.readyState == 4 && xhr.status === 200) {
					let response = JSON.parse(xhr.responseText);
					ok(response.id);
				} else if (xhr.readyState == 4 && xhr.status !== 200) {
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

			requestData.thumbPhotoId = photoIds[0];
			requestData.photos = photoIds;

			PhotoService.updateAlbum(albumId, requestData)
				.then(() => {
					if (callback)
						callback(albumId);
				});
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
		return new Promise((ok, err) => {
			$.get(PhotoService.baseUrl() + "/album/" + albumId)
				.done((response) => {
					ok(response);
				})
				.fail((error) => err(error));
		});
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
}

export default PhotoService;