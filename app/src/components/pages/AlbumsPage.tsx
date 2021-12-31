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
import DefaultHeaderContent from "../headers/DefaultHeaderContent";

const AlbumsPage: FC<PageProps> = (props: PageProps) => {
	const [newAlbumDialogOpen, setNewAlbumDialogOpen] = React.useState(false);
	const [albums] = useAlbums();
	const context = React.useContext(appStateContext);

	useTitle("Albums");
	React.useEffect(() => {
		props.setHeader({
			headerContentElement: <DefaultHeaderContent
				headerActions={<>
					{<button
						className="with-icon"
						onClick={() => setNewAlbumDialogOpen(true)}
						title="Create album">
						<IconCreate />Create album
					</button>}
				</>} />
		});
	}, []);

	return (
		<Content paddingTop={false}>
			<TagGroupedAlbums
				albums={albums}
				onAlbumClick={album => context.history.push("/album/" + album.id)} />

			<ModalCreateAlbum
				isOpen={newAlbumDialogOpen}
				onRequestClose={() => setNewAlbumDialogOpen(false)} />
		</Content>
	);
};

export default AlbumsPage;