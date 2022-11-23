import { StatusMessageLevel, Student } from "../types"
import "./StudentList.css"
import { AddCircleRounded, EditRounded, DeleteRounded, RefreshRounded } from "@mui/icons-material";
import { IconButton, List, ListItemButton, ListItemText, Paper } from "@mui/material";
import { useState } from "react";
import { AddStudentDialog, DeleteStudentDialog, EditStudentDialog } from "../components";

export function StudentList(p: Props) {
  const [editMode, setEditMode] = useState(false)
  const [addStudentDialogIsOpen, setAddStudentDialogIsOpen] = useState(false)
  const [studentToDelete, setStudentToDelete] = useState(null as Student | null)
  const [studentToEdit, setStudentToEdit] = useState(null as Student | null)

  const showAddStudentDialog = () => {
    setAddStudentDialogIsOpen(true)
  }

  const refreshStudents = () => {
    p.setStatusMessage("Getting students...", StatusMessageLevel.Info)
    p.refreshStudents();
  }

  let rows = p.students.map((st: Student) => {
    return (
      <ListItemButton
        key={st.id}
        selected={st === p.selectedStudent}
        onClick={() => p.selectStudent(st.id)}
        disableGutters
        disableRipple
      >
        <ListItemText primary={`${st.first_names} ${st.last_name}`} />
        <IconButton
          style={{ display: editMode ? "block" : "none", padding: 0 }}
          onClick={() => setStudentToEdit(st)}
        >
          <EditRounded />
        </IconButton>
        <IconButton
          color="error"
          style={{ display: editMode ? "block" : "none", padding: 0 }}
          onClick={() => setStudentToDelete(st)}
        >
          <DeleteRounded />
        </IconButton>
      </ListItemButton>
    )
  }
  )

  const deleteDialog = studentToDelete !== null ?
    (<DeleteStudentDialog
      refreshStudents={p.refreshStudents}
      setStudentToDelete={setStudentToDelete}
      studentToDelete={studentToDelete}
    />) : (<></>)

  const editDialog = studentToEdit !== null ?
    (<EditStudentDialog
      refreshStudents={p.refreshStudents}
      setStudentToEdit={setStudentToEdit}
      studentToEdit={studentToEdit}
    />) : (<></>)

  return (
    <>
      <Paper elevation={3} className="StudentList">
        <div id="menu">
          {/* <h2 className="pane-header">Students</h2> */}
          <IconButton onClick={showAddStudentDialog}>
            <AddCircleRounded />
          </IconButton>
          <IconButton onClick={() => setEditMode(!editMode)}>
            <EditRounded />
          </IconButton>
          <IconButton onClick={refreshStudents}>
            <RefreshRounded />
          </IconButton>
        </div>
        <List>{rows}</List>
      </Paper>
      <AddStudentDialog
        isOpen={addStudentDialogIsOpen}
        setIsOpen={setAddStudentDialogIsOpen}
        selectStudent={p.selectStudent}
        refreshStudents={p.refreshStudents}
        setStatusMessage={p.setStatusMessage}
      />
      {deleteDialog}
      {editDialog}
    </>
  );
}

export type Props = {
  students: Student[], // students currently cached in ui
  selectStudent: Function, // function to set the selected student
  selectedStudent: Student | null // currently selected student
  refreshStudents: Function,
  setStatusMessage: Function
}
