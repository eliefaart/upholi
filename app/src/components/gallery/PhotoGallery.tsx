import * as React from "react";
import { FC } from "react";
import { default as Gallery, PhotoProps } from "react-photo-gallery";
import { default as GalleryPhotoModel } from "../../models/GalleryPhoto";
import GalleryPhoto from "./GalleryPhoto";

/** Target height for algorithm, but exact height will vary a bit. */
const PHOTO_HEIGHT = 200;
const PHOTO_MARGIN = 3;

interface Props {
  photos: GalleryPhotoModel[];
  selectedItems: string[];
  onPhotoSelectionChanged?: (photoIds: string[]) => void;
  onClick: (photoId: string) => void;
}

/**
 * Convert GalleryPhotoModel to the model that 'react-photo-gallery' needs.
 */
function getGalleryViewModel(photos: GalleryPhotoModel[]): PhotoProps[] {
  const viewModelPhotos = photos.map<PhotoProps>((photo) => {
    return {
      key: photo.id,
      src: "", // GalleryPhoto component will set this.
      width: photo.width,
      height: photo.height,
    };
  });

  return viewModelPhotos;
}

/**
 * Add fake entries to the array until the array has length 'padToLength'.
 * This is because react-photo-gallery wants to fill the entire screen width,
 * and if only one or a few photos are available they will get stretched out a lot which looks bad.
 * By faking we have a a reasonable amount of photos, all photos should be around the desired size. (see 'PHOTO_HEIGHT')
 */
function padViewModel(photos: PhotoProps[], padToLength: number): PhotoProps[] {
  if (photos.length < padToLength) {
    for (let i = 0; i < 20 - photos.length; i++) {
      photos.push({
        key: `pad${i}`,
        src: "",
        width: 400,
        height: 300,
      });
    }
  }

  return photos;
}

const PhotoGallery: FC<Props> = (props) => {
  const anyPhotoSelected = props.selectedItems.length > 0;

  let galleryViewModelPhotos = getGalleryViewModel(props.photos);
  galleryViewModelPhotos = padViewModel(galleryViewModelPhotos, 20);

  return (
    <div className="photo-gallery">
      <Gallery
        photos={galleryViewModelPhotos}
        margin={PHOTO_MARGIN}
        targetRowHeight={PHOTO_HEIGHT}
        renderImage={(renderProps) => {
          const photoViewModel = props.photos.find((p) => p.id === renderProps.photo.key);

          if (!photoViewModel) {
            return null;
          } else {
            const photoId = photoViewModel.id;
            const selected = props.selectedItems.indexOf(photoId) !== -1;

            // Toggle selected state of current photo
            const fnToggleSelected = () => {
              if (props.onPhotoSelectionChanged) {
                // If selected, then on callback we must unselect. And visa versa
                const newSelectedPhotoIds = selected
                  ? props.selectedItems.filter((pId) => pId !== photoId)
                  : [photoId, ...props.selectedItems];

                props.onPhotoSelectionChanged(newSelectedPhotoIds);
              }
            };

            const gpiProps = {
              key: photoViewModel.id,
              photo: {
                ...photoViewModel,
                width: renderProps.photo.width,
                height: renderProps.photo.height,
              },
              margin: renderProps.margin,
              selected,
              anySiblingPhotoSelected: props.selectedItems.length > 0,
              onClick: () => {
                if (anyPhotoSelected) {
                  fnToggleSelected();
                } else {
                  props.onClick(photoId);
                }
              },
              onToggleSelect: fnToggleSelected,
            };
            return <GalleryPhoto {...gpiProps} />;
          }
        }}
      />
    </div>
  );
};

export default PhotoGallery;
