import { useEffect, useRef, useState } from "react"
import { Student } from "../Student"

export default function EditStudentDialog({showDialog, editStudent, closeModals, student}: Props) {

    const nameInput = useRef<HTMLInputElement>(null)
    const dobInput = useRef<HTMLInputElement>(null)

    function clearInputs() {
        nameInput.current!.value = ""
        dobInput.current!.value = ""
    }

    function handleEdit() {
        editStudent(student!.id, nameInput.current!.value, dobInput.current!.value)
        clearInputs()
        closeModals()
    }

    useEffect(() => {
        nameInput.current!.value = `${student!.first_names} ${student!.last_name}`
        dobInput.current!.value = student!.date_of_birth
    }, [student])

    return (
        <div id="edit-student-dialog" className="dialog" style={{display: showDialog?"flex":"none"}}>
            <div className="topbar">
                <span><em>Edit student</em></span>
                <i className="fa fa-close" onClick={() => {
                    clearInputs()
                    closeModals()
                }} />
            </div>
            <div className="row">
                <label>Name</label>
                <input id="name-input" type="text" ref={nameInput} />
            </div>
            <div className="row">
                <label>Date of birth</label>
                <input type="date" ref={dobInput} />
            </div>
            <div className="row">
                <button className="button wide" onClick={handleEdit}>Save</button>
            </div>
        </div>
    )
}

type Props = {
    showDialog: boolean,
    student: Student | null,
    editStudent: Function,
    closeModals: Function
}