import { Button, Paper, TextField } from "@mui/material";
import useEnhancedEffect from "@mui/material/utils/useEnhancedEffect";
import { Box } from "@mui/system";
import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import "../styles/SafmedScores.css"
import { Score, ScoreProps } from "../types";

export default function SafmedScores(p: ScoreProps) {
    const [correct, setCorrect] = useState("")
    const [incorrect, setIncorrect] = useState("")
    const [currDate, setCurrDate] = useState(new Date().toISOString().split('T')[0])


    const addScore = () => {
        if (p.selectedStudent !== null && p.selectedStudent !== undefined) {
            let correctNum = parseInt(correct, 10)
            let incorrectNum = parseInt(incorrect, 10)
            if (correctNum === null || incorrectNum === null) {
                console.error("Score not a number");
                return;
            }
            invoke("add_safmed_score", {
                id: p.selectedStudent!.id,
                correct: correctNum,
                incorrect: incorrectNum,
                date: currDate
            })
            .then(res => {
                showPlot();
            })
            .catch(err => {
                console.error(err)
            });
        }
    }

    const showPlot = () => {
        invoke("plot_safmed_scores", {studentId: p.selectedStudent!.id})
            .then((res) => {
                console.log(res)
                document.querySelector("#safmed-chart")!.innerHTML = res as string
            })
            .catch(err => {
                console.error(err)
            });
    }

    useEffect(() => {
        if (p.selectedStudent !== null) showPlot()
    }, [p.selectedStudent]);

    return (
        <Box className="tab-container">
            <Box id="safmed-chart" className="score-chart"></Box>
            <Box className="controls">
                <TextField id="correct" type="number" label="Correct" variant="outlined" value={correct} onChange={e => setCorrect(e.target.value)} />
                <TextField id="incorrect" type="number" label="Incorrect" variant="outlined" value={incorrect} onChange={e => setIncorrect(e.target.value)} />
                <TextField id="date" type="date" variant="outlined" value={currDate} onChange={e => setCurrDate(e.target.value)} />
                <Button onClick={addScore} >{"Add score"}</Button>
            </Box>
        </Box>
    )
}
