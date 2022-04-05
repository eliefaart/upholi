import { AlbumPlain } from "../models/Album";
import upholiService from "../services/UpholiService";
import { useApiResource } from "./useApiResource";

export default function useAlbums(): AlbumPlain[] {
	const [albums] = useApiResource<AlbumPlain[]>(() => upholiService.getAlbums(), []);

	return albums;
}