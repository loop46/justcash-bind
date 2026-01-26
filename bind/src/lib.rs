use ciborium::into_writer;
use justcash_prove::{Digest, init as jc_init, prove as jc_prove};

uniffi::setup_scaffolding!();

#[derive(uniffi::Record)]
pub struct Input {
    pub hashes: Vec<Vec<u8>>,
    pub directions: Vec<u8>,
    pub sk: Vec<u8>,
    pub nk: Vec<u8>,
}

#[derive(Debug, uniffi::Error)]
pub enum ProveError {
    InvalidHashes,
    InvalidDirections,
    InvalidSecretKey,
    InvalidNullKey,
    ErrorProving,
    ErrorResultEncoding,
}

impl std::fmt::Display for ProveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[uniffi::export]
pub fn prove(input: Input) -> Result<Vec<u8>, ProveError> {
    let rec = jc_prove(justcash_prove::Input {
        hashes: input
            .hashes
            .iter()
            .try_fold(vec![], |mut v, x| {
                let a = Digest::try_from(x.to_vec());
                v.push(a.map_err(|_| ProveError::InvalidHashes)?);
                Ok(v)
            })?
            .try_into()
            .map_err(|_| ProveError::InvalidHashes)?,
        directions: input
            .directions
            .try_into()
            .map_err(|_| ProveError::InvalidDirections)?,
        sk: input
            .sk
            .try_into()
            .map_err(|_| ProveError::InvalidSecretKey)?,
        nk: input
            .nk
            .try_into()
            .map_err(|_| ProveError::InvalidNullKey)?,
    })
    .map_err(|_| ProveError::ErrorProving)?;

    let mut output = Vec::new();

    into_writer(&rec, &mut output).map_err(|_| ProveError::ErrorProving)?;

    Ok(output)
}

#[uniffi::export]
pub fn init() {
    jc_init()
}
