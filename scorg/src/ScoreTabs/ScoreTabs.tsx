import { invoke } from "@tauri-apps/api"
import { useContext, useEffect, useRef, useState } from "react"
import SnackbarContext from "../snackbar-context"
import SafmedContent from "./SafmedContent"
import moment from "moment"
import { Score } from "./Score"

export default function ScoreTabs({ selected }: Props) {
  const [currentTab, setCurrentTab] = useState("safmeds")
  const [scores, setScores] = useState([] as Score[])
  const correctInput = useRef<HTMLInputElement>(null)
  const incorrectInput = useRef<HTMLInputElement>(null)
  const [currentDate, setCurrentDate] = useState(moment().format("YYYY-MM-DD"))
  const snack = useContext(SnackbarContext)

  function switchTab(tab: string) {
    setCurrentTab(tab)
  }

  function getScores() {
    invoke(`get_${currentTab}_scores`, { studentId: selected }).then((sc) => {
      // snack.success(`got plot`)
      setScores(sc as Score[])
    }).catch(e => snack.error(e.toString()))
  }

  function addSafmedsScore() {
    let correct = Number.parseInt(correctInput.current!.value)
    let incorrect = Number.parseInt(incorrectInput.current!.value)
    invoke("add_safmeds_score", {id: selected, date: currentDate, correct, incorrect}).then(() => {
      snack.success("added safmeds score"); 
      getScores();
      correctInput.current!.value = ""
      incorrectInput.current!.value = ""
    }).catch(e => snack.error(e.toString()))
  }

  useEffect(() => {
    getScores()
  }, [selected, currentTab])


  return (
    <div id="ScoreTabs">
      <div id="tabs">
        <div className={currentTab === "safmeds" ? "tab selected" : "tab"} onClick={() => switchTab("safmeds")}><span>SAFMEDs</span></div>
        {/* <div className={currentTab === "writing" ? "tab selected" : "tab"} onClick={() => switchTab("writing")}><span>Writing</span></div>
        <div className={currentTab === "reading" ? "tab selected" : "tab"} onClick={() => switchTab("reading")}><span>Reading</span></div> */}
      </div>
      <div id="content">
        {currentTab === "safmeds" && <SafmedContent scores={scores}/>}
        {/* {currentTab === "writing" && <span>WritingContent</span>}
        {currentTab === "reading" && <span>ReadingContent</span>} */}
        <div id="controls" className="row center">
          <input ref={correctInput} id="correct" type="number"></input>
          <input ref={incorrectInput} id="incorrect" type="number"></input>
          <input value={currentDate} id="date" type="date" onChange={e => setCurrentDate(e.target.value)}></input>
          <button id="add-button" className="button" onClick={addSafmedsScore}>Add</button>
        </div>
      </div>
    </div>
  )
}

interface Props {
  selected: string,
}