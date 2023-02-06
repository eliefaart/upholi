import { Album } from "../models/Album";
import upholiService from "../services/UpholiService";
import { useApiResource } from "./useApiResource";

export default function useAlbums(): Album[] {
	const [albums] = useApiResource<Album[]>(() => upholiService.getAlbums(), []);

	return albums;
}