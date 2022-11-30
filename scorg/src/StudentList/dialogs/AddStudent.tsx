export default function AddStudentDialog({
    showAddStudent,
    addName,
    setAddName,
    addDob,
    setAddDob,
    addStudent,
    closeModals,
}: Props) {
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
                    setAddName("")
                    setAddDob("")
                    addStudent()
                    closeModals()
                }}>Add</button>
            </div>
        </div>
    )
}

type Props = {
    showAddStudent: boolean,
    addName: string,
    setAddName: Function,
    addDob: string,
    setAddDob: Function,
    addStudent: Function,
    closeModals: Function,
}