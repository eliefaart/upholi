/**
 * Creates an absolute url to an OpenStreetMap location.
 * @param lat Latitude of location
 * @param lon Longtitude of location
 */
export function createMapsLocationUri(lat: number, lon: number): string {
  return "https://www.openstreetmap.org/#map=18/" + lat + "/" + lon;
}
