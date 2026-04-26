use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const HEADER_SIZE: usize = 18;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RealtimeFrame {
    pub sequence: u64,
    pub sent_millis: u64,
    pub payload: Vec<u8>,
}

impl RealtimeFrame {
    pub fn encode(&self) -> Vec<u8> {
        let payload_len = self.payload.len().min(u16::MAX as usize) as u16;
        let mut out = Vec::with_capacity(HEADER_SIZE + payload_len as usize);
        out.extend_from_slice(&self.sequence.to_be_bytes());
        out.extend_from_slice(&self.sent_millis.to_be_bytes());
        out.extend_from_slice(&payload_len.to_be_bytes());
        out.extend_from_slice(&self.payload[..payload_len as usize]);
        out
    }

    pub fn decode(raw: &[u8]) -> io::Result<Self> {
        if raw.len() < HEADER_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "packet smaller than frame header",
            ));
        }

        let sequence = u64::from_be_bytes(raw[0..8].try_into().expect("slice size checked"));
        let sent_millis = u64::from_be_bytes(raw[8..16].try_into().expect("slice size checked"));
        let payload_len = u16::from_be_bytes(raw[16..18].try_into().expect("slice size checked"));
        let expected = HEADER_SIZE + payload_len as usize;

        if raw.len() < expected {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "payload length exceeds packet size",
            ));
        }

        Ok(Self {
            sequence,
            sent_millis,
            payload: raw[HEADER_SIZE..expected].to_vec(),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct RealtimeStats {
    pub sent: u64,
    pub received: u64,
    pub echoed: u64,
    pub avg_rtt_ms: f64,
}

pub fn run_udp_echo_server(bind: &str, seconds: u64) -> io::Result<RealtimeStats> {
    let socket = UdpSocket::bind(bind)?;
    socket.set_read_timeout(Some(Duration::from_millis(250)))?;

    let deadline = Instant::now() + Duration::from_secs(seconds.max(1));
    let mut buf = [0_u8; 2048];
    let mut stats = RealtimeStats::default();

    while Instant::now() < deadline {
        match socket.recv_from(&mut buf) {
            Ok((size, from)) => {
                stats.received += 1;
                let packet = &buf[..size];
                if RealtimeFrame::decode(packet).is_ok() {
                    let _ = socket.send_to(packet, from)?;
                    stats.echoed += 1;
                }
            }
            Err(err)
                if err.kind() == io::ErrorKind::WouldBlock
                    || err.kind() == io::ErrorKind::TimedOut => {}
            Err(err) => return Err(err),
        }
    }

    Ok(stats)
}

pub fn run_udp_client(
    local_bind: &str,
    server: &str,
    seconds: u64,
    interval_ms: u64,
    payload_size: usize,
) -> io::Result<RealtimeStats> {
    let socket = UdpSocket::bind(local_bind)?;
    let server_addr: SocketAddr = server
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid server address"))?;

    socket.connect(server_addr)?;
    socket.set_read_timeout(Some(Duration::from_millis(25)))?;

    let deadline = Instant::now() + Duration::from_secs(seconds.max(1));
    let mut next_send = Instant::now();
    let send_interval = Duration::from_millis(interval_ms.max(5));

    let mut seq: u64 = 0;
    let mut stats = RealtimeStats::default();
    let mut rtt_sum = 0.0_f64;
    let mut rtt_count = 0_u64;
    let mut recv_buf = [0_u8; 2048];

    while Instant::now() < deadline {
        let now = Instant::now();
        if now >= next_send {
            let frame = RealtimeFrame {
                sequence: seq,
                sent_millis: now_millis(),
                payload: make_payload(payload_size),
            };
            let raw = frame.encode();
            socket.send(&raw)?;
            stats.sent += 1;
            seq = seq.saturating_add(1);
            next_send += send_interval;
        }

        match socket.recv(&mut recv_buf) {
            Ok(size) => {
                if let Ok(frame) = RealtimeFrame::decode(&recv_buf[..size]) {
                    stats.received += 1;
                    let rtt = now_millis().saturating_sub(frame.sent_millis);
                    rtt_sum += rtt as f64;
                    rtt_count += 1;
                }
            }
            Err(err)
                if err.kind() == io::ErrorKind::WouldBlock
                    || err.kind() == io::ErrorKind::TimedOut
                    || err.kind() == io::ErrorKind::ConnectionReset => {}
            Err(err) => return Err(err),
        }

        std::thread::sleep(Duration::from_millis(1));
    }

    if rtt_count > 0 {
        stats.avg_rtt_ms = rtt_sum / rtt_count as f64;
    }

    Ok(stats)
}

fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn make_payload(size: usize) -> Vec<u8> {
    let len = size.clamp(32, 1024);
    let mut payload = Vec::with_capacity(len);
    for i in 0..len {
        payload.push((i % 251) as u8);
    }
    payload
}

#[cfg(test)]
mod tests {
    use super::RealtimeFrame;

    #[test]
    fn frame_roundtrip_preserves_data() {
        let frame = RealtimeFrame {
            sequence: 42,
            sent_millis: 1_000,
            payload: vec![1, 2, 3, 4, 5],
        };

        let raw = frame.encode();
        let decoded = RealtimeFrame::decode(&raw).expect("decode should succeed");

        assert_eq!(decoded, frame);
    }

    #[test]
    fn decode_rejects_short_packet() {
        let err = RealtimeFrame::decode(&[1, 2, 3]).expect_err("short packet must fail");
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    }
}
