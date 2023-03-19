import * as React from "react";
import { FC } from "react";

interface Props {
  className?: string;
  prompt?: string;
  onSubmitPassword: (password: string) => void;
  lastPasswordIncorrect?: boolean;
}

const InputPassword: FC<Props> = (props) => {
  let className = "input-password";
  if (props.className) {
    className += ` ${props.className}`;
  }

  const passwordInput = React.createRef<HTMLInputElement>();

  const submitPassword = (): void => {
    if (passwordInput.current) {
      const password = passwordInput.current.value;
      props.onSubmitPassword(password);
    }
  };

  return (
    <div className={className}>
      {props.lastPasswordIncorrect !== true && !!props.prompt && <label htmlFor="input-password">{props.prompt}</label>}
      {props.lastPasswordIncorrect === true && <label className="error-text">Incorrect password</label>}
      <input id="input-password" type="password" placeholder="Password" ref={passwordInput} />
      <button onClick={submitPassword}>Submit</button>
    </div>
  );
};

export default InputPassword;
