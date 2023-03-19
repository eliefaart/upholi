import * as React from "react";
import { FC } from "react";
import { HeaderSettings } from "../../models/HeaderSettings";

interface Props {
  settings: HeaderSettings;
}

const Header: FC<Props> = (props) => {
  if (props.settings.headerContentElement === null) {
    return null;
  } else {
    return <header id="header">{props.settings.headerContentElement}</header>;
  }
};

export default Header;
