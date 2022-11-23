import { Button, Paper, TextField } from "@mui/material";
import { Box } from "@mui/system";
import { useState } from "react";
import "../styles/SafmedScores.css"
import { Score, ScoreProps } from "../types";

export default function SafmedScores(p: ScoreProps) {
    const [chart, setChart] = useState("") // dunno if this is what a 'null' chart should be
    const [scores, setScores] = useState([] as Score[])

    const [correct, setCorrect] = useState("")
    const [incorrect, setIncorrect] = useState("")
    const [currDate, setCurrDate] = useState(new Date().toISOString().split('T')[0])


    const addScore = () => {
        getScores()
    }

    const getScores = () => {
        return
    }

    return (
        <Box className="tab-container">
            <Box className="score-chart">
                <img className="chart"></img>
            </Box>
            <Box className="controls">
                <TextField id="correct" type="number" label="Correct" variant="outlined" value={correct} onChange={e => setCorrect(e.target.value)} />
                <TextField id="incorrect" type="number" label="Incorrect" variant="outlined" value={incorrect} onChange={e => setIncorrect(e.target.value)} />
                <TextField id="date" type="date" variant="outlined" value={currDate} onChange={e => setCurrDate(e.target.value)} />
                <Button onClick={addScore} >{"Add score"}</Button>
            </Box>
        </Box>
    )
}
