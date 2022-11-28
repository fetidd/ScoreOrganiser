import { Box, Button, Dialog, Paper, TextField, Typography } from "@mui/material";
import { invoke } from "@tauri-apps/api";
import { useState } from "react";
import { Student } from "../../types"

export default function DeleteStudentDialog(p: Props) {
    const [confirm, setConfirm] = useState("")

    const deleteStudent = () => {
        if (p.studentToDelete === null || p.studentToDelete === undefined) {
            console.error("studentToDelete was null or undefined")
            return
        }
        if (confirm === (p.studentToDelete as Student).last_name) {
            invoke("delete_student", { id: p.studentToDelete.id })
                .then(res => {
                    p.refreshStudents();
                    p.setStudentToDelete(null);
                    p.selectStudent(null)
                })
                .catch(err => {
                    console.error(`that shouldnt have failed: ${err}`);
                });
        } else {
            console.error("name didnt match, not deleting student");
        }
    }

    return (
        <Dialog
            open={true}
            onClose={(_event, reason) => {
                p.setStudentToDelete(null)
            }}
        >
            <Paper className="DeleteStudent" elevation={3}>
                <Box 
                sx={{display: "flex", justifyContent: "space-between"}}
                className="inputs">
                    <Typography variant="h5">{"Enter student's surname to confirm deletion"}</Typography>
                    <TextField
                        id="last_name"
                        sx={{ margin: "5px" }}
                        type="text"
                        label="Confirmation"
                        variant="outlined"
                        onChange={ev => setConfirm(ev.target.value)}
                    />
                </Box>
                <Box className="buttons">
                    <Button onClick={deleteStudent} >{"Delete"}</Button>
                    <Button onClick={() => p.setStudentToDelete(null)} >{"Cancel"}</Button>
                </Box>
            </Paper>
        </Dialog>
    )
}

type Props = {
    studentToDelete: Student | null,
    refreshStudents: Function,
    setStudentToDelete: Function,
    selectStudent: Function,
}
