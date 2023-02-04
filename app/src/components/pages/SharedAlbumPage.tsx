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

enum AuthorizedState {
	Undetermined,
	Authorized,
	Unauthorized,
}

const SharedAlbumPage: FC<Props> = (props: Props) => {
	const [authorized, setAuthorized] = useState(AuthorizedState.Undetermined);
	const [lastPasswordIncorrect, setLastPasswordIncorrect] = useState(false);
	const [album, setAlbum] = useState<Album | null>(null);
	const shareId = props.match.params.token;

	useTitle(album?.title ?? "");

	const setAuthorizedFromBoolean = (authorized: boolean) => setAuthorized(authorized ? AuthorizedState.Authorized : AuthorizedState.Unauthorized);

	const getAlbum = () => {
		upholiService.getShareAlbum(shareId)
			.then(setAlbum)
			.catch(console.error);
	};

	const tryauthorizeShare = (password: string): void => {
		if (password) {
			upholiService.authorizeShare(shareId, password)
				.then(authorized => {
					setAuthorizedFromBoolean(authorized);
					setLastPasswordIncorrect(!authorized);
				})
				.catch(console.error);
		}
		else {
			setLastPasswordIncorrect(false);
		}
	};

	React.useEffect(() => {
		upholiService.isAuthorizedForShare(shareId)
			.then(setAuthorizedFromBoolean)
			.catch(console.error);
	}, []);

	React.useEffect(() => {
		if (authorized == AuthorizedState.Authorized) {
			getAlbum();
		}
	}, [authorized]);

	return (
		<Content>
			{/* Password input box */}
			{authorized == AuthorizedState.Unauthorized && <InputPassword
				className="padding-top-50px"
				prompt="You need to provide a password to access this share."
				onSubmitPassword={(password) => tryauthorizeShare(password)}
				lastPasswordIncorrect={lastPasswordIncorrect} />}

			{!!album && <AlbumView album={album} />}
		</Content>
	);
};

export default SharedAlbumPage;