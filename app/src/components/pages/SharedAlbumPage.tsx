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
	const [authorized, setAuthorized] = useState(true);
	const [lastPasswordIncorrect, setLastPasswordIncorrect] = useState(false);
	const [album, setAlbum] = useState<Album | null>(null);
	const token = props.match.params.token;

	useTitle(album?.title ?? "");

	const onReceiveAlbum = (album: Album) => {
		setAuthorized(true);
		setLastPasswordIncorrect(false);
		setAlbum(album);
	};

	const tryUnlockShare = (password: string): void => {
		if (password) {
			upholiService.getAlbumFromShare(token, password)
				.then(onReceiveAlbum)
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

	React.useEffect(() => {
		// Attempt to open share without password,
		// If this fails the share is password protected and we'll render a password input.
		upholiService.getAlbumFromShare(token, "")
			.then(onReceiveAlbum)
			.catch(() => setAuthorized(false));
	}, []);

	return (
		<Content>
			{/* Password input box */}
			{!authorized && <InputPassword
				className="padding-top-50px"
				prompt="You need to provide a password to access this share."
				onSubmitPassword={(password) => tryUnlockShare(password)}
				lastPasswordIncorrect={lastPasswordIncorrect} />}

			{!!album && <AlbumView album={album} />}
		</Content>
	);
};

export default SharedAlbumPage;