export interface Share {
  id: string;
  password: string;
  type: "album";
  data: AlbumShareData;
}

interface AlbumShareData {
  album: {
    albumId: string;
  };
}

export interface LibraryShare {
  id: string;
  password: string;
  albumId: string;
}
