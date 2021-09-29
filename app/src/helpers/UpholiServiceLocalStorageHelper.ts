const LOCAL_STORAGE_KEY = "upholiService";

/**
 * Data this service stores in local storage
 */
interface LocalStorageData {
	key: string
}

/**
 * Helper class that takes care of storing data for UpholiService in localStorage.
 */
export default class UpholiServiceLocalStorageHelper {
	/**
	 * Gets currently stored master encryption key
	 */
	static getKey(): string | null {
		const localStorageDataJson = localStorage.getItem(LOCAL_STORAGE_KEY);
		if (localStorageDataJson) {
			const localStorageData: LocalStorageData = JSON.parse(localStorageDataJson);
			return localStorageData.key;
		}
		else {
			return null;
		}
	}

	/**
	 * Store a master encryption key
	 */
	static storeKey(key: string): void {
		// TODO: How to invalidate this when session changes,
		// how to keep it in sync with session cookie?
		// if session cookie expires.. the localStorage will still be there.

		const localStorageData: LocalStorageData = {
			key
		};
		const localStorageDataJson = JSON.stringify(localStorageData);
		localStorage.setItem(LOCAL_STORAGE_KEY, localStorageDataJson);
	}

	/**
	 * Delete all stored localStorage data managed by this class
	 */
	static clear(): void {
		localStorage.removeItem(LOCAL_STORAGE_KEY);
	}
}