import React, { useContext, useState, useEffect } from "react";
import { emit, listen } from "@tauri-apps/api/event";
import { save, message } from "@tauri-apps/api/dialog";

import { useDisclosure } from "@mantine/hooks";
import { Actions, Layout, TabNode } from "flexlayout-react";
import "flexlayout-react/style/light.css";
import { Stack, Center } from "@mantine/core";

import FileExplorer from "./FileExplorer";
import EditHistory from "./EditHistory";
import { FileInfoContext } from "./contexts/FileInfoContext";
import { TabsLayoutContext } from "./contexts/TabsLayoutContext";
import TransformDialog from "./TransformDialog";

import { showPopup } from "./PopupMenu";

import "./style/Previewer.css";
import { writeBinaryFile } from "@tauri-apps/api/fs";
import { invoke } from "@tauri-apps/api";

function Previewer() {
  const { currentFileState } = useContext(FileInfoContext);
  const [currentFile, setCurrentFile] = currentFileState;
  const { tabsModel, tabsLayoutRef } = useContext(TabsLayoutContext);

  const [
    transformDialogOpened,
    { open: openTransformDialog, close: closeTransformDialog },
  ] = useDisclosure(false);
  const [transform, setTransform] = useState(null);
  const [selectedTabNode, setSelectedTabNode] = useState(null);

  const [showingPopupMenu, setShowingPopupMenu] = useState(false);

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

  const ImageWithSize = ({ node }) => {
    const [dimensions, setDimensions] = useState({ width: 0, height: 0 });
    const config = node.getConfig();

    useEffect(() => {
      (async () => {
        const dims = await getImageDimensions(config.data);
        setDimensions(dims);
      })();
    }, []);

    return (
      <div
        className="image-container"
        onContextMenu={(event) => {
          if (showingPopupMenu) {
            return;
          }
          event.preventDefault();
          event.stopPropagation();
          console.log(node, event);
          if (node instanceof TabNode) {
            showPopup(
              "图片: " + node.getName(),
              tabsLayoutRef.current.getRootDiv(),
              event.clientX,
              event.clientY,
              ["保存到..."],
              async (item) => {
                if (item === "保存到...") {
                  let path = await save({
                    defaultPath: node.getName(),
                    filters: [
                      {
                        name: "Image",
                        extensions: ["bmp", "jpg", "jpeg", "png"],
                      },
                    ],
                  }).catch((error) => {
                    console.log("save error: ", error);
                  });
                  await invoke("save_image", {
                    path,
                    img: config.data,
                  });
                  await message(
                    `图片"${node.getName()}"已成功保存至"${path}"`,
                    {
                      title: "已保存",
                    }
                  );
                }
                setShowingPopupMenu(false);
              }
            );
            setShowingPopupMenu(true);
          }
        }}
      >
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
        return <ImageWithSize node={node} />;
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
        onContextMenu={(node, event) => {
          if (!showingPopupMenu) {
            event.preventDefault();
            event.stopPropagation();
            console.log(node, event);
            if (node instanceof TabNode) {
              showPopup(
                "图片: " + node.getName(),
                tabsLayoutRef.current.getRootDiv(),
                event.clientX,
                event.clientY,
                ["保存到..."],
                async (item) => {
                  if (item === "保存到...") {
                    let path = await save({
                      defaultPath: node.getName(),
                      filters: [
                        {
                          name: "Image",
                          extensions: ["bmp", "jpg", "jpeg", "png"],
                        },
                      ],
                    }).catch((error) => {
                      console.log("save error: ", error);
                    });
                    await invoke("save_image", {
                      path,
                      img: node.getConfig().data,
                    });
                    await message(
                      `图片"${node.getName()}"已成功保存至"${path}"`,
                      {
                        title: "已保存",
                      }
                    );
                  }
                  setShowingPopupMenu(false);
                }
              );
              setShowingPopupMenu(true);
            }
          }
        }}
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
