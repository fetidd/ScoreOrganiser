import { invoke } from "@tauri-apps/api/tauri";
import React, { useEffect, useState, useContext, useRef } from "react";
import SnackbarContext from "../snackbar-context";
import AddStudentDialog from "./dialogs/AddStudent";
import DeleteStudentDialog from "./dialogs/DeleteStudent";
import EditStudentDialog from "./dialogs/EditStudent";
import { Student } from "./Student"
import StudentRow from "./StudentRow";


interface Props {
  students: Student[],
  selected: string,
  select: Function,
  getStudents: Function,
  addStudent: Function,
  deleteStudent: Function,
  editStudent: Function,
  applyFilter: Function
}

export default function StudentList({ students, selected, select, getStudents, applyFilter, addStudent, deleteStudent, editStudent }: Props) {
  const [modal, setModal] = useState(false)
  const [showAddStudent, setShowAddStudent] = useState(false)
  const [showEditStudent, setShowEditStudent] = useState(false)
  const [editing, setEditing] = useState(null as Student | null)
  const [showDeleteStudent, setShowDeleteStudent] = useState(false)
  const [deleting, setDeleting] = useState(null as Student | null)
  const [file, setFile] = useState(null as File | null)
  const snackbarCtx = useContext(SnackbarContext)

  const filterInput = useRef<HTMLInputElement>(null)

  const closeModals = () => {
    setShowAddStudent(false)
    setShowEditStudent(false)
    setShowDeleteStudent(false)
    setModal(false)
  }

  const uploadFile = () => {
    snackbarCtx.info(file!.toString());
    (document.querySelector("#csv-input") as HTMLInputElement).value = ""
    setFile(null)
  }

  function handleAddStudentClick() {
    setModal(true)
    setShowAddStudent(true)
  }

  function handleFilterChange() {
    let filter = filterInput.current!.value
    applyFilter(filter)
  }

  // function handleFileDrop(event: React.DragEvent) {
  //   setFile(event.dataTransfer.files[0])
  //   snackbarCtx.info(file)
  // }

  const rows = () => {
    let rows = students.map(student => {
      return (
        <StudentRow key={student.id}
          student={student}
          select={select}
          selected={selected}
          setShowEditStudent={setShowEditStudent}
          setEditing={setEditing}
          setShowDeleteStudent={setShowDeleteStudent}
          setDeleting={setDeleting}
          setModal={setModal}
        />
      )
    })
    return rows
  };

  useEffect(() => { getStudents() }, [])

  return (
    <>
      <div id="StudentList">
        <div id="menubar-area">
          <div className="row">
            <input ref={filterInput} type="text" onChange={handleFilterChange}></input>
            {/* <button className="icon-button dark" onClick={handleFilterChange}>
              <i className="fa-solid fa-filter"></i>
            </button> */}
            <button className="icon-button dark" onClick={handleAddStudentClick}>
              <i className="fa-solid fa-plus"></i>
            </button>
          </div>
        </div>
        <div id="list-area">
          <ul id="student-list">
            {rows()}
          </ul>
        </div>
        <div id="list-bottom-bar-area">
        </div>
      </div>
      <div className="modal" onClick={() => closeModals()} style={{ display: modal ? "block" : "none", }}></div>
      <AddStudentDialog showDialog={showAddStudent} addStudent={addStudent} closeModals={closeModals} />
      {(editing && <EditStudentDialog showDialog={showEditStudent} student={editing} editStudent={editStudent} closeModals={closeModals} />)}
      {(deleting && <DeleteStudentDialog showDialog={showDeleteStudent} student={deleting} deleteStudent={deleteStudent} closeModals={closeModals} />)}
    </>
  )
}
