import { useState, useContext, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import {
  TextInput,
  Container,
  Badge,
  Button,
  Loader,
  NativeSelect,
  Image,
  Group,
  Stack,
  Grid,
  Modal,
  Center,
} from "@mantine/core";
import { Actions, Layout } from "flexlayout-react";

import {
  IconArrowBigRight,
  IconAlertCircleFilled,
  IconAlertTriangleFilled,
  IconInfoCircleFilled,
} from "@tabler/icons-react";

import { FileInfoContext } from "./contexts/FileInfoContext";
import { TabsLayoutContext } from "./contexts/TabsLayoutContext";
import {
  searchTreeNodeByIdAsync,
  searchTreeNodeInTreesById,
  extractHistories,
} from "./utils";

const transformNameMap = {
  "color/to_gray": "转灰度",
  "color/invert": "反色",
  "color/to_binary": "二值化",
  "color/exponential": "指数变换",
  "color/hist_equalize": "直方图均衡化",
  "geometric/rotate": "旋转",
  "geometric/resize": "缩放",
  "geometric/translate": "平移",
  "geometric/mirror": "镜像",
  "geometric/stretch": "拉伸",
  "binary_op/add": "加法",
  "binary_op/sub": "减法",
  "binary_op/mul": "乘法",
  "binary_op/div": "除法",
};

const argHintMap = {
  "color/to_gray": "无需参数",
  "color/invert": "无需参数",
  "color/to_binary": "输入阈值 (格式: x) (x取值范围为归一化后的0-1)",
  "color/exponential": "输入指数 (格式: x) (x为浮点数)",
  "color/hist_equalize": "无需参数",
  "geometric/rotate": "输入角度 (格式: x) (单位：角度，x=360N时为原图)",
  "geometric/resize": "输入缩放后尺寸 (格式: x,y) (单位：像素)",
  "geometric/translate": "输入平移距离 (格式: x,y) (单位：像素，x=y=0时为原图)",
  "geometric/mirror": "输入镜像轴 (格式: a) (其中a可选值为x或y)",
  "geometric/stretch": "输入拉伸比例 (格式: x,y) (比例，x=y=1时为原图)",
  "binary_op/add": "无需参数",
  "binary_op/sub": "无需参数",
  "binary_op/mul": "无需参数",
  "binary_op/div": "无需参数",
};

function TransformDialog({
  transform,
  selectedTabNode,
  setSelectedTabNode,
  opened: dialogOpened,
  onClose: onDialogCloseCallback,
}) {
  const { openedFilesState, historyTreesState, currentFileState } =
    useContext(FileInfoContext);
  const { tabsModel, tabsLayoutRef } = useContext(TabsLayoutContext);
  const [openedFiles, setOpenedFiles] = openedFilesState;
  const [currentFile, setCurrentFile] = currentFileState;
  const [historyTrees, setHistoryTrees] = historyTreesState;
  const [layoutRef, setLayoutRef] = useState();

  const [selectedImg2Id, setSelectedImg2Id] = useState(null);
  const [selectedImg2Data, setSelectedImg2Data] = useState(null);

  const [transformedImg, setTransformedImg] = useState(null);
  const [transformArg, setTransformArg] = useState("");
  const [transformedName, setTransformedName] = useState("");
  const [inProgress, setInProgress] = useState(false);
  const [historyList, setHistoryList] = useState([
    {
      label: "...",
      value: "...",
    },
  ]);

  useEffect(() => {
    if (selectedTabNode && dialogOpened) {
      setHistoryList(extractHistories(historyTrees));
      setTransformedName(
        `${selectedTabNode.getName()}-${transformNameMap[transform]}`
      );
    }
  }, [selectedTabNode, dialogOpened]);

  useEffect(() => {
    setLayoutRef(tabsLayoutRef);
  }, [tabsLayoutRef]);

  const doTransform = async () => {
    let img = selectedTabNode.getConfig().data;
    let img2 = selectedImg2Data;
    invoke("transform_image", {
      img,
      img2: img2 || "",
      transform,
      transformArg,
    })
      .then((imgBase64) => {
        if (imgBase64) {
          setTransformedImg(imgBase64);
        }
      })
      .catch((e) => {
        console.error(e);
      })
      .finally(() => {
        setInProgress(false);
      });
  };

  useEffect(() => {
    if (inProgress) {
      doTransform();
    }
  }, [inProgress]);

  const apply = () => {
    setInProgress(true);
  };

  const confirm = async () => {
    let nodeConfig = selectedTabNode.getConfig();
    console.log("confirm");
    let targetTabset =
      tabsModel.getActiveTabset() || tabsModel.getFirstTabset();
    let newId = layoutRef.current
      .addTabToTabSet(targetTabset.getId(), {
        component: "img",
        name: transformedName,
        config: {
          ...nodeConfig,
          data: transformedImg,
        },
      })
      .getId();

    // openedFiles[nodeConfig.path] = newId;
    // setOpenedFiles(openedFiles);

    console.log(historyTrees[nodeConfig.path], selectedTabNode.getId());
    let parentTreeNode = await searchTreeNodeByIdAsync(
      historyTrees[nodeConfig.path],
      selectedTabNode.getId()
    );
    parentTreeNode.children.push({
      name: transformedName,
      id: newId,
      children: [],
      metadata: { img: transformedImg, path: nodeConfig.path },
    });
    // setHistoryTrees(historyTrees);
    setCurrentFile({
      path: nodeConfig.path,
      tabId: newId,
    });

    close();
  };

  const close = async () => {
    setTransformedImg(null);
    setTransformedName("");
    setTransformArg("");
    onDialogCloseCallback();
  };

  return (
    <Modal
      opened={dialogOpened}
      onClose={close}
      title="图像变换"
      size="auto"
      centered
    >
      <Stack align="stretch">
        <Group w={700} justify="space-between" wrap="nowrap" align="center">
          <Center h={216} w={384} bg={"#eeeeee"}>
            {selectedTabNode ? (
              <Image
                mah={"95%"}
                maw={"95%"}
                fit="contain"
                src={
                  selectedTabNode
                    ? `data:image/bmp;base64,${
                        selectedTabNode.getConfig().data
                      }`
                    : null
                }
              />
            ) : (
              <Stack align="center" gap={"xs"}>
                <IconAlertTriangleFilled size={100} color="#aaaaaa" />
                <Badge size="xl" color="#aaaaaa">
                  无图像，请先选择图像
                </Badge>
              </Stack>
            )}
          </Center>
          {transform && transform.startsWith("binary_op") && (
            <Center h={216} w={384} bg={"#eeeeee"}>
              {selectedImg2Data ? (
                <Image
                  mah={"95%"}
                  maw={"95%"}
                  fit="contain"
                  src={`data:image/bmp;base64,${selectedImg2Data}`}
                />
              ) : (
                <Stack align="center" gap={"xs"}>
                  <IconAlertTriangleFilled size={100} color="#aaaaaa" />
                  <Badge size="xl" color="#aaaaaa">
                    无图像，请先选择另一图像
                  </Badge>
                </Stack>
              )}
            </Center>
          )}
          <Center>
            <IconArrowBigRight size={60} stroke={0.75} />
          </Center>
          <Center h={216} w={384} bg={"#eeeeee"}>
            {inProgress ? (
              <Loader color="gray" />
            ) : transformedImg ? (
              <Image
                mah={"95%"}
                maw={"95%"}
                fit="contain"
                // fallbackSrc={placeHolderImage}
                src={`data:image/bmp;base64,${transformedImg}`}
              />
            ) : (
              <Stack align="center" gap={"xs"}>
                <IconInfoCircleFilled size={100} color="#aaaaaa" />
                <Badge size="xl" color="#aaaaaa">
                  请应用后查看
                </Badge>
              </Stack>
            )}
          </Center>
        </Group>
        <Group w="100%" justify="center" align="center">
          <TextInput
            label="变换参数"
            w="100%"
            value={transformArg}
            placeholder={argHintMap[transform]}
            disabled={argHintMap[transform] === "无需参数"}
            onChange={(e) => {
              setTransformArg(e.target.value);
            }}
          />
          <NativeSelect
            label="输入图像"
            w="100%"
            onChange={(e) => {
              setTransformedImg(null);
              console.log(e.currentTarget.value);
              let id = e.currentTarget.value;
              if (id === "...") {
                setSelectedTabNode(null);
              } else {
                tabsModel.doAction(Actions.selectTab(id));
                setSelectedTabNode(
                  tabsModel.getActiveTabset().getSelectedNode()
                );
              }
            }}
            value={selectedTabNode ? selectedTabNode.getId() : "..."}
            data={historyList}
          />
          {transform && transform.startsWith("binary_op") && (
            <NativeSelect
              label="输入另一图像"
              w="100%"
              onChange={(e) => {
                setTransformedImg(null);
                console.log(e.currentTarget.value);
                let id = e.currentTarget.value;
                if (id === "...") {
                  setSelectedImg2Id(null);
                  setSelectedImg2Data(null);
                } else {
                  console.log(id);
                  console.log(Object.values(historyTrees));
                  let treeNode = searchTreeNodeInTreesById(historyTrees, id);
                  setSelectedImg2Id(id);
                  setSelectedImg2Data(treeNode.metadata.img);
                }
              }}
              value={selectedImg2Id || "..."}
              data={historyList}
            />
          )}
          <TextInput
            label="变换后名称"
            w="100%"
            value={transformedName}
            onChange={(e) => {
              setTransformedName(e.target.value);
            }}
          />
        </Group>

        <Grid justify="right" align="stretch">
          <Grid.Col span={2}>
            <Button
              fullWidth
              size="sm"
              onClick={selectedTabNode ? apply : null}
            >
              应用
            </Button>
          </Grid.Col>
          <Grid.Col span={2}>
            <Button
              fullWidth
              size="sm"
              disabled={
                inProgress ||
                transformedImg === null ||
                (transform &&
                  transform.startsWith("binary_op") &&
                  selectedImg2Id === null)
              }
              onClick={confirm}
            >
              确定
            </Button>
          </Grid.Col>
          <Grid.Col span={2}>
            <Button fullWidth size="sm" onClick={close}>
              取消
            </Button>
          </Grid.Col>
        </Grid>
      </Stack>
    </Modal>
  );
}

export default TransformDialog;
