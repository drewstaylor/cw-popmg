use cosmwasm_std::StdError;
use blake2_rfc::blake2b::{Blake2b, Blake2bResult};
use recur_fn::{recur_fn, RecurFn};
use hex::{encode as hex_encode};

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
            // let out_b = h.res.as_bytes();
            // let out = hex_encode(out_b);
            // println!("Depth {:?}: {:?}", h.n, out);

            // Instance
            let mut blake = Blake2b::new(32);
            blake.update(h.res.as_bytes());
            // Finalize
            let res = blake.finalize();
            h.res = res.clone();
            h.n += 1;
            // Result
            recur(h)
        }
    });

    let mut hasher = Blake2b::new(32);
    hasher.update(proof.as_bytes());

    let start = Hash {
        n: 1,
        res: hasher.finalize(),
    };
    
    let hash_result = recur.call(start);
    let res = hex_encode(hash_result.res.as_bytes());

    Ok(res)
}