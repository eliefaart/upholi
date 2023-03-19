/**
 * Copies the content of given HTML element to clipboard.
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

/**
 * Check if given element is at least partially visible within viewport.
 */
export function elementIsInViewport(element: HTMLElement): boolean {
  if (!element) {
    return false;
  } else {
    const myElementHeight = element.offsetHeight;
    const myElementWidth = element.offsetWidth;
    const bounding = element.getBoundingClientRect();

    return (
      bounding.top >= -myElementHeight &&
      bounding.left >= -myElementWidth &&
      bounding.right <= (window.innerWidth || document.documentElement.clientWidth) + myElementWidth &&
      bounding.bottom <= (window.innerHeight || document.documentElement.clientHeight) + myElementHeight
    );
  }
}
