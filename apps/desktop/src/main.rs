use sb_audio::{AudioConfig, AudioMode};
use sb_core::{ClientHello, Platform, QualityProfile, SessionLifecycle};
use sb_transport::{
    run_udp_client, run_udp_echo_server, DiscoveryService, Endpoint, PeerInfo, QuicTransport,
    Transport,
};

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let mode = parse_arg(&args, "--mode").unwrap_or_else(|| "server".to_string());
    let bind = parse_arg(&args, "--bind").unwrap_or_else(|| "127.0.0.1:7000".to_string());
    let server_addr = parse_arg(&args, "--server").unwrap_or_else(|| "127.0.0.1:7000".to_string());
    let seconds = parse_arg(&args, "--seconds")
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(20);

    let mut lifecycle = SessionLifecycle::default();
    lifecycle.start_discovery().expect("discovery should start");

    let mut discovery = DiscoveryService::default();
    discovery.advertise(PeerInfo {
        id: "mobile-demo".to_string(),
        endpoint: "127.0.0.1:7001".to_string(),
        platform: "android".to_string(),
    });

    let peer = discovery
        .find_by_id("mobile-demo")
        .expect("expected demo peer");
    let endpoint = Endpoint::parse(&peer.endpoint).expect("valid peer endpoint");

    lifecycle.start_connect().expect("connect should start");

    let transport = QuicTransport;
    let hello = ClientHello {
        client_id: "desktop-local".to_string(),
        platform: Platform::Windows,
        preferred_transport: "quic".to_string(),
        profile: QualityProfile::Voice,
    };

    let server = transport
        .establish_session(&endpoint, &hello)
        .expect("session establishment should succeed");
    lifecycle.mark_connected().expect("should become connected");

    let audio = AudioConfig {
        mode: AudioMode::Speaker,
        ..AudioConfig::default()
    };

    println!("SoundBridge Desktop");
    println!("Connected peer: {} ({})", peer.id, peer.platform);
    println!("Session id: {}", server.session_id);
    println!("Transport: {}", server.accepted_transport);
    println!("Target buffer: {} ms", server.target_buffer_ms);
    println!(
        "Audio mode: {:?}, {} Hz, {} channels",
        audio.mode, audio.sample_rate_hz, audio.channels
    );

    if mode.eq_ignore_ascii_case("client") {
        println!("Mode: realtime client");
        println!(
            "Sending to {} from {} for {} seconds",
            server_addr, bind, seconds
        );
        let stats = run_udp_client(&bind, &server_addr, seconds, 20, 256)
            .expect("realtime client run should succeed");
        println!(
            "Realtime stats -> sent: {}, echoed: {}, avg_rtt_ms: {:.2}",
            stats.sent, stats.received, stats.avg_rtt_ms
        );
    } else {
        println!("Mode: realtime server");
        println!("Listening on {} for {} seconds", bind, seconds);
        let stats =
            run_udp_echo_server(&bind, seconds).expect("realtime server run should succeed");
        println!(
            "Realtime stats -> received: {}, echoed: {}",
            stats.received, stats.echoed
        );
    }
}

fn parse_arg(args: &[String], key: &str) -> Option<String> {
    args.windows(2).find_map(|window| {
        if window[0] == key {
            Some(window[1].clone())
        } else {
            None
        }
    })
}
