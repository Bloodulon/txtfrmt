use crate::hook;
use crate::settings::Settings;
use crate::transform::Transform;
use std::sync::mpsc;
use std::time::Duration;
use tokio::sync::mpsc as tokio_mpsc;

#[derive(Debug, Clone)]
pub enum DaemonEvent {
    Transform(Transform),
    OpenSettings,
}

pub struct Daemon;

impl Daemon {
    pub async fn run(settings: &Settings, tx: tokio_mpsc::Sender<DaemonEvent>) {
        let (hook_tx, hook_rx) = mpsc::channel::<DaemonEvent>();
        let settings = settings.clone();

        std::thread::spawn(move || {
            hook::run_keyboard_hook(&settings, hook_tx);
        });

        loop {
            if let Ok(event) = hook_rx.try_recv() {
                if tx.send(event).await.is_err() {
                    break;
                }
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
}
