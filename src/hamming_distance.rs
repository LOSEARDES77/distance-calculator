use std::{error::Error, fmt, str::FromStr};

struct BitVec<const N: usize> {
    elems: [u8; N],
}

impl<const N: usize> BitVec<N> {
    pub fn new(elems: [u8; N]) -> Self {
        Self { elems }
    }

    pub fn hamming_distance(&self, other: &BitVec<N>) -> u8 {
        self.elems
            .iter()
            .zip(other.elems.iter())
            .map(|(s, o)| s ^ o)
            .sum()
    }
}

#[derive(Debug, Copy, Clone)]
enum BitVecError {
    InvalidLength,
    InvalidValue,
}

impl fmt::Display for BitVecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BitVecError::InvalidLength => "Invalid Length for BitVec",
                BitVecError::InvalidValue =>
                    "Invalid value for BitVec - elements can only be 0 or 1",
            }
        )
    }
}

impl Error for BitVecError {}

impl<const N: usize> FromStr for BitVec<N> {
    type Err = BitVecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.bytes().len() != N {
            return Err(BitVecError::InvalidLength);
        }

        s.chars()
            .map(|c| {
                if c != '0' && c != '1' {
                    Err(BitVecError::InvalidValue)
                } else {
                    Ok(c)
                }
            })
            .map(|c| Ok(c? as u8))
            .collect::<Result<Vec<u8>, BitVecError>>()?
            .try_into()
            .map_err(|_| BitVecError::InvalidValue)
            .map(|v| BitVec::new(v))
    }
}

pub fn hamming_distance(a: &str, b: &str) -> usize {
    let a = a.trim().parse::<BitVec<8>>().unwrap();
    let b = b.trim().parse::<BitVec<8>>().unwrap();
    a.hamming_distance(&b) as usize
}