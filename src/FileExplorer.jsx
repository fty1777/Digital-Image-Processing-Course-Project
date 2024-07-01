import React, { useEffect, useContext, useState } from "react";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";

import { Actions } from "flexlayout-react";
import TreeView, { flattenTree } from "react-accessible-treeview";
import { Flex, Button, ActionIcon, Space } from "@mantine/core";
import { IconReload } from "@tabler/icons-react";

import { FolderIcon, FileIcon } from "./Icons";

import { FileInfoContext } from "./contexts/FileInfoContext";
import { TabsLayoutContext } from "./contexts/TabsLayoutContext";

import "./style/Treeview.css";

async function sortTree(node) {
  if (node.children && Array.isArray(node.children)) {
    node.children.sort((a, b) => {
      if ((a.children && b.children) || (!a.children && !b.children)) {
        return a.name.localeCompare(b.name);
      }
      return (b.children ? 1 : 0) - (a.children ? 1 : 0);
    });

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

  const [openedFolder, setOpenedFolder] = openedFolderState;
  const [folderTree, setFolderTree] = folderTreeState;
  const [openedFiles, setOpenedFiles] = openedFilesState;
  const [historyTrees, setHistoryTrees] = historyTreesState;
  const [currentFile, setCurrentFile] = currentFileState;
  const [layoutRef, setLayoutRef] = useState();

  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    setLayoutRef(tabsLayoutRef);
  }, [tabsLayoutRef]);

  useEffect(() => {
    if (isLoading) {
      setIsLoading(false);
    }
  }, [folderTree]);

  const fetchTreeData = async () => {
    setIsLoading(true);
    if (openedFolder === null) {
      setFolderTree(flattenTree({ name: "", children: [] }));
      return;
    }
    invoke("read_folder", { path: openedFolder })
      .then((result) => {
        result = { name: "", children: result };
        sortTree(result).then(() => {
          setFolderTree(flattenTree(result));
        });
      })
      .catch((error) => {
        console.error("Failed to fetch directory tree:", error);
      });
  };

  useEffect(() => {
    fetchTreeData();
  }, [openedFolder]);

  return (
    <div
      className="ide-container"
      onContextMenu={(e) => {
        e.preventDefault();
      }}
    >
      <Flex gap="5">
        <Button
          h={30}
          dir="rtl"
          fullWidth
          variant="filled"
          color="#909090"
          radius="xs"
          loading={isLoading}
          onClick={async () => {
            open({
              directory: true,
              multiple: false,
            })
              .then((folderPath) => {
                if (folderPath !== null) {
                  setOpenedFolder(folderPath);
                }
              })
              .catch((error) => {
                console.log("Error opening folder:", error);
              });
          }}
        >
          {openedFolder || "打开文件夹"}
        </Button>
        <ActionIcon
          flex="none"
          h={30}
          w={30}
          variant="filled"
          radius="xs"
          color="#909090"
          className="reload-button"
        >
          <IconReload
            width={"60%"}
            height={"60%"}
            stroke={2.5}
            onClick={fetchTreeData}
          />
        </ActionIcon>
      </Flex>
      <Space h="5" />
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
              <div
                {...getNodeProps()}
                style={{ paddingLeft: 20 * (level - 1) }}
              >
                {isBranch ? (
                  <FolderIcon isOpen={isExpanded} />
                ) : (
                  <FileIcon filename={element.name} />
                )}
                {element.name}
              </div>
            )}
            onNodeSelect={({ element, isBranch, isSelected, treeState }) => {
              if (isBranch) {
                return;
              }
              let filename = element.name;
              let path = element.id;
              const extension = filename.slice(filename.lastIndexOf(".") + 1);
              if (
                extension === "bmp" ||
                extension === "jpg" ||
                extension === "jpeg" ||
                extension === "png"
              ) {
                if (path in historyTrees) {
                  let treeNode = historyTrees[path].children[0];
                  let id = treeNode.id;
                  tabsModel.doAction(Actions.selectTab(id));
                  let selectedNode = tabsModel
                    .getActiveTabset()
                    ?.getSelectedNode();
                  if (selectedNode.getId() === id) {
                    setCurrentFile({
                      path,
                      tabId: id,
                    });
                  } else {
                    let targetTabset =
                      tabsModel.getActiveTabset() || tabsModel.getFirstTabSet();
                    let newId = layoutRef.current
                      .addTabToTabSet(targetTabset.getId(), {
                        component: "img",
                        name: treeNode.name,
                        config: {
                          data: treeNode.metadata.img,
                          path: treeNode.metadata.path,
                        },
                      })
                      .getId();
                    treeNode.id = newId;
                    tabsModel.doAction(Actions.selectTab(openedFiles[path]));
                    setCurrentFile({ path, tabId: openedFiles[path] });
                  }
                } else {
                  invoke("open_image", { path })
                    .then((imgBase64) => {
                      let targetTabset =
                        tabsModel.getActiveTabset() ||
                        tabsModel.getFirstTabSet();
                      let node = layoutRef.current.addTabToTabSet(
                        targetTabset.getId(),
                        {
                          component: "img",
                          name: filename,
                          config: {
                            data: imgBase64,
                            filename,
                            path,
                          },
                        }
                      );
                      let id = node.getId();
                      openedFiles[path] = id;
                      setOpenedFiles(openedFiles);
                      historyTrees[path] = {
                        name: "",
                        children: [
                          {
                            name: `${filename}`,
                            id: id,
                            children: [],
                            metadata: { img: imgBase64, path },
                          },
                        ],
                      };
                    })
                    .catch((error) => {
                      console.log(`Error loading image "${path}", ${error}`);
                    });
                }
              }
            }}
          />
        </div>
      )}
    </div>
  );
}

export default FileExplorer;
