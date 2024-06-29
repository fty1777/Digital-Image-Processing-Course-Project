import React, { useEffect, useContext, useState } from "react";
import { Actions } from "flexlayout-react";
import TreeView, { flattenTree } from "react-accessible-treeview";

import { FaFileImage } from "react-icons/fa6";

import { FileInfoContext } from "./contexts/FileInfoContext";
import { TabsLayoutContext } from "./contexts/TabsLayoutContext";
import {
  IconSquareMinus,
  IconSquarePlus,
  IconPoint,
  IconSquareMinusFilled,
  IconArrowDownRight,
} from "@tabler/icons-react";

import "./style/Treeview.css";

const iconColor = "black";
// const BranchIcon = ({ isOpen }) => (
//   <IconArrowDownRight color={iconColor} className="icon" />
// );

const BranchIcon = ({ isOpen }) =>
  isOpen ? (
    <IconSquareMinusFilled color={iconColor} className="icon" />
  ) : (
    <IconSquarePlus color={iconColor} className="icon" />
  );

const LeafIcon = () => <IconPoint color={iconColor} className="icon" />;

function searchTreeNode(tree, id) {
  if (tree.id === id) {
    return tree;
  }

  if (tree.children && Array.isArray(tree.children)) {
    for (let child of tree.children) {
      let result = searchTreeNode(child, id);
      if (result) {
        return result;
      }
    }
  }

  // If no matching node is found, return null
  return null;
}

function EditHistory() {
  const {
    openedFolderState,
    folderTreeState,
    openedFilesState,
    historyTreesState,
    currentFileState,
  } = useContext(FileInfoContext);
  const { tabsModel, tabsLayoutRef } = useContext(TabsLayoutContext);
  const [historyTrees, setHistoryTrees] = historyTreesState;
  const [historyTree, setHistoryTree] = useState(null);
  const [currentFile, setCurrentFile] = currentFileState;
  const [layoutRef, setLayoutRef] = useState();

  useEffect(() => {
    setLayoutRef(tabsLayoutRef);
  }, [tabsLayoutRef]);

  const refreshTree = () => {
    if (currentFile && currentFile.path in historyTrees) {
      setHistoryTree(flattenTree(historyTrees[currentFile.path]));
    } else {
      setHistoryTree(null);
    }
  };

  useEffect(() => {
    refreshTree();
  }, [currentFile, historyTrees]);

  return (
    <div className="ide-container">
      <div className="ide">
        {historyTree ? (
          <TreeView
            data={historyTree}
            // expandedIds={historyTree.map(({ id }) => id)}
            aria-label="history tree"
            clickAction="EXCLUSIVE_SELECT"
            nodeRenderer={({
              element,
              isBranch,
              isExpanded,
              getNodeProps,
              level,
              handleSelect,
              handleExpand,
            }) => (
              <div
                {...getNodeProps()}
                style={{ paddingLeft: 20 * (level - 1) }}
              >
                {isBranch ? <BranchIcon isOpen={isExpanded} /> : <LeafIcon />}
                {element.name}
              </div>
            )}
            onNodeSelect={({ element, isBranch, isSelected, treeState }) => {
              let id = element.id;
              tabsModel.doAction(Actions.selectTab(id));
              let selectedNode = tabsModel.getActiveTabset()?.getSelectedNode();
              if (selectedNode.getId() === id) {
                setCurrentFile({
                  path: selectedNode.getConfig().path,
                  tabId: id,
                });
              } else {
                let {
                  name,
                  metadata: { img, path },
                } = element;
                let targetTabset =
                  tabsModel.getActiveTabset() || tabsModel.getFirstTabSet();
                let newId = layoutRef.current
                  .addTabToTabSet(targetTabset.getId(), {
                    component: "img",
                    name,
                    config: {
                      data: img,
                      path,
                    },
                  })
                  .getId();
                let treeNode = searchTreeNode(historyTrees[path], id);
                treeNode.id = newId;
                setCurrentFile({
                  path,
                  tabId: newId,
                });
              }
            }}
          />
        ) : (
          <div className="empty-tree">
            <p>无历史</p>
            <p>请先选择图片</p>
          </div>
        )}
      </div>
    </div>
  );
}

export default EditHistory;
