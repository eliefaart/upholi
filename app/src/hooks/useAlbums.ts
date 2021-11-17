import { AlbumPlain } from "../models/Album";
import upholiService from "../services/UpholiService";
import { useApiResource } from "./useApiResource";

export default function useAlbums(): [AlbumPlain[], () => void] {
	return useApiResource<AlbumPlain[]>(() => upholiService.getAlbums(), []);
}