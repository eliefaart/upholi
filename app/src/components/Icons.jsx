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

class IconLink extends React.Component {
	constructor(props) {
		super(props);
	}

	render() {
		return (<svg viewBox="0 0 24 24">
			<path fill="currentColor" d="M3.9,12C3.9,10.29 5.29,8.9 7,8.9H11V7H7A5,5 0 0,0 2,12A5,5 0 0,0 7,17H11V15.1H7C5.29,15.1 3.9,13.71 3.9,12M8,13H16V11H8V13M17,7H13V8.9H17C18.71,8.9 20.1,10.29 20.1,12C20.1,13.71 18.71,15.1 17,15.1H13V17H17A5,5 0 0,0 22,12A5,5 0 0,0 17,7Z" />
		</svg>);
	}
}

export {
    IconClose, IconDownload, IconContextMenu, IconLink
};