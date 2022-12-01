import { useState } from "react"

export default function AddStudentDialog({
    showDialog,
    addStudent,
    closeModals,
}: Props) {

    const [addName, setAddName] = useState("")
    const [addDob, setAddDob] = useState("")

    function handleAdd() {
        console.log(`adding ${addName} ${addDob}`)
        if (addName.trim() === "" || addDob.trim() === "") {
            console.error(`invalid add student input: name="${addName}" dob="${addDob}"`) // TODO snackbar
        } else {          
            let splitName = addName.split(" ")
            let last_name = splitName.pop()
            let first_names = splitName.join(" ")
            addStudent(first_names, last_name, addDob)
            setAddName("")
            setAddDob("")
            closeModals()
        }
    }

    return (
        <div 
        id="add-student-dialog"
        className="dialog"
        style={{
            display: showDialog?"flex":"none",
        }}>
            <div className="row">
                <input type="text" value={addName} onChange={(e => {setAddName(e.target.value)})}/>
                <input type="date" value={addDob} onChange={(e => {setAddDob(e.target.value)})}/>
                <button onClick={handleAdd}>Add</button>
            </div>
        </div>
    )
}

type Props = {
    showDialog: boolean,
    addStudent: Function,
    closeModals: Function,
}