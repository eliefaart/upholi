import PhotoService from "../services/PhotoService.js";

class Photo {

	constructor(id, width, height) {
		this.id = id;
		this.width = width;
		this.height = height;
	}

	getThumbUrl() {
		return this.getUrl("thumb");
	}

	getPreviewUrl() {
		return this.getUrl("preview");
	}

	getOriginalUrl() {
		return this.getUrl("original");
	}

	getUrl(variant) {
		return PhotoService.baseUrl() + "/photo/" + this.id + "/" + variant;
	}
}

export default Photo;