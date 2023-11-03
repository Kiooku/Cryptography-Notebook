class Lattice:
    def __init__(self, basis: Matrix):
        self.basis = basis


    def gram_schmidt_algorithm(self) -> Matrix:
        """ Gram Schmidt Algorithm

        Create an orthogonal_basis

        Output:
            (Matrix): Orthogonal basis
        """
        orthogonal_basis: list = [self.basis[0]]
        for i in range(1, len(self.basis.rows())):
            v_i: vector = self.basis[i]
            v_ip = v_i
            for j in range(i):
                v_j: vector = orthogonal_basis[j]
                u_ij = (v_i * v_j) / pow(v_j.norm(), 2)
                v_ip -= u_ij * v_j
            
            orthogonal_basis.append(v_ip)
        
        return Matrix(orthogonal_basis)
            


    def hadamard_ratio(self, *args: vector) -> float:
        """ Compute the Hadamard ratio
        
        The closer that the value is to 1, the more orthogonal are the vectors in the basis.
        
        Args:
            args (vector): vectors for the hadamard_ratio

        Output:
            hadamard_ratio (float): Hadamard ratio
        """
        temp: float = 1
        for arg in args:
            temp *= arg.norm()
        return pow(abs(self.basis.det()) / (temp), 1/len(args))


    
    def babai_algorithm(self, w: vector) -> vector:
        """ Solve the CVP if the basis is sufficiently orthogonal

        Args:
            w (vector)

        Output:
            (vector): vector in the lattice that is close to 'w'
        """
        t_i = self.basis.solve_left(w)
        v = zero_vector(len(t_i))
        for i in range(len(t_i)):
            v += round(t_i[i]) * self.basis[i]

        return v


    def gaussian_lattice_reduction(self) -> Matrix:
        """ Gaussian lattice reduction algorithm

        Allow lattice reduction in dimension 2. 
        Solve the SVP.

        Output:
            (vector): A shortest nonzero vector in L
        """
        if len(self.basis.rows()) != 2:
            raise Exception("Should be a lattice of dimension 2")

        v1: vector = self.basis[0]
        v2: vector = self.basis[1]
        while True:
            if v2.norm() < v1.norm():
                v1, v2 = v2, v1
            
            m: int = round((v1 * v2) / pow(v1.norm(), 2))
            if m == 0:
                return Matrix([v1, v2])
            
            v2 = v2 - m * v1

    
    def __str__(self) -> str:
        return str(self.basis)


if __name__ == "__main__":
    # An introduction to mathematical cryptography (Second edition)

    # Babai's Algorithm [Example 7.35]
    v1: vector = vector([137, 312])
    v2: vector = vector([215, -187])
    l: Lattice = Lattice(Matrix([v1, v2]))
    w: vector = vector([53172, 81743])

    v: vector = l.babai_algorithm(w)
    assert round((v - w).norm(), 2) == 76.12
    print(f"Close vector to w: {round((v - w).norm(), 2)}")

    print(f"Hadamard ratio: {round(l.hadamard_ratio(v1, v2), 3)}")

    # Gaussian Lattice Reduction
    l2: Lattice = Lattice(Matrix([vector([66586820, 65354729]), vector([6513996, 6393464])]))
    svp_solution: vector = l2.gaussian_lattice_reduction()[0]
    print(f"A shortest vector for L2: {svp_solution}")

    # Gram Schmidt Algorithm
    print(float(l2.hadamard_ratio(l2.basis[0], l2.basis[1])))
    l2_orthogonal_basis = l2.gram_schmidt_algorithm()
    print(l2_orthogonal_basis)
    print(float(Lattice(l2_orthogonal_basis).hadamard_ratio(l2_orthogonal_basis[0], l2_orthogonal_basis[1])))
