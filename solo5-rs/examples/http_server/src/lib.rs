#![feature(format_args_nl)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

use smoltcp::iface::{Config, Interface, SocketSet};
use smoltcp::socket::tcp;
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address};
use solo5_rs::net::smoltcp::Solo5NetInterface;

use crate::alloc::borrow::ToOwned;
use alloc::fmt::Write;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::str;
use core::str::FromStr;
use solo5_rs::time::clock_monotonic_ns;
use solo5_rs::{console, consoleln, Solo5StartInfo};

#[solo5_rs::main]
fn main(_start: Solo5StartInfo) {
    let mut device = Solo5NetInterface::new("net0").unwrap();

    // Create interface
    let a0 = EthernetAddress([0x7e, 0xb5, 0xa9, 0x1e, 0x4a, 0x56]);
    let a1 = EthernetAddress([0xf6, 0x55, 0xae, 0xc0, 0xa6, 0xd6]);
    let a2 = EthernetAddress([0xba, 0x01, 0x00, 0x63, 0xbe, 0x3a]);
    let mut config = Config::new(a2.into());

    let mut iface = Interface::new(
        config,
        &mut device,
        Instant::from_micros((clock_monotonic_ns() as i64) / 1000),
    );
    iface.update_ip_addrs(|ip_addrs| {
        ip_addrs
            .push(IpCidr::new(IpAddress::v4(10, 0, 1, 2), 24))
            .unwrap();
    });
    iface
        .routes_mut()
        .add_default_ipv4_route(Ipv4Address::new(10, 0, 1, 1))
        .unwrap();

    // Create sockets
    let tcp_rx_buffer = tcp::SocketBuffer::new(vec![0; 1024]);
    let tcp_tx_buffer = tcp::SocketBuffer::new(vec![0; 1024]);
    let tcp_socket = tcp::Socket::new(tcp_rx_buffer, tcp_tx_buffer);

    let mut sockets = SocketSet::new(vec![]);
    let tcp_handle = sockets.add(tcp_socket);

    let mut is_active = false;

    loop {
        let timestamp = Instant::from_micros((clock_monotonic_ns() as i64) / 1000);
        iface.poll(timestamp, &mut device, &mut sockets);

        let socket = sockets.get_mut::<tcp::Socket>(tcp_handle);
        if !socket.is_open() {
            consoleln!("Listening...");
            socket.listen(80).unwrap();
        } else if socket.may_send() {
            consoleln!("Sending...");
            let s = "HTTP/1.1 200 OK\r\nDate: Fri, 19 Jan 2024 14:08:15 GMT\r\nServer: solo5-rs\r\nContent-Length: 22\r\nContent-Type: text/plain; charset=UTF-8\r\n\r\nHello World via HTTP\r\n";
            socket.send_slice(s.as_bytes()).unwrap();
            socket.close();
        }

        /*
        if socket.is_active() && !is_active {
            consoleln!("Connected");
        } else if !socket.is_active() && is_active {
            consoleln!("Disconnected");
        }

        is_active = socket.is_active();

        if socket.may_recv() {
            let data = socket
                .recv(|buffer| {
                    let recvd_len = buffer.len();
                    let data = buffer.to_owned();
                    let data = String::from_utf8(data).unwrap();
                    if !data.is_empty() {
                        consoleln!("received----");
                        console!("{}", data);
                        consoleln!("------");
                    }
                    (recvd_len, data)
                })
                .unwrap();

            // TODO: Parse request
            if socket.can_send() && !data.is_empty() {
                consoleln!("Sending...");
                let s = "HTTP/1.1 200 OK\r\nDate: Fri, 19 Jan 2024 14:08:15 GMT\r\nServer: solo5-rs\r\nContent-Length: 22\r\nContent-Type: text/plain; charset=UTF-8\r\n\r\nHello World via HTTP\r\n";
                socket.send_slice(s.as_bytes()).unwrap();
                socket.close();
            }
        }
        */
        iface.poll_delay(timestamp, &sockets);
    }
}
