import { useEffect, useState } from "react";
import upholiService from "../services/UpholiService";

export default function usePhotoThumbnailSource(photoId: string, key?: string): string {
	const [source, setSource] = useState<string>("");

	useEffect(() => {
		if (photoId) {
			upholiService.getPhotoThumbnailImageSrc(photoId, key)
				.then(setSource);
		}
		else {
			setSource("");
		}
	}, [photoId]);


	return source;
}