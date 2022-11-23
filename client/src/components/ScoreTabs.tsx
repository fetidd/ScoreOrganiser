import { Student } from "../types"
import { SafmedScores, WritingScores, ReadingScores } from "../components"
import { Box, Tabs, Tab, Paper } from "@mui/material"
import "../styles/ScoreTabs.css"
import { useState } from "react"

function ScoreTabs(p: Props) {
  let [currentTab, setCurrentTab] = useState(Assessments.SAFMEDs)
  return (
    <Paper elevation={3} className="ScoreTabs">
      <Box>
        <Tabs className="tabs" value={currentTab} onChange={(_e, v) => setCurrentTab(v)}>
          <Tab value={Assessments.SAFMEDs} label="SAFMEDs" />
          {/* <Tab value={Assessments.Reading} label="Reading" />
          <Tab value={Assessments.Writing} label="Writing" /> */}
        </Tabs>
      </Box>
      <Box>
        {currentTab === Assessments.SAFMEDs && (
          <SafmedScores
            selectedStudent={p.selectedStudent}
            scores={[]}
          />
        )}
        {currentTab === Assessments.Writing && (
          <ReadingScores
            selectedStudent={p.selectedStudent}
            scores={[]}
          />
        )}
        {currentTab === Assessments.Reading && (
          <WritingScores
            selectedStudent={p.selectedStudent}
            scores={[]}
          />
        )}
      </Box>
    </Paper>
  )
}

type Props = {
  selectedStudent: Student | null,
  refreshStudents: Function,
}

enum Assessments { SAFMEDs, Reading, Writing }

export { ScoreTabs };
