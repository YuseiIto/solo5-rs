#![feature(format_args_nl)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

use smoltcp::iface::{Config, Interface, SocketSet};
use smoltcp::socket::tcp;
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address};
use solo5_rs::net::smoltcp::Solo5NetInterface;

use core::str;
use core::str::FromStr;

use crate::alloc::borrow::ToOwned;
use alloc::vec;
use solo5_rs::time::clock_monotonic_ns;
use solo5_rs::{consoleln, Solo5StartInfo};

#[solo5_rs::main]
fn main(_start: Solo5StartInfo) {
    let mut device = Solo5NetInterface::new("net0").unwrap();

    let start = clock_monotonic_ns();
    request_http(&mut device);
    let finish = clock_monotonic_ns();

    let elapsed = finish - start;
    consoleln!("Total time elapsed(ns): {}", elapsed);
    consoleln!(
        "Mean time per cycle(us): {}",
        (elapsed / (10000 * 1000)) as f64
    );
}
enum State {
    Connect,
    Request,
    Response,
}

fn request_http(device: &mut Solo5NetInterface) {
    // Create interface
    let mut config = Config::new(EthernetAddress([0xa2, 0xbe, 0x21, 0x22, 0x3a, 0x7e]).into());

    let address = IpAddress::from_str("10.0.0.3").expect("invalid address format");

    let url_port = 8080;
    let url_host = "10.0.0.3";
    let url_path = "/";

    let mut iface = Interface::new(
        config,
        device,
        Instant::from_micros((clock_monotonic_ns() as i64) / 1000),
    );
    iface.update_ip_addrs(|ip_addrs| {
        ip_addrs
            .push(IpCidr::new(IpAddress::v4(10, 0, 0, 2), 24))
            .unwrap();
    });
    iface
        .routes_mut()
        .add_default_ipv4_route(Ipv4Address::new(10, 0, 0, 1))
        .unwrap();

    // Create sockets
    let tcp_rx_buffer = tcp::SocketBuffer::new(vec![0; 1024]);
    let tcp_tx_buffer = tcp::SocketBuffer::new(vec![0; 1024]);
    let tcp_socket = tcp::Socket::new(tcp_rx_buffer, tcp_tx_buffer);
    let mut sockets = SocketSet::new(vec![]);
    let tcp_handle = sockets.add(tcp_socket);

    let mut state = State::Connect;

    for _ in 0..10000 {
        loop {
            let timestamp = Instant::from_micros((clock_monotonic_ns() as i64) / 1000);
            iface.poll(timestamp, device, &mut sockets);

            let socket = sockets.get_mut::<tcp::Socket>(tcp_handle);
            let cx = iface.context();

            state = match state {
                State::Connect if !socket.is_active() => {
                    consoleln!("[LOG] connecting");
                    let local_port = 49152; // randomize
                    socket.connect(cx, (address, url_port), local_port).unwrap();
                    State::Request
                }
                State::Request if socket.may_send() => {
                    consoleln!("[LOG] sending request");
                    let http_get = "GET ".to_owned() + url_path + " HTTP/1.1\r\n";
                    socket.send_slice(http_get.as_ref()).expect("cannot send");
                    let http_host = "Host: ".to_owned() + url_host + "\r\n";
                    socket.send_slice(http_host.as_ref()).expect("cannot send");
                    socket
                        .send_slice(b"Connection: close\r\n")
                        .expect("cannot send");
                    socket.send_slice(b"\r\n").expect("cannot send");
                    State::Response
                }
                State::Response if socket.can_recv() => {
                    socket
                        .recv(|data| {
                            consoleln!("{}", str::from_utf8(data).unwrap_or("(invalid utf8)"));
                            (data.len(), ())
                        })
                        .unwrap();
                    State::Response
                }
                State::Response if !socket.may_recv() => {
                    consoleln!("[LOG] Response received!");
                    break;
                }
                _ => state,
            };

            //iface.poll_delay(timestamp, &sockets);
        }
    }
}
