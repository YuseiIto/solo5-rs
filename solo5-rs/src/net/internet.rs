pub mod arp;
pub mod ip;
use arp::ArpPacket;
use ip::IpPacket;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum InternetProtocol {
    Ip(IpPacket),
    Arp(ArpPacket),
}
