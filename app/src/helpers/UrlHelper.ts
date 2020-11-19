import * as queryString from "query-string";

export default class UrlHelper {

	constructor() { }

	/**
	 * Get the value of a query string parameter. Returns null if parameter is not present in query string.
	 * @param qs Query string
	 * @param name Name of query string parameter
	 */
	static getQueryStringParamValue(qs: string, name: string) : string {
		const parsed = queryString.parse(qs);
		return parsed[name] as string;
	}

	/**
	 * Set a query string parameter to some value in a query string
	 * @param qs Query string
	 * @param name Name of query string parameter
	 * @param value Value of query string parameter
	 */
	static setQueryStringParam(qs: string, name: string, value: string) : string {
		const parsed = queryString.parse(qs);
		parsed[name] = value;

		return queryString.stringify(parsed);
	}

	/**
	 * Remove a query string parameter from a query string
	 * @param qs Query string
	 * @param name Name of query string parameter
	 */
	static removeQueryStringParam(qs: string, name: string) : string {
		const parsed = queryString.parse(qs);
		delete parsed[name];

		return queryString.stringify(parsed);
	}
}