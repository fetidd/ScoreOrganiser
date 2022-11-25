import { Student } from "../types"
import { AddCircleRounded, EditRounded, DeleteRounded, RefreshRounded } from "@mui/icons-material";
import { Box, IconButton, List, ListItemButton, ListItemText, Paper, TextField } from "@mui/material";
import { useState } from "react";
import { AddStudentDialog, DeleteStudentDialog, EditStudentDialog } from "../components";
import { invoke } from "@tauri-apps/api";

export function StudentList(p: Props) {
  const [editMode, setEditMode] = useState(false)
  const [addStudentDialogIsOpen, setAddStudentDialogIsOpen] = useState(false)
  const [studentToDelete, setStudentToDelete] = useState(null as Student | null)
  const [studentToEdit, setStudentToEdit] = useState(null as Student | null)
  const [file, setFile] = useState(null as File|null)

  const showAddStudentDialog = () => {
    setAddStudentDialogIsOpen(true)
  }

  const refreshStudents = () => {
    p.refreshStudents();
  }

  const uploadFile = () => {
    
    if (file !== null) {
      file.text().then(t => {
        invoke("import_csv", {file: t})
          .then(res => {
            refreshStudents()
          })
          .catch(err => {
            console.error(err)
          })
      })
    }
  }

  let rows = p.students.map((st: Student) => {
    return (
      <ListItemButton
        className={"studentrow"}
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

  return (
    <>
    <Paper className="StudentList" elevation={3}>
      <div id="menubar">
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
      <List id="list">{rows}</List>
      <Box>
        <input
          id="import" 
          type="file"
          onChange={e => {
            let fileList = (e.target as HTMLInputElement).files;
            if (fileList && fileList.length === 1) {
              setFile(fileList[0])
            }
          }}
        />
          <IconButton 
            onClick={uploadFile}
          >
            <RefreshRounded />
          </IconButton>
      </Box>
    </Paper>


    <AddStudentDialog
      isOpen={addStudentDialogIsOpen}
      setIsOpen={setAddStudentDialogIsOpen}
      selectStudent={p.selectStudent}
      refreshStudents={p.refreshStudents}
    />
    {studentToDelete !== null && (
      <DeleteStudentDialog
        refreshStudents={p.refreshStudents}
        setStudentToDelete={setStudentToDelete}
        studentToDelete={studentToDelete}
      />)}
    {studentToEdit !== null && (
      <EditStudentDialog
        refreshStudents={p.refreshStudents}
        setStudentToEdit={setStudentToEdit}
        studentToEdit={studentToEdit}
      />)}
    </>
  );
}

export type Props = {
  students: Student[], // students currently cached in ui
  selectStudent: Function, // function to set the selected student
  selectedStudent: Student | null // currently selected student
  refreshStudents: Function,
}
