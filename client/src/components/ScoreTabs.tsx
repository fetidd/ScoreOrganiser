import { Student } from "../types"
import { SafmedScores, WritingScores, ReadingScores } from "../components"
import { Box, Tabs, Tab, Paper } from "@mui/material"
import { useState } from "react"

function ScoreTabs(p: Props) {
  let [currentTab, setCurrentTab] = useState(Assessments.SAFMEDs)
  return (
    <Paper elevation={3} className="ScoreTabs">
      <Tabs id="tabs" value={currentTab} onChange={(_e, v) => setCurrentTab(v)}>
        <Tab value={Assessments.SAFMEDs} label="SAFMEDs" />
        {/* <Tab value={Assessments.Reading} label="Reading" />
        <Tab value={Assessments.Writing} label="Writing" /> */}
      </Tabs>
      <Box id="tab-container">
        {currentTab === Assessments.SAFMEDs && (
          <SafmedScores
            selectedStudent={p.selectedStudent}
          />
        )}
        {currentTab === Assessments.Writing && (
          <ReadingScores
            selectedStudent={p.selectedStudent}
          />
        )}
        {currentTab === Assessments.Reading && (
          <WritingScores
            selectedStudent={p.selectedStudent}
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
