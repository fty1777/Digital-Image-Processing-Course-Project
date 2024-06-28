import { createTheme, MantineProvider } from '@mantine/core';

import Previewer from './Previewer';
import { FileInfoProvider } from './contexts/FileInfoContext';
import { TabsLayoutProvider } from './contexts/TabsLayoutContext';

import '@mantine/core/styles.css';
import './style/App.css';

// Your theme configuration is merged with default theme
const theme = createTheme({
  fontFamily: 'Montserrat, sans-serif',
  defaultRadius: 'md',
});

function App() {
  return (
    <MantineProvider theme={theme}>
      <FileInfoProvider>
        <TabsLayoutProvider>
          <div className="App">
            <main className="App-main">
              <Previewer />
            </main>
          </div>
        </TabsLayoutProvider>
      </FileInfoProvider>
    </MantineProvider>
  )
}

export default App
