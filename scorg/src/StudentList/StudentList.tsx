import { invoke } from "@tauri-apps/api/tauri";
import React, { useEffect, useState } from "react";
import AddStudentDialog from "./dialogs/AddStudent";
import DeleteStudentDialog from "./dialogs/DeleteStudent";
import EditStudentDialog from "./dialogs/EditStudent";
import { Student } from "./Student"
import StudentRow from "./StudentRow";


interface Props {
    students: Student[],
    selected: string,
    select: Function,
    getStudents: Function
}

export default function StudentList({ students, selected, select, getStudents }: Props) {
    const [modal, setModal] = useState(false)

    const [showAddStudent, setShowAddStudent] = useState(false)

    const [showEditStudent, setShowEditStudent] = useState(false)
    const [editing, setEditing] = useState(null as Student | null)
    const [editName, setEditName] = useState("")
    const [editDob, setEditDob] = useState("")

    const [showDeleteStudent, setShowDeleteStudent] = useState(false)
    const [deleting, setDeleting] = useState(null as Student | null)

    const [file, setFile] = useState(null as File | null)

    const addStudentToTauri = async (firstNames: string, lastName: string, dateOfBirth: string) => { // TODOINVOKE
        try {
            await invoke("add_student", {firstNames, lastName, dateOfBirth})
            console.log("added student") // TODO snackbar
            getStudents()
          } catch (error) {
            console.error("failed to add student") // TODO snackbar
            console.error(error)
          }
    }

    const deleteStudentFromTauri = async (id: string) => { // TODOINVOKE
        try {
            await invoke("delete_student", {id})
            console.log("added student") // TODO snackbar
            getStudents()
          } catch (error) {
            console.error("failed to delete student") // TODO snackbar
            console.error(error)
          }
    }

    const editStudentInTauri = async () => { // TODOINVOKE
        let splitName = editName.split(" ")
        let last_name = splitName.pop()
        let first_names = splitName.join(" ")
        try {
            await invoke("edit_student", {update: {id: editing, first_names: first_names, last_name: last_name, date_of_birth: editDob}})
            console.log("added student") // TODO snackbar
            getStudents()
          } catch (error) {
            console.error("failed to edit student") // TODO snackbar
            console.error(error)
          }
    }

    const closeModals = () => {
        setShowAddStudent(false)
        setShowEditStudent(false)
        setShowDeleteStudent(false)
        setModal(false)
    }
    
    const uploadFile = () => {
        console.log(file);
        (document.querySelector("#csv-input") as HTMLInputElement).value = ""
        setFile(null)
    }
    
    useEffect(() => { getStudents() }, [])

    function handleAddStudentClick() {
        setModal(true)
        setShowAddStudent(true)
    }

    function handleFileChange(e: React.FormEvent) {
        const files = (e.target as HTMLInputElement).files
        if (files !== null && files[0] !== null) {
            setFile(files[0])
        }
    }

    const rows = students.map(student => {
        return (
            <StudentRow key={student.id}
            student={student}
            select={select}
            selected={selected}
            setShowEditStudent={setShowEditStudent}
            setEditName={setEditName}
            setEditing={setEditing}
            setEditDob={setEditDob}
            setShowDeleteStudent={setShowDeleteStudent}
            setDeleting={setDeleting}
            setModal={setModal} 
            />
        )
    });

    return (
        <>
            <div id="StudentList">
                <div id="menubar-area">
                    <button className="icon-button dark"onClick={() => {handleAddStudentClick()}}>
                        <i className="fa-solid fa-plus"></i>
                    </button>
                </div>
                <div id="list-area">
                    <ul id="student-list">
                        {rows}
                    </ul>
                </div>
                <div id="import-csv-area">
                    <div className="row">
                        <input id="csv-input" type="file" name="csv-input" accept=".csv" onChange={e => {handleFileChange(e)}}/>
                        <button className="button" onClick={uploadFile}>Upload</button>
                    </div>
                </div>
            </div>
            <div className="modal" onClick={() => closeModals()} style={{display: modal ? "block" : "none",}}></div>
            <AddStudentDialog showDialog={showAddStudent} addStudent={addStudentToTauri} closeModals={closeModals} />
            <EditStudentDialog showDialog={showEditStudent} editName={editName} editDob={editDob} setEditName={setEditName} setEditDob={setEditDob} editStudent={editStudentInTauri} closeModals={closeModals} />
            <DeleteStudentDialog showDialog={showDeleteStudent} student={deleting}  deleteStudent={deleteStudentFromTauri} closeModals={closeModals} />
        </>
    )
}
