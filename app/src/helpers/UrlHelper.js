import queryString from "query-string";

export default class UrlHelper {

	constructor() { }

	/**
	 * Add a query string parameter to a query string
	 * @param {string} qs Query string to add parameter to
	 * @param {string} name Name of query string parameter
	 * @param {string} value Value of query string parameter
	 */
	static addQueryStringParam(qs, name, value) {
		const parsed = queryString.parse(qs);
		parsed[name] = value;

		return queryString.stringify(parsed);
	}

	/**
	 * Remove a query string parameter from a query string
	 * @param {string} qs Query string to remove parameter from
	 * @param {string} name Name of query string parameter
	 */
	static removeQueryStringParam(qs, name) {
		const parsed = queryString.parse(qs);
		delete parsed[name];

		return queryString.stringify(parsed);
	}
}