use ark_ff::fields::{Fp256, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "21888242871839275222246405745257275088696311157297823662689037894645226208583"]
#[generator = "3"]
pub struct FrConfig;
pub type Fr = Fp256<MontBackend<FrConfig, 4>>;