import { useEffect, useState } from "react";
import { AlbumNew } from "../models/Album";
import upholiService from "../services/UpholiService";

export default function useAlbums(): AlbumNew[] {
	const [data, setData] = useState<AlbumNew[]>([]);

	useEffect(() => {
		const fetchData = async () =>{
			const albums = await upholiService.getAlbums();
			setData(albums);
		};
		fetchData();
	});

	return data;
}