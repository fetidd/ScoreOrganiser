import { Paper, TextField } from "@mui/material";
import { Box } from "@mui/system";
import { ScoreProps } from "../types";
import "./SafmedScores.css"

export default function SafmedScores(p: ScoreProps) {
    const fieldStyle = {margin: "0 5px"};

    return (
        <Box>
            <Box>
                <Box className="score-chart">{"p"}</Box>
            </Box>
            <Box sx={{padding: "10px"}}>
                <TextField sx={fieldStyle} id="correct" type="number" label="Correct" variant="outlined" />
                <TextField sx={fieldStyle} id="incorrect" type="number" label="Incorrect" variant="outlined" />
                <TextField sx={fieldStyle} id="date" type="date" variant="outlined" />
            </Box>
        </Box>
    )
}
