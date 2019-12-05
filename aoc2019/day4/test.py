#!/usr/bin/env python3

import unittest

from solver import test_valid_pass, test_valid_strict


class TestMe(unittest.TestCase):
    def test_part1(self):
        self.assertTrue(test_valid_pass(111111))
        self.assertFalse(test_valid_pass(223450))
        self.assertFalse(test_valid_pass(123789))

    def test_part2(self):
        self.assertTrue(test_valid_strict(112233))
        self.assertTrue(test_valid_strict(111122))
        self.assertFalse(test_valid_strict(123444))


unittest.main()
