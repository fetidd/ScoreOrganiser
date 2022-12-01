export default function EditStudentDialog({
    showDialog, 
    editName, 
    setEditName, 
    editDob, 
    setEditDob, 
    editStudent, 
    closeModals
}: Props) {

    return (
        <div 
        id="edit-student-dialog"
        className="dialog"
        style={{
            display: showDialog?"flex":"none",
        }}>
            <div className="row">
                <input type="text" value={editName} onChange={(e => {setEditName(e.target.value)})}/>
                <input type="date" value={editDob} onChange={(e => {setEditDob(e.target.value)})}/>
                <button onClick={() => {
                    setEditName("")
                    setEditDob("")
                    editStudent()
                    closeModals()
                }}>Save</button>
            </div>
        </div>
    )
}

type Props = {
    showDialog: boolean,
    editName: string,
    editDob: string,
    setEditName: Function,
    setEditDob: Function,
    editStudent: Function,
    closeModals: Function
}