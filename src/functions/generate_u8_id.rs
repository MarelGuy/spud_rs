pub(crate) fn generate_u8_id(id_vec: &mut Vec<bool>) -> u8 {
    let mut id: [u8; 1] = [0_u8; 1];

    let id: u8 = match getrandom::fill(&mut id) {
        Ok(()) => id[0],
        Err(e) => {
            tracing::error!("Failed to generate ID: {e}");
            panic!("Closing...")
        }
    };

    if id_vec[id as usize] {
        return generate_u8_id(id_vec);
    }

    id_vec[id as usize] = true;

    id
}
