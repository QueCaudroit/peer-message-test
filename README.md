# Peer messaging test

## Goal of this exercise

The goal was to design a system able to tranfer a message from one user to the other in peer to peer, without using relays, using any technologies available. Since the focus of this test was not defined, I decided to try to implement some nat hole punching (no TURN server because no relay) while neglecting scalability and security.

## Installation

Clone the sources and execute `cargo run` at the project root folder.

The server must be accessible to both client and server and able to listen on port 8090

When asked for the server address, you need to give both ipv4 and port (example: `127.0.0.1:8090`)

## Limitations

- Since we cannot use relays, there are some network configurations where the receiver and sender cannot communicate. This implementation should be able to punch holes in some NAT, but in testing it did not work on the NAT I'm behind.

- This solution was designed as a toy, and supports only 2 users. However, it would be easy to greatly expand the number of users by just using a hash map instead of 2 variables.

- Messages cannot be larger than 999 bytes, because of the buffer sized used and the current design assuming that messages are sent in 1 packet

## Scalability

We'll assume in this part that the improvement mentionned in limitation 2 is implemented. We'll study the constraints on the server, since both sender and receiver are unafected by the number of other users.

### RAM

If we implement a hash map to store users, we'll have to store a key and a hash in addition to the addresses of the user. So we need 32 bytes for the ipv4 address (on windows), 1 for the hashmap metadata and 10 for a unique id, for a total of 43 bytes per user. so on a 8GB, we can have roughly 200 M users, so we should not have issues here

### Bandwith

Here we have two potential bottlenecks, the number of users that are asking for their peers and the number of address the server can send per second.

- Sending an address takes at most 86 bytes (64 for the header and 22 for the content, since we're sending it as a string and adding a char) so 866b. On a 1Gb/s symetrical connection, we have a limit of roughly 1.5 M users / second

- Polling for the peer address takes 74 bytes (64 header + 10 for the id) so 592b. It polls twice a second, so 1 user takes 1184b/s and the connection above can handle roughly 900k users

### CPU

I/O is slow so the service shouldn't be CPU-bound. If we take the limit above of 900k users polling twice a second, the loop time must be above 0.5 us for the CPU to become blocking

### Wrapping it up

This design should handle 900k users at the same time or 1.5M users/s, which will be limiting if users wait less than 0.6s on average

## Security

The system isn't secure at all.

- Messages are sent in clear text, this could be improved by using TCP + TLS instead of UDP
- Anyone can connect as the receiver or the sender. This could be improved by having some auth mechanism.

## Compatibility

The system should be compatible with Linux, Mac and Windows.

