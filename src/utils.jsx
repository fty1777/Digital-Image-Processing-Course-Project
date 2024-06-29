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

export { searchTreeNodeById, searchTreeNodeByIdAsync };
