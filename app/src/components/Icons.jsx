import React from 'react';

class Icon extends React.Component {
	constructor(props) { super(props); }

	render() {
		return (<svg viewBox="0 0 24 24">
			{this.props.children}
		</svg>);
	}
}

class IconClose extends React.Component {
	constructor(props) { super(props); }

	render() { 
		return (<Icon>
			<path fill="currentColor" d="M19,6.41L17.59,5L12,10.59L6.41,5L5,6.41L10.59,12L5,17.59L6.41,19L12,13.41L17.59,19L19,17.59L13.41,12L19,6.41Z" />
		</Icon>);
	}
}

class IconCopy extends React.Component {
	constructor(props) { super(props); }

	render() {
		return (<Icon>
			<path fill="currentColor" d="M19,21H8V7H19M19,5H8A2,2 0 0,0 6,7V21A2,2 0 0,0 8,23H19A2,2 0 0,0 21,21V7A2,2 0 0,0 19,5M16,1H4A2,2 0 0,0 2,3V17H4V3H16V1Z" />
		</Icon>);
	}
}

class IconDownload extends React.Component {
	constructor(props) { super(props); }

	render() {
		return (<Icon>
			<path fill="currentColor" d="M5,20H19V18H5M19,9H15V3H9V9H5L12,16L19,9Z" />
		</Icon>);
	}
}

class IconUpload extends React.Component {
	constructor(props) { super(props); }

	render() {
		return (<Icon>
			<path fill="currentColor" d="M9,16V10H5L12,3L19,10H15V16H9M5,20V18H19V20H5Z" />
		</Icon>);
	}
}

class IconContextMenu extends React.Component {
	constructor(props) { super(props); }

	render() {
		return (<Icon>
			<path fill="currentColor" d="M12,16A2,2 0 0,1 14,18A2,2 0 0,1 12,20A2,2 0 0,1 10,18A2,2 0 0,1 12,16M12,10A2,2 0 0,1 14,12A2,2 0 0,1 12,14A2,2 0 0,1 10,12A2,2 0 0,1 12,10M12,4A2,2 0 0,1 14,6A2,2 0 0,1 12,8A2,2 0 0,1 10,6A2,2 0 0,1 12,4Z" />
		</Icon>);
	}
}

class IconLink extends React.Component {
	constructor(props) { super(props); }

	render() {
		return (<Icon>
			<path fill="currentColor" d="M3.9,12C3.9,10.29 5.29,8.9 7,8.9H11V7H7A5,5 0 0,0 2,12A5,5 0 0,0 7,17H11V15.1H7C5.29,15.1 3.9,13.71 3.9,12M8,13H16V11H8V13M17,7H13V8.9H17C18.71,8.9 20.1,10.29 20.1,12C20.1,13.71 18.71,15.1 17,15.1H13V17H17A5,5 0 0,0 22,12A5,5 0 0,0 17,7Z" />
		</Icon>);
	}
}

class IconDelete extends React.Component {
	constructor(props) { super(props); }

	render() {
		return (<Icon>
			<path fill="currentColor" d="M19,4H15.5L14.5,3H9.5L8.5,4H5V6H19M6,19A2,2 0 0,0 8,21H16A2,2 0 0,0 18,19V7H6V19Z" />
		</Icon>);
	}
}

class IconCreate extends React.Component {
	constructor(props) { super(props); }

	render() {
		return (<Icon>
			<path fill="currentColor" d="M19,19V5H5V19H19M19,3A2,2 0 0,1 21,5V19A2,2 0 0,1 19,21H5A2,2 0 0,1 3,19V5C3,3.89 3.9,3 5,3H19M11,7H13V11H17V13H13V17H11V13H7V11H11V7Z" />
		</Icon>);
	}
}

class IconRemove extends React.Component {
	constructor(props) { super(props); }

	render() {
		return (<Icon>
		 	<path fill="currentColor" d="M18 11H10V9H18M20 4V16H8V4H20M20 2H8C6.9 2 6 2.9 6 4V16C6 17.11 6.9 18 8 18H20C21.11 18 22 17.11 22 16V4C22 2.9 21.11 2 20 2M4 6H2V20C2 21.11 2.9 22 4 22H18V20H4V6Z" />
		</Icon>);
	}
}

class IconAddToAlbum extends React.Component {
	constructor(props) { super(props); }

	render() {
		return (<Icon>
			<path fill="currentColor" d="M18 11H15V14H13V11H10V9H13V6H15V9H18M20 4V16H8V4H20M20 2H8C6.9 2 6 2.9 6 4V16C6 17.11 6.9 18 8 18H20C21.11 18 22 17.11 22 16V4C22 2.9 21.11 2 20 2M4 6H2V20C2 21.11 2.9 22 4 22H18V20H4V6Z" />
		</Icon>);
	}
}

class IconImage extends React.Component {
	constructor(props) { super(props); }

	render() {
		return (<Icon>
			<path fill="currentColor" d="M14,2L20,8V20A2,2 0 0,1 18,22H6A2,2 0 0,1 4,20V4A2,2 0 0,1 6,2H14M18,20V9H13V4H6V20H18M17,13V19H7L12,14L14,16M10,10.5A1.5,1.5 0 0,1 8.5,12A1.5,1.5 0 0,1 7,10.5A1.5,1.5 0 0,1 8.5,9A1.5,1.5 0 0,1 10,10.5Z" />
		</Icon>);
	}
}



export {
	IconClose, IconCopy, IconDownload, IconUpload, IconContextMenu,
	IconRemove, IconDelete, IconCreate,
	IconLink, IconImage, IconAddToAlbum
}