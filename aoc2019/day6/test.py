#!/usr/bin/env python3

import unittest

from solver import make_tree, total_depth


class TestMe(unittest.TestCase):
    def test_part1(self):
        data = """COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L""".split("\n")
        _, trees = make_tree(data)
        self.assertEqual(total_depth(trees.values()), 42)

    def test_part2(self):
        data = """COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN""".split("\n")
        _, trees = make_tree(data)
        _, total = trees["YOU"].closest_root(trees["SAN"])
        self.assertEqual(total, 4)


if __name__ == "__main__":
    unittest.main()
