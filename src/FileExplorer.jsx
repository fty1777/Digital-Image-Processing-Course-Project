import React, { useEffect, useContext, useState } from 'react';
import { open } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api/tauri';

import { Actions } from 'flexlayout-react';
import TreeView, { flattenTree } from "react-accessible-treeview";
import { FaFolderOpen, FaFolderClosed, FaFileImage, FaFile } from "react-icons/fa6";
import { Button } from '@mantine/core';

import { FileInfoContext } from './contexts/FileInfoContext';
import { TabsLayoutContext } from './contexts/TabsLayoutContext';

import './style/Treeview.css';


const folderColor = "black";
const imageFileColor = "red";
const otherFileColor = "grey"

const FolderIcon = ({ isOpen }) =>
  isOpen ? (
    <FaFolderOpen color={folderColor} className="icon" />
  ) : (
    <FaFolderClosed color={folderColor} className="icon" />
  );

const FileIcon = ({ filename }) => {
  const extension = filename.slice(filename.lastIndexOf(".") + 1);
  switch (extension) {
    case "bmp":
    case "jpg":
    case "jpeg":
    case "raw":
      return <FaFileImage color={imageFileColor} className="icon" />
    default:
      return <FaFile color={otherFileColor} className="icon" />;
  }
};

function sortTree(node) {
  if (node.children && Array.isArray(node.children)) {
    // Sort the children
    node.children.sort((a, b) => {
      // If both have children or both don't have children, sort by name
      if ((a.children && b.children) || (!a.children && !b.children)) {
        return a.name.localeCompare(b.name);
      }
      // Place nodes with children at the beginning
      return (b.children ? 1 : 0) - (a.children ? 1 : 0);
    });
    
    // Recursively sort the children of each child node
    node.children.forEach(sortTree);
  }
}

function FileExplorer() {

  const {
    openedFolderState,
    folderTreeState,
    openedFilesState,
    historyTreesState,
    currentFileState,
  } = useContext(FileInfoContext);
  const { tabsModel, tabsLayoutRef } = useContext(TabsLayoutContext);

  const [openedFolder, setFolder] = openedFolderState;
  const [folderTree, setFolderTree] = folderTreeState;
  const [openedFiles, setOpenedFiles] = openedFilesState;
  const [historyTrees, setHistoryTrees] = historyTreesState;
  const [currentFile, setCurrentFile] = currentFileState;
  const [layoutRef, setLayoutRef] = useState();

  useEffect(() => {
    setLayoutRef(tabsLayoutRef);
  }, [tabsLayoutRef]);

  const fetchTreeData = async () => {
    if (openedFolder === null) {
      setFolderTree(flattenTree({
        name: "",
        children: []
      }));
      return;
    }
    invoke('read_folder', { path: openedFolder }).then((result) => {
      result = {
        name: "",
        children: result
      }
      sortTree(result)
      setFolderTree(flattenTree(result));
      console.log('Directory tree fetched:', result);
    }).catch((error) => {
      console.error('Failed to fetch directory tree:', error);
    })
  };

  useEffect(() => {
    fetchTreeData();
  }, [openedFolder]);

  return (
    <div className="ide-container">
      <div className='open-folder-button-container'>
        <Button
          fullWidth
          variant="filled"
          color='#909090'
          size="compact-sm"
          radius="xs"
          loading={folderTree === null}
          onClick={async () => {
            open({
              directory: true,
              multiple: false,
            }).then((folderPath) => {
              console.log(folderPath)
              if (folderPath !== null) {
                setFolder(folderPath);
              }
            }).catch((error) => {
              console.log('Error opening folder:', error);
            })
          }} >
          Open Folder...
        </Button>
      </div>
      {folderTree !== null && (
        <div className="ide">
          <TreeView
            data={folderTree}
            aria-label="folder tree"
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
                {isBranch ? (
                  <FolderIcon isOpen={isExpanded} />
                ) : (
                  <FileIcon filename={element.name} />
                )}
                {element.name}
              </div>
            )}
            onNodeSelect={({ element, isBranch, isSelected, treeState }) => {
              if (!isBranch) {
                let filename = element.name;
                let path = element.id;
                console.log(filename, path);
                const extension = filename.slice(filename.lastIndexOf(".") + 1);
                console.log(extension);
                if (extension === "bmp") {
                  if (path in openedFiles) {
                    tabsModel.doAction(Actions.selectTab(openedFiles[path]));
                    return;
                  }
                  invoke('open_image', { path }).then((imgBase64) => {
                    let targetTabset = tabsModel.getActiveTabset() || tabsModel.getFirstTabSet();
                    let node = layoutRef.current.addTabToTabSet(targetTabset.getId(),
                      {
                        component: "img",
                        name: filename,
                        config: {
                          data: imgBase64,
                          path,
                        },
                      })
                    let id = node.getId();
                    openedFiles[path] = id;
                    setOpenedFiles(openedFiles);
                    historyTrees[path] = {
                      name: '',
                      children: [{
                        name: `(root)${filename}`,
                        children: [],
                        metadata: { imgBase64 },
                      }],
                    };
                    setHistoryTrees(historyTrees);
                  }).catch((error) => {
                    console.log(`Error loading image "${path}", ${error}`);
                  })
                }
              }
            }}
          />
        </div>
      )}
    </div>
  )
}

export default FileExplorer