import React from 'react';

class PhotoDetail extends React.Component {
	constructor(props) {
		super(props);
	}

	render() {
		let containerStyle = {
			"width": "100%",
			"maxWidth": "100%",
			"height": "100%",
			"maxHeight": "100%",
			"display": "flex",
			"alignItems": "center",
			"justifyContent": "center"
		};

		return <div style={containerStyle} className="photoDetail">
			<img src={this.props.src} />
		</div>;
	}
}

export default PhotoDetail;