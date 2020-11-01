import PhotoService from "../services/PhotoService";
import CollectionPageBase from "./CollectionPageBase.jsx";

class SharedCollectionPage extends CollectionPageBase {
	constructor(props) {
		super(props, PhotoService.getCollectionByShareToken(props.match.params.token));
	}
}

export default SharedCollectionPage;