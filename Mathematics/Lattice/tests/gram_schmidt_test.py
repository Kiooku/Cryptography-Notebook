import unittest
from sage.all import *
from src import Lattice, gram_schmidt

class TestGramSchmidtAlgorithm(unittest.TestCase):
    l: Lattice = Lattice(matrix(ZZ, [[2,0],[1,2]]))
    
    def test_gram_schmidt_orthogonalization(self):
        orthogonal_basis: matrix = matrix(RR, [[2,0], [0,2]])
        self.assertEqual(orthogonal_basis, gram_schmidt(self.l.basis))
        
    # TODO test gram_schmidt_lattice