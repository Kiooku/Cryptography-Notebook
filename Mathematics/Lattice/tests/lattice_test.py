import unittest
from sage.all import *
from src import Lattice


class TestLattice(unittest.TestCase):
    basis: matrix = matrix(ZZ, [[-1, -2], [-1, -1]])
    l: Lattice = Lattice(basis)
    l2: Lattice = Lattice(matrix(ZZ, [[1, 2],[0, 3]]))
    l3: Lattice =  Lattice(matrix(ZZ, [[1, 2], [-1, 1]]))
    l4: Lattice = Lattice(matrix(ZZ, [[2, 1]]))

    def test_should_return_the_lattice_basis(self):
        self.assertEqual(self.basis, self.l.get_basis())

    def test_should_return_the_dimension_of_the_lattice(self):
        self.assertEqual(2, self.l.dimension())

    def test_should_return_the_rank_of_the_lattice(self):
        self.assertEqual(2, self.l.rank())

    def test_should_be_full_rank(self):
        self.assertTrue(self.l.is_full_rank())

    def test_should_not_be_full_rank(self):
        self.assertFalse(self.l4.is_full_rank())
        
    def test_hadamard_ratio(self):
        l_temp: Lattice = Lattice(matrix(ZZ, [[137,312],[215,-187]]))
        self.assertEqual(0.977, l_temp.hadamard_ratio())

    def test_lattice_equality(self):
        self.assertTrue(self.l2 == self.l3)
        self.assertFalse(self.l == self.l2)
        
    def test_lattice_inequality(self):
        self.assertTrue(self.l != self.l2)
        self.assertFalse(self.l2 != self.l3)
        
    def test_lattice_to_string(self):
        res: str = "[-1 -2]\n[-1 -1]"
        self.assertEqual(res, self.l.__str__())