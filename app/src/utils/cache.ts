/**
 * A very simple cache
 */
export class Cache {
	private cache: Map<string, unknown>;

	constructor() {
		this.cache = new Map<string, unknown>();
	}

	/**
	 * Try to get an item from cache by key.
	 * If the cache does not have any item with given key,
	 * then the 'getter' is executed and the result will be cached with 'key'.
	 * @param key
	 * @param getter
	 * @returns
	 */
	async getOr<T>(key: string, getter: () => Promise<T>): Promise<T> {
		if (this.cache.has(key)) {
			return this.cache.get(key) as T;
		}
		else {
			const value = await getter();
			this.cache.set(key, value);
			return value;
		}
	}

	/** Delete an item from the cache */
	delete(key: string): void {
		this.cache.delete(key);
	}
}