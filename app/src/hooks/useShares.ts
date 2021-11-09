import { useEffect, useState } from "react";
import { Share } from "../models/Share";
import upholiService from "../services/UpholiService";

export default function useShares(): Share[] {
	const [shares, setShares] = useState<Share[]>([]);

	useEffect(() => {
		upholiService.getShares()
			.then(setShares)
			.catch(console.error);
	}, []);

	return shares;
}