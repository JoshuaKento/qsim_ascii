use num_complex::Complex64;

pub struct QState {
    nqubits: usize,
    amps: Vec<Complex64>,
}

impl QState {
    // initialize (build) QState
    pub fn zero(nqubits: usize) -> Self {
        // dimension = 2^n
        let dim = 1usize << nqubits;

        // fill amps with 0 + 0i
        let mut amps = vec![Complex64::new(0.0, 0.0); dim];

        // first index in amps will be 1 + 0i
        amps[0] = Complex64::new(1.0, 0.0);
        return QState { nqubits, amps };
    }

    /// return map a -> a.norm_spr for amps
    pub fn probabilities(&self) -> Vec<f64> {
        self.amps.iter().map(|a| a.norm_sqr()).collect()
    }

    /// get a mutable slice of amps
    pub(crate) fn get_amps(&mut self) -> &mut [Complex64] {
        &mut self.amps
    }

    /// get a borrow slice of amps
    pub fn amps(&self) -> &[Complex64] {
        &self.amps
    }

    pub fn get_nqubits(&self) -> usize {
        self.nqubits
    }

    ///
    /// ToDo HIGH: Exception handling
    pub fn is_entangled_two_qubit(&self) -> bool {
        // assert: nqubits = 2
        // assert: amps.len() = 4

        let a00 = self.amps()[0];
        let a01 = self.amps()[1];
        let a10 = self.amps()[2];
        let a11 = self.amps()[3];

        // separable iff a00*a11 == a01*a10
        let det = a00 * a11 - a01 * a10;

        // floating tolerance
        let eps = 1e-12;
        det.norm_sqr() > eps * eps
    }
}
