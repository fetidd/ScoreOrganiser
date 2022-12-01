import React, { useEffect, useState } from "react";
import AddStudentDialog from "./dialogs/AddStudent";
import DeleteStudentDialog from "./dialogs/DeleteStudent";
import EditStudentDialog from "./dialogs/EditStudent";
import { Student } from "./Student"
import StudentRow from "./StudentRow";


interface Props {
    students: Student[],
    setStudents: Function,
    selected: string,
    select: Function,
    getStudents: Function
}

export default function StudentList({ students, setStudents, selected, select, getStudents }: Props) {
    const [idCount, setIdCount] = useState(5) // TODO remove, just for dev

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

    const _addStudent = () => { // TODOINVOKE
        let newStudent: Student = { id: `st${idCount}`, name: addName, dob: addDob }
        setIdCount(idCount+1)
        students.push(newStudent)
        setStudents(students)
    }

    const _deleteStudent = () => { // TODOINVOKE
        let newStudents = students.filter(st => st.id !== deleting)
        setStudents(newStudents)
        setDeleting("")
    }

    const _editStudent = () => { // TODOINVOKE
        let edited = students.find(st => st.id === editing)
        if (edited !== null && edited !== undefined) {
            edited.name = editName
            edited.dob = editDob
        }
        setEditing("")
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

    function handleAddStudent() {
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
                setConfirmDelete={setConfirmDelete}
                setDeleteConfirmationTarget={setDeleteConfirmationTarget}
                setModal={setModal}
            />
        )
    });

    return (
        <>
            <div id="StudentList">
                <div id="menubar-area">
                    <button className="icon-button dark"onClick={() => {handleAddStudent()}}>
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
            <AddStudentDialog showAddStudent={showAddStudent} addName={addName} setAddName={setAddName} addDob={addDob} setAddDob={setAddDob} addStudent={_addStudent} closeModals={closeModals} />
            <EditStudentDialog showEditStudent={showEditStudent} editName={editName} editDob={editDob} setEditName={setEditName} setEditDob={setEditDob} editStudent={_editStudent} closeModals={closeModals} />
            <DeleteStudentDialog showDeleteStudent={showDeleteStudent} confirmDelete={confirmDelete} setConfirmDelete={setConfirmDelete} deleteConfirmationTarget={deleteConfirmationTarget} deleteStudent={_deleteStudent} closeModals={closeModals} setDeleteConfirmationTarget={setDeleteConfirmationTarget} />
        </>
    )
}
