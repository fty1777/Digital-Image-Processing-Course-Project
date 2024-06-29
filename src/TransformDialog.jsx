import { useState, useContext, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import {
  TextInput,
  Container,
  Badge,
  Button,
  Loader,
  Image,
  Group,
  Stack,
  Grid,
  Modal,
  Center,
} from "@mantine/core";

import {
  IconArrowBigRight,
  IconAlertCircleFilled,
  IconAlertTriangleFilled,
  IconInfoCircleFilled,
} from "@tabler/icons-react";

import { FileInfoContext } from "./contexts/FileInfoContext";
import { TabsLayoutContext } from "./contexts/TabsLayoutContext";

async function searchTreeNode(tree, id) {
  if (tree.id === id) {
    return tree;
  }

  if (tree.children && Array.isArray(tree.children)) {
    for (let child of tree.children) {
      let result = await searchTreeNode(child, id);
      if (result !== null && result !== undefined) {
        return result;
      }
    }
  }

  // If no matching node is found, return null
  return null;
}

function TransformDialog({
  transform,
  selectedTabNode,
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

  const [transformedImg, setTransformedImg] = useState(null);
  const [transformArg, setTransformArg] = useState("");
  const [inProgress, setInProgress] = useState(false);

  useEffect(() => {
    setLayoutRef(tabsLayoutRef);
  }, [tabsLayoutRef]);

  const doTransform = async () => {
    let img = selectedTabNode.getConfig().data;
    invoke("transform_image", { img, transform, transformArg })
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
        name: transform,
        config: {
          ...nodeConfig,
          data: transformedImg,
        },
      })
      .getId();

    openedFiles[nodeConfig.path] = newId;
    // setOpenedFiles(openedFiles);

    console.log(historyTrees[nodeConfig.path], selectedTabNode.getId());
    let parentTreeNode = await searchTreeNode(
      historyTrees[nodeConfig.path],
      selectedTabNode.getId()
    );
    parentTreeNode.children.push({
      name: transform,
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
                  无图像，请先打开并选择图像
                </Badge>
              </Stack>
            )}
          </Center>
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
                src={
                  transformedImg
                    ? `data:image/bmp;base64,${transformedImg}`
                    : null
                }
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
            onChange={(e) => {
              setTransformArg(e.target.value);
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
              disabled={!inProgress && transformedImg === null}
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
