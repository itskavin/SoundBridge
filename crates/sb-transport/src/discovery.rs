use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerInfo {
    pub id: String,
    pub endpoint: String,
    pub platform: String,
}

#[derive(Debug, Default)]
pub struct DiscoveryService {
    peers: BTreeMap<String, PeerInfo>,
}

impl DiscoveryService {
    pub fn advertise(&mut self, peer: PeerInfo) {
        self.peers.insert(peer.id.clone(), peer);
    }

    pub fn list(&self) -> Vec<PeerInfo> {
        self.peers.values().cloned().collect()
    }

    pub fn find_by_id(&self, id: &str) -> Option<PeerInfo> {
        self.peers.get(id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::{DiscoveryService, PeerInfo};

    #[test]
    fn discovery_returns_advertised_peers() {
        let mut svc = DiscoveryService::default();
        svc.advertise(PeerInfo {
            id: "desktop-1".to_string(),
            endpoint: "127.0.0.1:7000".to_string(),
            platform: "windows".to_string(),
        });

        let peers = svc.list();
        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0].id, "desktop-1");
    }

    #[test]
    fn discovery_find_returns_expected_peer() {
        let mut svc = DiscoveryService::default();
        svc.advertise(PeerInfo {
            id: "mobile-1".to_string(),
            endpoint: "127.0.0.1:7001".to_string(),
            platform: "android".to_string(),
        });

        let peer = svc.find_by_id("mobile-1").expect("peer should exist");
        assert_eq!(peer.endpoint, "127.0.0.1:7001");
    }
}
