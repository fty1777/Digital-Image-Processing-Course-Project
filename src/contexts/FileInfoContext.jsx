import React, { createContext, useState } from 'react';

const FileInfoContext = createContext();

function FileInfoProvider({ children }) {
  const [openedFolder, setOpenedFolder] = useState(null);
  const [folderTree, setFolderTree] = useState(null);
  const [openedFiles, setOpenedFiles] = useState({});
  const [historyTrees, setHistoryTrees] = useState({});
  const [currentFile, setCurrentFile] = useState(null);

  return (
    <FileInfoContext.Provider value={{
      openedFolderState: [openedFolder, setOpenedFolder],
      folderTreeState: [folderTree, setFolderTree],
      openedFilesState: [openedFiles, setOpenedFiles],
      historyTreesState: [historyTrees, setHistoryTrees],
      currentFileState: [currentFile, setCurrentFile],
    }}>
      {children}
    </FileInfoContext.Provider>
  );
}

export { FileInfoContext, FileInfoProvider };