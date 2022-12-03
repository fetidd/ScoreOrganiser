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
  const [studentCache, setStudentCache] = useState([] as Student[])
  const [currFilter, setCurrFilter] = useState("")
  const snackbarCtx = useContext(SnackbarContext)

  const getStudentsFromTauri = async () => {
    try {
      let students: Student[] = await invoke("all_students")
      setStudents(students)
      setStudentCache(students)
    } catch (error) {
      snackbarCtx.error(`failed to get students: ${error!.toString()}`)
    }
  }

  const addStudentToTauri = async (firstNames: string, lastName: string, dateOfBirth: string) => {
    try {
        await invoke("add_student", { firstNames, lastName, dateOfBirth })
        snackbarCtx.success(`added ${firstNames} ${lastName}`)
        getStudentsFromTauri()
    } catch (error) {
        snackbarCtx.error(`failed to add student: ${error!.toString()}`)
    }
}

  const deleteStudentFromTauri = async (id: string) => {
    try {
        await invoke("delete_student", { id })
        let deleted = students.find(st => st.id === id)!
        snackbarCtx.success(`deleted ${deleted.first_names} ${deleted.last_name}`)
        getStudentsFromTauri()
    } catch (error) {
        snackbarCtx.error(`failed to delete student: ${error!.toString()}`)
    }
  }

  const editStudentInTauri = async (id: string, editName: string, editDob: string) => {
    let splitName = editName.split(" ")
    let last_name = splitName.pop()
    let first_names = splitName.join(" ")
    try {
        await invoke("edit_student", { update: { id: id, first_names: first_names, last_name: last_name, date_of_birth: editDob } })
        snackbarCtx.success("edited student")
        getStudentsFromTauri()
    } catch (error) {
        snackbarCtx.error(`failed to edit student: ${error!.toString()}`)
    }
  }

  const getSafmedPlot = () => {
    let plot; 
    invoke("plot_safmed_scores", { id: selected })
      .then(pl => plot = pl)
      .catch(error => snackbarCtx.error(`failed to get safmed plot: ${error!.toString()}`))
    return plot
  }

  const applyFilter = (filter: string) => {
    console.log(`filtering on ${filter}`)
    setStudents(studentCache.filter(st => `${st.first_names} ${st.last_name}`.toLowerCase().includes(filter.toLowerCase())))
    setCurrFilter(filter)
  }

  return (
    <div className="container">
      <div id="student-list-area">
        <StudentList students={students} selected={selected} select={setSelected} getStudents={getStudentsFromTauri} addStudent={addStudentToTauri} deleteStudent={deleteStudentFromTauri} editStudent={editStudentInTauri} applyFilter={applyFilter} />
      </div>
      <div id="score-tab-area">
        <ScoreTabs id={selected} getSafmedPlot={getSafmedPlot} />
      </div>
      {snackbarCtx.isDisplayed && <SnackBar />}
    </div>
  );
}

export default App;
