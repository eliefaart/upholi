import { useRef, useState } from "react";
import { PhotoMinimal } from "../models/Photo";
import upholiService from "../services/UpholiService";

interface PhotoSource {
	photoId: string,
	src: string
}

const cache: PhotoSource[] = [];

export default function usePhotoThumbnailSources(photos: PhotoMinimal[]): PhotoSource[] {
	const [sources, setSources] = useState<PhotoSource[]>([]);
	const sourcesRef = useRef<PhotoSource[]>([]);
	sourcesRef.current = sources;

	// Find photos that we still have to fetch.
	const notInCache = photos.filter(photo => !cache.some(ci => ci.photoId === photo.id));

	// fetch source for new photos
	for (const photo of notInCache) {
		const cacheItem = {
			photoId: photo.id,
			src: ""
		};
		cache.push(cacheItem);

		upholiService.getPhotoThumbnailImageSrc(photo.id)
			.then(src => {
				// Update the cache
				cacheItem.src = src;

				// Set sources from cache
				setSources(cache.filter(ci => photos.some(photo => photo.id === ci.photoId)));
			});
	}

	// Initial sources from cache
	const notInSources = photos.filter(photo => !sources.some(p => p.photoId === photo.id));
	if (notInSources.length > 0) {
		const toAddToSources = cache.filter(ci => notInSources.some(photo => photo.id === ci.photoId));
		setSources(sources.concat(toAddToSources));
	}

	return sources;
}