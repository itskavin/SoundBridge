use sb_core::{QualityProfile, SessionInfo};

fn main() {
    let session = SessionInfo::new("local-dev", QualityProfile::Voice);
    println!(
        "SoundBridge session {} initialized with {:?}",
        session.id, session.profile
    );
}
