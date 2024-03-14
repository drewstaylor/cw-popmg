use cosmwasm_std::StdError;
use blake2_rfc::blake2b::{Blake2b, Blake2bResult};
// use recur_fn::{recur_fn, RecurFn};
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
    // Create hasher
    let mut hasher = Blake2b::new(32);
    hasher.update(&hex_decode(proof).unwrap());
    // Instance of hash
    let mut h = Hash {
        n: 1,
        res: hasher.finalize(),
    };
    // Recurse
    for _n in 0..depth-1 {
        let mut blake = Blake2b::new(32);
        blake.update(h.res.as_bytes());
        h.res = blake.finalize();
        h.n += 1;
        // let res = hex_encode(h.res.as_bytes());
        // dbg!(h.n, res);
    }
    // Result
    let res = hex_encode(h.res.as_bytes());
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_proof() {
        // Depth of 1
        let chain_size: u32 = 2;
        let depth: u32 = chain_size - 1;
        let proof = "6dca8d85358b735f7b0fb4031fa2ba3be75cc4fea9648accd0cfb747092dced7";
        let expected_hash_result = "df69b9d584c7594c819796d31b8c9b174a3c2f45f3a1e9f3443ce4831584c074";
        let res: String = generate_proof_as_string(depth, proof.to_string()).unwrap();
        assert_eq!(res, expected_hash_result.to_string());

        // Depth of 1000
        let chain_size: u32 = 1000;
        let depth: u32 = chain_size - 1;
        let proof = "28d9a5b289fbbac7a8f94fbc6c0952f890e247537008d905a49ce22ff2b607e0";
        let expected_hash_result = "6c41476aa9a032e1c7d465151bc2546ddcc052b4f70aabc8e3389083aa10eaae";
        let res: String = generate_proof_as_string(depth, proof.to_string()).unwrap();
        assert_eq!(res, expected_hash_result.to_string());
    }

    #[test]
    fn large_chain() {
        let size: u32 = 10000;
        let hex = "6dca8d85358b735f7b0fb4031fa2ba3be75cc4fea9648accd0cfb747092dced7";
        let res: String = generate_proof_as_string(size, hex.to_string()).unwrap();
        assert_eq!(res, "e5814b4459b13e00cc02ef2bc2c5a834860966c1084575c3faba69804586ff0a".to_string());
    }
}