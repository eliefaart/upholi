import { PhotoMinimal } from "./Photo";

/**
 * Album info, contains basic information of the photos it contains.
 */
export interface AlbumHydrated {
  id: string;
  title: string;
  thumbPhoto: PhotoMinimal | null;
  photos: PhotoMinimal[];
  tags: string[];
}

/**
 * Album info, only contains IDs of photos it contains.
 */
export interface Album {
  id: string;
  title: string;
  tags: string[];
  photos: string[];
  thumbnailPhotoId: string;
}
