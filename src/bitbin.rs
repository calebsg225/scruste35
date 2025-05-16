//! src/bitbin.rs

use num_bigint::{BigInt, Sign};

/// define possible charsets to decode into
pub enum CharSet {
    ASCII,
    UTF8,
}

/// BitRead takes a byte array and converts it into a large integer.
/// Several methods are availale for slicing off bits.
pub struct BitRead {
    idx: usize,
    bit_size: usize,
    bits: BigInt,
}

impl BitRead {
    pub fn from(bytes: &[u8]) -> BitRead {
        Self {
            idx: bytes.len() << 3,
            bit_size: bytes.len() << 3,
            bits: BigInt::from_bytes_be(Sign::Plus, bytes),
        }
    }

    /// returns the current BitRead index
    pub fn get_idx(&self) -> usize {
        self.idx
    }

    /// starting at `self.idx`, slice off `num_bits` bits
    pub fn as_int(&mut self, num_bits: usize) -> u64 {
        if self.idx < num_bits {
            return 0;
        }
        self.forward(num_bits);
        (&self.bits >> self.idx & !(!BigInt::ZERO << num_bits))
            .to_u64_digits()
            .1[0]
    }

    /// returns the hex value of `num_bits` of bits
    pub fn as_hex(&mut self, num_bits: usize) -> String {
        let hex = format!("{:x}", self.as_int(num_bits));
        format!("0x{}{}", ["", "0"][hex.len() & 1], hex)
    }

    /// returns `num_bits` bits as a byte vector
    pub fn as_bytes(&mut self, num_bits: usize) -> Vec<u8> {
        self.as_int(num_bits).to_be_bytes().to_vec()
    }

    /// returns one bit (true or false)
    pub fn as_flag(&mut self) -> bool {
        self.as_int(1) == 1
    }

    /// returns `num_bits` bits as 90k time
    pub fn as_90k(&mut self, num_bits: usize) -> f64 {
        let int_time = self.as_int(num_bits);
        let sig_figs = 6;
        let p = u64::pow(10, sig_figs);
        ((int_time * p) as f64 / 90000.0).round() / p as f64
    }

    /// returns `num_bits` bits as bytes decoded as a char string
    pub fn as_charset(&mut self, num_bits: usize, charset: CharSet) -> String {
        let bytes = self.as_bytes(num_bits);
        match charset {
            CharSet::ASCII => {
                for byte in bytes.iter() {
                    if *byte >= 128 {
                        panic!("Invalid ASCII sequence");
                    }
                }
                std::str::from_utf8(&bytes).expect("Invalid utf-8 sequence")
            }
            CharSet::UTF8 => std::str::from_utf8(&bytes).expect("Invalid utf-8 sequence"),
        }
        .to_string()
    }

    /// skips `num_bits` bits
    pub fn forward(&mut self, num_bits: usize) {
        self.idx -= num_bits;
    }
}

pub struct BitWrite {}
