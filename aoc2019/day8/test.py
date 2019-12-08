#!/usr/bin/env python3

import unittest

from solver import PIXELS, get_layers, part1, part2


class TestMe(unittest.TestCase):
    def test_layers(self):
        self.assertEqual(
            get_layers("123456789012", width=3, height=2),
            [
                [
                    [1, 2, 3],
                    [4, 5, 6],
                ],
                [
                    [7, 8, 9],
                    [0, 1, 2],
                ],
            ],
        )

    def test_part2(self):
        layers = get_layers("0222112222120000", width=2, height=2)
        self.assertEqual(
            part2(layers, width=2, height=2), "".join((
                PIXELS[0],
                PIXELS[1],
                "\n",
                PIXELS[1],
                PIXELS[0],
            )))


if __name__ == "__main__":
    unittest.main()
