type PeerList = Arc<StdMutex<Vec<Arc<Peer>>>>;


// #[derive(Clone)]
pub struct P2PManage {
    cnf: NodeConf,
    msghandler: Arc<MsgHandler>,
    // 
    backbones: PeerList, // 4
    offshoots: PeerList, // 200
    // close mark
    closer: Closer,
}

impl P2PManage {

    pub fn new(cnf: &NodeConf, msghl: Arc<MsgHandler>) -> P2PManage {
        P2PManage {
            cnf: cnf.clone(),
            msghandler: msghl,
            backbones: StdMutex::new(vec![]).into(),
            offshoots: StdMutex::new(vec![]).into(),
            // closech: StdMutex::new(Some(closerx)),
            closer: Closer::new(),
        }
    }

    /**
    * return: maybe drop one
    */
    fn insert(&self, peer: Arc<Peer>) -> Option<Arc<Peer>> {
        let mypid = &self.cnf.node_key;
        let mut lmax = self.cnf.offshoot_peers;
        let mut list = self.offshoots.clone();
        if peer.is_public {
            // add in backbones
            lmax = self.cnf.backbone_peers;
            list = self.backbones.clone();
        }
        let droped = insert_peer_to_dht_list(list, lmax, mypid, peer.clone());
        if droped.is_none() {
            return None // insert ok
        }
        let droped = droped.unwrap();
        if !peer.is_public || !droped.is_cntome {
            return Some(droped) // not
        }
        // second insert to offshoots
        insert_peer_to_dht_list(self.offshoots.clone(), self.cnf.offshoot_peers, mypid, droped)
    }

    fn publics(&self) -> Vec<Arc<Peer>> {
        let mut resps = vec![];
        let peers = vec![ self.backbones(), self.offshoots() ].concat();
        for p in peers {
            if p.is_public {
                resps.push(p);
            }
        }
        resps
    }

    fn backbones(&self) -> Vec<Arc<Peer>> {
        self.backbones.lock().unwrap().clone()
    }

    fn offshoots(&self) -> Vec<Arc<Peer>> {
        self.offshoots.lock().unwrap().clone()
    }

    async fn disconnect_all_peers(&self) {
        let peers = vec![ self.backbones(), self.offshoots() ].concat();
        for p in peers {
            p.disconnect().await
        }
    }

    fn print_conn_peers(&self) {
        let l1 = self.backbones.lock().unwrap().len();
        let l2 = self.offshoots.lock().unwrap().len();
        flush!("[P2P] {} public and {} subnet nodes connected.\n", l1, l2);
    }

    pub fn close(&self) {
        self.closer.close();
    }

}

