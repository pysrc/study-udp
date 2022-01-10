use std::ops::Range;

/*
RFC:  791

    0                   1                   2                   3
    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |Version|  IHL  |Type of Service|          Total Length         |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |         Identification        |Flags|      Fragment Offset    |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |  Time to Live |    Protocol   |         Header Checksum       |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                       Source Address                          |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                    Destination Address                        |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                    Options                    |    Padding    |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

                    Example Internet Datagram Header

*/
// -----IPv4头
const IPv4_VER_IHL: usize = 0;
const IPv4_DSCP_ECN: usize = 1;
const IPv4_LENGTH: Range<usize> = 2..4;
const IPv4_IDENT: Range<usize> = 4..6;
const IPv4_FLG_OFF: Range<usize> = 6..8;
const IPv4_TTL: usize = 8;
const IPv4_PROTOCOL: usize = 9;
const IPv4_CHECKSUM: Range<usize> = 10..12;
const IPv4_SRC_ADDR: Range<usize> = 12..16;
const IPv4_DST_ADDR: Range<usize> = 16..20;

#[inline]
pub fn ipv4_version(d: &[u8]) -> u8 {
    return d[IPv4_VER_IHL] >> 4;
}

#[inline]
pub fn ipv4_ihl(d: &[u8]) -> u8 {
    return d[IPv4_VER_IHL] & 0b1111;
}

#[inline]
pub fn ipv4_dscp_ecn(d: &[u8]) -> u8 {
    return d[IPv4_DSCP_ECN];
}

#[inline]
pub fn ipv4_length(d: &[u8]) -> u16 {
    return ((d[IPv4_LENGTH][0] as u16) << 8) | (d[IPv4_LENGTH][1] as u16);
}

#[inline]
pub fn ipv4_ident(d: &[u8]) -> u16 {
    return ((d[IPv4_IDENT][0] as u16) << 8) | (d[IPv4_IDENT][1] as u16);
}

#[inline]
pub fn ipv4_flag(d: &[u8]) -> u8 {
    return d[IPv4_FLG_OFF][0] & 0b1110_0000;
}

#[inline]
pub fn ipv4_offset(d: &[u8]) -> u16 {
    return (d[IPv4_FLG_OFF][0] & 0b0001_111) as u16 | (d[IPv4_FLG_OFF][1] as u16);
}

#[inline]
pub fn ipv4_ttl(d: &[u8]) -> u8 {
    return d[IPv4_TTL];
}

#[inline]
pub fn ipv4_protocol(d: &[u8]) -> u8 {
    return d[IPv4_PROTOCOL];
}

#[inline]
pub fn ipv4_checksum(d: &[u8]) -> u16 {
    return ((d[IPv4_CHECKSUM][0] as u16) << 8) | (d[IPv4_CHECKSUM][1] as u16);
}

#[inline]
pub fn ipv4_src_addr(d: &[u8]) -> &[u8] {
    return &d[IPv4_SRC_ADDR];
}

#[inline]
pub fn ipv4_dst_addr(d: &[u8]) -> &[u8] {
    return &d[IPv4_DST_ADDR];
}

#[inline]
pub fn ipv4_data(d: &[u8]) -> &[u8] {
    return &d[ipv4_ihl(d) as usize * 4..];
}

/*
RFC 768

    0      7 8     15 16    23 24    31
    +--------+--------+--------+--------+
    |     Source      |   Destination   |
    |      Port       |      Port       |
    +--------+--------+--------+--------+
    |                 |                 |
    |     Length      |    Checksum     |
    +--------+--------+--------+--------+
    |
    |          data octets ...
    +---------------- ...

        User Datagram Header Format

*/
// -----UDP头
const UDP_SRC_PORT: Range<usize> = 0..2;
const UDP_DST_PORT: Range<usize> = 2..4;
const UDP_LENGTH: Range<usize> = 4..6;
const UDP_CHECKSUM: Range<usize> = 6..8;

#[inline]
pub fn udp_src_port(d: &[u8]) -> u16 {
    return ((d[UDP_SRC_PORT][0] as u16) << 8) | (d[UDP_SRC_PORT][1] as u16);
}

#[inline]
pub fn udp_dst_port(d: &[u8]) -> u16 {
    return ((d[UDP_DST_PORT][0] as u16) << 8) | (d[UDP_DST_PORT][1] as u16);
}

#[inline]
pub fn udp_length(d: &[u8]) -> u16 {
    return ((d[UDP_LENGTH][0] as u16) << 8) | (d[UDP_LENGTH][1] as u16);
}

#[inline]
pub fn udp_checksum(d: &[u8]) -> u16 {
    return ((d[UDP_CHECKSUM][0] as u16) << 8) | (d[UDP_CHECKSUM][1] as u16);
}

#[inline]
pub fn udp_data(d: &[u8]) -> &[u8] {
    return &d[8..];
}

// UdpBuilder
#[inline]
fn u32c(x: u8, y: u8) -> u32 {
    ((x as u32) << 8) | y as u32
}

pub fn ipv4_udp_build(
    ip_pack: &mut [u8],
    src_ip: &[u8],
    src_port: u16,
    dst_ip: &[u8],
    dst_port: u16,
    data: &[u8],
) {
    // 组装IPv4头
    ip_pack[IPv4_VER_IHL] = (4 << 4) | 5;
    ip_pack[IPv4_DSCP_ECN] = 0;
    let length = ip_pack.len();
    ip_pack[IPv4_LENGTH][0] = (length >> 8) as u8;
    ip_pack[IPv4_LENGTH][1] = length as u8;
    ip_pack[IPv4_IDENT][0] = 0;
    ip_pack[IPv4_IDENT][1] = 0;
    ip_pack[IPv4_FLG_OFF][0] = 0b0100_0000;
    ip_pack[IPv4_FLG_OFF][1] = 0;
    ip_pack[IPv4_CHECKSUM][0] = 0;
    ip_pack[IPv4_CHECKSUM][1] = 0;
    ip_pack[IPv4_TTL] = 64;
    ip_pack[IPv4_PROTOCOL] = 17;
    ip_pack[IPv4_SRC_ADDR].clone_from_slice(src_ip);
    ip_pack[IPv4_DST_ADDR].clone_from_slice(dst_ip);
    // 计算ipv4 Checksum
    let mut checksum: u32 = 0;
    let mut i = 1;
    while i <= 19 {
        let u = u32c(ip_pack[i - 1], ip_pack[i]);
        checksum += u;
        i += 2;
    }
    let mut h = checksum >> 16; // 高16位
    while h != 0 {
        checksum = h + (checksum & 0xffff);
        h = checksum >> 16;
    }
    let checksum16 = !(checksum as u16);
    ip_pack[IPv4_CHECKSUM][0] = (checksum16 >> 8) as u8;
    ip_pack[IPv4_CHECKSUM][1] = checksum16 as u8;

    // 组装UDP头
    let udp_pack = &mut ip_pack[20..];
    udp_pack[UDP_SRC_PORT][0] = (src_port >> 8) as u8;
    udp_pack[UDP_SRC_PORT][1] = src_port as u8;
    udp_pack[UDP_DST_PORT][0] = (dst_port >> 8) as u8;
    udp_pack[UDP_DST_PORT][1] = dst_port as u8;
    let udp_len = 8 + data.len();
    udp_pack[UDP_LENGTH][0] = (udp_len >> 8) as u8;
    udp_pack[UDP_LENGTH][1] = udp_len as u8;
    udp_pack[UDP_CHECKSUM][0] = 0;
    udp_pack[UDP_CHECKSUM][1] = 0;
    udp_pack[8..].copy_from_slice(data);
    // 计算UDP Checksum

    /*
    RFC 768 UDP伪头部
                      0      7 8     15 16    23 24    31
                     +--------+--------+--------+--------+
                     |          source address           |
                     +--------+--------+--------+--------+
                     |        destination address        |
                     +--------+--------+--------+--------+
                     |  zero  |protocol|   UDP length    |
                     +--------+--------+--------+--------+
    */
    let mut checksum: u32 = 0;
    checksum += u32c(src_ip[0], src_ip[1]);
    checksum += u32c(src_ip[2], src_ip[3]);
    checksum += u32c(dst_ip[0], dst_ip[1]);
    checksum += u32c(dst_ip[2], dst_ip[3]);
    checksum += u32c(0, 17);
    checksum += udp_len as u32;

    let mut i = 1;
    let end = udp_len as usize - 1;
    while i <= end {
        let u = u32c(udp_pack[i - 1], udp_pack[i]);
        checksum += u;
        i += 2;
    }

    if i - end == 1 {
        // 奇数
        checksum += u32c(udp_pack[end], 0);
    }

    let mut h = checksum >> 16; // 高16位
    while h != 0 {
        checksum = h + (checksum & 0xffff);
        h = checksum >> 16;
    }
    let checksum16 = !(checksum as u16);
    udp_pack[UDP_CHECKSUM][0] = (checksum16 >> 8) as u8;
    udp_pack[UDP_CHECKSUM][1] = checksum16 as u8;
}
