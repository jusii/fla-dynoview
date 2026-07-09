//! Minimal IBM code page 437 → Unicode decoding.
//!
//! The FLA 203 stores 8.3 filenames and the `FLA.CFG` shop name in CP437, so a
//! Finnish shop name like "Ahonen & Pöllä" round-trips correctly (ö = 0x94).

/// CP437 high half (0x80..=0xFF) mapped to Unicode scalar values.
#[rustfmt::skip]
const HIGH: [char; 128] = [
    'Ç','ü','é','â','ä','à','å','ç','ê','ë','è','ï','î','ì','Ä','Å',
    'É','æ','Æ','ô','ö','ò','û','ù','ÿ','Ö','Ü','¢','£','¥','₧','ƒ',
    'á','í','ó','ú','ñ','Ñ','ª','º','¿','⌐','¬','½','¼','¡','«','»',
    '░','▒','▓','│','┤','╡','╢','╖','╕','╣','║','╗','╝','╜','╛','┐',
    '└','┴','┬','├','─','┼','╞','╟','╚','╔','╩','╦','╠','═','╬','╧',
    '╨','╤','╥','╙','╘','╒','╓','╫','╪','┘','┌','█','▄','▌','▐','▀',
    'α','ß','Γ','π','Σ','σ','µ','τ','Φ','Θ','Ω','δ','∞','φ','ε','∩',
    '≡','±','≥','≤','⌠','⌡','÷','≈','°','∙','·','√','ⁿ','²','■','\u{00A0}',
];

/// Decode a single CP437 byte to a Unicode `char`.
#[inline]
pub fn cp437_char(b: u8) -> char {
    if b < 0x80 {
        b as char
    } else {
        HIGH[(b - 0x80) as usize]
    }
}

/// Decode a CP437 byte slice into a `String`.
pub fn cp437_to_string(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| cp437_char(b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_passthrough() {
        assert_eq!(cp437_to_string(b"FLA.CFG"), "FLA.CFG");
    }

    #[test]
    fn finnish_high_bytes() {
        // ä=0x84, ö=0x94, Å=0x8F
        assert_eq!(cp437_to_string(&[0x84, 0x94, 0x8F]), "äöÅ");
    }
}
