import { Paper, Dialog, TextField, Button, Box } from "@mui/material"
import { useEffect, useState } from "react"
import { Student } from "../../types"
import { invoke } from "@tauri-apps/api"

export default function EditStudentDialog(p: Props) {
  const [entered, setEntered] = useState({
    firstNames: "",
    lastName: "",
    dateOfBirth: ""
  });


  function editStudent() {
    console.log(`EDITING ${entered.firstNames} ${entered.lastName} ${entered.dateOfBirth}`);
    let first = entered.firstNames.trim();
    let last = entered.lastName.trim();
    let dob = entered.dateOfBirth.trim();
    if (first === "" || last === "" || dob === "") {
      // validation error
      console.error("invalid student input");
      return;
    }
    invoke("edit_student", { firstNames: entered.firstNames, lastName: entered.lastName, dateOfBirth: entered.dateOfBirth })
      .then((id) => {
        p.refreshStudents();
        p.setStudentToEdit(null);
      })
      .catch(err => {
        console.error(err);
      });
  }

  return (
    <Dialog
      open={true}
      onClose={(_event, reason) => {
        console.log(`closing because ${reason}`);
        p.setStudentToEdit(null)
      }}
    >
      <Paper className="EditStudent" elevation={3}>
        <Box className="inputs">
          <TextField
            id="first_names"
            sx={{ margin: "5px" }}
            type="text"
            label="First names"
            variant="outlined"
            onChange={(e) => setEntered({ ...entered, firstNames: e.target.value })}
          />
          <TextField
            id="last_names"
            sx={{ margin: "5px" }}
            type="text"
            label="Surname"
            variant="outlined"
            onChange={(e) => setEntered({ ...entered, lastName: e.target.value })}
          />
          <TextField
            id="date_of_birth"
            sx={{ margin: "5px" }}
            type="date"
            variant="outlined"
            onChange={(e) => setEntered({ ...entered, dateOfBirth: e.target.value })}
          />
        </Box>
        <Box className="buttons">
          <Button onClick={editStudent} >{"Save"}</Button>
          <Button onClick={() => p.setStudentToEdit(null)} >{"Cancel"}</Button>
        </Box>
      </Paper>
    </Dialog>
  )
}

type Props = {
  studentToEdit: Student | null,
  setStudentToEdit: Function,
  refreshStudents: Function
}

