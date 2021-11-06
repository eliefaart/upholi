import { useEffect, useState } from "react";
import { AlbumNew } from "../models/Album";
import upholiService from "../services/UpholiService";

export default function useAlbums(): AlbumNew[] {
	const [albums, setAlbums] = useState<AlbumNew[]>([]);

	useEffect(() => {
		upholiService.getAlbums()
			.then(setAlbums)
			.catch(console.error);
	}, []);

	return albums;
}