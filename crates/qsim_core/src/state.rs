use num_complex::Complex64;

pub struct QState {
    nqbits: usize,
    amps: Vec<Complex64>,
}

impl QState {
    // Initialize basis state |0...0>.
    pub fn zero(nqbits: usize) -> Self {
        let dim = 1usize << nqbits;
        let mut amps = vec![Complex64::new(0.0, 0.0); dim];
        amps[0] = Complex64::new(1.0, 0.0);
        QState { nqbits, amps }
    }

    pub fn total_probability(&self) -> f64 {
        self.amps.iter().map(|a| a.norm_sqr()).sum()
    }

    pub fn is_normalized(&self, eps: f64) -> bool {
        (self.total_probability() - 1.0).abs() <= eps
    }

    pub fn normalize(&mut self) {
        let total = self.total_probability();
        assert!(
            total.is_finite() && total > 0.0,
            "cannot normalize state with non-positive total probability"
        );
        let scale = total.sqrt().recip();
        for amp in &mut self.amps {
            *amp *= scale;
        }
    }

    pub fn validate(&self, eps: f64) -> Result<(), &'static str> {
        if self.amps.len() != (1usize << self.nqbits) {
            return Err("state dimension mismatch");
        }
        if !self.total_probability().is_finite() {
            return Err("state has non-finite probability mass");
        }
        if !self.is_normalized(eps) {
            return Err("state is not normalized");
        }
        Ok(())
    }

    /// Return basis probabilities p[k] = |a[k]|^2.
    pub fn probabilities(&self) -> Vec<f64> {
        self.amps.iter().map(|a| a.norm_sqr()).collect()
    }

    /// Get mutable amplitudes.
    pub(crate) fn get_amps(&mut self) -> &mut [Complex64] {
        &mut self.amps
    }

    /// Get immutable amplitudes.
    pub fn amps(&self) -> &[Complex64] {
        &self.amps
    }

    pub fn get_nqbits(&self) -> usize {
        self.nqbits
    }

    pub fn is_entangled_two_qubit(&self) -> bool {
        assert!(
            self.nqbits == 2 && self.amps.len() == 4,
            "is_entangled_two_qubit requires a 2-qubit state"
        );

        let a00 = self.amps()[0];
        let a01 = self.amps()[1];
        let a10 = self.amps()[2];
        let a11 = self.amps()[3];

        // Separable iff a00*a11 == a01*a10.
        let det = a00 * a11 - a01 * a10;

        let eps = 1e-12;
        det.norm_sqr() > eps * eps
    }
}
