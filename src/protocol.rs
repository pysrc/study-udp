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
    return d[IPv4_VER_IHL]>>4;
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

