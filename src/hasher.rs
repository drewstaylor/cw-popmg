use cosmwasm_std::StdError;
use blake2_rfc::blake2b::{Blake2b, Blake2bResult};
use recur_fn::{recur_fn, RecurFn};
use hex::{encode as hex_encode, decode as hex_decode};

#[derive(Debug)]
pub struct Hash {
    pub n: u32, 
    pub res: Blake2bResult,
}

pub fn generate_proof_as_string(
    depth: u32,
    proof: String,
) -> Result<String, StdError> {
    // Recursor
    let recur = recur_fn(|recur, mut h: Hash| {
        if h.n >= depth {
            h
        } else {
            // Instance
            let mut blake = Blake2b::new(32);
            blake.update(h.res.as_bytes());
            // Finalized
            h.res = blake.finalize();
            h.n += 1;
            // Result
            recur(h)
        }
    });

    let mut hasher = Blake2b::new(32);
    hasher.update(&hex_decode(proof).unwrap());

    let start = Hash {
        n: 1,
        res: hasher.finalize(),
    };
    
    let hash_result = recur.call(start);
    let res = hex_encode(hash_result.res.as_bytes());

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hasher() {
        let chain_size: u32 = 2;
        let depth: u32 = chain_size - 1;
        let proof = "6dca8d85358b735f7b0fb4031fa2ba3be75cc4fea9648accd0cfb747092dced7";
        let expected_hash_result = "df69b9d584c7594c819796d31b8c9b174a3c2f45f3a1e9f3443ce4831584c074";
        let res: String = generate_proof_as_string(depth, proof.to_string()).unwrap();
        assert_eq!(res, expected_hash_result.to_string());
    }

    #[test]
    fn test_speed() {
        let size: u32 = 10001;
        let hex = "6dca8d85358b735f7b0fb4031fa2ba3be75cc4fea9648accd0cfb747092dced7";

        let mut hasher = Blake2b::new(32);
        hasher.update(&hex_decode(hex).unwrap());

        let mut cur_hash = Hash {
            n: 0,
            res: hasher.finalize(),
        };

        let mut i: u32 = 0;
        while i < size {
            let mut blake = Blake2b::new(32);
            blake.update(cur_hash.res.as_bytes());
            cur_hash.res = blake.finalize();
            cur_hash.n += 1;
            i += 1;
        }

        let res = hex_encode(cur_hash.res.as_bytes());
        assert_eq!(res, "adbffab8a4deeab6717559f3c6fb695f4ea27e759db764daa487e46d30644acf".to_string());
    }
}