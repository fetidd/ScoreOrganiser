import { useState } from "react"
import { Student } from "../Student"

export default function EditStudentDialog({showDialog, editStudent, closeModals, student}: Props) {

    const [editName, setEditName] = useState(`${student!.first_names} ${student!.last_name}`)
    const [editDob, setEditDob] = useState(student!.date_of_birth)

    function handleEdit() {
        editStudent(student!.id, editName, editDob)
        setEditName("")
        setEditDob("")
        closeModals()
    }

    return (
        <div id="edit-student-dialog" className="dialog" style={{display: showDialog?"flex":"none"}}>
            <div className="row">
                <input type="text" value={editName} onChange={(e => {setEditName(e.target.value)})}/>
                <input type="date" value={editDob} onChange={(e => {setEditDob(e.target.value)})}/>
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