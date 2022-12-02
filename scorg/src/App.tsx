import { useContext, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import StudentList from "./StudentList/StudentList";
import { Student } from "./StudentList/Student";
import SnackbarContext from "./snackbar-context";
import SnackBar from "./Snackbar";
import ScoreTabs from "./ScoreTabs/ScoreTabs";

function App() {
  const [selected, setSelected] = useState("")
  const [students, setStudents] = useState([] as Student[])
  const snackbarCtx = useContext(SnackbarContext)

  const getStudentsFromTauri = async () => {
    try {
      let students: Student[] = await invoke("all_students")
      setStudents(students)
    } catch (error) {
      console.error("failed to get students") // TODO snackbar
    }
  }

  return (
    <div className="container">
      <div id="student-list-area">
        <StudentList students={students} selected={selected} select={setSelected} getStudents={getStudentsFromTauri} />
      </div>
      <div id="score-tab-area">
        <ScoreTabs />
      </div>
      {snackbarCtx.isDisplayed && <SnackBar />}
    </div>
  );
}

export default App;
