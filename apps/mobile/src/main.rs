use sb_audio::{AudioConfig, AudioMode};
use sb_core::{ClientHello, Platform, QualityProfile, SessionLifecycle};
use sb_transport::{run_udp_client, Endpoint, QuicTransport, Transport};

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let endpoint_arg = parse_arg(&args, "--server")
        .or_else(|| args.first().cloned())
        .unwrap_or_else(|| "127.0.0.1:7000".to_string());
    let local_bind = parse_arg(&args, "--bind").unwrap_or_else(|| "127.0.0.1:0".to_string());
    let seconds = parse_arg(&args, "--seconds")
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(20);

    let endpoint = Endpoint::parse(&endpoint_arg).expect("endpoint must be host:port");

    let mut lifecycle = SessionLifecycle::default();
    lifecycle.start_connect().expect("connect should start");

    let transport = QuicTransport;
    let hello = ClientHello {
        client_id: "mobile-local".to_string(),
        platform: Platform::Android,
        preferred_transport: "quic".to_string(),
        profile: QualityProfile::UltraLowLatency,
    };

    let server = transport
        .establish_session(&endpoint, &hello)
        .expect("mobile session establishment should succeed");
    lifecycle.mark_connected().expect("should become connected");

    let audio = AudioConfig {
        mode: AudioMode::Microphone,
        ..AudioConfig::default()
    };

    println!("SoundBridge Mobile");
    println!("Endpoint: {}:{}", endpoint.host, endpoint.port);
    println!("Session id: {}", server.session_id);
    println!("Transport: {}", server.accepted_transport);
    println!("Target buffer: {} ms", server.target_buffer_ms);
    println!(
        "Audio mode: {:?}, {} Hz, {} channels",
        audio.mode, audio.sample_rate_hz, audio.channels
    );

    println!(
        "Realtime client sending to {} for {} seconds",
        endpoint_arg, seconds
    );
    let stats = run_udp_client(&local_bind, &endpoint_arg, seconds, 20, 256)
        .expect("realtime client run should succeed");
    println!(
        "Realtime stats -> sent: {}, echoed: {}, avg_rtt_ms: {:.2}",
        stats.sent, stats.received, stats.avg_rtt_ms
    );
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
