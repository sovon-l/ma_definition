pub fn str_to_asset(s: &str) -> [u8; 6] {
    let mut rt = [0u8; 6];
    rt[..s.len()].copy_from_slice(s.to_lowercase().as_bytes());
    rt
}

pub fn asset_to_str(u: &[u8]) -> &str {
    let mut tokens = u.split(|v| *v == 0u8);
    let token = tokens.next().unwrap();
    std::str::from_utf8(token).unwrap()
}
