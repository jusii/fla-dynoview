//! Parser for `\FLA\FLA.CFG` — the FLA 203 settings/calibration file (300 B).
//!
//! Byte 0 onward holds the workshop/owner name as a CP437, space-padded ASCII
//! field terminated by a NUL; the binary calibration constants follow further
//! in. The calibration is deliberately preserved verbatim (`raw`) so the
//! disk-reset feature can keep the machine's calibration lineage intact.

use crate::cp437::cp437_to_string;

/// Shop/owner info extracted from `FLA.CFG`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShopInfo {
    /// The workshop/owner name, e.g. `"Pellinen Motorsport"` (or the factory
    /// default `"BOSCH FLA 203"`).
    pub name: String,
    /// The complete file, kept so calibration bytes are never lost on rewrite.
    pub raw: Vec<u8>,
}

/// Parse a `FLA.CFG` byte slice.
pub fn parse_cfg(bytes: &[u8]) -> ShopInfo {
    // Name field: CP437 text up to the first NUL, trailing spaces trimmed.
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    let name = cp437_to_string(&bytes[..end]).trim_end().to_string();
    ShopInfo {
        name,
        raw: bytes.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_space_padded_name() {
        let mut b = vec![b' '; 300];
        b[..19].copy_from_slice(b"Pellinen Motorsport");
        b[56] = 0; // NUL terminator after the padded field
        for x in b.iter_mut().skip(57) {
            *x = 0xAB; // pretend calibration bytes
        }
        let s = parse_cfg(&b);
        assert_eq!(s.name, "Pellinen Motorsport");
        assert_eq!(s.raw.len(), 300);
    }

    #[test]
    fn factory_default_name() {
        let mut b = vec![0u8; 300];
        b[..13].copy_from_slice(b"BOSCH FLA 203");
        assert_eq!(parse_cfg(&b).name, "BOSCH FLA 203");
    }
}
