#!/usr/bin/env python3

import unittest

from solver import test_valid_pass


class TestMe(unittest.TestCase):
    def test_part1(self):
        self.assertTrue(test_valid_pass(111111))
        self.assertFalse(test_valid_pass(223450))
        self.assertFalse(test_valid_pass(123789))


unittest.main()
