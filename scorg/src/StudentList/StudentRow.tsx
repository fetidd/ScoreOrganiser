import React, { useState } from "react";
import { Student } from "./Student";


interface Props {
    student: Student,
    select: Function,
    selected: string,
    setShowEditStudent: Function,
    setEditing: Function,
    setShowDeleteStudent: Function,
    setDeleting: Function,
    setModal: Function,
}

export default function StudentRow({
    student,
    select,
    selected,
    setShowEditStudent,
    setEditing,
    setShowDeleteStudent,
    setDeleting,
    setModal,
}: Props) {

    const [hasContextFocus, setHasContextFocus] = useState(false)

    const isSelected = student.id === selected;
    let classes = "student-row"
    if (isSelected) classes += " selected"

    function handleEditClick(e: React.MouseEvent) {
        e.stopPropagation()
        setModal(true)
        setEditing(student)
        setShowEditStudent(true)
    }

    function handleDeleteClick(e: React.MouseEvent) {
        e.stopPropagation()
        setModal(true)
        setDeleting(student)
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
                <span>{`${student.first_names} ${student.last_name}`}</span>
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
