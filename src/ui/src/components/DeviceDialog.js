import { useState } from 'react';

import { 
    Button, 
    TextField, 
    Dialog, 
    DialogActions, 
    DialogContent, 
    DialogContentText, 
    DialogTitle, 
    Input,
} from '@mui/material';

import { updateDevice, deleteDevice } from '../services/api';

export default function DeviceDialog(props) {
    const [open, setOpen] = useState(false);
    const [id, setID] = useState(props.device._id['$oid']);

    const handleClickOpen = () => {
        setOpen(true);
    }

    const handleClose = () => {
        
        setOpen(false);
    }

    const handleSave = (event) => {
        event.preventDefault();
        
        handleClose();
    }

    const handleDelete = (event) => {
        deleteDevice(id)
        .then( response => {
            console.log(response.data);
        })
        .catch( error => {
            console.log(error);
        })
        .finally( () => {
            handleClose();
        });
    }

    return (
        <div>
            <Button variant='filled' onClick={handleClickOpen}>
                {props.label}
            </Button>
            <Dialog open={open} onClose={handleClose}>
                <DialogTitle>Edit: {props.device.name} {props.device.device_id}</DialogTitle>
                <DialogContent>
                    <Input 
                        margin="dense"
                        id="name"
                        label="name"
                        type="text"
                        fullWidth
                        variant="standard"
                    />
                    
                </DialogContent>
                <DialogActions>
                    <Button onClick={handleClose}>Cancel</Button>
                    <Button onClick={handleSave}>Save</Button>
                    <Button onClick={handleDelete}>Delete</Button>
                </DialogActions>
            </Dialog>
        </div>
    )
}