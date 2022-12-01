import { useState } from "react"

export default function AddStudentDialog({
    showAddStudent,
    addStudent,
    closeModals,
}: Props) {

    const [addName, setAddName] = useState("")
    const [addDob, setAddDob] = useState("")

    return (
        <div 
        id="add-student-dialog"
        className="dialog"
        style={{
            display: showAddStudent?"flex":"none",
        }}>
            <div className="row">
                <input type="text" value={addName} onChange={(e => {setAddName(e.target.value)})}/>
                <input type="date" value={addDob} onChange={(e => {setAddDob(e.target.value)})}/>
                <button onClick={() => {
                    console.log(`adding ${addName} ${addDob}`)
                    let splitName = addName.split(" ")
                    let last_name = splitName.pop()
                    let first_names = splitName.join(" ")
                    addStudent(first_names, last_name, addDob)
                    setAddName("")
                    setAddDob("")
                    closeModals()
                }}>Add</button>
            </div>
        </div>
    )
}

type Props = {
    showAddStudent: boolean,
    addStudent: Function,
    closeModals: Function,
}