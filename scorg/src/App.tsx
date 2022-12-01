import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import StudentList from "./StudentList/StudentList";
import { Student } from "./StudentList/Student";

type DbError = {
  DbError: string
}

function App() {

  const [selected, setSelected] = useState("")
  const [students, setStudents] = useState([] as Student[])

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
        <StudentList students={students} selected={selected} select={setSelected} getStudents={getStudentsFromTauri}/>
      </div>
      <div id="score-tab-area"></div>
    </div>
  );
}

export default App;
