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

export {
    IconClose, IconDownload
};