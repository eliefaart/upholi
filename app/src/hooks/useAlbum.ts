import { useEffect, useState } from "react";
import { Album } from "../models/Album";
import upholiService from "../services/UpholiService";

export default function useAlbum(albumId: string): Album | null {
	const [album, setAlbum] = useState<Album | null>(null);

	useEffect(() => {
		upholiService.getAlbum(albumId)
			.then(setAlbum)
			.catch(console.error);
	}, [albumId]);

	return album;
}