import { useContext, useEffect, useState } from "react"
import SnackbarContext from "../../snackbar-context"
import { Student } from "../Student"

export default function DeleteStudentDialog({showDialog, student, deleteStudent, closeModals}: Props) {
    const [confirmDelete, setConfirmDelete] = useState("")
    const snack = useContext(SnackbarContext)

    function handleDelete() {
        if (confirmDelete === student!.last_name) {
            deleteStudent(student!.id)
            setConfirmDelete("")
            closeModals()
        }
    }

    useEffect(() => {
        if (showDialog) {
            document.getElementById("confirm-input")?.focus()
        }
    }, [showDialog])

    return (
        <div id="delete-student-dialog" className="dialog" style={{display: showDialog?"flex":"none"}}>
            <div className="row">
                <span>Enter student's last name to confirm delete</span>
            </div>
            <div className="row">
                <input id="confirm-input" type="text" value={confirmDelete} onChange={(e => {setConfirmDelete(e.target.value)})}/>
                <button className="button" onClick={handleDelete}>Confirm</button>
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