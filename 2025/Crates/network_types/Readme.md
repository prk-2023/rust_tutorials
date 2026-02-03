# network_types:

## Introduction:

**network_types** is a lightweight crate that defines common networking protocols and data structures,
things like Ethernet, IPv4/IPv6, TCP, UDP, ICMP headers, in a way that's safe, zero-copy and `no_std`
friendly. 

Main purpose of this crate: memory-layout compatible Rust definitions for standard network protocols.

It’s most commonly used in packet parsing, network tooling, and eBPF/XDP programs, where you need to 
interpret raw bytes as structured network headers without allocations or parsing overhead.

“Gives a safe-ish Rust structs that map directly onto on-the-wire packet formats.”

Because it is designed for low-level systems programming, it avoids the "heavy" abstractions found in 
standard library networking and focuses instead on how bits and bytes are physically arranged in a packet.

Core Philosophy: The Zero-Copy "Overlay":
In XDP or high-performance networking, you cannot afford to copy data from a buffer into a new object.
Instead, you tell Rust "Treat these 20 bytes of memory as an IPv4 header"

`network_types` make this safe and easy by ensuring all its structures use:

`#[repr(C)]` and `#[repr(packed)]`, this guarantees the Rust compiler won't reorder fields or add padding
keeping the memory layout identical to the IETF standards (RFCs)

    Headers are laid out exactly like the real packet data. No parsing, no copying.


### Key Features:

1. Protocols that are covered:
Crate covers most essential layers of OSI model:

- Layer 2 ( Data Link ): Ethernet ( `EthHdr` )
- Layer 3 ( Networking ): IPv4 ( `Ipv4Hdr` ), IPv6 ( `Ipv6Hdr `), ICMP ...
- Layer 4 ( Transport ): TCP ( `TcpHdr` ), UDP ( `UdpHdr` )

2. Native Bitfields and Enums:

Rather then forcing you to remember that `0x0800` is IPv4, the crate provides type-safe enums.
Ex: 
    `EthType::Ipv4` or `IpProto::Tcp` allows your code to remain readable while still compiling down to
    simple inter comparisons.

3. No-Std Support:

XDP runs inside kernels, they cannot use Rust std library (`std`).
`network_types` is a `#![no_std]` crate => has zero dependencies on OS, suitable for embedded firmwares,
kernel, eBPF..

### How to use:

Example: while writing XDP program, you typically use `network_types` to "walk" the packet. 
- You start at the Ethernet header, check its type, and then Jump to IP header that follows it:

```rust 
use network_types::eth::{EthHdr, EtherType};
use network_types::ip::Ipv4Hdr;

// 1. Point to the start of the packet
let eth_ptr = data_start as *const EthHdr;

// 2. Perform a bounds check (Required for the eBPF Verifier)
if (eth_ptr as usize) + EthHdr::LEN > data_end {
    return xdp_action::XDP_ABORTED;
}

// 3. Access fields directly
let ether_type = unsafe { u16::from_be((*eth_ptr).ether_type) };

if ether_type == EtherType::Ipv4 as u16 {
    // Move the pointer forward to the start of the IP header
    let ip_ptr = (eth_ptr as usize + EthHdr::LEN) as *const Ipv4Hdr;
    // ... further parsing ...
}
```

### Why use this instead of manual byte offset?

1. safety: It's much harder to accidentally read the wrong byte when you're using named fields like
   `src_addr`.

2. Maintenance: If you're building a tool that needs to support complex headers (like IPv6 options), the
   crate handles the math for you.

3. Standardization: Using the same types as the rest of the Aya community makes your eBPF code easier for
   other to audit and contribute to.



### Handle Endianness (Network Byte Order) :

Especially in XDP you’ll quickly run into the "Big-Endian vs. Little-Endian" wall.

Most modern CPUs (like x86_64) are Little-Endian, meaning they store the least significant byte of a number
first. 

However, network protocols are almost universally Big-Endian (also known as Network Byte Order).

Because `network_types` maps directly to raw memory, it does not automatically swap bytes for you. 
If you read a port number directly without converting it, your code will see port 80 (0x0050) as 
20480 (0x5000).

1. The Conversion Functions

To handle this, you’ll use the standard library (or core in no_std) conversion methods. 
Since `network_types` `structs` give you the raw values, you apply these functions immediately after
access:

- `u16::from_be(val)`: Converts a 16-bit value from Big-Endian to Native.

- `u32::from_be(val)`: Converts a 32-bit value from Big-Endian to Native.

- `to_be()`: Used when you are writing a value back to a packet (e.g., packet mangling).

2. Example: Parsing a Port:

Check if a UDP packet is heading for a specific port ( like DNS on port 53 ).

```rust 
use network_types::udp::UdpHdr;

// Assume we have verified bounds and have a pointer to the UDP header
let udp_ptr = transport_header_ptr as *const UdpHdr;

unsafe {
    // Read the destination port
    let raw_port = (*udp_ptr).dest; 
    
    // Convert from Big-Endian (Network) to Native (Host)
    let dest_port = u16::from_be(raw_port);

    if dest_port == 53 {
        // This is DNS traffic!
    }
}
```

3. Working with IP Addresses:

IP addresses are handled a bit differently in `network_types`. 

Instead of a single integer, they are often represented as arrays or specialized `structs` to keep the 
memory footprint exact.

- *IPv4*: Usually an [u8; 4] or a u32.

- *IPv6*: Always an [u8; 16].

When comparing IP addresses, you don't necessarily need to swap bytes if you compare them against another 
Big-Endian value. 

For example, comparing two [u8; 4] arrays works regardless of endianness because you're comparing 
byte-by-byte.

4. Why `network_types` stays "Raw"

You might wonder why the crate doesn't just return native types automatically.

- Transparency: 
    XDP is about raw speed. Implicit conversions hidden in a getter method would hide the CPU cycles being
    used.

-  Consistency: 
    In eBPF, you often want to keep data in Network Byte Order if you are just passing it along or using 
    it as a key in a `BPF` Map, as this avoids unnecessary double-swapping.

5. Pro-Tip for XDP

When defining constants for comparison (like a magic number or a specific port), you can use the .to_be()
method on the constant itself so the comparison happens in "Network Order" without needing to swap the 
packet data at runtime:

```rust 
const DNS_PORT_BE: u16 = 53u16.to_be();

// Much faster: compares raw packet bytes directly to a pre-swapped constant
if unsafe { (*udp_ptr).dest } == DNS_PORT_BE { 
    // ...
}
```
---

## XDP project with Aya.

- Generate XDP template using aya template.

- add `network_types`  to eBPF code:
```bash 
  $ cd my-project-ebpf/
  $ cargo add network_types 
```

- A Complete "Port Filter" Example:

kernel-space code: Program inspects incoming packets and drops anything directed at a specific port.

```rust 
#![no_std]
#![no_main]

use aya_ebpf::{macros::xdp, programs::XdpContext};
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{Ipv4Hdr, IpProto},
    udp::UdpHdr,
};

#[xdp]
pub fn my_filter(ctx: XdpContext) -> u32 {
    match try_my_filter(ctx) {
        Ok(action) => action,
        Err(_) => 0, // XDP_ABORTED
    }
}

fn try_my_filter(ctx: XdpContext) -> Result<u32, ()> {
    let start = ctx.data();
    let end = ctx.data_end();

    // 1. Parse Ethernet Header
    let eth = unsafe { &*(start as *const EthHdr) };
    if start + EthHdr::LEN > end { return Err(()); }

    // Only care about IPv4
    if u16::from_be(eth.ether_type) != EtherType::Ipv4 as u16 {
        return Ok(0); // XDP_PASS
    }

    // 2. Parse IPv4 Header (starts right after Ethernet)
    let ip = unsafe { &*((start + EthHdr::LEN) as *const Ipv4Hdr) };
    if start + EthHdr::LEN + Ipv4Hdr::LEN > end { return Err(()); }

    // Only care about UDP
    if ip.proto != IpProto::Udp {
        return Ok(0); // XDP_PASS
    }

    // 3. Parse UDP Header
    let udp = unsafe { &*((start + EthHdr::LEN + Ipv4Hdr::LEN) as *const UdpHdr) };
    if start + EthHdr::LEN + Ipv4Hdr::LEN + UdpHdr::LEN > end { return Err(()); }

    // Drop traffic on port 4000
    if u16::from_be(udp.dest) == 4000 {
        return Ok(1); // XDP_DROP
    }

    Ok(0) // XDP_PASS
}
```

5. Essential Tips for XDP Development
- The Verifier is King: The Linux kernel "Verifier" checks your code before it runs. If you forget a single
if (start + LEN > end) check, the Verifier will reject your program with a "permission denied" error.

- Cursor Pattern: As your packet parsing gets more complex, consider creating a Cursor or Buffer struct to 
track your current offset so you don't have to keep writing start + EthHdr::LEN + Ipv4Hdr::LEN manually.

- Fast Iteration: Use RUST_LOG=info cargo xtask run (if using the Aya template) to compile, load, and view
logs from your XDP program in one go.


## Use BPF Maps to share data ( ex: blacklist of IP addresses ) between XDP program and user-space program.

In XDP, your program is often "stateless" it sees one packet at a time and forgets it instantly. 

To make it "smart" (like blocking IPs on the fly), you need **BPF Maps**.

BPF Map is like a shared hash table or array that lives in the kernel. 
Your XDP program can read from it, and your userspace Rust app can write to it.


### 1. Defining the Map

In your **eBPF code**, you define a map using Aya's macros. 
A `HashMap` is the most common for blacklists. 
We use `u32` for the IPv4 address (the key) and a `u8` just as a boolean flag (the value).

```rust
use aya_ebpf::macros::map;
use aya_ebpf::maps::HashMap;

#[map]
static BLACKLIST: HashMap<u32, u8> = HashMap::with_max_entries(1024, 0);

```

---

### 2. Using the Map in XDP

Now, inside your `try_my_filter` function, you can check if the source IP of the incoming packet exists in that map.

```rust
// ... after parsing the IPv4 header 'ip' ...

let src_ip = ip.src_addr; // This is already in Network Byte Order (u32)

// Look up the IP in the map
if let Some(_flag) = BLACKLIST.get(&src_ip) {
    return Ok(1); // XDP_DROP: This IP is blacklisted!
}

```

---

### 3. Updating the Map from Userspace

In your **Userspace code** (the part that runs on your desktop/server), you use the `aya` crate to "reach into" the kernel and add an IP address to that map.

```rust
use aya::maps::HashMap;
use std::net::Ipv4Addr;

// 1. Load the BPF program
let mut bpf = aya::Bpf::load(include_bytes_aligned!(path_to_ebpf_bin))?;

// 2. Get a handle to the "BLACKLIST" map
let mut blacklist: HashMap<_, u32, u8> = HashMap::try_from(bpf.map_mut("BLACKLIST").unwrap())?;

// 3. Add an IP (e.g., 1.2.3.4) to the blacklist
let block_ip = Ipv4Addr::new(1, 2, 3, 4);

// Note: .into() converts Ipv4Addr to u32 in Network Byte Order automatically
blacklist.insert(u32::from(block_ip).to_be(), 1, 0)?; 

println!("Successfully blocked {}", block_ip);

```
### 4. Why this is powerful

1. **Dynamic Control:** You can write a CLI tool or a web dashboard that updates the `BLACKLIST` map in real-time without ever restarting the XDP program.
2. **Performance:** Map lookups in the kernel are extremely fast (O(1) for HashMaps).
3. **Scalability:** You can store thousands of IPs, and the XDP program remains just as fast because it isn't looping through a list; it's doing a single hash lookup.

### Summary of Data Flow

| Component | Responsibility | Uses |
| --- | --- | --- |
| **`network_types`** | Interprets raw packet bytes into headers | `EthHdr`, `Ipv4Hdr` |
| **XDP Program** | Logic: Checks headers against BPF Maps | `BLACKLIST.get()` |
| **BPF Map** | Storage: Keeps state in kernel memory | `HashMap` |
| **Userspace** | Management: Feeds data into the Map | `blacklist.insert()` |
