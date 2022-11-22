import { useEffect, useState } from "react"
import { StudentList, ScoreTabs } from "./components"
import { Student } from "./types"
import { invoke } from "@tauri-apps/api"
import "./UI.css"

export default function UI() {
  let [students, setStudents] = useState([] as Student[]);
  
  function refreshStudents() {
    console.log("REFRESHING STUDENTS");
    invoke("all_students")
    .then((res) => {
      console.log(res);
      setStudents(res as Student[]);
    })
    .catch((err) => {
      console.error(err);
    });
  }
  
  let [selectedStudent, setSelectedStudent] = useState(null as Student|null);

  function selectStudent(id: string) {
    let found: Student[] = students.filter((st: Student) => st.id === id);
    if (found.length === 1) {
      setSelectedStudent(found[0]);
    } else setSelectedStudent(students[0]);
  }

  useEffect(refreshStudents, []);

  return (
    <div className="UI">
      <StudentList 
        students={students} 
        selectedStudent={selectedStudent} 
        selectStudent={selectStudent}
        refreshStudents={refreshStudents}
      />
      <ScoreTabs 
        selectedStudent={selectedStudent}
        refreshStudents={refreshStudents}
      />
    </div>
  )
}
