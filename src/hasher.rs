use blake2_rfc::blake2b::{Blake2b, Blake2bResult};
use cosmwasm_std::StdError;
use hex::{decode as hex_decode, encode as hex_encode};

#[derive(Clone, Debug)]
pub struct Hash {
    pub n: u32,
    pub res: Blake2bResult,
}

#[derive(Clone, Debug)]
pub struct Proof {
    pub depth: u32,
    pub hash: String,
}

pub fn generate_proof_as_string(depth: u32, proof: String) -> Result<String, StdError> {
    // Create hasher
    let mut hasher = Blake2b::new(32);
    hasher.update(&hex_decode(proof).unwrap());
    // Instance of hash
    let mut h = Hash {
        n: 1,
        res: hasher.finalize(),
    };
    // Recurse
    for _n in 0..depth - 1 {
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

pub fn valid_proof(proof: Proof, pubkey: Proof) -> bool {
    if proof.depth >= pubkey.depth {
        return false;
    }
    let depth: u32 = pubkey.depth - proof.depth;
    let res: String = generate_proof_as_string(depth, proof.hash).unwrap();

    res == pubkey.hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_proof() {
        // Depth of 1
        let chain_size: u32 = 2;
        let proof_index: u32 = 1;
        let proof = "6dca8d85358b735f7b0fb4031fa2ba3be75cc4fea9648accd0cfb747092dced7";
        let depth: u32 = chain_size - proof_index;
        let expected_hash_result =
            "df69b9d584c7594c819796d31b8c9b174a3c2f45f3a1e9f3443ce4831584c074";
        let res: String = generate_proof_as_string(depth, proof.to_string()).unwrap();
        assert_eq!(res, expected_hash_result.to_string());

        // Depth of 1000
        let chain_size: u32 = 1001;
        let proof_index: u32 = 1;
        let proof = "28d9a5b289fbbac7a8f94fbc6c0952f890e247537008d905a49ce22ff2b607e0";
        let depth: u32 = chain_size - proof_index;
        let expected_hash_result =
            "afbda72bc5ca82bc61d800fcc8fdfa4f059d95e58879795863b34525ded88fce";
        let res: String = generate_proof_as_string(depth, proof.to_string()).unwrap();
        assert_eq!(res, expected_hash_result.to_string());
    }

    #[test]
    fn validate_proof() {
        // Proof to be validated
        let proof = Proof {
            depth: 1_u32,
            hash: "6dca8d85358b735f7b0fb4031fa2ba3be75cc4fea9648accd0cfb747092dced7".to_string(),
        };
        // Public Secret
        let pubkey = Proof {
            depth: 2_u32,
            hash: "df69b9d584c7594c819796d31b8c9b174a3c2f45f3a1e9f3443ce4831584c074".to_string(),
        };

        // Correct proof must be valid
        let is_valid = valid_proof(proof, pubkey.clone());
        assert!(is_valid);

        // Using an incorrect proof must be invalid
        let invalid_proof = Proof {
            depth: 1_u32,
            hash: "282903d2e04faa9978a1f370c4f17e220536fe580dd4162e68e2c39b5f34de48".to_string(),
        };
        let is_valid = valid_proof(invalid_proof, pubkey);
        assert!(!is_valid);

        // Chain size must not impact proof validity
        // Long and short evidence chains must be provable
        let proof_long = Proof {
            depth: 1_u32, // Evidence chain size: 1000
            hash: "28d9a5b289fbbac7a8f94fbc6c0952f890e247537008d905a49ce22ff2b607e0".to_string(),
        };
        let proof_short = Proof {
            depth: 950_u32, // Evidence chain size: 51
            hash: "88a9caab9d9aa32089584df6e193dd3dc22147498e0dc73d5a3c5c471892b92c".to_string(),
        };
        let pubkey = Proof {
            depth: 1001_u32,
            hash: "afbda72bc5ca82bc61d800fcc8fdfa4f059d95e58879795863b34525ded88fce".to_string(),
        };
        let is_valid = valid_proof(proof_long, pubkey.clone());
        assert!(is_valid);
        let is_valid = valid_proof(proof_short, pubkey);
        assert!(is_valid);
    }

    #[test]
    fn generate_large_chain() {
        let size: u32 = 10000;
        let hex = "6dca8d85358b735f7b0fb4031fa2ba3be75cc4fea9648accd0cfb747092dced7";
        let res: String = generate_proof_as_string(size, hex.to_string()).unwrap();
        assert_eq!(
            res,
            "e5814b4459b13e00cc02ef2bc2c5a834860966c1084575c3faba69804586ff0a".to_string()
        );
    }
}
