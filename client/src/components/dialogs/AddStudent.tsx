import { Paper, Dialog, TextField, Button, Box } from "@mui/material"
import { useState } from "react"
import { Student } from "../../types"
import { invoke } from "@tauri-apps/api"
import "./AddStudent.css"

export default function AddStudentDialog(p: Props) {
  const [entered, setEntered] = useState({
    firstNames: "",
    lastName: "",
    dateOfBirth: ""
  });

  function addStudent() {
    console.log(`ADDING ${entered.firstNames} ${entered.lastName} ${entered.dateOfBirth}`);
    let first = entered.firstNames.trim();
    let last = entered.lastName.trim();
    let dob = entered.dateOfBirth.trim();
    if (first === "" || last === "" || dob === "") {
      // validation error
      console.error("invalid student input");
      return;
    }
    invoke("add_student", {firstNames: entered.firstNames, lastName: entered.lastName, dateOfBirth: entered.dateOfBirth})
      .then((id) => {
        p.setIsOpen(false);
        p.refreshStudents();
        p.selectStudent(id);
      })
      .catch(err => {
        console.error(err);
      });
  }

  return (
    <Dialog
      open={p.isOpen}
      onClose={(_event, reason) => {
        console.log(`closing because ${reason}`);
        p.setIsOpen(false)
      }}
    >
      <Paper className="AddStudent" elevation={3}>
        <Box className="inputs">
          <TextField 
            id="first_names" 
            sx={{margin: "5px"}}
            type="text" 
            label="First names" 
            variant="outlined"
            onChange={(e) => setEntered({...entered, firstNames: e.target.value})}
            />
          <TextField 
            id="last_names" 
            sx={{margin: "5px"}}
            type="text" 
            label="Surname" 
            variant="outlined" 
            onChange={(e) => setEntered({...entered, lastName: e.target.value})}
            />
          <TextField 
            id="date_of_birth" 
            sx={{margin: "5px"}}
            type="date" 
            variant="outlined" 
            onChange={(e) => setEntered({...entered, dateOfBirth: e.target.value})}
            />
        </Box>
        <Box className="buttons">
          <Button onClick={addStudent} >{"Add student"}</Button>
        </Box>
      </Paper>
    </Dialog>
  )
}

type Props = {
  isOpen: boolean,
  setIsOpen: Function,
  selectStudent: Function,
  refreshStudents: Function
}

