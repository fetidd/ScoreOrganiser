import { useState } from "react"
import { Student } from "../Student"

export default function DeleteStudentDialog({showDialog, student, deleteStudent, closeModals}: Props) {
    const [confirmDelete, setConfirmDelete] = useState("")

    function handleDelete() {
        console.log(confirmDelete)
        console.log(student!.last_name)
        if (confirmDelete === student!.last_name) {
            deleteStudent(student!.id)
            setConfirmDelete("")
            closeModals()
        }
    }

    return (
        <div id="delete-student-dialog" className="dialog" style={{display: showDialog?"flex":"none"}}>
            <div className="row">
                <span>Type student's last name to confirm delete</span>
            </div>
            <div className="row">
                <input type="text" value={confirmDelete} onChange={(e => {setConfirmDelete(e.target.value)})}/>
                <button onClick={handleDelete}>Confirm</button>
            </div>
        </div>
    )
}

type Props = {
    showDialog: boolean,
    student: Student | null,
    deleteStudent: Function,
    closeModals: Function,
}