use snarkos_dpc::base_dpc::{instantiated::Components, parameters::CircuitParameters, BaseDPCComponents, DPC};
use snarkos_errors::dpc::DPCError;
use snarkos_models::algorithms::SNARK;
use snarkos_utilities::{bytes::ToBytes, to_bytes};

use rand::thread_rng;
use std::{
    fs::File,
    io::{Result as IoResult, Write},
    path::PathBuf,
};

pub fn setup<C: BaseDPCComponents>() -> Result<(Vec<u8>, Vec<u8>), DPCError> {
    let rng = &mut thread_rng();
    let circuit_parameters = CircuitParameters::<C>::load()?;

    let predicate_snark_parameters = DPC::<C>::generate_predicate_snark_parameters(&circuit_parameters, rng)?;
    let predicate_snark_pk = to_bytes![predicate_snark_parameters.proving_key]?;
    let predicate_snark_vk: <C::PredicateSNARK as SNARK>::VerificationParameters =
        predicate_snark_parameters.verification_key.into();
    let predicate_snark_vk = to_bytes![predicate_snark_vk]?;

    println!("predicate_snark_pk.params\n\tsize - {}", predicate_snark_pk.len());
    println!("predicate_snark_vk.params\n\tsize - {}", predicate_snark_vk.len());
    Ok((predicate_snark_pk, predicate_snark_vk))
}

pub fn store(path: &PathBuf, bytes: &Vec<u8>) -> IoResult<()> {
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    drop(file);
    Ok(())
}

pub fn main() {
    let (predicate_snark_pk, predicate_snark_vk) = setup::<Components>().unwrap();
    store(&PathBuf::from("predicate_snark_pk.params"), &predicate_snark_pk).unwrap();
    store(&PathBuf::from("predicate_snark_vk.params"), &predicate_snark_vk).unwrap();
}