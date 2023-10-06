"""Module to create lattices using the Lattice class"""
from __future__ import annotations # https://peps.python.org/pep-0563/
from math import sqrt
from functools import reduce
from random import randint
from sage.all import *



class Lattice:
    """Allows to create a lattice with a basis defined from a matrix Z
    """

    def __init__(self, basis: matrix):
        self.basis: matrix = basis

    def is_full_rank(self) -> bool:
        """Determine if a lattice is a full rank lattice.

        Returns:
            bool: true if it's full rank, false otherwise
        """
        return self.rank() == self.dimension()

    def dimension(self) -> int:
        """Get the dimension of the lattice
        
        (Dimension of the basis vectors)
        
        Returns:
            int: dimension of the lattice
        """
        return len(self.basis[0])

    def rank(self) -> int:
        """Get the rank of the lattice
        
        (Number of vector in a basis for L)

        Returns:
            int: rank
        """
        return len(list(self.basis))

    def determinant(self) -> float:
        """Get the determinant of the lattice

        Returns:
            float: determinant
        """
        if self.basis.nrows() == self.basis.ncols():
            return abs(self.basis.det())
        temp_matrix = self.basis.transpose() * self.basis
        return sqrt(temp_matrix.det())

    def get_new_random_basis(self) -> matrix:
        """Return a new basis using the existing one
        The basis belong to the lattice

        Returns:
            matrice: new basis from the existing one
        """
        #TODO
        pass

    def get_basis(self) -> matrix:
        """Get the basis of the lattice

        Returns:
            matrice: basis
        """
        return self.basis

    def hadamard_ratio(self, precision: int = 3) -> float:
        """Compute the Hadamard ratio of the basis 
        
        'Thus 0 < H(B) ≤ 1, and the closer that the value is to 1, 
        the more orthogonal are the vectors in the basis.'

        Returns:
            float: hadamard ratio
        """
        temp: float = 1.0
        for i in range(self.basis.nrows()):
            temp *= self.basis[i].norm()

        res: float = pow(self.determinant() / temp, 1 / self.basis.nrows())
        return round(res, precision)

    def __str__(self) -> str:
        return str(self.basis)

    def __eq__(self, __value: Lattice) -> bool:
        #Suppose B, C are bases of the same lattice L(B) = L(C). Then, det(B) =± det(C).
        # https://math.stackexchange.com/questions/4435793/check-if-two-bases-form-the-same-lattice
        #TODO
        return span(self.basis) == span(__value.basis)
    