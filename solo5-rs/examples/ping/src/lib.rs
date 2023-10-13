#![feature(format_args_nl)]
#![feature(lang_items)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;
use ethox::layer::{arp, eth, icmp, ip};
use ethox::managed::{List, Slice};
use ethox::nic::Device;
use ethox::wire::{
    ethernet::Address,
    ip::v4::{Address as IpV4Address, Cidr},
};
use solo5_rs::{console, consoleln, Solo5StartInfo};
mod ethox_solo5;
use alloc::{string::ToString, vec};
use ethox_solo5::Solo5NetInterface;

#[solo5_rs::main]
fn main(_start: Solo5StartInfo) -> u64 {
    let name = "net0".to_string();

    let mut interface =
        Solo5NetInterface::new(&name, vec![0; 1 << 14]).expect("Couldn't initialize interface");

    /*
        let host = Cidr::new(IpV4Address([10, 0, 0, 2]), 24);
        let hostmac = interface.mac_addr();

        let gateway = Cidr::new(IpV4Address([10, 0, 0, 1]), 24);
        let gatemac = Address::from_bytes(&[0x7e, 0xb5, 0xa9, 0x1e, 0x4a, 0x56]);
    */

    let host = Cidr::new(IpV4Address([10, 0, 0, 1]), 24);
    let hostmac = interface.mac_addr();

    let gateway = Cidr::new(IpV4Address([192, 168, 11, 1]), 24);
    //let gatemac = Address::from_bytes(&[0x7e, 0xb5, 0xa9, 0x1e, 0x4a, 0x56]);
    let gatemac = Address::from_bytes(&[0x74, 0x03, 0xbd, 0x30, 0x59, 0xf3]);

    let mut eth = eth::Endpoint::new(hostmac);

    let mut neighbors = [arp::Neighbor::default(); 5];
    let neighbors = {
        let mut eth_cache = arp::NeighborCache::new(&mut neighbors[..]);
        eth_cache
            .fill(gateway.address().into(), gatemac, None)
            .unwrap();
        eth_cache
    };

    // Gatewayは一つなので、Routing tableの中身は一行で良い
    let mut ip = [ip::Route::new_ipv4_gateway(gateway.address()); 1];
    let routes = ip::Routes::import(List::new_full(ip.as_mut().into()));
    let mut ip = ip::Endpoint::new(Slice::One(host.into()), routes, neighbors);

    consoleln!("Started arp endpoint\n");
    let mut icmp = icmp::Endpoint::new();

    loop {
        let rx_ok = interface.rx(1, eth.recv(ip.recv(icmp.answer())));
        let tx_ok = interface.tx(1, eth.send(ip.layer_internal()));

        let result = rx_ok.and_then(|x| tx_ok.map(|y| x + y));

        result.unwrap_or_else(|err| {
            panic!("Error during receive {:?} {:?}", err, interface.last_err());
        });
    }
}
