import { Box, Button, Dialog, Paper, TextField, Typography } from "@mui/material";
import { invoke } from "@tauri-apps/api";
import { useState } from "react";
import { Student } from "../../types"

export default function DeleteStudentDialog(p: Props) {
    const [confirm, setConfirm] = useState("")

    const deleteStudent = () => {
        if (p.studentToDelete === null) return;
        if (confirm === (p.studentToDelete as Student).last_name) {
            invoke("delete_student", {id: p.studentToDelete.id})
                .then(res => {
                    p.refreshStudents();
                    p.setIsOpen(false);
                })
                .catch(err => {
                    console.error("that shouldnt have failed");
                });
        } else {
            console.error("name didnt match, not deleting student");
        }
    }

    return (
        <Dialog
          open={p.isOpen}
          onClose={(_event, reason) => {
            console.log(`closing because ${reason}`);
            p.setIsOpen(false)
          }}
        >
          <Paper className="DeleteStudent" elevation={3}>
            <Box className="inputs">
              <Typography variant="h5">{"Enter student's surname to confirm deletion"}</Typography>
              <TextField 
                id="last_name" 
                sx={{margin: "5px"}}
                type="text" 
                label="Confirmation" 
                variant="outlined"
                onChange={ev => setConfirm(ev.target.value)}
                />
            </Box>
            <Box className="buttons">
              <Button onClick={deleteStudent} >{"Delete"}</Button>
              <Button onClick={() => p.setIsOpen(false)} >{"Cancel"}</Button>
            </Box>
          </Paper>
        </Dialog>
      )
}

type Props = {
    studentToDelete: Student|null,
    isOpen: boolean,
    setIsOpen: Function,
    refreshStudents: Function
}