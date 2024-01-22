use windows_volume_control::{AudioController, CoinitMode};

pub static mut CONTROLLER: Option<AudioController> = None;

fn initialize() -> &'static mut AudioController {
    unsafe {
        let controller = CONTROLLER.as_mut().unwrap_or_else(|| {
            CONTROLLER = Some(AudioController::init(Some(CoinitMode::ApartmentThreaded)));
            return CONTROLLER.as_mut().unwrap();
        });
        controller.GetSessions();
        controller.GetDefaultAudioEnpointVolumeControl();
        controller.GetAllProcessSessions();
        return controller;
    }
}

pub fn get_all_session_names() -> Vec<String> {
    unsafe {
        return initialize().get_all_session_names();
    }
}

pub fn get_session_mute(name: &str) -> bool {
    unsafe {
        if let Some(session) = initialize().get_session_by_name(name.to_string()) {
            return session.getMute() || session.getVolume() == 0.0;
        }
        return false;
    }
}

pub fn set_session_mute(name: &str, mute: bool) -> bool {
    unsafe {
        if let Some(session) = initialize().get_session_by_name(name.to_string()) {
            if get_session_mute(name) != mute {
                session.setMute(mute);
            }
            return true;
        }
        return false;
    }
}
