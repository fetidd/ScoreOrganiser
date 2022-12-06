import { useRef, useContext, useEffect } from "react"
import SnackbarContext from "../../snackbar-context"

export default function AddStudentDialog({ showDialog, addStudent, closeModals }: Props) {
    const nameInput = useRef<HTMLInputElement>(null)
    const dobInput = useRef<HTMLInputElement>(null)
    const snackCtx = useContext(SnackbarContext)

    function clearInputs() {
        nameInput.current!.value = ""
        dobInput.current!.value = ""
    }

    useEffect(() => {
        if (showDialog) {
            document.getElementById("name-input")?.focus()
        }
    }, [showDialog])

    function handleAdd() {
        let addName: string = nameInput.current!.value.trim()
        let addDob: string = dobInput.current!.value.trim()
        if (addName === "" || addDob === "") {
            snackCtx.error(`invalid add student input: name="${addName}" dob="${addDob}"`)
        } else {
            let splitName = addName.split(" ")
            let last_name = splitName.pop()
            let first_names = splitName.join(" ")
            addStudent(first_names, last_name, addDob)
            clearInputs()
            closeModals()
        }
    }

    return (
        <div id="add-student-dialog" className="dialog" style={{ display: showDialog ? "flex" : "none" }}>
            <div className="topbar">
                <span><em>Add student</em></span>
                <i className="fa fa-close" onClick={() => {
                    clearInputs()
                    closeModals()
                }} />
            </div>
            <div className="row">
                <label>Name</label>
                <input id="name-input" type="text" ref={nameInput} />
            </div>
            <div className="row">
                <label>Date of birth</label>
                <input type="date" ref={dobInput} />
            </div>
            <div className="row">
                <button className="button wide" onClick={handleAdd}>Add</button>
            </div>
        </div>
    )
}

interface Props {
    showDialog: boolean,
    addStudent: Function,
    closeModals: Function,
}
