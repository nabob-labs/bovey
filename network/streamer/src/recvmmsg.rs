//! The `recvmmsg` module provides recvmmsg() API implementation

pub use bovey_perf::packet::NUM_RCVMMSGS;
#[cfg(target_os = "linux")]
use {
    crate::msghdr::create_msghdr,
    itertools::izip,
    libc::{iovec, mmsghdr, sockaddr_storage, socklen_t, AF_INET, AF_INET6, MSG_WAITFORONE},
    std::{
        mem::{self, MaybeUninit},
        net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
        os::unix::io::AsRawFd,
    },
};
use {
    crate::packet::{Meta, Packet},
    std::{cmp, io, net::UdpSocket},
};

#[cfg(not(target_os = "linux"))]
pub fn recv_mmsg(socket: &UdpSocket, packets: &mut [Packet]) -> io::Result</*num packets:*/ usize> {
    debug_assert!(packets.iter().all(|pkt| pkt.meta() == &Meta::default()));
    let mut i = 0;
    let count = cmp::min(NUM_RCVMMSGS, packets.len());
    for p in packets.iter_mut().take(count) {
        p.meta_mut().size = 0;
        match socket.recv_from(p.buffer_mut()) {
            Err(_) if i > 0 => {
                break;
            }
            Err(e) => {
                return Err(e);
            }
            Ok((nrecv, from)) => {
                p.meta_mut().size = nrecv;
                p.meta_mut().set_socket_addr(&from);
                if i == 0 {
                    socket.set_nonblocking(true)?;
                }
            }
        }
        i += 1;
    }
    Ok(i)
}

#[cfg(target_os = "linux")]
fn cast_socket_addr(addr: &sockaddr_storage, hdr: &mmsghdr) -> Option<SocketAddr> {
    use libc::{sa_family_t, sockaddr_in, sockaddr_in6};
    const SOCKADDR_IN_SIZE: usize = std::mem::size_of::<sockaddr_in>();
    const SOCKADDR_IN6_SIZE: usize = std::mem::size_of::<sockaddr_in6>();
    if addr.ss_family == AF_INET as sa_family_t
        && hdr.msg_hdr.msg_namelen == SOCKADDR_IN_SIZE as socklen_t
    {
        // ref: https://github.com/rust-lang/socket2/blob/65085d9dff270e588c0fbdd7217ec0b392b05ef2/src/sockaddr.rs#L167-L172
        let addr = unsafe { &*(addr as *const _ as *const sockaddr_in) };
        return Some(SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::from(addr.sin_addr.s_addr.to_ne_bytes()),
            u16::from_be(addr.sin_port),
        )));
    }
    if addr.ss_family == AF_INET6 as sa_family_t
        && hdr.msg_hdr.msg_namelen == SOCKADDR_IN6_SIZE as socklen_t
    {
        // ref: https://github.com/rust-lang/socket2/blob/65085d9dff270e588c0fbdd7217ec0b392b05ef2/src/sockaddr.rs#L174-L189
        let addr = unsafe { &*(addr as *const _ as *const sockaddr_in6) };
        return Some(SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::from(addr.sin6_addr.s6_addr),
            u16::from_be(addr.sin6_port),
            addr.sin6_flowinfo,
            addr.sin6_scope_id,
        )));
    }
    error!(
        "recvmmsg unexpected ss_family:{} msg_namelen:{}",
        addr.ss_family, hdr.msg_hdr.msg_namelen
    );
    None
}

#[cfg(target_os = "linux")]
pub fn recv_mmsg(sock: &UdpSocket, packets: &mut [Packet]) -> io::Result</*num packets:*/ usize> {
    // Should never hit this, but bail if the caller didn't provide any Packets
    // to receive into
    if packets.is_empty() {
        return Ok(0);
    }
    // Assert that there are no leftovers in packets.
    debug_assert!(packets.iter().all(|pkt| pkt.meta() == &Meta::default()));
    const SOCKADDR_STORAGE_SIZE: socklen_t = mem::size_of::<sockaddr_storage>() as socklen_t;

    let mut iovs = [MaybeUninit::uninit(); NUM_RCVMMSGS];
    let mut addrs = [MaybeUninit::zeroed(); NUM_RCVMMSGS];
    let mut hdrs = [MaybeUninit::uninit(); NUM_RCVMMSGS];

    let sock_fd = sock.as_raw_fd();
    let count = cmp::min(iovs.len(), packets.len());

    for (packet, hdr, iov, addr) in
        izip!(packets.iter_mut(), &mut hdrs, &mut iovs, &mut addrs).take(count)
    {
        let buffer = packet.buffer_mut();
        iov.write(iovec {
            iov_base: buffer.as_mut_ptr() as *mut libc::c_void,
            iov_len: buffer.len(),
        });

        let msg_hdr = create_msghdr(addr, SOCKADDR_STORAGE_SIZE, iov);

        hdr.write(mmsghdr {
            msg_len: 0,
            msg_hdr,
        });
    }

    let mut ts = libc::timespec {
        tv_sec: 1,
        tv_nsec: 0,
    };
    // TODO: remove .try_into().unwrap() once rust libc fixes recvmmsg types for musl
    #[allow(clippy::useless_conversion)]
    let nrecv = unsafe {
        libc::recvmmsg(
            sock_fd,
            hdrs[0].assume_init_mut(),
            count as u32,
            MSG_WAITFORONE.try_into().unwrap(),
            &mut ts,
        )
    };
    let nrecv = if nrecv < 0 {
        return Err(io::Error::last_os_error());
    } else {
        usize::try_from(nrecv).unwrap()
    };
    for (addr, hdr, pkt) in izip!(addrs, hdrs, packets.iter_mut()).take(nrecv) {
        // SAFETY: We initialized `count` elements of `hdrs` above. `count` is
        // passed to recvmmsg() as the limit of messages that can be read. So,
        // `nrevc <= count` which means we initialized this `hdr` and
        // recvmmsg() will have updated it appropriately
        let hdr_ref = unsafe { hdr.assume_init_ref() };
        // SAFETY: Similar to above, we initialized this `addr` and recvmmsg()
        // will have populated it
        let addr_ref = unsafe { addr.assume_init_ref() };
        pkt.meta_mut().size = hdr_ref.msg_len as usize;
        if let Some(addr) = cast_socket_addr(addr_ref, hdr_ref) {
            pkt.meta_mut().set_socket_addr(&addr);
        }
    }

    for (iov, addr, hdr) in izip!(&mut iovs, &mut addrs, &mut hdrs).take(count) {
        // SAFETY: We initialized `count` elements of each array above
        //
        // It may be that `packets.len() != NUM_RCVMMSGS`; thus, some elements
        // in `iovs` / `addrs` / `hdrs` may not get initialized. So, we must
        // manually drop `count` elements from each array instead of being able
        // to convert [MaybeUninit<T>] to [T] and letting `Drop` do the work
        // for us when these items go out of scope at the end of the function
        unsafe {
            iov.assume_init_drop();
            addr.assume_init_drop();
            hdr.assume_init_drop();
        }
    }

    Ok(nrecv)
}

#[cfg(test)]
mod tests {
    use {
        crate::{packet::PACKET_DATA_SIZE, recvmmsg::*},
        bovey_net_utils::{bind_to, bind_to_localhost},
        std::{
            net::{SocketAddr, UdpSocket},
            time::{Duration, Instant},
        },
    };

    type TestConfig = (UdpSocket, SocketAddr, UdpSocket, SocketAddr);

    fn test_setup_reader_sender(ip_str: &str) -> io::Result<TestConfig> {
        let sock_addr: SocketAddr = ip_str
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        let reader = bind_to(sock_addr.ip(), sock_addr.port(), /*reuseport:*/ false)?;
        let addr = reader.local_addr()?;
        let sender = bind_to(sock_addr.ip(), sock_addr.port(), /*reuseport:*/ false)?;
        let saddr = sender.local_addr()?;
        Ok((reader, addr, sender, saddr))
    }

    const TEST_NUM_MSGS: usize = 32;
    #[test]
    pub fn test_recv_mmsg_one_iter() {
        let test_one_iter = |(reader, addr, sender, saddr): TestConfig| {
            let sent = TEST_NUM_MSGS - 1;
            for _ in 0..sent {
                let data = [0; PACKET_DATA_SIZE];
                sender.send_to(&data[..], addr).unwrap();
            }

            let mut packets = vec![Packet::default(); TEST_NUM_MSGS];
            let recv = recv_mmsg(&reader, &mut packets[..]).unwrap();
            assert_eq!(sent, recv);
            for packet in packets.iter().take(recv) {
                assert_eq!(packet.meta().size, PACKET_DATA_SIZE);
                assert_eq!(packet.meta().socket_addr(), saddr);
            }
        };

        test_one_iter(test_setup_reader_sender("127.0.0.1:0").unwrap());

        match test_setup_reader_sender("::1:0") {
            Ok(config) => test_one_iter(config),
            Err(e) => warn!("Failed to configure IPv6: {:?}", e),
        }
    }

    #[test]
    pub fn test_recv_mmsg_multi_iter() {
        let test_multi_iter = |(reader, addr, sender, saddr): TestConfig| {
            let sent = TEST_NUM_MSGS + 10;
            for _ in 0..sent {
                let data = [0; PACKET_DATA_SIZE];
                sender.send_to(&data[..], addr).unwrap();
            }

            let mut packets = vec![Packet::default(); TEST_NUM_MSGS];
            let recv = recv_mmsg(&reader, &mut packets[..]).unwrap();
            assert_eq!(TEST_NUM_MSGS, recv);
            for packet in packets.iter().take(recv) {
                assert_eq!(packet.meta().size, PACKET_DATA_SIZE);
                assert_eq!(packet.meta().socket_addr(), saddr);
            }

            packets
                .iter_mut()
                .for_each(|pkt| *pkt.meta_mut() = Meta::default());
            let recv = recv_mmsg(&reader, &mut packets[..]).unwrap();
            assert_eq!(sent - TEST_NUM_MSGS, recv);
            for packet in packets.iter().take(recv) {
                assert_eq!(packet.meta().size, PACKET_DATA_SIZE);
                assert_eq!(packet.meta().socket_addr(), saddr);
            }
        };

        test_multi_iter(test_setup_reader_sender("127.0.0.1:0").unwrap());

        match test_setup_reader_sender("::1:0") {
            Ok(config) => test_multi_iter(config),
            Err(e) => warn!("Failed to configure IPv6: {:?}", e),
        }
    }

    #[test]
    pub fn test_recv_mmsg_multi_iter_timeout() {
        let reader = bind_to_localhost().expect("bind");
        let addr = reader.local_addr().unwrap();
        reader.set_read_timeout(Some(Duration::new(5, 0))).unwrap();
        reader.set_nonblocking(false).unwrap();
        let sender = bind_to_localhost().expect("bind");
        let saddr = sender.local_addr().unwrap();
        let sent = TEST_NUM_MSGS;
        for _ in 0..sent {
            let data = [0; PACKET_DATA_SIZE];
            sender.send_to(&data[..], addr).unwrap();
        }

        let start = Instant::now();
        let mut packets = vec![Packet::default(); TEST_NUM_MSGS];
        let recv = recv_mmsg(&reader, &mut packets[..]).unwrap();
        assert_eq!(TEST_NUM_MSGS, recv);
        for packet in packets.iter().take(recv) {
            assert_eq!(packet.meta().size, PACKET_DATA_SIZE);
            assert_eq!(packet.meta().socket_addr(), saddr);
        }
        reader.set_nonblocking(true).unwrap();

        packets
            .iter_mut()
            .for_each(|pkt| *pkt.meta_mut() = Meta::default());
        let _recv = recv_mmsg(&reader, &mut packets[..]);
        assert!(start.elapsed().as_secs() < 5);
    }

    #[test]
    pub fn test_recv_mmsg_multi_addrs() {
        let reader = bind_to_localhost().expect("bind");
        let addr = reader.local_addr().unwrap();

        let sender1 = bind_to_localhost().expect("bind");
        let saddr1 = sender1.local_addr().unwrap();
        let sent1 = TEST_NUM_MSGS - 1;

        let sender2 = bind_to_localhost().expect("bind");
        let saddr2 = sender2.local_addr().unwrap();
        let sent2 = TEST_NUM_MSGS + 1;

        for _ in 0..sent1 {
            let data = [0; PACKET_DATA_SIZE];
            sender1.send_to(&data[..], addr).unwrap();
        }

        for _ in 0..sent2 {
            let data = [0; PACKET_DATA_SIZE];
            sender2.send_to(&data[..], addr).unwrap();
        }

        let mut packets = vec![Packet::default(); TEST_NUM_MSGS];

        let recv = recv_mmsg(&reader, &mut packets[..]).unwrap();
        assert_eq!(TEST_NUM_MSGS, recv);
        for packet in packets.iter().take(sent1) {
            assert_eq!(packet.meta().size, PACKET_DATA_SIZE);
            assert_eq!(packet.meta().socket_addr(), saddr1);
        }
        for packet in packets.iter().skip(sent1).take(recv - sent1) {
            assert_eq!(packet.meta().size, PACKET_DATA_SIZE);
            assert_eq!(packet.meta().socket_addr(), saddr2);
        }

        packets
            .iter_mut()
            .for_each(|pkt| *pkt.meta_mut() = Meta::default());
        let recv = recv_mmsg(&reader, &mut packets[..]).unwrap();
        assert_eq!(sent1 + sent2 - TEST_NUM_MSGS, recv);
        for packet in packets.iter().take(recv) {
            assert_eq!(packet.meta().size, PACKET_DATA_SIZE);
            assert_eq!(packet.meta().socket_addr(), saddr2);
        }
    }
}
