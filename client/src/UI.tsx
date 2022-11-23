import { useEffect, useState } from "react"
import { StudentList, ScoreTabs, InfoBar } from "./components"
import { Student, StatusMessageLevel } from "./types"
import { invoke } from "@tauri-apps/api"
import "./UI.css"

export default function UI() {
  // have this maintain a stack so messages dont just get skipped
  const [statusMessage, setStatusMessageObj] = useState({msg: "", level: StatusMessageLevel.Info})
  const setStatusMessage = (msg: string, level: StatusMessageLevel) => {
    setStatusMessageObj({msg: msg, level: level})
    setTimeout(() => {setStatusMessageObj({msg: "", level: StatusMessageLevel.Info})}, 3000)
  }

  let [students, setStudents] = useState([] as Student[]);

  const refreshStudents = () => {
    setStatusMessage("Refreshing students", StatusMessageLevel.Debug)
    invoke("all_students")
      .then((res) => {
        setStudents(res as Student[]);
      })
      .catch((err) => {
        setStatusMessage(err, StatusMessageLevel.Error)
      });
  }

  let [selectedStudent, setSelectedStudent] = useState(null as Student | null);

  const selectStudent = (id: string) => {
    let found: Student[] = students.filter((st: Student) => st.id === id);
    if (found.length === 1) {
      setSelectedStudent(found[0]);
    } else setSelectedStudent(students[0]);
  }

  useEffect(refreshStudents, []);

  return (
    <>
    <div className="UI">
      <StudentList
        students={students}
        selectedStudent={selectedStudent}
        selectStudent={selectStudent}
        refreshStudents={refreshStudents}
        setStatusMessage={setStatusMessage}
      />
      <ScoreTabs
        selectedStudent={selectedStudent}
        refreshStudents={refreshStudents}
        setStatusMessage={setStatusMessage}
      />
    </div>
    <InfoBar statusMessage={statusMessage}/>
    </>
  )
}


