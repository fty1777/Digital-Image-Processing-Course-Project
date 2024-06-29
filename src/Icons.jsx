import {
  FaFolderOpen,
  FaFolderClosed,
  FaFileImage,
  FaFile,
} from "react-icons/fa6";

const folderColor = "black";
const imageFileColor = "red";
const otherFileColor = "grey";

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
      return <FaFileImage color={imageFileColor} className="icon" />;
    default:
      return <FaFile color={otherFileColor} className="icon" />;
  }
};

export { FolderIcon, FileIcon };
