import queryString from "query-string";

export default class UrlHelper {

	constructor() { }

	/**
	 * Get the value of a query string parameter. Returns null if parameter is not present in query string.
	 * @param {string} qs Query string
	 * @param {string} name Name of query string parameter
	 */
	static getQueryStringParamValue(qs, name) {
		const parsed = queryString.parse(qs);
		return parsed[name] || null;
	}

	/**
	 * Set a query string parameter to some value in a query string
	 * @param {string} qs Query string
	 * @param {string} name Name of query string parameter
	 * @param {string} value Value of query string parameter
	 */
	static setQueryStringParam(qs, name, value) {
		const parsed = queryString.parse(qs);
		parsed[name] = value;

		return queryString.stringify(parsed);
	}

	/**
	 * Remove a query string parameter from a query string
	 * @param {string} qs Query string
	 * @param {string} name Name of query string parameter
	 */
	static removeQueryStringParam(qs, name) {
		const parsed = queryString.parse(qs);
		delete parsed[name];

		return queryString.stringify(parsed);
	}
}