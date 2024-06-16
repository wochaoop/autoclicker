use enigo::*;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let device_state = DeviceState::new();
    let clicking = Arc::new(Mutex::new(false));
    let clicking_clone = Arc::clone(&clicking);

    // 启动点击线程
    thread::spawn(move || {
        let settings = Settings {
            windows_dw_extra_info: Some(EVENT_MARKER as usize),
            release_keys_when_dropped: true,
            ..Default::default()
        };
        let mut enigo = Enigo::new(&settings).unwrap();
        loop {
            let is_clicking = *clicking_clone.lock().unwrap();
            if is_clicking {
                enigo.button(Button::Left, Direction::Press).unwrap();
                enigo.button(Button::Middle, Direction::Press).unwrap();
                enigo.button(Button::Right, Direction::Press).unwrap();
                thread::sleep(Duration::from_millis(1));
                enigo.button(Button::Left, Direction::Release).unwrap();
                enigo.button(Button::Middle, Direction::Release).unwrap();
                enigo.button(Button::Right, Direction::Release).unwrap();
            }
        }
    });

    // 监听键盘事件
    println!("Press F8 to toggle clicking. Press ESC to exit.");
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        if keys.contains(&Keycode::F8) {
            let mut is_clicking = clicking.lock().unwrap();
            *is_clicking = !*is_clicking;
            println!("Clicking: {}", if *is_clicking { "On" } else { "Off" });
            // 防止按键重复触发
            thread::sleep(Duration::from_millis(300));
        } else if keys.contains(&Keycode::Escape) {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
}