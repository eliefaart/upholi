import { PageBaseComponentProps } from "../components/PageBaseComponent";
import PhotoService from "../services/PhotoService";
import CollectionPageBase from "./CollectionPageBase";

class SharedCollectionPage extends CollectionPageBase {
	constructor(props: PageBaseComponentProps) {
		super(props, PhotoService.getCollectionByShareToken(props.match.params.token));
	}
}

export default SharedCollectionPage;