pub mod ethernet;
use ethernet::EthernetFrame;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LinkProtocols {
    Ethernet(EthernetFrame),
}
