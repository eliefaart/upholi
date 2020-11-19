import PhotoService from "../services/PhotoService";
import CollectionPageBase from "./CollectionPageBase";
import { PageBaseComponentProps } from "../components/PageBaseComponent";


class CollectionPage extends CollectionPageBase {
	constructor(props: PageBaseComponentProps) {
		super(props, PhotoService.getCollection(props.match.params.collectionId));
	}
}

export default CollectionPage;