pub mod udp;
use udp::UdpPacket;

pub mod icmp;
use icmp::IcmpPacket;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TransportProtocol {
    Udp(UdpPacket),
    Icmp(IcmpPacket),
}
