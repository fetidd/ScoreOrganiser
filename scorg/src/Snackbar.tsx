import React, { useContext } from "react";
import SnackbarContext from "./snackbar-context";
import "./Snackbar.css"

const Snackbar = () => {
  const ctx = useContext(SnackbarContext)

  const colors = (lvl: string) => {
    return {
      "success": "green",
      "error": "red",
      "info": "blue",
    }[lvl]
  }

  return (
    <div className="snackbar__container" onClick={ctx.onClose} style={{ backgroundColor: colors(ctx.level) }}>
      <div className="snackbar__label">{ctx.msg}</div>
    </div>
  );
};
export default Snackbar;
