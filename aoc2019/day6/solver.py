#!/usr/bin/env python3


class UnorderedTree:
    def __init__(self, val, children=[], parent=None):
        self.val, self.children, self.parent = val, children, parent

    def add_child(self, child):
        child.parent = self
        self.children.append(child)

    def is_root(self):
        return self.parent is None

    def get_depth(self):
        return 0 if self.is_root() else 1 + self.parent.get_depth()

    def get_parents(self):
        if self.is_root():
            return []
        return [self.parent] + self.parent.get_parents()

    def closest_root(self, target):
        for i, own_par in enumerate(self.get_parents()):
            for o, tar_par in enumerate(target.get_parents()):
                if own_par == tar_par:
                    return (own_par, i + o)
        raise ValueError("Nodes have no common root")


def make_tree(data, sep=")"):
    trees = dict()
    for parent_val, child_val in (x.split(sep) for x in data):
        for val in (parent_val, child_val):
            if trees.get(val) is None:
                trees[val] = UnorderedTree(val)
        trees[parent_val].add_child(trees[child_val])
    return trees


if __name__ == "__main__":
    trees = make_tree(
        x.replace("\n", "") for x in open("data.txt", "rt").readlines())

    part1 = sum(t.get_depth() for t in trees.values())
    print(f"{part1=}")

    _, part2 = trees["YOU"].closest_root(trees["SAN"])
    print(f"{part2=}")
