import React, { useState } from "react";

const SnackbarContext = React.createContext({
  msg: "",
  isDisplayed: false,
  displayMsg: (msg: string) => { },
  onClose: () => { },
});

export default SnackbarContext;

let timer;

export const SnackBarContextProvider = (props) => {
  const [msg, setMsg] = useState("");
  const [isDisplayed, setIsDisplayed] = useState(false);

  const displayHandler = (msg: string) => {
    setMsg(msg);
    setIsDisplayed(true);
    timer = setTimeout(() => {
      setIsDisplayed(false);
    }, 3000); // close snackbar after 3 seconds
  };

  const closeHandler = () => {
    clearTimeout(timer)
    setIsDisplayed(false)
  };

  return (
    <SnackbarContext.Provider
      value={{
        msg,
        isDisplayed,
        displayMsg: displayHandler,
        onClose: closeHandler,
      }}
    >
      {props.children}
    </SnackbarContext.Provider>
  );
};
