import React, { createContext, useState, createRef } from 'react';

import { Layout, Model } from 'flexlayout-react';

import folderIcon from '../assets/folder.svg';
import historyIcon from '../assets/history.svg';


const TabsLayoutContext = createContext();
const tabsLayoutRef = createRef();

function TabsLayoutProvider({ children }) {
  const [tabsModel, setTabsModel] = useState(Model.fromJson(initConf));

  return (
    <TabsLayoutContext.Provider value={{
      tabsModel, setTabsModel, tabsLayoutRef
    }}>
      {children}
    </TabsLayoutContext.Provider>
  );
}

const initConf = {
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
          name: "文件夹",
          altName: "The Explorer Tab",
          component: "file_explorer",
          enableClose: false,
          enableDrag: false,
          icon: folderIcon,
        },
        {
          type: "tab",
          name: "编辑历史",
          altName: "The Edit History Tab",
          component: "edit_history",
          enableClose: false,
          enableDrag: false,
          icon: historyIcon,
        }
      ]
    },
  ],
  layout: {}
};

export { TabsLayoutContext, TabsLayoutProvider };