use std::io;

use std::net::SocketAddr;
use std::net::UdpSocket;
use std::str::{from_utf8, FromStr};

enum Role {
    Receiver,
    Sender,
}

fn main() {
    let sender_address = read_address("sender address?");
    let receiver_address = read_address("receiver address?");
    let role = read_role();
    match role {
        Role::Receiver => receive_message(receiver_address, sender_address),
        Role::Sender => send_message(sender_address, receiver_address),
    }
}

fn send_message(own_address: SocketAddr, other_address: SocketAddr) {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("unable to read input");
    let socket = UdpSocket::bind(own_address).expect("unable to bind socket");
    socket
        .connect(other_address)
        .expect("unable to connect to peer");
    socket
        .send(input.as_bytes())
        .expect("failed to send message");
}

fn receive_message(own_address: SocketAddr, other_address: SocketAddr) {
    let mut buffer = [0; 1000];
    let socket = UdpSocket::bind(own_address).expect("unable to bind socket");
    socket
        .connect(other_address)
        .expect("unable to connect to peer");
    socket.recv(&mut buffer).expect("failed to send message");
    let message = from_utf8(&buffer).expect("message received isn't an utf8 encoded string");
    println!("{}", message);
}

fn read_address(prompt: &str) -> SocketAddr {
    let mut input = String::new();
    loop {
        println!("{}", prompt);
        io::stdin()
            .read_line(&mut input)
            .expect("unable to read input");
        if let Ok(address) = SocketAddr::from_str(&input.trim()) {
            return address;
        }
        println!("unable to parse this address");
    }
}

fn read_role() -> Role {
    let mut input = String::new();
    loop {
        println!("role? (1: receiver, 2: sender)");
        io::stdin()
            .read_line(&mut input)
            .expect("unable to read input");
        match input.trim() {
            "1" => return Role::Receiver,
            "2" => return Role::Sender,
            &_ => (),
        }
        println!("press just a 1 or 2 and enter");
    }
}
