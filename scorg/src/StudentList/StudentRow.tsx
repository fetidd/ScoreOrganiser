import { useState } from "react";
import { Student } from "./Student";

export default function StudentRow({student, selected, select, setModal, setShowEditStudentDialog, setShowDeleteStudentDialog, setEditing, setDeleting}: Props) {
    const [hasContextFocus, setHasContextFocus] = useState("")

    const isSelected = student.id === selected;
    let classlist = "student-row"
    if (isSelected) classlist += " selected"

    const rightClick = (e: any) => {
        e.preventDefault()
        select(student.id)
        setHasContextFocus(student.id)
    }

    return (
        <li>
            <div className={classlist} onClick={() => {select(student.id)}} onContextMenu={e => rightClick(e)} onMouseLeave={() => setHasContextFocus("")}>
                <span>{student.name}</span>
                <button className="icon-button" style={{
                        display: (hasContextFocus === student.id) ? "block" : "none",
                        justifySelf: "end"
                    }}
                    onClick={e => {
                        e.stopPropagation()
                        setEditing(student.id)
                        setShowEditStudentDialog(true)
                        setModal(true)
                    }}
                ><i className="fa-solid fa-pen"></i></button>
                <button
                    className="icon-button red"
                    style={{
                        display: (hasContextFocus === student.id) ? "block" : "none",
                        justifySelf: "end"
                    }}
                    onClick={e => {
                        e.stopPropagation()
                        setDeleting(student.id)
                        setShowDeleteStudentDialog(true)
                        setModal(true)
                    }}
                ><i className="fa-solid fa-trash-can"></i></button>
            </div>

        </li>
    )
}

type Props = {
    student: Student,
    selected: string,
    select: Function,
    handleDeleteStudent: Function,
    handleEditStudent: Function,
    setModal: Function,
    setShowDeleteStudentDialog: Function,
    setShowEditStudentDialog: Function,
    setEditing: Function,
    setDeleting: Function
}