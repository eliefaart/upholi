import { useEffect, useState } from "react";
import upholiService from "../services/UpholiService";

export default function usePhotoThumbnailSource(photoId: string): string {
	const [source, setSource] = useState<string>("");

	useEffect(() => {
		if (photoId) {
			upholiService.getPhotoThumbnailImageSrc(photoId)
				.then(setSource);
		}
		else {
			setSource("");
		}
	}, [photoId]);


	return source;
}