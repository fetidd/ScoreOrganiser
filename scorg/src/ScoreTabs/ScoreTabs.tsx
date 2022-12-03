import { useState } from "react"
import SafmedContent from "./SafmedContent"

export default function ScoreTabs({ getSafmedPlot, id }: Props) {
  const [currentTab, setCurrentTab] = useState("")

  function switchTab(tab: string) {
    setCurrentTab(tab)
  }

  return (
    <div id="ScoreTabs">
      <div id="tabs">
        <div className={currentTab === "safmeds" ? "tab selected" : "tab"} onClick={() => switchTab("safmeds")}><span>SAFMEDs</span></div>
        <div className={currentTab === "writing" ? "tab selected" : "tab"} onClick={() => switchTab("writing")}><span>Writing</span></div>
        <div className={currentTab === "reading" ? "tab selected" : "tab"} onClick={() => switchTab("reading")}><span>Reading</span></div>
      </div>
      <div id="content">
        {currentTab === "safmeds" && <SafmedContent id={id} getPlot={getSafmedPlot}/>}
        {currentTab === "writing" && <span>WritingContent</span>}
        {currentTab === "reading" && <span>ReadingContent</span>}
      </div>
    </div>
  )
}

interface Props {
  getSafmedPlot: Function,
  id: string
}
