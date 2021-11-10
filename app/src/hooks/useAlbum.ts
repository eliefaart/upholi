import { Album } from "../models/Album";
import upholiService from "../services/UpholiService";
import { useApiResource } from "./useApiResource";

export default function useAlbum(albumId: string): [Album | undefined, () => void] {
	return useApiResource<Album | undefined>(() => upholiService.getAlbum(albumId), undefined);
}