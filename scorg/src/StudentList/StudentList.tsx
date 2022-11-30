import { useEffect, useState } from "react";
import { Student } from "./Student"

export default function StudentList({students, setStudents, selected, select, getStudents}: Props) {
    const [file, setFile] = useState(null as File | null)
    
    const [modal, setModal] = useState(false)
    
    const [showAddStudent, setShowAddStudent] = useState(false)
    const [addName, setAddName] = useState("")
    const [addDob, setAddDob] = useState("")

    const [showEditStudent, setShowEditStudent] = useState(false)
    const [editing, setEditing] = useState("")
    const [editName, setEditName] = useState("")
    const [editDob, setEditDob] = useState("")

    const [contextMode, setContextMode] = useState(false)
    const [hasContextFocus, setHasContextFocus] = useState("")


    const addStudent = () => { // TODOINVOKE
        let newStudent: Student = {id: `st${students.length}`, name: addName, dob: addDob}
        students.push(newStudent)
        setStudents(students)
    }

    const deleteStudent = () => { // TODOINVOKE
        let newStudents = students.filter(st => st.id !== hasContextFocus)
        setStudents(newStudents)
        setContextMode(false)
        setHasContextFocus("")
    }

    const editStudent = () => { // TODOINVOKE
        let edited = students.find(st => st.id === editing)
        if (edited !== null && edited !== undefined) {
            edited.name = editName
            edited.dob = editDob
        }
        setHasContextFocus("")
        setEditing("")
    }

    useEffect(() => {getStudents()}, [])

    const closeModals = () => {
        setShowAddStudent(false)
        setShowEditStudent(false)
        setModal(false)
    }

    const uploadFile = () => {
        console.log(file);
        (document.querySelector("#csv-input") as HTMLInputElement).value = ""
        setFile(null)
    }

    const rows = students.map(student => {
        const isSelected = student.id === selected;
        return (
            <li key={student.id}>
                <div 
                style={{
                    backgroundColor: isSelected?"red":"transparent",
                    color: isSelected?"white":"black",
                    display: "flex",
                    alignItems: "center",
                    height: "30px",
                }} 
                className="student-row" 
                onClick={() => {
                    select(student.id)
                }}
                onContextMenu={e => {
                    e.preventDefault()
                    select(student.id)
                    setContextMode(!contextMode)
                    setHasContextFocus(student.id)
                }}
                onMouseLeave={() => {
                    setContextMode(false)
                    setHasContextFocus("")
                }}
                >
                    <span
                    style={{
                        flexGrow: "1",
                    }}
                    >{student.name}</span>
                    <button 
                    style={{
                        display: (contextMode && hasContextFocus===student.id)?"block":"none",
                        justifySelf: "end"
                    }}
                    onClick={e => {
                        e.stopPropagation()
                        deleteStudent()
                    }}
                    >Delete</button>
                    <button 
                    style={{
                        display: (contextMode && hasContextFocus===student.id)?"block":"none",
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
                    >Edit</button>
                </div>

                <div 
                    id="edit-student-dialog"
                    className="dialog"
                    style={{
                        display: showEditStudent?"flex":"none",
                    }}>
                    <input type="text" value={editName} onChange={(e => {setEditName(e.target.value)})}/>
                    <input type="date" value={editDob} onChange={(e => {setEditDob(e.target.value)})}/>
                    <button onClick={() => {
                        console.log(`editing ${editName} ${editDob}`)
                        setEditName("")
                        setEditDob("")
                        editStudent()
                        closeModals()
                    }}>Save</button>
                </div>
            </li>
        )
    });

    return (
        <>
        <div id="StudentList">
            <div id="menubar-area">
                <button onClick={() => {
                    setModal(true)
                    setShowAddStudent(true)
                }}>Add student</button>
            </div>

            <div id="list-area">
                <ul id="student-list">
                    {rows}
                </ul>
            </div>

            <div id="import-csv-area">
                <input 
                id="csv-input"
                type="file"
                accept=".csv"
                onChange={e => {
                    const files = e.target.files
                    if (files !== null && files[0] !== null) {
                        setFile(files[0])
                    }
                }}/>
                <button onClick={uploadFile}>Upload</button>
            </div>
        </div>
        
        <div 
        className="modal" 
        onClick={() => closeModals()}
        style={{
            display: modal?"block":"none",
        }}></div>
        <div 
        id="add-student-dialog"
        className="dialog"
        style={{
            display: showAddStudent?"flex":"none",
        }}>
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
        </>
    )
}

type Props = {
    students: Student[],
    setStudents: Function,
    selected: string,
    select: Function,
    getStudents: Function
}

