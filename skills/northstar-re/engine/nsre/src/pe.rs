//! Bounded PE32/PE32+ header and section-range parser. Never loads target code.
use serde_json::{json, Value};
use std::io;
fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
fn word(bytes: &[u8], offset: usize) -> io::Result<u16> {
    let end = offset
        .checked_add(2)
        .ok_or_else(|| invalid("PE offset overflow"))?;
    Ok(u16::from_le_bytes(
        bytes
            .get(offset..end)
            .ok_or_else(|| invalid("truncated PE word"))?
            .try_into()
            .unwrap(),
    ))
}
fn dword(bytes: &[u8], offset: usize) -> io::Result<u32> {
    let end = offset
        .checked_add(4)
        .ok_or_else(|| invalid("PE offset overflow"))?;
    Ok(u32::from_le_bytes(
        bytes
            .get(offset..end)
            .ok_or_else(|| invalid("truncated PE dword"))?
            .try_into()
            .unwrap(),
    ))
}
fn range_overlap(ranges: &mut Vec<(u64, u64)>, start: u64, size: u64) -> io::Result<()> {
    let end = start
        .checked_add(size)
        .ok_or_else(|| invalid("PE range overflow"))?;
    if size > 0 && ranges.iter().any(|&(a, b)| start < b && a < end) {
        return Err(invalid("overlapping PE section ranges"));
    }
    if size > 0 {
        ranges.push((start, end));
    }
    Ok(())
}
pub(crate) fn layout(prefix: &[u8], file_size: u64) -> io::Result<Value> {
    if prefix.len() > 65536 || file_size < (prefix.len() as u64) || !prefix.starts_with(b"MZ") {
        return Err(invalid(
            "expected a PE prefix up to 64 KiB and consistent file size",
        ));
    }
    let pe = dword(prefix, 0x3c)? as usize;
    if !(64..=65536 - 24).contains(&pe) || prefix.get(pe..pe + 4) != Some(b"PE\0\0") {
        return Err(invalid("invalid PE signature offset"));
    }
    let machine = word(prefix, pe + 4)?;
    let count = word(prefix, pe + 6)? as usize;
    if !(1..=96).contains(&count) {
        return Err(invalid("section count outside supported 1..=96"));
    }
    let optional_size = word(prefix, pe + 20)? as usize;
    if !(96..=4096).contains(&optional_size) {
        return Err(invalid("optional header outside parser bounds"));
    }
    let optional = pe + 24;
    let section_table = optional + optional_size;
    let table_end = section_table + count * 40;
    if table_end > prefix.len() {
        return Err(invalid(
            "PE headers or section table exceed available prefix",
        ));
    }
    let magic = word(prefix, optional)?;
    let (format, directory_offset, architecture) = match magic {
        0x10b => ("PE32", 96, if machine == 0x14c { "x86" } else { "other" }),
        0x20b => (
            "PE32+",
            112,
            match machine {
                0x8664 => "x64",
                0xaa64 => "arm64",
                _ => "other",
            },
        ),
        _ => return Err(invalid("unsupported PE optional header magic")),
    };
    if optional_size < directory_offset
        || (machine == 0x14c && magic != 0x10b)
        || (matches!(machine, 0x8664 | 0xaa64) && magic != 0x20b)
    {
        return Err(invalid("inconsistent machine or optional header size"));
    }
    let directory_count = dword(prefix, optional + directory_offset - 4)? as usize;
    if directory_count > 16 || directory_offset + directory_count * 8 > optional_size {
        return Err(invalid(
            "data directory count exceeds supported header bounds",
        ));
    }
    let file_alignment = dword(prefix, optional + 36)? as u64;
    let section_alignment = dword(prefix, optional + 32)? as u64;
    let header_size = dword(prefix, optional + 60)? as u64;
    let image_size = dword(prefix, optional + 56)? as u64;
    let entry_rva = dword(prefix, optional + 16)? as u64;
    if !file_alignment.is_power_of_two()
        || file_alignment > 65536
        || !section_alignment.is_power_of_two()
        || section_alignment < file_alignment
        || (section_alignment < 4096 && section_alignment != file_alignment)
        || (section_alignment >= 4096 && file_alignment < 512)
        || header_size < (table_end as u64)
        || header_size > file_size
        || !header_size.is_multiple_of(file_alignment)
        || image_size == 0
        || !image_size.is_multiple_of(section_alignment)
    {
        return Err(invalid("inconsistent PE alignment or image/header sizes"));
    }
    let mut raw_ranges = Vec::new();
    let mut virtual_ranges = Vec::new();
    let mut sections = Vec::new();
    let mut raw_end = header_size;
    let mut entry_mapped = entry_rva == 0 || entry_rva < header_size;
    for index in 0..count {
        let base = section_table + index * 40;
        let name_bytes = &prefix[base..base + 8];
        let name = String::from_utf8_lossy(
            &name_bytes[..name_bytes.iter().position(|&b| b == 0).unwrap_or(8)],
        )
        .into_owned();
        let virtual_size = dword(prefix, base + 8)? as u64;
        let rva = dword(prefix, base + 12)? as u64;
        let size = dword(prefix, base + 16)? as u64;
        let offset = dword(prefix, base + 20)? as u64;
        let flags = dword(prefix, base + 36)?;
        let extent = virtual_size.max(size);
        if !rva.is_multiple_of(section_alignment) || rva < header_size || rva + extent > image_size
        {
            return Err(invalid("PE section virtual range outside image bounds"));
        }
        if size > 0 {
            if offset < header_size
                || !offset.is_multiple_of(file_alignment)
                || !size.is_multiple_of(file_alignment)
                || offset + size > file_size
            {
                return Err(invalid("PE section raw range outside file bounds"));
            }
            range_overlap(&mut raw_ranges, offset, size)?;
            raw_end = raw_end.max(offset + size);
        }
        range_overlap(&mut virtual_ranges, rva, extent)?;
        entry_mapped |= entry_rva >= rva && entry_rva < rva + extent;
        sections.push(json!({"index":index,"name":name,"rva":rva,"virtual_size":virtual_size,"raw_offset":offset,"raw_size":size,
            "characteristics":flags,"readable":flags&0x40000000!=0,"writable":flags&0x80000000!=0,"executable":flags&0x20000000!=0}));
    }
    Ok(
        json!({"schema":1,"passed":true,"validated":true,"format":format,"machine":machine,"architecture":architecture,
        "file_size":file_size,"prefix_bytes":prefix.len(),"header_size":header_size,"image_size":image_size,
        "file_alignment":file_alignment,"section_alignment":section_alignment,"data_directories":directory_count,
        "entry_rva":entry_rva,"entry_mapped":entry_mapped,"sections":sections,"trailing_bytes":file_size-raw_end,
        "full_format_validated":false,"target_executed":false,"scope":"bounded PE headers and section ranges; directory contents, signatures and loader acceptance unverified"}),
    )
}
#[cfg(test)]
pub(crate) fn fixture(plus: bool) -> Vec<u8> {
    let mut b = vec![0u8; 1024];
    b[..2].copy_from_slice(b"MZ");
    b[0x3c..0x40].copy_from_slice(&128u32.to_le_bytes());
    b[128..132].copy_from_slice(b"PE\0\0");
    let machine: u16 = if plus { 0x8664 } else { 0x14c };
    b[132..134].copy_from_slice(&machine.to_le_bytes());
    b[134..136].copy_from_slice(&1u16.to_le_bytes());
    let opt: u16 = if plus { 240 } else { 224 };
    b[148..150].copy_from_slice(&opt.to_le_bytes());
    let o = 152;
    b[o..o + 2].copy_from_slice(&(if plus { 0x20bu16 } else { 0x10b }).to_le_bytes());
    for (offset, value) in [
        (16, 4096u32),
        (32, 4096),
        (36, 512),
        (56, 8192),
        (60, 512),
        (if plus { 108 } else { 92 }, 16),
    ] {
        b[o + offset..o + offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    let s = o + opt as usize;
    b[s..s + 5].copy_from_slice(b".text");
    for (offset, value) in [
        (8, 1u32),
        (12, 4096),
        (16, 512),
        (20, 512),
        (36, 0x60000020),
    ] {
        b[s + offset..s + offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    b
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pe32_and_pe32_plus_normalized_sections() {
        for plus in [false, true] {
            let b = fixture(plus);
            let r = layout(&b, b.len() as u64).unwrap();
            assert_eq!(r["validated"], true);
            assert_eq!(r["entry_mapped"], true);
            assert_eq!(r["sections"][0]["name"], ".text");
            assert_eq!(r["sections"][0]["executable"], true);
            assert_eq!(r["full_format_validated"], false);
        }
    }
    #[test]
    fn every_header_truncation_and_large_offset_are_rejected() {
        let b = fixture(true);
        for end in 0..432 {
            assert!(layout(&b[..end], 1024).is_err(), "accepted prefix {end}");
        }
        let mut b = fixture(false);
        b[0x3c..0x40].copy_from_slice(&u32::MAX.to_le_bytes());
        assert!(layout(&b, 1024).is_err());
    }
    #[test]
    fn raw_virtual_overlap_alignment_and_directory_bounds() {
        let valid = fixture(false);
        let s = 376;
        for (offset, value) in [
            (s + 20, 1024u32),
            (s + 16, u32::MAX),
            (s + 12, u32::MAX),
            (152 + 92, 17),
            (152 + 36, 0),
            (152 + 60, 128),
        ] {
            let mut b = valid.clone();
            b[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
            assert!(layout(&b, 1024).is_err(), "accepted mutation {offset}");
        }
        let mut b = valid.clone();
        b[134..136].copy_from_slice(&2u16.to_le_bytes());
        b.copy_within(s..s + 40, s + 40);
        assert!(layout(&b, 1024).is_err());
        b[s + 40 + 20..s + 40 + 24].copy_from_slice(&1024u32.to_le_bytes());
        assert!(layout(&b, 1536).is_err());
    }
}
