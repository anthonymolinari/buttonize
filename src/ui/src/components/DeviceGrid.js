import { useState, useEffect } from "react";
import { getDevices } from "../services/api";
import DeviceCard from "./DeviceCard";

import { CircularProgress, Box } from "@mui/material";
import Grid from '@mui/material/Unstable_Grid2';

export default function DeviceGrid() {

    const [ loading, setLoading ] = useState(true);
    const [ error, setError ] = useState(false);
    const [ data, setData ] = useState([]);

    useEffect(() => {
        getDevices()
        .then( response => {
            setData(response.data);
        })
        .catch( error => {
            setError(true);
            console.log(error);
        })
        .finally( () => {
            setLoading(false);
        })
    }, []);

    
    if (loading)
        return (
            <Box sx={{ display: 'flex' }}>
                <CircularProgress/>
            </Box>
        );

    if (error)
        return <span>Error Connot Load Devices</span>

    return (
        <Box sx={{ flexGrow: 1, m: 10, minWidth: 608, maxWidth: 1000 }}>
            <Grid container spacing={1} >
                {data.map((device) => (
                    <DeviceCard device={device}/>
                ))}
            </Grid>
        </Box>
    );
}
