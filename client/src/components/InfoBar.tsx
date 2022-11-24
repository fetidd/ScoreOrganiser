import { Box, Paper } from "@mui/material"

export default function InfoBar(p: Props) {
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