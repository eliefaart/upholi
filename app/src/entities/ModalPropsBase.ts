export default interface ModalPropsBase {
	isOpen: boolean,
	className?: string,
	onRequestClose: () => void
}