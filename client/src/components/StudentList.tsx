import { Student } from "../types"
import "./StudentList.css"
import { AddCircleRounded, EditRounded, DeleteRounded, RefreshRounded } from "@mui/icons-material";
import { IconButton, List, ListItemButton, ListItemText, Paper } from "@mui/material";
import { useState } from "react";
import {AddStudentDialog} from "../components";
import { invoke } from "@tauri-apps/api";

export function StudentList(p: Props) {
  const [editMode, setEditMode] = useState(false)
  const [addStudentDialogIsOpen, setAddStudentDialogIsOpen] = useState(false)

  function showAddStudentDialog() {
    setAddStudentDialogIsOpen(true)
  }

  function showEditStudentDialog(student: Student) {
    console.log("edit")
  }

  function showDeleteStudentDialog(student: Student) {
    deleteStudent(student.id);
  }

  function refreshStudents() {
    p.refreshStudents();
  }

  function deleteStudent(studentId: string) {
    console.debug(`deleting ${studentId}`);
    invoke("delete_student", {id: studentId})
      .then(res => {
        refreshStudents();
      })
      .catch(err => {
        console.error(`failed to delete student ${studentId}: ${err}`)
      });
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
          style={{display: editMode?"block":"none", padding:0}} 
          onClick={() => showEditStudentDialog(st)}
        >
          <EditRounded />
        </IconButton>
        <IconButton 
          color="error" 
          style={{display: editMode?"block":"none", padding:0}} 
          onClick={() => showDeleteStudentDialog(st)}
        >
          <DeleteRounded />
        </IconButton>
      </ListItemButton>
    )}
  )

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
    />
    </>
  );
}

export type Props = {
    students: Student[], // students currently cached in ui
    selectStudent: Function, // function to set the selected student
    selectedStudent: Student | null // currently selected student
    refreshStudents: Function
}


