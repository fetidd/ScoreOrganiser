import React, { useState } from "react";

interface Props {
  children: any
}

const SnackbarContext = React.createContext({
  level: "",
  msg: "",
  isDisplayed: false,
  info: (msg: string) => { },
  error: (msg: string) => { },
  success: (msg: string) => { },
  onClose: () => { },
});

export default SnackbarContext;

let timer: NodeJS.Timeout;

export const SnackBarContextProvider = ({ children }: Props) => {
  const [msg, setMsg] = useState("");
  const [isDisplayed, setIsDisplayed] = useState(false);
  const [level, setLevel] = useState("")

  const displayInfo = (msg: string) => {
    setLevel("info")
    _display(msg)
  };

  const displayError = (msg: string) => {
    setLevel("error")
    _display(msg)
  }

  const displaySuccess = (msg: string) => {
    setLevel("success")
    _display(msg)
  }

  const _display = (msg: string) => {
    setMsg(msg);
    setIsDisplayed(true);
    timer = setTimeout(() => {
      setIsDisplayed(false);
    }, 3000); // close snackbar after 3 seconds

  }

  const closeHandler = () => {
    clearTimeout(timer)
    setIsDisplayed(false)
  };

  return (
    <SnackbarContext.Provider
      value={{
        level,
        msg,
        isDisplayed,
        info: displayInfo,
        error: displayError,
        success: displaySuccess,
        onClose: closeHandler,
      }}
    >
      {children}
    </SnackbarContext.Provider>
  );
};
