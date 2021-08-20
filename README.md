# MD5 IP Dehasher
This program exists to demonstrate how trivially easy it is to get an IPv4 from it's md5 hash using the least efficient method: brute force.

Since this program exists purely for educational purposes, it only supports 1 format for the hashed IP: the hash of 4 bytes representing an IPv4.
This could easily be "improved" to add support for other formats.

# Usage
1. Download the program from [releases](https://github.com/wait-what/ip-dehash/releases) or compile it yourself
1. Input the hash. For example, you could use `a54f0041a9e15b050f25c463f1db7449` for benchmarking.
1. The results will be printed to the terminal

# Compiling
1. Have **nightly** rust installed
1. Clone the repository
1. `cargo run --release`

# License
This project is licensed under [MIT](./LICENSE)
