import * as React from "react";
import { FC, useEffect } from "react";
import { toast } from "react-toastify";
import appStateContext from "../../contexts/AppStateContext";
import useAuthenticationStatus, { AuthenticationStatus } from "../../hooks/useAuthenticationStatus";
import { useTitle } from "../../hooks/useTitle";
import upholiService from "../../services/UpholiService";
import Content from "../layout/Content";
import ButtonBar from "../misc/ButtonBar";
import Errors from "../misc/Errors";
import { IconChevronLeft } from "../misc/Icons";

const RegisterPage: FC = () => {
  useTitle("Register");
  const [authenticationStatus, resetAuthenticationStatus] = useAuthenticationStatus();
  const context = React.useContext(appStateContext);
  const usernameInput = React.createRef<HTMLInputElement>();
  const passwordInput = React.createRef<HTMLInputElement>();
  const confirmPasswordInput = React.createRef<HTMLInputElement>();
  const [errors, setErrors] = React.useState<string[]>([]);

  const toHomePage = (): void => {
    context.history.push("/");
  };

  const create = (): void => {
    setErrors([]);

    if (usernameInput.current && passwordInput.current && confirmPasswordInput.current) {
      const username = usernameInput.current.value;
      const password = passwordInput.current.value;
      const confirmPassword = confirmPasswordInput.current.value;

      if (password !== confirmPassword) {
        setErrors(["Passwords do not match"]);
      } else {
        upholiService
          .register(username, password)
          .then(() => {
            toast.info("User created.");
            resetAuthenticationStatus(true);
          })
          .catch((error) => {
            setErrors([error ?? "Error creating user"]);
          });
      }
    }
  };

  // If user is already logged in, or when the AuthenticationStatus changes to Authenticated after creating a new user,
  // then redirect to home page.
  useEffect(() => {
    if (authenticationStatus === AuthenticationStatus.Authenticated) {
      toHomePage();
    }
  }, [authenticationStatus]);

  return (
    <Content className="form-small">
      <h1>Create new user</h1>

      <Errors errors={errors} />

      <input type="text" placeholder="username" ref={usernameInput} />
      <input type="password" placeholder="password" ref={passwordInput} />
      <input type="password" placeholder="confirm password" ref={confirmPasswordInput} />

      <ButtonBar
        left={
          <button onClick={toHomePage}>
            <IconChevronLeft />
            Sign in instead
          </button>
        }
        right={
          <button className="primary" onClick={create}>
            Create
          </button>
        }
      />
    </Content>
  );
};

export default RegisterPage;
