import Exif from "./Exif";

export interface PhotoMinimal {
  id: string;
  width: number;
  height: number;
}

export interface Photo {
  id: string;
  hash: string;
  width: number;
  height: number;
  contentType: string;
  exif: Exif;
}
