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
            <div className="row">
                <input type="text" ref={nameInput} />
                <input type="date" ref={dobInput} />
                <button onClick={handleEdit}>Save</button>
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