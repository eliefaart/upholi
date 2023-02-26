import * as React from "react";
import { FC } from "react";
import appStateContext from "../../contexts/AppStateContext";
import useAuthenticationStatus, { AuthenticationStatus } from "../../hooks/useAuthenticationStatus";

interface Props {
  children?: React.ReactNode;
  requiresAuthentication: boolean;
}

const Authentication: FC<Props> = (props) => {
  const [authenticationStatus] = useAuthenticationStatus();
  const context = React.useContext(appStateContext);

  if (
    authenticationStatus === AuthenticationStatus.Unknown ||
    authenticationStatus === AuthenticationStatus.Refreshing
  ) {
    // User info is still being fetched.
    return <></>;
  } else if (authenticationStatus === AuthenticationStatus.Unauthenticad && props.requiresAuthentication) {
    context.history.push("/login");
    return <></>;
  } else {
    return <>{props.children}</>;
  }
};

export default Authentication;
