pub const ANCHOR_DISCRIMINANT_SIZE: usize = 8;

pub const fn anchor_ix_sighash(ix_name: &'static str) -> [u8; 8] {
    let result = sha2_const::Sha256::new()
        .update(b"global:")
        .update(ix_name.as_bytes())
        .finalize();
    let mut output = [0u8; 8];
    let mut i = 0;
    while i < 8 {
        output[i] = result[i];
        i += 1;
    }
    output
}

pub const fn anchor_account_sighash(acc_name: &'static str) -> [u8; 8] {
    let result = sha2_const::Sha256::new()
        .update(b"account:")
        .update(acc_name.as_bytes())
        .finalize();
    let mut output = [0u8; 8];
    let mut i = 0;
    while i < 8 {
        output[i] = result[i];
        i += 1;
    }
    output
}

/// Concatenate a list of arrays into a single array.
///
/// Should we used with macro `const_array_concat!`.
pub const fn const_concat<const N: usize, const M: usize>(
    arrays: [&'static [u8]; N],
) -> (usize, [u8; M]) {
    let mut output = [0u8; M];
    let mut i = 0;
    let mut j = 0;
    let mut expected_len = 0;
    while i < N {
        let array = arrays[i as usize];
        let mut k = 0;
        expected_len += array.len();
        while k < array.len() {
            output[j] = array[k];
            j += 1;
            k += 1;
        }
        i += 1;
    }
    (expected_len, output)
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn check_sighash() {
        assert_eq!(
            anchor_ix_sighash("swap"),
            [0xf8, 0xc6, 0x9e, 0x91, 0xe1, 0x75, 0x87, 0xc8]
        );
        assert_eq!(
            anchor_account_sighash("Whirlpool"),
            [0x3f, 0x95, 0xd1, 0x0c, 0xe1, 0x80, 0x63, 0x09]
        );
    }
}
