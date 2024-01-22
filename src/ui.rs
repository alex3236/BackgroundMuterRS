extern crate winapi;

use crate::{audio, config, systray};
use fltk::app::event_clicks;
use fltk::button::{Button, CheckButton};
use fltk::enums::Event;
use fltk::image::IcoImage;
use fltk::{app, prelude::*, *};
use fltk_theme::WidgetTheme;

fn reload_channels(t: &mut tree::Tree) {
    t.clear();
    t.add("Active sound channels");
    t.add("Listed sound channels");
    let applications = config::applications();
    for ele in audio::get_all_session_names() {
        if !applications.contains(&ele) && !(&ele == "master") {
            t.add(&("Active sound channels/".to_string() + &ele));
        }
    }
    for ele in applications {
        t.add(&("Listed sound channels/".to_string() + &ele));
    }
    t.redraw();
}

pub fn init() {
    let app = app::App::default().with_scheme(app::Scheme::Oxy);

    let theme = WidgetTheme::new(fltk_theme::ThemeType::Metro);
    theme.apply();

    let mut win = window::Window::default().with_size(390, 350);
    win.set_label("Background Muter RS");
    let bytes = crate::Asset::get("icon.ico").unwrap();

    let icon: IcoImage = IcoImage::from_data(&bytes.data).unwrap();
    win.set_icon(Some(icon));

    let mut tree = tree::Tree::default().with_size(390, 300);
    tree.set_show_collapse(false);
    tree.set_show_root(false);
    tree.handle(move |t, ev| {
        if ev != Event::Push {
            return false;
        }
        if event_clicks() {
            if let Some(selected) = t.first_selected_item() {
                if let Some(parent) = selected.parent() {
                    if parent.is_root() {
                        return false;
                    }
                    let parent_label = &mut parent.label().unwrap();
                    let label = &mut selected.label().unwrap().to_string();
                    if parent_label == "Listed sound channels" {
                        config::remove(label.to_string());
                        audio::set_session_mute(&label, false);
                    } else {
                        config::append(label.to_string());
                        if crate::get_foreground_name().unwrap_or_default() != *label {
                            audio::set_session_mute(&selected.label().unwrap(), true);
                        }
                    }
                    reload_channels(t);
                    return true;
                }
            }
            println!("Double click");
        }
        return false;
    });

    let mut reload_btn = Button::default().with_pos(10, 310).with_size(50, 20);
    reload_btn.set_label("Reload");
    reload_btn.set_callback(move |_| {
        reload_channels(&mut tree);
    });

    let mut exclude_btn = CheckButton::default().with_size(110, 20).right_of(&reload_btn, 10);
    exclude_btn.set_label("Exclude explorer");
    exclude_btn.set_checked(config::exclude_explorer());
    exclude_btn.set_callback(|b| {
        config::set_exclude_explorer(b.is_checked());
    });

    reload_btn.do_callback();
    // reload_listed(&mut list_tree);

    // let mut c_l = list_tree.clone();
    

    // list_tree.set_callback(move |t| {
    //     if let Some(item) = t.first_selected_item() {
    //         println!("{} selected", item.label().unwrap());
    //     }
    // });

    // win.show();
    // win.platform_hide();

    // system tray logic
    win.set_callback(|w| {
        // We intercept the closing of the window here
        w.platform_hide();
        // w.hide();
    });

    win.end();
    // win.show();

    use crate::systray::NativeUi;
    systray::init().expect("Failed to init Native Windows GUI");
    let _ui = systray::SystemTray::build_ui(Default::default()).expect("Failed to build UI");

    systray::dispatch_thread_events_with_callback(move || unsafe {
        if systray::FIRST_CLICKED == true {
            win.show();
            // systray::FIRST_CLICKED = false;
        }
        if win.shown() {
            crate::WINDOW = win.raw_handle();
            app.run().unwrap();
        } else {
            app::sleep(0.030);
        }
    });
}
