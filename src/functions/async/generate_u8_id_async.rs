use crate::SpudError;

type VecBool<'a> = tokio::sync::MutexGuard<'a, Vec<bool>>;

pub(crate) fn generate_u8_id_async(id_vec: &mut VecBool) -> Result<u8, SpudError> {
    let mut id: [u8; 1] = [0_u8; 1];

    getrandom::fill(&mut id)?;

    let id: u8 = id[0];

    if id_vec[id as usize] {
        return generate_u8_id_async(id_vec);
    }

    id_vec[id as usize] = true;

    Ok(id)
}

#[cfg(test)]
mod tests {
    use tokio::sync::Mutex;

    use super::*;

    #[test]
    fn test_generate_u8_id_success() {
        #[cfg(not(feature = "async"))]
        let mut id_tracker: VecBool = vec![false; 256];

        #[cfg(feature = "async")]
        let binding: Mutex<Vec<bool>> = Mutex::new(vec![false; 256]);
        #[cfg(feature = "async")]
        let mut id_tracker = binding.try_lock().unwrap();

        let result: Result<u8, SpudError> = generate_u8_id_async(&mut id_tracker);

        assert!(result.is_ok(), "Function should return a valid ID");
        let generated_id = result.unwrap();

        assert!(
            id_tracker[generated_id as usize],
            "The generated ID should be marked as used in the tracker"
        );
    }
}
