import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import '@mantine/core/styles.css';

import { createTheme, MantineProvider } from '@mantine/core';
import Previewer from './Previewer.jsx'

// Your theme configuration is merged with default theme
const theme = createTheme({
  fontFamily: 'Montserrat, sans-serif',
  defaultRadius: 'md',
});

function App() {
  const [count, setCount] = useState(0)

  return (
    <MantineProvider theme={theme}>
      <div className="App">
        <main className="App-main">
          <Previewer />
        </main>
      </div>
    </MantineProvider>
  )
}

export default App
