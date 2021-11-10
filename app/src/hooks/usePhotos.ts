import { PhotoMinimal } from "../models/Photo";
import upholiService from "../services/UpholiService";
import { useApiResource } from "./useApiResource";

export default function usePhotos(): [PhotoMinimal[], () => void] {
	return useApiResource<PhotoMinimal[]>(() => upholiService.getPhotos(), []);
}