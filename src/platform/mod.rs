#[cfg(target_os = "linux")]
mod linux;
#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::transform_selection;
#[cfg(windows)]
pub use windows::run_daemon;

#[cfg(not(any(target_os = "linux", windows)))]
compile_error!("txtfrmt supports Linux/Wayland and Windows targets");
