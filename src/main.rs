use std::io;

use std::net::SocketAddr;
use std::net::UdpSocket;
use std::str::{from_utf8, FromStr};
use std::thread;
use std::time::Duration;

enum Role {
    Receiver,
    Sender,
    Server,
}

const SENDER_FLAG: &str = "s";
const RECEIVER_FLAG: &str = "r";
const ADDRESS_FLAG: &str = "a";
const MESSAGE_FLAG: &str = "m";

const SOCKET_TIMEOUT_MILLISECOND: u64 = 500;

fn main() {
    let role = read_role();
    match role {
        Role::Receiver => receiver_main(),
        Role::Sender => sender_main(),
        Role::Server => server_main(),
    }
}

fn server_main() {
    let mut buffer = [0; 1000];
    let mut receiver_address: Option<SocketAddr> = None;
    let mut sender_address: Option<SocketAddr> = None;
    let socket = UdpSocket::bind("0.0.0.0:8090").expect("unable to bind socket");
    loop {
        if let Ok((data_size, socket_address)) = socket.recv_from(&mut buffer) {
            if data_size == 0 {
                println!("no data received");
                continue;
            } else {
                if buffer[0] == SENDER_FLAG.as_bytes()[0] {
                    sender_address = Some(socket_address);
                    if let Some(address) = receiver_address {
                        let mut response = address.to_string();
                        response.insert_str(0, ADDRESS_FLAG);
                        socket.send_to(response.as_bytes(), socket_address);
                        println!("sent to sender: {}", response);
                    }
                } else if buffer[0] == RECEIVER_FLAG.as_bytes()[0] {
                    receiver_address = Some(socket_address);
                    if let Some(address) = sender_address {
                        let mut response = address.to_string();
                        response.insert_str(0, ADDRESS_FLAG);
                        socket.send_to(response.as_bytes(), socket_address);
                        println!("sent to receiver: {}", response);
                    }
                } else {
                    println!("invalid flag");
                }
            }
        }
    }
}

fn sender_main() {
    let mut buffer = [0; 1000];
    let server_address = read_address("server address?");
    let mut peer_address = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("message?");
    let mut message = String::new();
    io::stdin()
        .read_line(&mut message)
        .expect("unable to read input");
    message.insert_str(0, MESSAGE_FLAG);
    let socket = get_socket_to(server_address);
    loop {
        socket.send(SENDER_FLAG.as_bytes());
        if let Ok(message_size) = socket.recv(&mut buffer) {
            if message_size > 0 && buffer[0] == ADDRESS_FLAG.as_bytes()[0] {
                if let Ok(address_string) = from_utf8(&buffer[1..message_size]) {
                    println!("peer address received: {}", address_string);
                    if let Ok(address) = SocketAddr::from_str(address_string) {
                        peer_address = address;
                        break;
                    }
                }
            }
            println!("invalid response");
        }
    }
    socket
        .connect(peer_address)
        .expect("unable to connect to peer");
    thread::sleep(Duration::from_millis(SOCKET_TIMEOUT_MILLISECOND));
    loop {
        if let Ok(_) = socket.send(message.as_bytes()) {
            let result = socket.recv(&mut buffer);
            if let Ok(_) = result {
                return;
            }
        }
    }
}

fn receiver_main() {
    let mut buffer = [0; 1000];
    let server_address = read_address("server address?");
    let mut peer_address = SocketAddr::from(([127, 0, 0, 1], 8080));
    let socket = get_socket_to(server_address);
    loop {
        socket.send(RECEIVER_FLAG.as_bytes());
        if let Ok(message_size) = socket.recv(&mut buffer) {
            if message_size > 0 && buffer[0] == ADDRESS_FLAG.as_bytes()[0] {
                if let Ok(address_string) = from_utf8(&buffer[1..message_size]) {
                    println!("peer address received: {}", address_string);
                    if let Ok(address) = SocketAddr::from_str(address_string) {
                        peer_address = address;
                        break;
                    }
                }
            }
            println!("invalid response");
        }
    }
    socket
        .connect(peer_address)
        .expect("unable to connect to peer");
    loop {
        let result = socket.recv(&mut buffer);
        if let Ok(message_size) = result {
            if message_size > 0 && buffer[0] == MESSAGE_FLAG.as_bytes()[0] {
                if let Ok(message) = from_utf8(&buffer[1..message_size]) {
                    println!("message received: {}", message);
                    socket.send(&[]);
                    return;
                }
            }
            println!("invalid message format");
        }
        let result = socket.send(RECEIVER_FLAG.as_bytes());
    }
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
        input.clear();
    }
}

fn read_role() -> Role {
    let mut input = String::new();
    loop {
        println!("role? (1: receiver, 2: sender, 3: server)");
        io::stdin()
            .read_line(&mut input)
            .expect("unable to read input");
        match input.trim() {
            "1" => return Role::Receiver,
            "2" => return Role::Sender,
            "3" => return Role::Server,
            &_ => (),
        }
        println!("press just a 1, 2 or 3 and enter");
        input.clear();
    }
}

fn get_socket_to(address: SocketAddr) -> UdpSocket {
    let addresses = [
        SocketAddr::from(([0, 0, 0, 0], 8091)),
        SocketAddr::from(([0, 0, 0, 0], 8092)),
        SocketAddr::from(([0, 0, 0, 0], 8093)),
        SocketAddr::from(([0, 0, 0, 0], 8094)),
        SocketAddr::from(([0, 0, 0, 0], 8095)),
        SocketAddr::from(([0, 0, 0, 0], 8096)),
        SocketAddr::from(([0, 0, 0, 0], 8097)),
        SocketAddr::from(([0, 0, 0, 0], 8098)),
        SocketAddr::from(([0, 0, 0, 0], 8099)),
    ];
    let socket = UdpSocket::bind(&addresses[..]).expect("unable to bind a socket");
    socket
        .set_read_timeout(Some(Duration::from_millis(SOCKET_TIMEOUT_MILLISECOND)))
        .expect("unable to set timeout");
    socket
        .set_write_timeout(Some(Duration::from_millis(SOCKET_TIMEOUT_MILLISECOND)))
        .expect("unable to set timeout");
    socket
        .connect(address)
        .expect("unable to connect to server");
    return socket;
}
