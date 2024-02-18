#![feature(format_args_nl)]
#![feature(lang_items)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;
use ethox::wire::{
    ethernet::Address,
    ip::v4::{self, Address as IpV4Address, Cidr},
};
use solo5_rs::{consoleln, Solo5StartInfo};
mod ethox_solo5;
use alloc::{
    format,
    string::{String, ToString},
    vec,
};
use ethox_solo5::Solo5NetInterface;

use ethox::layer::{arp, eth, ip, tcp};
use ethox::managed::{List, Map, Slice, SlotMap};
use ethox::nic::{Device, Protocol};

#[solo5_rs::main]
fn main(_start: Solo5StartInfo) {
    let name = "net0".to_string();

    let mut interface =
        Solo5NetInterface::new(&name, vec![0; 1 << 14]).expect("Couldn't initialize interface");

    let host = Cidr::new(IpV4Address([10, 0, 0, 1]), 24);
    let hostmac = interface.mac_addr();

    let gateway = Cidr::new(IpV4Address([192, 168, 10, 1]), 24);
    let mut eth = eth::Endpoint::new(hostmac);

    // Buffer space for arp neighbor cache
    let mut neighbors = [arp::Neighbor::default(); 1];
    // Buffer space for routes, we only have a single state one.
    let mut routes = [ip::Route::new_ipv4_gateway(gateway.address()); 1];
    let mut ip = ip::Endpoint::new(
        Slice::One(host.into()),
        ip::Routes::import(List::new_full(routes.as_mut().into())),
        arp::NeighborCache::new(&mut neighbors[..]),
    );

    let mut tcp = tcp::Endpoint::new(
        Map::Pairs(List::new(Slice::One(Default::default()))),
        SlotMap::new(
            Slice::One(Default::default()),
            Slice::One(Default::default()),
        ),
        tcp::IsnGenerator::from_secret_key_bytes([0x10; 16]),
    );

    let message = "GET / HTTP/1.1\r\n\r\n";
    let mut tcp_client = tcp::Client::new(
        IpV4Address([93, 184, 216, 34]).into(),
        80,
        tcp::io::RecvInto::new(vec![0; 1 << 20]),
        tcp::io::SendFrom::once(message.as_bytes()),
    );

    consoleln!("Sending..");

    loop {
        let rx = interface
            .rx(10, eth.recv(ip.recv(tcp.recv(&mut tcp_client))))
            .unwrap();
        let tx = interface
            .tx(10, eth.send(ip.send(tcp.send(&mut tcp_client))))
            .unwrap();

        if tcp_client.is_closed() {
            break;
        }
    }

    let received = tcp_client.recv().received();
    let http = String::from_utf8_lossy(received);
    let header_end = http
        .find("\r\n\r\n")
        .expect(&format!("Expected http header end in {}", http));
    consoleln!("{}", &http[header_end + 4..]);
}
