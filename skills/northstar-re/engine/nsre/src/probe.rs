//! Bounded header inspection. Classification hints, not decryption or full validation.
use crate::unsafe_entry;
use serde_json::json;
use std::{
    fs::{self, File},
    io::{self, Read, Seek, SeekFrom},
    path::Path,
};
fn u16_at(b: &[u8], o: usize) -> Option<u16> {
    Some(u16::from_le_bytes(
        b.get(o..o.checked_add(2)?)?.try_into().ok()?,
    ))
}
fn u32_at(b: &[u8], o: usize) -> Option<u32> {
    Some(u32::from_le_bytes(
        b.get(o..o.checked_add(4)?)?.try_into().ok()?,
    ))
}
fn entropy(b: &[u8]) -> f64 {
    if b.is_empty() {
        return 0.0;
    }
    let mut c = [0u32; 256];
    for v in b {
        c[*v as usize] += 1;
    }
    c.iter()
        .filter(|&&n| n > 0)
        .map(|&n| {
            let p = n as f64 / b.len() as f64;
            -p * p.log2()
        })
        .sum()
}
fn signature(b: &[u8]) -> &'static str {
    if b.starts_with(b"MZ") {
        "dos-or-pe"
    } else if b.starts_with(b"7z\xBC\xAF\x27\x1C") {
        "7z"
    } else if b.starts_with(b"PK\x03\x04") || b.starts_with(b"PK\x05\x06") {
        "zip"
    } else if b.starts_with(b"DDS ") {
        "dds"
    } else if b.starts_with(b"\x89PNG\r\n\x1a\n") {
        "png"
    } else if b.starts_with(b"SQLite format 3\0") {
        "sqlite"
    } else if b.starts_with(b"\x7fELF") {
        "elf"
    } else if b.starts_with(b"\xff\xfe") {
        "utf16le-bom"
    } else if b.starts_with(b"\xef\xbb\xbf") {
        "utf8-bom"
    } else {
        "unknown"
    }
}
pub fn probe(path: &Path) -> io::Result<bool> {
    let before = fs::symlink_metadata(path)?;
    if unsafe_entry(&before) || !before.is_file() {
        return Err(io::Error::other("regular non-reparse file required"));
    }
    let mut f = File::open(path)?;
    let mut head = Vec::new();
    (&mut f).take(65536).read_to_end(&mut head)?;
    let mut pe = serde_json::Value::Null;
    if head.starts_with(b"MZ") {
        if let Some(off) = u32_at(&head, 0x3c).map(u64::from) {
            if off >= 64 && off.checked_add(24).is_some_and(|end| end <= before.len()) {
                f.seek(SeekFrom::Start(off))?;
                let mut h = [0u8; 24];
                f.read_exact(&mut h)?;
                if &h[..4] == b"PE\0\0" {
                    let machine = u16_at(&h, 4).unwrap_or(0);
                    pe = json!({"machine":machine,"architecture":match machine{0x14c=>"x86",0x8664=>"x64",0xaa64=>"arm64",_=>"unknown"},"sections":u16_at(&h,6),"optional_header_size":u16_at(&h,20),"characteristics":u16_at(&h,22),"pe_offset":off});
                }
            }
        }
    }
    let after = fs::symlink_metadata(path)?;
    if unsafe_entry(&after)
        || before.len() != after.len()
        || before.modified()? != after.modified()?
    {
        return Err(io::Error::other("file changed during probe"));
    }
    println!(
        "{}",
        json!({"schema":1,"signature_hint":signature(&head),"size":before.len(),"sample_bytes":head.len(),"sample_entropy_bits":entropy(&head),"pe_header_hint":pe,"warning":"Header hints only; entropy does not establish encryption; PE bounds and sections not fully validated"})
    );
    Ok(true)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn known_headers() {
        assert_eq!(signature(b"DDS "), "dds");
        assert_eq!(signature(b"MZ"), "dos-or-pe");
        assert_eq!(signature(b"PK\x05\x06"), "zip");
        assert_eq!(u32_at(b"abc", 0), None);
        assert_eq!(entropy(&[0; 8]), 0.0);
        assert_eq!(entropy(&[0, 1, 0, 1]), 1.0);
    }
}
