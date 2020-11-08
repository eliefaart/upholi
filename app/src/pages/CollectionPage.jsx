import PhotoService from "../services/PhotoService.ts"
import CollectionPageBase from "./CollectionPageBase.jsx";

class CollectionPage extends CollectionPageBase {
	constructor(props) {
		super(props, PhotoService.getCollection(props.match.params.collectionId));
	}
}

export default CollectionPage;