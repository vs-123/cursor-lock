use enigo::{Enigo, MouseControllable};
use inputbot::{KeybdKey::*, *};
use std::{sync::atomic::{AtomicBool, AtomicI32, Ordering}, thread, time, sync::Arc};

fn main() {
    println!("Cursor Lock");
    println!("Help: Press F6 key to activate/deactivate");

    let is_activated = Arc::new(AtomicBool::new(false));
    let is_activated_thread = Arc::clone(&is_activated);

    let mouse_x = Arc::new(AtomicI32::new(0));
    let mouse_x_thread = Arc::clone(&mouse_x);

    let mouse_y = Arc::new(AtomicI32::new(0));
    let mouse_y_thread = Arc::clone(&mouse_y);

    thread::spawn(move || {
        let mut enigo = Enigo::new();
        loop {
            if is_activated_thread.load(Ordering::Relaxed) {
                enigo.mouse_move_to(mouse_x_thread.load(Ordering::Relaxed), mouse_y_thread.load(Ordering::Relaxed));
                thread::sleep(time::Duration::from_millis(3));
            }
        }
    });

    F6Key.bind(move || {
        is_activated.store(!is_activated.load(Ordering::Relaxed), Ordering::Relaxed);

        if is_activated.load(Ordering::Relaxed) {
            println!("Cursor Lock Activated");
        } else {
            println!("Cursor Lock Deactivated");
        }

        let mouse_position = Enigo::mouse_location();
        mouse_x.store(mouse_position.0, Ordering::Relaxed);
        mouse_y.store(mouse_position.1, Ordering::Relaxed);
    });
    
    handle_input_events();
}
