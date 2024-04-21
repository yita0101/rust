use tokio::time::*;

pub fn secs(s: u64) -> Duration {
    Duration::from_secs(s)
}

pub async fn asleep(t: u64) {
    tokio::time::sleep(secs(t)).await;
}

pub async fn new_ticker(inst: u64, dura: u64) -> Interval {
    let mut intv = interval_at(
        Instant::now() + Duration::from_secs(inst),
        Duration::from_secs(dura),
    );
    intv.set_missed_tick_behavior(MissedTickBehavior::Delay);
    intv
}

