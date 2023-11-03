use cosmwasm_std::StdError;
use blake2_rfc::blake2b::{Blake2b, Blake2bResult};
use recur_fn::{recur_fn, RecurFn};
use hex::{encode as hex_encode, decode as hex_decode};

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
            let res = blake.finalize();
            h.res = res.clone();
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
}