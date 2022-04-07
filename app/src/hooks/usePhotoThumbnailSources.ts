import { useEffect, useState } from "react";
import { AlbumPhoto } from "../models/Album";
import { PhotoMinimal } from "../models/Photo";
import upholiService from "../services/UpholiService";

interface PhotoSource {
	photoId: string,
	src: string
}

export default function usePhotoThumbnailSources(photos: (PhotoMinimal | AlbumPhoto)[]): PhotoSource[] {
	const [sources, setSources] = useState<PhotoSource[]>([]);

	useEffect(() => {
		// Clear the ones no longer needed
		const photoSourcesStillRelavant = sources.filter(src => photos.some(p => p.id === src.photoId));
		if (photoSourcesStillRelavant.length !== sources.length) {
			setSources(photoSourcesStillRelavant);
		}

		// Fetch photos we don't have in state yet.
		const notYetFetched = photos.filter(photo => !sources.some(ci => ci.photoId === photo.id));
		for (const photo of notYetFetched) {
			upholiService.getPhotoThumbnailImageSrc(photo.id, (<AlbumPhoto>photo).key ?? undefined)
				.then(src => {
					setSources(prev => {
						const updated = [...prev];
						updated.push({
							photoId: photo.id,
							src
						});
						return updated;
					});
				});
		}
	}, [photos]);

	return sources;
}