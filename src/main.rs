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
        let mut enigo = Enigo::new();
        loop {
            let is_clicking = *clicking_clone.lock().unwrap();
            if is_clicking {
                enigo.mouse_click(MouseButton::Left);
                enigo.mouse_click(MouseButton::Middle);
                enigo.mouse_click(MouseButton::Right);
                thread::sleep(Duration::from_millis(1));
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
