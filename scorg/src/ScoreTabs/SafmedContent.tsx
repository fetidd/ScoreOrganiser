import { invoke } from "@tauri-apps/api"
import { useContext, useEffect, useState } from "react"
import SnackbarContext from "../snackbar-context"

export default function SafmedContent({ id, currentTab }: Props) {
  const [plot, setPlot] = useState("")
  const snack = useContext(SnackbarContext)

  function getPlot() {
    invoke("get_safmed_plot", { studentId: id }).then(pl => {
      setPlot(pl)
      snack.success(`got plot`)
      document.querySelector("#safmed-chart")!.innerHTML = plot;
    }).catch(e => snack.error(e.toString()))
  }

  useEffect(getPlot, [id, currentTab])

  return (
    <div id="safmed-content">
      <div id="safmed-chart" className="chart"></div>
    </div>
  )
}

interface Props {
  id: string,
  currentTab: string
}
