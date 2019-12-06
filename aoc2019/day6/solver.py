#!/usr/bin/env python3


class Tree:
    def __init__(self, val, subtrees=[], parent=None):
        self.val = val
        self.parent = parent
        self.subtrees = subtrees

    def add_child(self, child):
        child.parent = self
        self.subtrees.append(child)

    def is_leaf(self):
        return self.subtrees == []

    def is_root(self):
        return self.parent is None

    def get_root(self):
        return self if self.is_root() else self.parent.get_root()

    def get_depth(self):
        return 0 if self.is_root() else 1 + self.parent.get_depth()

    def get_parents(self):
        if self.is_root():
            return []
        return [self.parent] + self.parent.get_parents()

    def closest_root(self, target):
        own_pars = self.get_parents()
        tar_pars = target.get_parents()
        for i, own_par in enumerate(own_pars):
            for o, tar_par in enumerate(tar_pars):
                if own_par == tar_par:
                    return (own_par, i + o)


def make_tree(data, sep=")"):
    trees = dict()
    for parent_val, child_val in (x.split(sep) for x in data):
        parent_tree = trees.get(parent_val)
        if parent_tree is None:
            parent_tree = Tree(parent_val)
            trees[parent_val] = parent_tree
        child_tree = trees.get(child_val)
        if child_tree is None:
            child_tree = Tree(child_val)
            trees[child_val] = child_tree
        parent_tree.add_child(child_tree)

    return (next(iter(trees.values())).get_root(), trees)


def total_depth(trees):
    return sum(tree.get_depth() for tree in trees)


if __name__ == "__main__":
    data = map(lambda x: x.replace("\n", ""),
               open("data.txt", "rt").readlines())

    _, trees = make_tree(data)

    part1 = total_depth(trees.values())
    print(f"{part1=}")

    _, part2 = trees["YOU"].closest_root(trees["SAN"])
    print(f"{part2=}")
