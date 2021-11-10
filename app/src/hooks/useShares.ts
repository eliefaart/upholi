import { Share } from "../models/Share";
import upholiService from "../services/UpholiService";
import { useApiResource } from "./useApiResource";

export default function useShares(): [Share[], () => void] {
	return useApiResource<Share[]>(() => upholiService.getShares(), []);
}