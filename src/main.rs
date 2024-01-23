#![windows_subsystem = "windows"]

extern crate winapi;
use chrono::Local;
use lazy_static::lazy_static;
use rust_embed::RustEmbed;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::ptr;
use std::sync::Mutex;
use win_event_hook::events::{Event, NamedEvent};
use winapi::shared::minwindef::DWORD;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::GetModuleBaseNameW;
use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
use winapi::um::winuser::{GetForegroundWindow, GetWindowThreadProcessId};
use windows_volume_control::AudioController;

type HWND = *mut std::os::raw::c_void;
pub static mut WINDOW: HWND = std::ptr::null_mut();
pub static mut CONTROLLER: Option<AudioController> = None;

lazy_static! {
    static ref LAST_FOREGROUND: Mutex<String> = Mutex::new(String::new());
}

mod audio;
mod config;
mod systray;
mod ui;

#[derive(RustEmbed)]
#[folder = "src/icon"]
struct Asset;

fn get_foreground_process() -> Option<u32> {
    unsafe {
        let mut process_id = 0;
        let foreground_window = GetForegroundWindow();
        GetWindowThreadProcessId(foreground_window, &mut process_id);
        if process_id != 0 {
            Some(process_id)
        } else {
            None
        }
    }
}

fn get_foreground_name() -> Option<String> {
    unsafe {
        let process_id = get_foreground_process();
        if process_id.is_none() {
            return None;
        }
        let process_handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | winapi::um::winnt::PROCESS_VM_READ,
            winapi::shared::minwindef::FALSE,
            process_id.unwrap(),
        );

        if process_handle.is_null() {
            return None;
        }

        let mut process_name: [u16; 1024] = [0; 1024];
        let size = GetModuleBaseNameW(
            process_handle,
            ptr::null_mut(),
            process_name.as_mut_ptr(),
            process_name.len() as DWORD,
        );
        let process_name_str = OsString::from_wide(&process_name[..size as usize]);
        let mut process_name_string = process_name_str.to_string_lossy().into_owned();

        process_name_string.truncate(process_name_string.len() - 4);

        return Some(process_name_string);
    }
}

fn initialize_bg_mute() {
    let applications = config::applications();
    let foreground = get_foreground_name().unwrap_or("".to_string());
    for ele in applications {
        if ele.to_string() != foreground {
            // println!("{} muted", ele);
            audio::set_session_mute(&ele, true);
        }
    }
}

pub fn main() {
    // create our hook config
    let config = win_event_hook::Config::builder()
        .skip_own_process()
        .with_dedicated_thread()
        .with_events(vec![
            Event::Named(NamedEvent::SystemForeground),
            Event::Named(NamedEvent::SystemMinimizeEnd),
            Event::Named(NamedEvent::SystemMinimizeStart),
        ])
        .finish();

    // and our handler
    let handler = |_, _, _, _, _, _| {
        let current_time: String = Local::now().format("%H:%M:%S").to_string();
        let mut last_foreground = LAST_FOREGROUND.lock().unwrap();
        if let Some(foreground) = get_foreground_name() {
            if foreground != *last_foreground {
                if !config::exclude_explorer() || foreground != "Explorer" {
                    println!(
                        "[{}] Foreground change: {} -> {}",
                        current_time, last_foreground, foreground
                    );
                    if config::applications().contains(&last_foreground.to_string()) {
                        audio::set_session_mute(&last_foreground, true);
                    }
                    if config::applications().contains(&foreground) {
                        audio::set_session_mute(&foreground, false);
                    }
                    *last_foreground = foreground;
                }
            }
        }
    };

    // install the hook
    let _hook = win_event_hook::WinEventHook::install(config, handler).unwrap();
    initialize_bg_mute();
    ui::init();
    // config_window::init_tray();

    // loop{}
    // loop {
    //     thread::sleep(Duration::from_millis(4000));
    // }
}
