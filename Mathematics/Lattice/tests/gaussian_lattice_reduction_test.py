import unittest
from sage.all import *
from src import Lattice, gaussian_lattice_reduction

class TestGramSchmidtAlgorithm(unittest.TestCase):
    l: Lattice = Lattice(matrix(ZZ, [[66586820, 65354729],[6513996, 6393464]]))
    wrong_lattice_for_gaussian_lattice_reduction: Lattice = Lattice(matrix(ZZ, [[1,2,3], [4,5,6], [7,8,9]]))

    def test_gaussian_lattice_reduction(self):
        solution: matrix = matrix(ZZ, [[2280, -1001],[-1324, -2376]])
        self.assertEqual(solution, gaussian_lattice_reduction(self.l.basis))
        
    def test_should_return_an_error_lattice_dimension_upper_than_two(self):
        self.assertRaises(AssertionError, gaussian_lattice_reduction, self.wrong_lattice_for_gaussian_lattice_reduction.basis)