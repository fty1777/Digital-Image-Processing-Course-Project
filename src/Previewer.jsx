import React, { useContext } from 'react';

import { Actions, Layout } from 'flexlayout-react';
import 'flexlayout-react/style/light.css';

import FileExplorer from './FileExplorer';
import EditHistory from './EditHistory';
import { FileInfoContext } from './contexts/FileInfoContext';
import { TabsLayoutContext } from './contexts/TabsLayoutContext';

import './style/Previewer.css';

function Previewer() {
  const {
    openedFolderState,
    folderTreeState,
    openedFilesState,
    historyTreesState,
    currentFileState,
  } = useContext(FileInfoContext);
  const [openedFiles, setOpenedFiles] = openedFilesState;
  const [currentFile, setCurrentFile] = currentFileState;
  const [historyTrees, setHistoryTrees] = historyTreesState;
  const { tabsModel, tabsLayoutRef } = useContext(TabsLayoutContext);

  const factory = (node) => {
    const component = node.getComponent();

    switch (component) {
      case "text":
        return (
          <div> {node.getName()} </div>
        );
      case "img":
        let config = node.getConfig();
        node.setEventListener("close", (p) => {
          delete openedFiles[config.path];
          setOpenedFiles(openedFiles);
          delete historyTrees[config.path];
          setHistoryTrees(historyTrees);

          let activeTabset = tabsModel.getActiveTabset()
          if (activeTabset.getSelectedNode() === undefined) {
            activeTabset = tabsModel.getFirstTabSet()
          }
          let selectedNode = activeTabset.getSelectedNode()
          if (selectedNode === undefined || selectedNode.getComponent() !== "img") {
            console.log("No active tab")
            setCurrentFile(null)
          } else {
            tabsModel.doAction(Actions.selectTab(selectedNode.getId()))
            setCurrentFile(selectedNode.getConfig().path)
          }
        });
        return (
          <div className='image-container'>
            <img src={`data:image/bmp;base64,${config.data}`} alt={node.getName()} />
          </div>
        );
      case "file_explorer":
        return (
          <FileExplorer />
        );
      case "edit_history":
        return (
          <EditHistory />
        );
      default:
        return <div>Unknown component {component}</div>;
    }
  };

  return (
    <div className="previewer">
      <Layout
        ref={tabsLayoutRef}
        model={tabsModel}
        factory={factory}
        realtimeResize={false}
        onAction={(action) => {
          // This interception is before the actual action is to be applied.
          if (action.type === "FlexLayout_AddNode") {
            if (action.data.json.component === "img") {
              setCurrentFile(action.data.json.config.path)
            }
          } else if (action.type === "FlexLayout_SelectTab") {
            let node = tabsModel.getNodeById(action.data.tabNode)
            if (node.getComponent() === "img") {
              setCurrentFile(node.getConfig().path)
            }
          } else if (action.type === "FlexLayout_SetActiveTabset") {
            let node = tabsModel.getNodeById(action.data.tabsetNode)
            let activeTab = node.getSelectedNode()
            console.log(node)
            console.log(activeTab)
            if (activeTab.getComponent() === "img") {
              setCurrentFile(activeTab.getConfig().path)
              console.log("Active Tab:", activeTab.getConfig().path)
            }
          }
          console.log("Action:", action)
          return action;
        }}
      />
    </div>
  );
}

export default Previewer