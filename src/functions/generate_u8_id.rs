use crate::SpudError;

#[cfg(not(feature = "async"))]
type VecBool = Vec<bool>;

#[cfg(feature = "async")]
type VecBool<'a> = tokio::sync::MutexGuard<'a, Vec<bool>>;

pub(crate) fn generate_u8_id(id_vec: &mut VecBool) -> Result<u8, SpudError> {
    let mut id: [u8; 1] = [0_u8; 1];

    getrandom::fill(&mut id)?;

    let id: u8 = id[0];

    if id_vec[id as usize] {
        return generate_u8_id(id_vec);
    }

    id_vec[id as usize] = true;

    Ok(id)
}
