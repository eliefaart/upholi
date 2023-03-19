import * as React from "react";
import { FC } from "react";

interface Props {
  left?: JSX.Element;
  right?: JSX.Element;
}

const ButtonBar: FC<Props> = (props) => {
  return (
    <div className="button-bar">
      <div className="left">{props.left}</div>
      <div className="right">{props.right}</div>
    </div>
  );
};

export default ButtonBar;
