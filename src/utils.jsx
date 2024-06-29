function searchTreeNodeById(tree, id) {
  if (tree.id === id) {
    return tree;
  }

  if (tree.children && Array.isArray(tree.children)) {
    for (let child of tree.children) {
      let result = searchTreeNodeById(child, id);
      if (result) {
        return result;
      }
    }
  }

  console.log("mismatch", tree, id);
  return null;
}
async function searchTreeNodeByIdAsync(tree, id) {
  if (tree.id === id) {
    return tree;
  }

  if (tree.children && Array.isArray(tree.children)) {
    for (let child of tree.children) {
      let result = await searchTreeNodeByIdAsync(child, id);
      if (result !== null && result !== undefined) {
        return result;
      }
    }
  }

  return null;
}

function searchTreeNodeInTreesById(trees, id) {
  for (let tree of Object.values(trees)) {
    let result = searchTreeNodeById(tree, id);
    if (result) {
      return result;
    }
  }

  return null;
}

const extractHistories = (trees) => {
  const traverse = (tree) => {
    let histories = [];
    if (tree.id !== undefined) {
      histories.push({
        label: tree.name,
        value: tree.id,
      });
    }
    tree.children.forEach((child) => {
      histories = histories.concat(traverse(child));
    });
    return histories;
  };

  let histories = [
    {
      label: "...",
      value: "...",
    },
  ];
  Object.values(trees).forEach((tree) => {
    histories = histories.concat(traverse(tree));
  });
  console.log(histories);
  return histories;
};

export {
  searchTreeNodeById,
  searchTreeNodeByIdAsync,
  extractHistories,
  searchTreeNodeInTreesById,
};
