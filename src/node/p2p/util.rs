
/**
 * ipport(6bytes) + key(16byte)
 */
fn serialize_public_nodes(peerlist: &Vec<Arc<Peer>>, max: usize) -> Vec<u8> {
    let mut listbts = vec![];
    let mut count = 0;
    for p in peerlist {
        if !p.is_public || !p.addr.is_ipv4() {
            continue
        }
        let ipbts = match p.addr.ip() {
            IpAddr::V4(ip) => ip.octets(),
            _ => continue,
        };
        listbts.push(vec![
            ipbts.to_vec(),
            p.addr.port().to_be_bytes().to_vec(),
            p.key.to_vec(),
        ].concat());
        count+=1;
        if count >= 200 {
            break // end max
        }
    }
    listbts.concat()
}


fn parse_public_nodes(bts: &[u8]) -> Vec<(SocketAddr, PeerKey)> {
    let sn = 4 + 2 + 16; // ip port key
    let num = bts.len() / sn;
    let mut addr = Vec::with_capacity(num);
    for i in 0..num {
        let one = &bts[i*sn .. i*sn+sn];
        let ip: [u8;4] = one[0..4].try_into().unwrap();
        let port: [u8;2] = one[4..6].try_into().unwrap() ;
        let key: [u8;16] = one[6..22].try_into().unwrap() ;
        addr.push((SocketAddr::new(
            IpAddr::from(ip), 
            u16::from_be_bytes(port)
        ), key));
    }
    addr
}

