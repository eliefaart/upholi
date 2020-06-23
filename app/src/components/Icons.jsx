import React from 'react';

class IconClose extends React.Component {
	constructor(props) {
		super(props);
	}

	render() {
		return (<svg viewBox="0 0 24 24">
			<path fill="currentColor" d="M19,6.41L17.59,5L12,10.59L6.41,5L5,6.41L10.59,12L5,17.59L6.41,19L12,13.41L17.59,19L19,17.59L13.41,12L19,6.41Z" />
		</svg>);
	}
}

class IconDownload extends React.Component {
	constructor(props) {
		super(props);
	}

	render() {
		return (<svg viewBox="0 0 24 24">
			<path fill="currentColor" d="M5,20H19V18H5M19,9H15V3H9V9H5L12,16L19,9Z" />
		</svg>);
	}
}

class IconContextMenu extends React.Component {
	constructor(props) {
		super(props);
	}

	render() {
		return (<svg viewBox="0 0 24 24">
			<path fill="currentColor" d="M12,16A2,2 0 0,1 14,18A2,2 0 0,1 12,20A2,2 0 0,1 10,18A2,2 0 0,1 12,16M12,10A2,2 0 0,1 14,12A2,2 0 0,1 12,14A2,2 0 0,1 10,12A2,2 0 0,1 12,10M12,4A2,2 0 0,1 14,6A2,2 0 0,1 12,8A2,2 0 0,1 10,6A2,2 0 0,1 12,4Z" />
		</svg>);
	}
}

export {
    IconClose, IconDownload, IconContextMenu
};