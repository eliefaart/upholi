import * as React from "react";
import { FC } from "react";
import Content from "../layout/Content";
import ModalCreateAlbum from "../modals/ModalCreateAlbum";
import appStateContext from "../../contexts/AppStateContext";
import { IconCreate } from "../misc/Icons";
import { useTitle } from "../../hooks/useTitle";
import useAlbums from "../../hooks/useAlbums";
import { PageProps } from "../../models/PageProps";
import TagGroupedAlbums from "../misc/TagGroupedAlbums";

const AlbumsPage: FC<PageProps> = (props: PageProps) => {
	const [ newAlbumDialogOpen, setNewAlbumDialogOpen ] = React.useState(false);
	const [albums] = useAlbums();
	const context = React.useContext(appStateContext);

	useTitle("Albums");
	React.useEffect(() => {
		props.setHeader({
			visible: true,
			headerActions: <React.Fragment>
				{<button
					className="iconOnly"
					onClick={() => setNewAlbumDialogOpen(true)}
					title="Create album">
					<IconCreate/>
				</button>}
			</React.Fragment>
		});
	}, []);

	return (
		<Content paddingTop={false}>
			<TagGroupedAlbums
				albums={albums}
				onAlbumClick={album => context.history.push("/album/" + album.id)}/>

			<ModalCreateAlbum
				isOpen={newAlbumDialogOpen}
				onRequestClose={() => setNewAlbumDialogOpen(false)}/>
		</Content>
	);
};

export default AlbumsPage;