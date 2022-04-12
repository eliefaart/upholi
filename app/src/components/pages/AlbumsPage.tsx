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
import Button from "../misc/Button";

const AlbumsPage: FC<PageProps> = (props: PageProps) => {
	const [newAlbumDialogOpen, setNewAlbumDialogOpen] = React.useState(false);
	const albums = useAlbums();
	const context = React.useContext(appStateContext);

	useTitle("Albums");
	React.useEffect(() => {
		props.setHeader({
			headerContentElement: <DefaultHeaderContent
				actions={<>
					<Button onClick={() => setNewAlbumDialogOpen(true)}
						label="Create album"
						icon={<IconCreate />} />
				</>} />
		});
	}, []);

	/**
	 * Sort albums.
	 * Basically albums are sorted by title alphabetically.
	 * However, albums with a year (eg 2022) in the title are moved to the front and are sorted by year descending.
	 * It's a bit magical, but that is fine for now. :D
	 */
	const sortedAlbums = albums.sort((one, two) => {
		const regex = new RegExp(/.?[0-9]{4}.?/);

		// Create a string that album can be sorted by, and returns if album title contains a year.
		const getSortingTitle = (title: string): [string, boolean] => {
			const matches = regex.exec(title);
			const containsYear = matches !== null && matches.length > 0;

			if (containsYear) {
				const year = matches[0];
				title = year + title;
			}

			return [title, containsYear];
		};

		const [oneTitle, oneContainsYear] = getSortingTitle(one.title);
		const [twoTitle, twoContainsYear] = getSortingTitle(two.title);

		if (oneContainsYear && twoContainsYear) {
			// Sort as descending
			return oneTitle > twoTitle ? -1 : 1;
		}
		else if (oneContainsYear !== twoContainsYear) {
			// Sort so that the title with the year is before the one without year
			return oneContainsYear ? -1 : 1;
		}
		else {
			// Sort as ascending
			return oneTitle > twoTitle ? 1 : -1;
		}
	});

	return (
		<Content paddingTop={false}>
			<TagGroupedAlbums
				albums={sortedAlbums}
				onAlbumClick={album => context.history.push("/album/" + album.id)} />

			<ModalCreateAlbum
				isOpen={newAlbumDialogOpen}
				onRequestClose={() => setNewAlbumDialogOpen(false)} />
		</Content>
	);
};

export default AlbumsPage;