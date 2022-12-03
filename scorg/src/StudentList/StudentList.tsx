import { invoke } from "@tauri-apps/api/tauri";
import React, { useEffect, useState, useContext } from "react";
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
    getStudents: Function
}

export default function StudentList({ students, selected, select, getStudents }: Props) {
    const [modal, setModal] = useState(false)
    const [showAddStudent, setShowAddStudent] = useState(false)
    const [showEditStudent, setShowEditStudent] = useState(false)
    const [editing, setEditing] = useState(null as Student | null)
    const [showDeleteStudent, setShowDeleteStudent] = useState(false)
    const [deleting, setDeleting] = useState(null as Student | null)
    const [file, setFile] = useState(null as File | null)
    const snackbarCtx = useContext(SnackbarContext)

    const addStudentToTauri = async (firstNames: string, lastName: string, dateOfBirth: string) => {
        try {
            await invoke("add_student", { firstNames, lastName, dateOfBirth })
            snackbarCtx.success(`added ${firstNames} ${lastName}`)
            getStudents()
        } catch (error) {
            snackbarCtx.error(`failed to add student: ${error!.toString()}`)
        }
    }

    const deleteStudentFromTauri = async (id: string) => {
        try {
            await invoke("delete_student", { id })
            snackbarCtx.success("deleted student")
            getStudents()
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
            getStudents()
        } catch (error) {
            snackbarCtx.error(`failed to edit student: ${error!.toString()}`)
        }
    }

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
                setEditing={setEditing}
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
                    <button className="icon-button dark" onClick={() => { handleAddStudentClick() }}>
                        <i className="fa-solid fa-plus"></i>
                    </button>
                </div>
                <div id="list-area">
                    <ul id="student-list">
                        {rows}
                    </ul>
                </div>
            </div>
            <div className="modal" onClick={() => closeModals()} style={{ display: modal ? "block" : "none", }}></div>
            <AddStudentDialog showDialog={showAddStudent} addStudent={addStudentToTauri} closeModals={closeModals} />
            {(editing && <EditStudentDialog showDialog={showEditStudent} student={editing} editStudent={editStudentInTauri} closeModals={closeModals} />)}
            {(deleting && <DeleteStudentDialog showDialog={showDeleteStudent} student={deleting} deleteStudent={deleteStudentFromTauri} closeModals={closeModals} />)}
        </>
    )
}
