import { useEffect, useState } from "react";
import { Share } from "../models/Share";
import upholiService from "../services/UpholiService";
import { useApiResource } from "./useApiResource";

export default function useFoundAlbumShare(albumId: string): [Share | undefined, () => void] {
	return useApiResource<Share | undefined>(() => upholiService.findAlbumShare(albumId), undefined);
	//const [share, setShare] = useState<Share | null>(null);

	// useEffect(() => {
	// 	upholiService.findAlbumShare(albumId)
	// 		.then(setShare)
	// 		.catch(console.error);
	// }, [albumId]);

	// return share;
}