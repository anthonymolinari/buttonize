import { useState } from 'react';

import { createTheme, ThemeProvider } from '@mui/material/styles';
import { Alert, CssBaseline } from '@mui/material';

import Header from './components/Header';

import './App.css';

import HomePage from './pages/HomePage';
import AutomationPage from './pages/AutomationPage';
import SettingsPage from './pages/SettingsPage';

import HomeIcon from '@mui/icons-material/Home';
import BoltIcon from '@mui/icons-material/Bolt';
import SettingsIcon from '@mui/icons-material/Settings';
import ThemeSwitcher from './components/ThemeSwitcher';

const darkTheme = createTheme({
  palette: {
    mode: 'dark',
  },
})

const views = [
  {
    "name": "Home",
    "icon": <HomeIcon/>,
    "component": <HomePage/>
  },
  {
    "name": "Automations",
    "icon": <BoltIcon/>,
    "component": <AutomationPage/>
  },
  {
    "name": "Settings",
    "icon": <SettingsIcon/>,
    "component": <SettingsPage/>
  }
];

function App() {
  const [activeView, setActiveView] = useState("Home");
  const [darkMode, setDarkMode] = useState(false);

  return (
    <div className="App">
      <Header 
        activeView={activeView} 
        setActiveView={setActiveView} 
        views={views}
      />
      {(views.find( v => {
        console.log(v.name)
        if (v.name === activeView)
          return v;
        return null;
      })).component}
      {/* <ThemeSwitcher darkMode={darkMode} setDarkMode={setDarkMode}/> */}
    </div>
  );
}

export default App;
