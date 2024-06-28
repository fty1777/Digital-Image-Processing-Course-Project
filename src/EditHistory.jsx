import React, { useEffect, useContext, useState } from 'react';
import { Actions } from 'flexlayout-react';
import TreeView, { flattenTree } from "react-accessible-treeview";

import { FaFileImage } from "react-icons/fa6";

import { FileInfoContext } from './contexts/FileInfoContext';
import { TabsLayoutContext } from './contexts/TabsLayoutContext';

import './style/Treeview.css';

const imageFileColor = "red";

function EditHistory() {

  const {
    openedFolderState,
    folderTreeState,
    openedFilesState,
    historyTreesState,
    currentFileState,
  } = useContext(FileInfoContext);
  const [historyTrees, setHistoryTrees] = historyTreesState;
  const [historyTree, setHistoryTree] = useState(null);
  const [currentFile, setCurrentFile] = currentFileState;

  useEffect(() => {
    if (currentFile && currentFile in historyTrees) {
      setHistoryTree(flattenTree(historyTrees[currentFile]));
    } else {
      setHistoryTree(null);
    }
    console.log(currentFile, historyTree)
  }, [currentFile]);

  return (

    <div className="ide-container">
      <div className="ide">
        {historyTree ? (
          <TreeView
            data={historyTree}
            aria-label="history tree"
            togglableSelect
            clickAction="EXCLUSIVE_SELECT"
            nodeRenderer={({
              element,
              isBranch,
              isExpanded,
              getNodeProps,
              level,
              handleSelect,
            }) => (
              <div {...getNodeProps()} style={{ paddingLeft: 20 * (level - 1) }}>
                <FaFileImage color={imageFileColor} className="icon" />
                {element.name}
              </div>
            )}
            onNodeSelect={({ element, isBranch, isSelected, treeState }) => {
              // if (!isBranch) {
              //   let filename = element.name;
              //   let path = element.id;
              //   console.log(filename, path);
              //   const extension = filename.slice(filename.lastIndexOf(".") + 1);
              //   console.log(extension);
              //   if (extension === "bmp") {
              //     if (path in openedFiles) {
              //       tabsModel.doAction(Actions.selectTab(openedFiles[path]));
              //       return;
              //     }
              //     invoke('open_image', { path }).then((imgBase64) => {
              //       let node = layoutRef.current.addTabToActiveTabSet({
              //         component: "img",
              //         name: filename,
              //         config: {
              //           data: imgBase64,
              //         },
              //       })
              //       let id = node.getId()
              //       openedFiles[path] = id
              //       setOpenedFiles(openedFiles)

              //     }).catch((error) => {
              //       console.log(`Error loading image "${path}", ${error}`);
              //     })
              //   }
              // }
            }}
          />
        ) : (<div className='empty-tree'>
          <p>No history</p>
          <p>Open an image first</p>
        </div>)}
      </div>
    </div>
  );
}

export default EditHistory