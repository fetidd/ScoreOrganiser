export default function DeleteStudentDialog({
    showDeleteStudent,
    confirmDelete,
    setConfirmDelete,
    deleteConfirmationTarget,
    deleteStudent,
    closeModals,
    setDeleteConfirmationTarget
}: Props) {
    return (
        <div 
        id="delete-student-dialog"
        className="dialog"
        style={{
            display: showDeleteStudent?"flex":"none",
        }}>
            <div className="row">
                <span>Type student's last name to confirm delete</span>
            </div>
            <div className="row">
                <input type="text" value={confirmDelete} onChange={(e => {setConfirmDelete(e.target.value)})}/>
                <button onClick={() => {
                    if (confirmDelete === deleteConfirmationTarget) {
                        deleteStudent()
                        closeModals()
                    }
                    setConfirmDelete("")
                    setDeleteConfirmationTarget("")
                }}>Confirm</button>
            </div>
        </div>
    )
}

type Props = {
    showDeleteStudent: boolean,
    confirmDelete: string,
    setConfirmDelete: Function,
    deleteConfirmationTarget: string,
    deleteStudent: Function,
    closeModals: Function,
    setDeleteConfirmationTarget: Function,
}