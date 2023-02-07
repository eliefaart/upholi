import { LibraryShare } from "../models/Share";
import upholiService from "../services/UpholiService";
import { useApiResource } from "./useApiResource";

export default function useShares(): [LibraryShare[], () => void] {
	return useApiResource<LibraryShare[]>(() => upholiService.getShares(), []);
}