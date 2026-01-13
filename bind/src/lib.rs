use justcash_prove::{DIM, Digest, Receipt, init as jc_init, prove as jc_prove};

uniffi::setup_scaffolding!();

#[derive(uniffi::Record)]
pub struct Input {
    pub hashes: Vec<Vec<u8>>,
    pub directions: Vec<u8>,
    pub sk: Vec<u8>,
    pub nk: Vec<u8>,
}

#[derive(uniffi::Error)]
pub enum ProveError {
}

#[uniffi::export]
fn prove(input: Input) -> Vec<u8> {
    jc_prove(justcash_prove::Input {
        hashes: input
            .hashes
            .iter()
            .map(|x| Digest::try_from(x.to_vec()).unwrap())
            .collect::<Vec<Digest>>()
            .try_into()
            .unwrap(),
        directions: input.directions.try_into().unwrap(),
        sk: Digest::ZERO,
        nk: Digest::ZERO,
    });
    [0u8; 1].to_vec()
}
