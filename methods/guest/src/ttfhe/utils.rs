pub fn encode(msg: u8) -> u64 {
    (msg as u64) << 60
}
pub fn round_value(val: u64) -> u64 {
    let mut rounded_val = val >> 47;
    rounded_val += rounded_val & 1;
    rounded_val >>= 1;
    rounded_val
}