//! encode Vec<u8> objects into a Base58Check encoded `String`
//! This crate is a direct port of the npm package `noble-base58check`
//! Details here: <https://github.com/pas1ko/noble-base58check/blob/master/base.ts>
//! 
//! This crate does NOT decode Base58Check encoded strings back to Vec<u8>
//! 

use std::error::Error;
use sha2::{Digest, Sha256};

mod base58check_config;
use base58check_config::Base58CheckConfig;

#[cfg(test)]
mod tests;


/// Encodes a Vec<u8> into a Base58Check encoded string, without checksumming the source Vec<u8>
pub fn encode_b58c_plain(source: Vec<u8>) -> Result<String, Box<dyn Error>> {

    let b_config = Base58CheckConfig::default();

    if source.len() <= 0 { return Ok("".to_string()); }

    let mut zeroes: usize = 0;
    let mut length: usize = 0;
    let mut pbegin: usize = 0;

    let pend = source.len();

    while pbegin != source.len() && source[pbegin] == 0 {
        pbegin += 1;
        zeroes += 1;
    }

    let size = ((pend as f64 - pbegin as f64) * b_config.i_factor + 1.0) as u64 >> 0;

    let mut b58: Vec<u8> = vec![0; size as usize]; // https://stackoverflow.com/questions/29530011/creating-a-vector-of-zeros-for-a-specific-size
    while pbegin != pend {
        let mut carry = source[pbegin] as u64;
        
        let mut it1 = (size - 1) as i64;
        let mut i = 0 as usize;

        while (carry != 0 || i < length) && it1 != -1 {

            carry += (256.0 * b58[it1 as usize] as f64) as u64 >> 0;
            let carry_mod_base = (carry as f64 % b_config.base as f64) as u64 >> 0;
            b58[it1 as usize] = carry_mod_base as u8;
            carry = ((&carry / b_config.base) as f64) as u64 >> 0;

            it1 = &it1 - 1;
            i = &i + 1;
        }

        if carry != 0 {
            panic!("carry is non-zero: c{} b{} i{} l{} b{} e{}", &carry, b58[it1 as usize], &i, &length, &pbegin, &pend);
        }
        length = i.to_owned();
        pbegin += 1;
    }

    let mut it2 = (size as u64) - (length as u64);
    

    while it2 != size && b58[it2 as usize] == 0 {
        it2 += 1;
    }
    let mut str_leader = (format!("{}", b_config.leader)).as_str().repeat(zeroes);

    while it2 < size {
        it2 += 1;

        let str_slice = format!("{}", b_config.alphabet_vec[b58[it2 as usize - 1 as usize] as usize]); 

        str_leader = format!("{}{}", &str_leader, &str_slice);
    }    

    Ok(str_leader)
}

fn double_sha256(payload: &[u8]) -> Vec<u8> {
    let hasher = Sha256::new().chain_update(&payload);
    let output: Vec<_> = hasher.finalize().into_iter().collect();

    let hasher = Sha256::new().chain_update(&output);
    hasher.finalize().into_iter().collect()
}

/// Converts a Vec<u8> into a checksummed base58check encoded string
pub fn encode(payload: Vec<u8>) -> Result<String, Box<dyn Error>> {
    let payload_u8a: &[u8] = &*payload;
    let checksum = double_sha256(payload_u8a);

    let mut the_buffer: Vec<u8> = vec![0; payload.len() + 4];
    
    let xx = payload_u8a.len();
 

    
    let ii = 0;
    for i in ii..xx {
        the_buffer[i] = payload_u8a[i];
    }

    the_buffer[ xx + 0] = checksum[0];
    the_buffer[ xx + 1] = checksum[1];
    the_buffer[ xx + 2] = checksum[2];
    the_buffer[ xx + 3] = checksum[3];

    encode_b58c_plain(the_buffer)
}


