import { AlbumNew } from "../models/Album";
import upholiService from "../services/UpholiService";
import { useApiResource } from "./useApiResource";

export default function useAlbums(): [AlbumNew[], () => void] {
	return useApiResource<AlbumNew[]>(() => upholiService.getAlbums(), []);
}