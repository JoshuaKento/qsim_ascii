use num_complex::Complex64;

pub struct QState {
    pub nqubits: usize,
    pub amps: Vec<Complex64>,
}

impl QState {
    pub fn zero(nqubits: usize) -> Self {
        // dimension = 2^n
        let dim = 1usize << nqubits;

        // fill amps with 0 + 0i
        let mut amps = vec![Complex64::new(0.0, 0.0); dim];

        // first index in amps will be 1 + 0i
        amps[0] = Complex64::new(1.0, 0.0);
        return QState { nqubits, amps };
    }
    pub fn probabilities(&self) -> Vec<f64> {
        // return map a -> a.norm_spr for amps
        self.amps.iter().map(|a| a.norm_sqr()).collect()
    }
}
