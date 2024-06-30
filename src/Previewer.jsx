import React, { useContext, useState, useEffect } from "react";
import { emit, listen } from "@tauri-apps/api/event";

import { useDisclosure } from "@mantine/hooks";
import { Actions, Layout } from "flexlayout-react";
import "flexlayout-react/style/light.css";
import { Stack, Center } from "@mantine/core";

import FileExplorer from "./FileExplorer";
import EditHistory from "./EditHistory";
import { FileInfoContext } from "./contexts/FileInfoContext";
import { TabsLayoutContext } from "./contexts/TabsLayoutContext";
import TransformDialog from "./TransformDialog";

import "./style/Previewer.css";

function Previewer() {
  const {
    currentFileState,
  } = useContext(FileInfoContext);
  const [currentFile, setCurrentFile] = currentFileState;
  const { tabsModel, tabsLayoutRef } = useContext(TabsLayoutContext);

  const [
    transformDialogOpened,
    { open: openTransformDialog, close: closeTransformDialog },
  ] = useDisclosure(false);
  const [transform, setTransform] = useState(null);
  const [selectedTabNode, setSelectedTabNode] = useState(null);

  listen("menu_event", (event) => {
    handleMenuEvent(event.payload.menu_item);
  });

  function handleMenuEvent(menu_item) {
    openTransformDialog();
    setSelectedTabNode(tabsModel.getActiveTabset().getSelectedNode());
    setTransform(menu_item);
  }

  const onDialogClose = () => {
    closeTransformDialog();
  };

  const getImageDimensions = (base64) => {
    return new Promise((resolve) => {
      const img = new Image();
      img.onload = () => {
        resolve({ width: img.width, height: img.height });
      };
      img.src = `data:image/bmp;base64,${base64}`;
    });
  };

  const ImageWithSize = ({ config }) => {
    const [dimensions, setDimensions] = useState({ width: 0, height: 0 });

    useEffect(() => {
      (async () => {
        const dims = await getImageDimensions(config.data);
        setDimensions(dims);
      })();
    }, [config]);

    return (
      <div className="image-container">
        <img src={`data:image/bmp;base64,${config.data}`} alt={config.name} />
        <p className="image-size">
          {dimensions.width} x {dimensions.height}
        </p>
      </div>
    );
  };

  const factory = (node) => {
    const component = node.getComponent();

    switch (component) {
      case "text":
        return <div> {node.getName()} </div>;
      case "img":
        let config = node.getConfig();
        node.setEventListener("close", (p) => {
          let activeTabset = tabsModel.getActiveTabset();
          if (activeTabset.getSelectedNode() === undefined) {
            activeTabset = tabsModel.getFirstTabSet();
          }
          let selectedNode = activeTabset.getSelectedNode();
          if (
            selectedNode === undefined ||
            selectedNode.getComponent() !== "img"
          ) {
            setCurrentFile(null);
          } else {
            tabsModel.doAction(Actions.selectTab(selectedNode.getId()));
            setCurrentFile({
              path: selectedNode.getConfig().path,
              tabId: selectedNode.getId(),
            });
          }
        });
        return <ImageWithSize config={config} />;
      case "file_explorer":
        return <FileExplorer />;
      case "edit_history":
        return <EditHistory />;
      default:
        return <div>Unknown component {component}</div>;
    }
  };

  return (
    <div className="previewer">
      <TransformDialog
        transform={transform}
        selectedTabNode={selectedTabNode}
        setSelectedTabNode={setSelectedTabNode}
        opened={transformDialogOpened}
        onClose={onDialogClose}
      />

      <Layout
        ref={tabsLayoutRef}
        model={tabsModel}
        factory={factory}
        realtimeResize={false}
        onAction={(action) => {
          // This interception is before the actual action is to be applied.
          if (action.type === "FlexLayout_AddNode") {
            if (action.data.json.component === "img") {
              setCurrentFile({
                path: action.data.json.config.path,
                tabId: action.data.json.id,
              });
            }
          } else if (action.type === "FlexLayout_SelectTab") {
            let node = tabsModel.getNodeById(action.data.tabNode);
            if (node.getComponent() === "img") {
              setCurrentFile({
                path: node.getConfig().path,
                tabId: node.getId(),
              });
            }
          } else if (action.type === "FlexLayout_SetActiveTabset") {
            let node = tabsModel.getNodeById(action.data.tabsetNode);
            let activeTab = node.getSelectedNode();
            if (activeTab.getComponent() === "img") {
              setCurrentFile({
                path: activeTab.getConfig().path,
                tabId: activeTab.getId(),
              });
            }
          }
          return action;
        }}
      />
    </div>
  );
}

export default Previewer;
