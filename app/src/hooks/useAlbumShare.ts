import { LibraryShare } from "../models/Share";
import upholiService from "../services/UpholiService";
import { useApiResource } from "./useApiResource";

export default function useAlbumShare(albumId: string): [LibraryShare | undefined, () => void] {
	return useApiResource<LibraryShare | undefined>(() => upholiService.getAlbumShare(albumId), undefined);
}