use crate::SpudError;

pub(crate) fn generate_u8_id(id_vec: &mut Vec<bool>) -> Result<u8, SpudError> {
    let mut id: [u8; 1] = [0_u8; 1];

    getrandom::fill(&mut id)?;

    let id: u8 = id[0];

    if id_vec[id as usize] {
        return generate_u8_id(id_vec);
    }

    id_vec[id as usize] = true;

    Ok(id)
}
