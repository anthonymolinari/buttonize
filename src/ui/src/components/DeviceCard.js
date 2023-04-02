import { useState } from 'react';
import {
    Card, 
    CardActions, 
    CardContent, 
    Button, 
    Typography, 
    Chip,
    Divider,
    Paper
} from "@mui/material";
import { Stack } from "@mui/system";

import DeviceDialog from "./DeviceDialog";

export default function DeviceCard(props) {

    const [online, setOnline] = useState(true);

    // lookup automations linked to this device via api

    const device = props.device;
    return (
        <Card sx={{ minWidth: 300, maxWidth: 300, minHeight: 210, maxHeight: 210, backgroundColor: 'whitesmoke', p: 0.25, m: 1 }}>
            <CardContent sx={{ backgroundColor: 'lightblue', borderRadius: 1 }}>
                <Stack spacing={0.25}>
                    <Typography variant="h5" component="div">
                        {device.name}
                    </Typography>
                    <Typography>
                        Buttons: {device.buttons.length}
                    </Typography>
                    <Typography>
                        Automations: 0
                    </Typography>
                </Stack>
                <Divider sx={{ m: 1, width: '100%' }}/>
                {online ? 
                    (<Chip label="Online" sx={{backgroundColor: 'yellowgreen'}}/>): 
                    (<Chip label="Offline" sx={{backgroundColor: 'red'}}/>)
                }
            </CardContent>
            <CardActions>
                <DeviceDialog label="Edit" device={device} />
            </CardActions>
        </Card>
    )
}
