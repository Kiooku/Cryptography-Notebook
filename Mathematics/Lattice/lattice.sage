class Lattice:
    def __init__(self, basis: Matrix):
        self.basis = basis

    
    def det(self) -> int:
        """ Determinant of the lattice

        Output:
            (int): determinant
        """
        if self.basis.is_square():
            return abs(self.basis.determinant())
        
        return sqrt(abs(self.basis.transpose() * self.basis))

    
    def mu(self, i: int, j: int, b_i: Matrix, b_j: Matrix) -> float:
        """ (v_i * v_j) / ||v_j||²

        Input:
            i (int)
            j (int)
            b_i (Matrix): first basis
            b_j (Matrix): second basis

        Output:
            (float)
        """
        return (b_i[i] * b_j[j]) / pow(b_j[j].norm(), 2)


    def gram_schmidt_algorithm(self, b: Matrix) -> Matrix:
        """ Gram Schmidt Algorithm

        Create an orthogonal_basis

        Input:
            b (Matrix): basis

        Output:
            (Matrix): Orthogonal basis
        """
        orthogonal_basis: list = [b[0]]
        for i in range(1, len(b.rows())):
            v_i: vector = b[i]
            v_ip = v_i
            for j in range(i):
                v_j: vector = orthogonal_basis[j]
                v_ip -= self.mu(i, j, b, Matrix(orthogonal_basis)) * v_j
            
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
        """ Solve the apprCVP if the basis is sufficiently orthogonal
        Babai's closest vertext algorithm

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

    
    def babai_closest_plane_algorithm(self, t: vector) -> vector:
        """ Solve the apprCVP with normaly better result than the Babai's closest vertext algorithm

        Args:
            t (vector)

        Output:
            (vector): vector in the lattice that is close to 't'
        """
        v_star: Matrix = self.gram_schmidt_algorithm(self.basis)
        w: vector = t
        for i in range(len(t)-1, -1, -1):
            w -= round((w * v_star[i]) / pow(v_star[i].norm(), 2)) * self.basis[i]
        
        return t - w


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

    
    def LLL(self) -> Matrix:
        """ LLL latice reduction algorithm

        Output:
            (Matrix): LLL reduced basis
        """
        k: int = 1
        n: int = len(self.basis.rows())
        b: Matrix = copy(self.basis) # Copy to don't modify the latice basis
        while k < n:
            orthogonal_basis: Matrix = self.gram_schmidt_algorithm(b)
            for j in range(k-1, -1, -1):
                # Size reduction
                b[k] = b[k] - round(self.mu(k, j, b, orthogonal_basis)) * b[j]
            
            # Lovász Condition
            if pow(orthogonal_basis[k].norm(), 2) >= (3/4 - pow(self.mu(k, k-1, b, orthogonal_basis), 2)) * pow(orthogonal_basis[k-1].norm(), 2):
                k += 1
            else:
                # Swap Step
                b[k], b[k-1] = b[k-1], b[k]
                k = max(k-1, 1)

        return b

    
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

    v2: vector= l.babai_closest_plane_algorithm(w)
    print(f"Babai's closest plane algorithm result: {round((v2 - w).norm(), 2)}")

    # Gaussian Lattice Reduction [Example 7.67]
    l2: Lattice = Lattice(Matrix([vector([66586820, 65354729]), vector([6513996, 6393464])]))
    svp_solution: vector = l2.gaussian_lattice_reduction()[0]
    print(f"A shortest vector for L2: {svp_solution}")

    # Gram-Schmidt Algorithm
    print(float(l2.hadamard_ratio(l2.basis[0], l2.basis[1])))
    l2_orthogonal_basis = l2.gram_schmidt_algorithm(l2.basis)
    print(l2_orthogonal_basis)
    print(float(Lattice(l2_orthogonal_basis).hadamard_ratio(l2_orthogonal_basis[0], l2_orthogonal_basis[1])))

    # LLL [Example 7.75]
    M: Matrix = Matrix([[19,2,32,46,3,33],
                    [15,42,11,0,3,24],
                    [43,15,0,24,4,16],
                    [20,44,44,0,18,15],
                    [0,48,35,16,31,31],
                    [48,33,32,9,1,29]])

    M_LLL: Matrix = Matrix([[7,-12,-8,4,19,9],
                    [-20,4,-9,16,13,16],
                    [5,2,33,0,15,-9],
                    [-6,-7,-20,-21,8,-12],
                    [-10,-24,21,-15,-6,-11],
                    [7,4,-9,-11,1,31]])

    print(M_LLL.determinant(), M.determinant())

    l3 = Lattice(M)
    res_LLL: Matrix = l3.LLL()
    print(res_LLL)
    print(f"LLL give the expected basis: {res_LLL == M_LLL}")
    print(f"The lattice basis has not been modified: {l3.basis == M}")