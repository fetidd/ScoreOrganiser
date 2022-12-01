import React, { useState } from "react";
import { Student } from "./Student";


type Props = {
    student: Student,
    select: Function,
    selected: string,
    setShowEditStudent: Function,
    setEditName: Function,
    setEditing: Function,
    setEditDob: Function,
    setShowDeleteStudent: Function,
    setDeleting: Function,
    setConfirmDelete: Function,
    setDeleteConfirmationTarget: Function,
    setModal: Function,
}

export default function StudentRow({
    student,
    select,
    selected,
    setShowEditStudent,
    setEditName,
    setEditing,
    setEditDob,
    setShowDeleteStudent,
    setDeleting,
    setDeleteConfirmationTarget,
    setModal,
}: Props) {

    const [hasContextFocus, setHasContextFocus] = useState(false)

    const isSelected = student.id === selected;
    let classes = "student-row"
    if (isSelected) classes += " selected"

    function handleEditClick(e: React.MouseEvent) {
        e.stopPropagation()
        setModal(true)
        setEditing(student.id)
        setEditName(student.name)
        setEditDob(student.dob)
        setShowEditStudent(true)
    }

    function handleDeleteClick(e: React.MouseEvent) {
        e.stopPropagation()
        setModal(true)
        setDeleting(student.id)
        const splitName = student.name.split(" ")
        setDeleteConfirmationTarget(splitName[splitName.length - 1])
        setShowDeleteStudent(true)
    }

    function handleRightClick(e: React.MouseEvent) {
        e.preventDefault()
        select(student.id)
        setHasContextFocus(true)
    }

    return (
        <li>
            <div className={classes} onClick={() => {select(student.id)}} onContextMenu={e => {handleRightClick(e)}} onMouseLeave={() => {setHasContextFocus(false)}} >
                <span>{student.name}</span>
                <button className="icon-button" style={{display: (hasContextFocus) ? "block" : "none", justifySelf: "end"}} onClick={e => {handleEditClick(e)}}>
                    <i className="fa-solid fa-pen"></i>
                </button>
                <button className="icon-button red" style={{display: (hasContextFocus) ? "block" : "none", justifySelf: "end"}} onClick={e => {handleDeleteClick(e)}} >
                    <i className="fa-solid fa-trash-can"></i>
                </button>
            </div>
        </li>
    )
}
