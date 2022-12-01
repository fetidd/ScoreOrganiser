import { useState } from "react";
import DeleteStudentDialog from "./dialogs/DeleteStudent";
import EditStudentDialog from "./dialogs/EditStudent";
import { Student } from "./Student";

export default function StudentRow ({
    student,
    select,
    selected,

    showEditStudent,
    setShowEditStudent,
    editName,
    editDob,
    setEditName,
    setEditing,
    setEditDob,
    editStudent,
    showDeleteStudent,
    setShowDeleteStudent,
    setDeleting,
    confirmDelete,
    setConfirmDelete,
    deleteConfirmationTarget,
    setDeleteConfirmationTarget,
    deleteStudent,
    closeModals,
    setModal,
}: Props) {

    const [hasContextFocus, setHasContextFocus] = useState(false)

    const isSelected = student.id === selected;
    let classes = "student-row"
    if (isSelected) classes += " selected"
    return (
        <li>
            <div
                className={classes}
                onClick={() => {
                    select(student.id)
                }}
                onContextMenu={e => {
                    e.preventDefault()
                    select(student.id)
                    setHasContextFocus(true)
                }}
                onMouseLeave={() => {
                    setHasContextFocus(false)
                }}
            >
                <span>{student.name}</span>
                <button
                    className="icon-button"
                    style={{
                        display: (hasContextFocus) ? "block" : "none",
                        justifySelf: "end"
                    }}
                    onClick={e => {
                        e.stopPropagation()
                        setEditName(student.name)
                        setEditDob(student.dob)
                        setModal(true)
                        setEditing(student.id)
                        setShowEditStudent(true)
                    }}
                ><i className="fa-solid fa-pen"></i></button>
                <button
                    className="icon-button red"
                    style={{
                        display: (hasContextFocus) ? "block" : "none",
                        justifySelf: "end"
                    }}
                    onClick={e => {
                        e.stopPropagation()
                        setShowDeleteStudent(true)
                        setModal(true)
                        setDeleting(student.id)
                        const splitName = student.name.split(" ")
                        setDeleteConfirmationTarget(splitName[splitName.length - 1])
                    }}
                ><i className="fa-solid fa-trash-can"></i></button>
            </div>

            <EditStudentDialog
                showEditStudent={showEditStudent}
                editName={editName}
                editDob={editDob}
                setEditName={setEditName}
                setEditDob={setEditDob}
                editStudent={editStudent}
                closeModals={closeModals}
            />

            <DeleteStudentDialog
                showDeleteStudent={showDeleteStudent}
                confirmDelete={confirmDelete}
                setConfirmDelete={setConfirmDelete}
                deleteConfirmationTarget={deleteConfirmationTarget}
                deleteStudent={deleteStudent}
                closeModals={closeModals}
                setDeleteConfirmationTarget={setDeleteConfirmationTarget}
            />

        </li>
    )
}

type Props = {
    student: Student,
    select: Function,
    selected: string,

    showEditStudent: boolean,
    setShowEditStudent: Function,
    editName: string, 
    editDob: string,
    setEditName: Function,
    setEditing: Function,
    setEditDob: Function, 
    editStudent: Function,

    showDeleteStudent: boolean,
    setShowDeleteStudent: Function,
    setDeleting: Function,
    confirmDelete: string,
    setConfirmDelete: Function,
    deleteConfirmationTarget: string,
    setDeleteConfirmationTarget: Function,
    deleteStudent: Function,

    closeModals: Function,
    setModal: Function,
}