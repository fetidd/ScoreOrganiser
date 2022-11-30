import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import StudentList from "./StudentList/StudentList";

function App() {

  const [selected, setSelected] = useState("")
  const [students, setStudents] = useState(devStudents)

  const getStudents = () => {
    setStudents(devStudents)
  }

  return (
    <div className="container">
      <div id="student-list-area">
        <StudentList students={students} selected={selected} select={setSelected} setStudents={setStudents} getStudents={getStudents}/>
      </div>
      <div id="score-tab-area"></div>
    </div>
  );
}

export default App;

const devStudents = [
  {id: "st0", name: "Ben Jones", dob: "1990-01-23"},
  {id: "st1", name: "Gemma Victoria Mercer-Forbes", dob: "1988-09-30"},
  {id: "st2", name: "Daisy Enfys Forbes-Jones", dob: "2020-11-04"},
];
