import { useEffect, useState } from "react";
import AddStudentDialog from "./dialogs/AddStudent";
import DeleteStudentDialog from "./dialogs/DeleteStudent";
import EditStudentDialog from "./dialogs/EditStudent";
import { Student } from "./Student"
import SnackBar from "../Snackbar"

export default function StudentList({ students, setStudents, selected, select, getStudents }: Props) {
    const [hasContextFocus, setHasContextFocus] = useState("")
    const [modal, setModal] = useState(false)
    const [showAddStudent, setShowAddStudent] = useState(false)
    const [addName, setAddName] = useState("")
    const [addDob, setAddDob] = useState("")
    const [showEditStudent, setShowEditStudent] = useState(false)
    const [editing, setEditing] = useState("")
    const [editName, setEditName] = useState("")
    const [editDob, setEditDob] = useState("")
    const [showDeleteStudent, setShowDeleteStudent] = useState(false)
    const [deleting, setDeleting] = useState("")
    const [confirmDelete, setConfirmDelete] = useState("")
    const [deleteConfirmationTarget, setDeleteConfirmationTarget] = useState("")
    const [file, setFile] = useState(null as File | null)

    const addStudent = () => { // TODOINVOKE
        let newStudent: Student = { id: `st${students.length}`, name: addName, dob: addDob }
        students.push(newStudent)
        setStudents(students)
    }

    const deleteStudent = () => { // TODOINVOKE
        let newStudents = students.filter(st => st.id !== deleting)
        setStudents(newStudents)
        setHasContextFocus("")
        setDeleting("")
    }

    const editStudent = () => { // TODOINVOKE
        let edited = students.find(st => st.id === editing)
        if (edited !== null && edited !== undefined) {
            edited.name = editName
            edited.dob = editDob
        }
        setHasContextFocus("")
        setEditing("")
    }

    useEffect(() => { getStudents() }, [])

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

    const rows = students.map(student => {
        const isSelected = student.id === selected;
        return (
            <li key={student.id}>
                <div
                    style={{
                        backgroundColor: isSelected ? "#28262b" : "#eee",
                        color: isSelected ? "white" : "black",
                        display: "flex",
                        alignItems: "center",
                        height: "30px",
                    }}
                    className="student-row"
                    onClick={() => {
                        select(student.id)
                    }}
                    onContextMenu={e => {
                        e.preventDefault()
                        select(student.id)
                        setHasContextFocus(student.id)
                    }}
                    onMouseLeave={() => {
                        setHasContextFocus("")
                    }}
                >
                    <span
                        style={{
                            flexGrow: "1",
                        }}
                    >{student.name}</span>
                    <button
                        className="icon-button"
                        style={{
                            display: (hasContextFocus === student.id) ? "block" : "none",
                            justifySelf: "end"
                        }}
                        onClick={e => {
                            e.stopPropagation()
                            setEditName(student.name)
                            setEditDob(student.dob)
                            setModal(true)
                            setEditing(student.id)
                            setShowEditStudent(true)
                        }}
                    ><i className="fa-solid fa-pen"></i></button>
                    <button
                        className="icon-button red"
                        style={{
                            display: (hasContextFocus === student.id) ? "block" : "none",
                            justifySelf: "end"
                        }}
                        onClick={e => {
                            e.stopPropagation()
                            setShowDeleteStudent(true)
                            setModal(true)
                            setDeleting(student.id)
                            const splitName = student.name.split(" ")
                            setDeleteConfirmationTarget(splitName[splitName.length - 1])
                        }}
                    ><i className="fa-solid fa-trash-can"></i></button>
                </div>

                <EditStudentDialog
                    showEditStudent={showEditStudent}
                    editName={editName}
                    editDob={editDob}
                    setEditName={setEditName}
                    setEditDob={setEditDob}
                    editStudent={editStudent}
                    closeModals={closeModals}
                />

                <DeleteStudentDialog
                    showDeleteStudent={showDeleteStudent}
                    confirmDelete={confirmDelete}
                    setConfirmDelete={setConfirmDelete}
                    deleteConfirmationTarget={deleteConfirmationTarget}
                    deleteStudent={deleteStudent}
                    closeModals={closeModals}
                    setDeleteConfirmationTarget={setDeleteConfirmationTarget}
                />

            </li>
        )
    });

    return (
        <>
            <div id="StudentList">
                <div id="menubar-area">
                    <button
                        className="icon-button dark"
                        onClick={() => {
                            setModal(true)
                            setShowAddStudent(true)
                        }}><i className="fa-solid fa-plus"></i></button>
                </div>

                <div id="list-area">
                    <ul id="student-list">
                        {rows}
                    </ul>
                </div>

                <div id="import-csv-area">
                    <div className="row">
                        <input
                            id="csv-input"
                            type="file"
                            name="csv-input"
                            accept=".csv"
                            onChange={e => {
                                const files = e.target.files
                                if (files !== null && files[0] !== null) {
                                    setFile(files[0])
                                }
                            }}
                        />
                        <button
                            className="button"
                            onClick={uploadFile}
                        >Upload</button>
                    </div>
                </div>
            </div>

            <div
                className="modal"
                onClick={() => closeModals()}
                style={{
                    display: modal ? "block" : "none",
                }}></div>

            <AddStudentDialog
                showAddStudent={showAddStudent}
                addName={addName}
                setAddName={setAddName}
                addDob={addDob}
                setAddDob={setAddDob}
                addStudent={addStudent}
                closeModals={closeModals}
            />
        </>
    )
}

type Props = {
    students: Student[],
    setStudents: Function,
    selected: string,
    select: Function,
    getStudents: Function
}

