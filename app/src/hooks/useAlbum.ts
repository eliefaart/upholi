import { AlbumHydrated } from "../models/Album";
import upholiService from "../services/UpholiService";
import { useApiResource } from "./useApiResource";

export default function useAlbum(albumId: string): [AlbumHydrated | undefined, () => void] {
  return useApiResource<AlbumHydrated | undefined>(() => upholiService.getAlbum(albumId), undefined);
}
