import { Box, Paper } from "@mui/material"
import { StatusMessageLevel } from "../types"

export default function InfoBar(p: Props) {
    let color: string
    switch (p.statusMessage.level) {
        case StatusMessageLevel.Info: color = "black"; break;
        case StatusMessageLevel.Warning: color = "orange"; break;
        case StatusMessageLevel.Error: color = "red"; break;
        case StatusMessageLevel.Debug: color = "blue"; break;
    }
    return (
        <Paper elevation={3}>
            <Box className="infobar">
                <span style={{fontSize: "1.5em", margin: "5px", color: color}} id="status-message">{p.statusMessage.msg || "Idle"}</span>
            </Box>
        </Paper>
    )
}

type Props = {
    statusMessage: {msg: string, level: StatusMessageLevel}
}