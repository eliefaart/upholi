import * as React from "react";
import {FC} from "react";
import Content from "../layout/Content";
import appStateContext from "../../contexts/AppStateContext";
import upholiService from "../../services/UpholiService";
import { Share } from "../../models/Share";
import CopyUrl from "../misc/CopyUrl";
import { useTitle } from "../../hooks/useTitle";
import { setHeader } from "../../hooks/useHeader";
import useAlbums from "../../hooks/useAlbums";
import useShares from "../../hooks/useShares";

const SharedPage: FC = () => {
	const context = React.useContext(appStateContext);
	const albums = useAlbums();
	const shares = useShares();

	useTitle("Shared");
	setHeader({
		visible: true
	});

	const deleteShare = (share: Share): void => {
		upholiService.deleteShare(share.id)
			.then(() => {
				// update shares hook somehow?
			})
			.catch(console.error);
	};

	return (
		<Content paddingTop={true} className="shares">
			{shares.map(share => {
				const shareUrl = document.location.origin + "/s/" + share.id;
				const shareAlbum = albums.find(album => album.id === share.data.album.albumId);

				return <div key={share.id} className="share">
					<div className="head">
						<h2 onClick={() => context.history.push("/album/" + shareAlbum?.id)}>
							{shareAlbum?.title}
						</h2>
					</div>
					<div className="body">
						<CopyUrl shareUrl={shareUrl}/>
						<div className="actions">
						<button onClick={() => deleteShare(share)}>
							Delete share
						</button>
						</div>
					</div>
				</div>;
			})}
		</Content>
	);
};

export default SharedPage;