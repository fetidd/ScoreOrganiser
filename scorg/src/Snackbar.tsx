import React, { useContext } from "react";
import SnackbarContext from "./snackbar-context";
import "./Snackbar.css"

const Snackbar = () => {
  const ctx = useContext(SnackbarContext)

  return (
    <div className="snackbar__container">
      <div className="snackbar__label">{ctx.msg}</div>
      <div className="snackbar__dismiss" onClick={ctx.onClose}>&times;</div>
    </div>
  );
};
export default Snackbar;
