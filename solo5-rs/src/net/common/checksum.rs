/// Compute IP/UDP/TCP checksum.
/// It's computed as 16-bit one's complement of  sum of the one's complement.

pub fn checksum(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    let mut s = data.len();

    for i in 0..s / 2 {
        sum += (((data[i * 2 + 1] as u16) << 8) + data[i * 2] as u16) as u32;
        s -= 2;
        if sum & 0x80000000 == 1 {
            sum = (sum >> 16) + (sum & 0xffff);
        }
    }

    if s == 1 {
        sum += data[data.len() - 1] as u32;
    }

    while sum >> 16 > 1 {
        sum = (sum >> 16) + (sum & 0xffff);
    }

    sum = (sum >> 8) + ((sum << 8) & 0xff00);

    !sum as u16
}
