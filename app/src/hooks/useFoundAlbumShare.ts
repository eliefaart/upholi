import { useEffect, useState } from "react";
import { Share } from "../models/Share";
import upholiService from "../services/UpholiService";

export default function useFoundAlbumShare(albumId: string): Share | null {
	const [share, setShare] = useState<Share | null>(null);

	useEffect(() => {
		upholiService.findAlbumShare(albumId)
			.then(setShare)
			.catch(console.error);
	}, [albumId]);

	return share;
}