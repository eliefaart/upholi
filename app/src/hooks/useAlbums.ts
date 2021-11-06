import { useEffect, useState } from "react";
import { AlbumNew } from "../models/Album";
import upholiService from "../services/UpholiService";

export default function useAlbums(): AlbumNew[] {
	const [albums, setAlbums] = useState<AlbumNew[]>([]);

	useEffect(() => {
		const fetchData = async () =>{
			setAlbums(await upholiService.getAlbums());
		};
		fetchData();
	});

	return albums;
}