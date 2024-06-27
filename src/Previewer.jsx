import React, { useState } from 'react';
import { Layout, Model, TabNode } from 'flexlayout-react';

import FileExplorer from './FileExplorer';
import History from './History';

import folderIcon from './assets/folder.svg';
import historyIcon from './assets/history.svg';

import 'flexlayout-react/style/light.css';

const json = {
  global: {
    tabEnableFloat: false,
    tabEnableClose: true,
    tabEnableDrag: true,
    tabEnableRename: false,
    borderEnableDrop: false,
  },
  borders: [
    {
      type: "border",
      selected: 0,
      location: "left",
      children: [
        {
          type: "tab",
          name: "Explorer",
          altName: "The Explorer Tab",
          component: "file_explorer",
          enableClose: false,
          icon: folderIcon,
        }
      ]
    },
    {
      type: "border",
      selected: 0,
      location: "right",
      children: [
        {
          type: "tab",
          name: "History",
          altName: "The History Tab",
          component: "history",
          enableClose: false,
          icon: historyIcon,
        }
      ]
    },
  ],
  layout: {
    type: "row",
    weight: 100,
    children: [
      {
        type: "tabset",
        weight: 50,
        children: [
          {
            type: "tab",
            name: "Tab 1",
            component: "text",
          },
        ],
      },
      {
        type: "tabset",
        weight: 50,
        children: [
          {
            type: "tab",
            name: "Tab 2",
            component: "text",
          },
          {
            type: "tab",
            name: "Tab 2",
            component: "text",
          },
          {
            type: "tab",
            name: "Tab 2",
            component: "text",
          },
          {
            type: "tab",
            name: "Tab 2",
            component: "text",
          },
        ],
      },
    ],
  },
};

function Previewer() {
  const [model] = useState(Model.fromJson(json));

  const factory = (node) => {
    const component = node.getComponent();

    switch (component) {
      case "text":
        return <div>{node.getName()}</div>;
      case "file_explorer":
        return (
          <div>
            <FileExplorer />
          </div>
        );
      case "history":
        return (
          <div>
            <History />
          </div>
        );
      default:
        return <div>Unknown component {component}</div>;
    }
  };

  return (
    <div className="previewer">
      <Layout model={model} factory={factory} />
    </div>
  );
}

export default Previewer