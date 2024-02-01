#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
use crate::linux::disappear;
#[cfg(target_os = "windows")]
use crate::windows::disappear;

fn main() {
    if let Err(e) = disappear() {
        eprintln!("Error : {:?}", e);
    }
}
