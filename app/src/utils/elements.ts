/**
 * Copies the content of given HTML element to clipboard
 */
export function copyElementContentToClipboard(element: HTMLInputElement): void {
	// Select text
	element.select();
	element.setSelectionRange(0, 99999);

	// Copy to clipboard
	document.execCommand("copy");

	// Deselect text
	element.blur();
}