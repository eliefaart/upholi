import * as React from "react";
import { FC } from "react";
import Content from "../layout/Content";
import appStateContext from "../../contexts/AppStateContext";
import upholiService from "../../services/UpholiService";
import CopyUrl from "../misc/CopyUrl";
import { useTitle } from "../../hooks/useTitle";
import useAlbums from "../../hooks/useAlbums";
import useShares from "../../hooks/useShares";
import { PageProps } from "../../models/PageProps";
import DefaultHeaderContent from "../headers/DefaultHeaderContent";

const SharedPage: FC<PageProps> = (props: PageProps) => {
  const context = React.useContext(appStateContext);
  const albums = useAlbums();
  const [shares, refreshShares] = useShares();

  useTitle("Shared");
  React.useEffect(() => {
    props.setHeader({
      headerContentElement: <DefaultHeaderContent />,
    });
  }, []);

  const deleteShare = (id: string): void => {
    upholiService
      .deleteShare(id)
      .then(() => {
        // update shares hook somehow?
        refreshShares();
      })
      .catch(console.error);
  };

  return (
    <Content className="shares">
      {shares.map((share) => {
        const shareUrl = document.location.origin + "/s/" + share.id;
        const shareAlbum = albums.find((album) => album.id === share.albumId);

        return (
          <div key={share.id} className="share">
            <div className="head">
              <h2 onClick={() => context.history.push("/album/" + shareAlbum?.id)}>{shareAlbum?.title}</h2>
            </div>
            <div className="body">
              <CopyUrl shareUrl={shareUrl} />
              <div className="actions">
                <button onClick={() => deleteShare(share.id)}>Delete</button>
              </div>
            </div>
          </div>
        );
      })}
    </Content>
  );
};

export default SharedPage;
