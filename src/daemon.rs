use crate::hook;
use crate::settings::Settings;
use crate::transform::Transform;
use std::sync::mpsc;
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

        tokio::task::spawn_blocking(move || {
            while let Ok(event) = hook_rx.recv() {
                if tx.blocking_send(event).is_err() {
                    break;
                }
            }
        }).await.unwrap();
    }
}
