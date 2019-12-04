#!/usr/bin/env python3
import unittest

from solver import run_code


class TestMe(unittest.TestCase):
    def test_first(self):
        first_test_data = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
        run_code(first_test_data)
        self.assertEqual(
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            first_test_data,
        )

    def test_other_given(self):
        further_test_data = [
            (
                [1, 0, 0, 0, 99],
                [2, 0, 0, 0, 99],
            ),
            (
                [2, 3, 0, 3, 99],
                [2, 3, 0, 6, 99],
            ),
            (
                [2, 4, 4, 5, 99, 0],
                [2, 4, 4, 5, 99, 9801],
            ),
            (
                [1, 1, 1, 4, 99, 5, 6, 0, 99],
                [30, 1, 1, 4, 2, 5, 6, 0, 99],
            ),
        ]

        for start, result in further_test_data:
            run_code(start)
            self.assertEqual(start, result)


if __name__ == "__main__":
    unittest.main()
