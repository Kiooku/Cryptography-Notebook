import unittest
from sage.all import *
from src import LLL, Lattice

class TestLLLAlgorithm(unittest.TestCase):
    basis: matrix = matrix(ZZ, [[19, 2, 32, 46, 3, 33], 
                                [15, 42, 11, 0, 3, 24],
                                [43, 15, 0, 24, 4, 16],
                                [20, 44, 44, 0, 18, 15],
                                [0, 48, 35, 16, 31, 31],
                                [48, 33, 32, 9, 1, 29]])
    l: Lattice = Lattice(basis)
    
    def test_LLL_reduction(self):
        res: matrix = matrix(ZZ, [[15, 42, 11, 0, 3, 24], 
                                [4, -40, 21, 46, 0, 9],
                                [28, -27, -11, 24, 1, -8],
                                [5, 2, 33, 0, 15, -9],
                                [-20, 4, -9, 16, 13, 16],
                                [28, -11, -12, 9, -17, 14]])
        l_res: Lattice = Lattice(res)
        self.assertEqual(res, LLL(self.basis))
        self.assertTrue(self.l.determinant() == res.determinant() or self.l.determinant() == -res.determinant())
        self.assertLess(self.l.hadamard_ratio(5), l_res.hadamard_ratio(5))