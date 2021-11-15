import * as React from "react";
import { FC, useState } from "react";
import Content from "../layout/Content";
import { Album } from "../../models/Album";
import InputPassword from "../misc/InputPassword";
import upholiService from "../../services/UpholiService";
import AlbumView from "../AlbumView";
import { useTitle } from "../../hooks/useTitle";
import { PageProps } from "../../models/PageProps";

interface Props extends PageProps {
	// Note; this field represents the object set by react router
	match: {
		params: {
			token: string
		}
	};
}

const SharedAlbumPage: FC<Props> = (props: Props) => {
	const [authorized, setAuthorized] = useState(false);
	const [lastPasswordIncorrect, setLastPasswordIncorrect] = useState(false);
	const [album, setAlbum] = useState<Album | null>(null);

	useTitle(album?.title ?? "");

	const token = props.match.params.token;

	const tryUnlockShare = (password: string): void => {
		if (password) {
			upholiService.getAlbumFromShare(token, password)
				.then(album => {
					setAuthorized(true);
					setLastPasswordIncorrect(false);
					setAlbum(album);
				})
				.catch(error => {
					if (error) {
						console.log(error);
					}

					setLastPasswordIncorrect(true);
				});
		}
		else {
			setLastPasswordIncorrect(false);
		}
	};

	return (
		<Content>
			{/* Password input box */}
			{!authorized && <InputPassword
				className="padding-top-50px"
				prompt="You need to provide a password to access this share."
				onSubmitPassword={(password) => tryUnlockShare(password)}
				lastPasswordIncorrect={lastPasswordIncorrect}/>}

			{album != null && <AlbumView album={album} />}
		</Content>
	);
};

export default SharedAlbumPage;