import * as React from 'react';

import ArticleIcon from '@mui/icons-material/Article';
import HelpIcon from '@mui/icons-material/Help';
import GitHubIcon from '@mui/icons-material/GitHub';

import Stack from '@mui/material/Stack';
import Grid2 from '@mui/material/Unstable_Grid2/Grid2';

import DrawerMenu from './DrawMenu';

const links = [
    {name: "Documentation", href: "", icon: <ArticleIcon/>},
    {name: "Help", href: "", icon: <HelpIcon/>},
    {name: "Repository", href: "", icon: <GitHubIcon/>}
]

export default function Header(props) {
    const items = links.map((link) => (
        <div className='link-container'>
            <i href={link.href} title={link.name}>{link.icon}</i>
        </div>
    ));


    return (
        <div className='app-header'>
            <Grid2 container spacing={1}>
                <Grid2 xs={6}>
                    <DrawerMenu activeView={props.activeView} setActiveView={props.setActiveView} views={props.views}/>
                </Grid2>
                <Grid2 xs={6}>
                    <Stack direction="row" spacing={2} mt={2} mb={2} mr={2} justifyContent='flex-end'>
                        {items}
                    </Stack>
                </Grid2>
            </Grid2>
        </div>
    )
}