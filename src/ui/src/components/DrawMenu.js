import { useState } from 'react';
import { 
    Drawer,
    Button,
    List,
    ListItem,
    ListItemButton,
    ListItemText ,
    Box,
    ListItemIcon
} from '@mui/material';
import MenuIcon from '@mui/icons-material/Menu';

export default function DrawerMenu(props) {
    const [show, setShow] = useState(false);

    const toggle = (open) => (event) => {
        if (event.type === 'keydown' && (event.key === 'Tab' || event.key === 'Shift') ) {
            return;
        }
        setShow(open);
    };

    const changeView = (name) => (event) => {
        props.setActiveView(name);
    }

    return (
        <div className='drawer-menu'>
            <Button onClick={toggle(true)}><MenuIcon/></Button>
            <Drawer
                anchor='left'
                open={show}
                onClose={toggle(false)}
            >
                <Box
                    sx={{ width: 250 }}
                    role="presentation"
                    onClick={toggle(false)}
                    onKeyDown={toggle(false)}
                >
                <List>
                    {props.views.map((page) => (
                        <ListItem key={page.name} disablePadding>
                            <ListItemButton onClick={changeView(page.name)}>
                                <ListItemIcon>
                                    {page.icon}
                                </ListItemIcon>
                                <ListItemText primary={page.name}/>
                            </ListItemButton>
                        </ListItem>
                    ))}
                </List>
                </Box>
            </Drawer>
        </div>
    )
}
