import { useEffect, useState } from "react";
import { PhotoMinimal } from "../models/Photo";
import upholiService from "../services/UpholiService";

export default function usePhotos(): PhotoMinimal[] {
	const [photos, setPhotos] = useState<PhotoMinimal[]>([]);

	useEffect(() => {
		upholiService.getPhotos()
			.then(setPhotos)
			.catch(console.error);
	}, []);

	return photos;
}