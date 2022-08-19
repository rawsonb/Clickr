#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

static mut KEYBIND: Vec<Keycode> = vec![];
static mut CLICKING: bool = false;

use std::str::FromStr;
use std::{thread, time};

use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Enigo, MouseButton, MouseControllable};
use rand::{thread_rng, Rng};
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![bind_key, start])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn start(cps: i32, rand: i32) {
    unsafe {
        CLICKING = false;
    }
    thread::spawn(move || unsafe {
        if KEYBIND.len() != 0 {
            CLICKING = true;
        }
        println!("here");
        let mut enigo = Enigo::new();
        let mut rng = thread_rng();
        let device_state = DeviceState::new();
        let mut rng_val: i32 = rand;
        let mut rng_reset = 10;
        let mut click = false;
        while CLICKING {
            let keys: Vec<Keycode> = device_state.get_keys();
            if rng_val != 0 {
                if rng_reset > cps {
                    rng_val = rng.gen_range(rand - rand * 2..rand);
                    rng_reset = 0;
                } else {
                    rng_reset += 1
                }
            }
            click = true;
            for i in &KEYBIND {
                if !keys.contains(i) {
                    click = false;
                }
            }
            if keys == KEYBIND && click {
                enigo.mouse_click(MouseButton::Left);
                thread::sleep(time::Duration::from_millis(
                    ((1000 / (cps + rng_val)) as i64).try_into().unwrap(),
                ));
            }
        }
    });
}

#[tauri::command]
fn bind_key(key: String) -> String {
    let keys: Vec<&str> = key.split(" ").collect();
    unsafe {
        KEYBIND = vec![];
    }
    for i in &keys {
        unsafe {
            match device_query::Keycode::from_str(i) {
                Ok(k) => KEYBIND.push(k),
                _ => return ("Bad Keybind").to_string(),
            }
        }
    }
    return key;
}
