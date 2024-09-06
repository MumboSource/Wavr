#![windows_subsystem = "console"]
use std::env;

use tray_icon::{menu::{Menu, MenuEvent, MenuItemBuilder, MenuItem}, Icon, MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use winit::event_loop::EventLoop;
use winit::application::ApplicationHandler;

struct App {
    start_item: MenuItem,
    end_item: MenuItem,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        
    }

    fn window_event(
            &mut self,
            _event_loop: &winit::event_loop::ActiveEventLoop,
            _window_id: winit::window::WindowId,
            _event: winit::event::WindowEvent,
        ) {
            
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Ok(event) = MenuEvent::receiver().try_recv() {
            println!("menu event: {:?}", event);

            match &event.id.0[..] {
                "1001" => {
                    std::process::exit(0);
                }
                "1002" => {
                    println!("Folder pressed");
                }
                "1003" => {
                    self.start_item.set_enabled(false);
                    self.end_item.set_enabled(true);
                }
                "1004" => {
                    self.start_item.set_enabled(true);
                    self.end_item.set_enabled(false);
                }
                _ => {
                    println!("Unkown ctx pressed");
                }
            }
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let mut path = env::current_exe().unwrap();

    path.pop();
    path.push("assets");
    path.push("icon.ico");

    let icon: Icon = Icon::from_path(path, None).unwrap();

    let menu = Menu::new();
    let quit_item = MenuItemBuilder::new()
        .text("Quit")
        .enabled(true)
        .build();
    let folder_item = MenuItemBuilder::new()
        .text("Output Folder")
        .enabled(true)
        .build();
    let start_item = MenuItemBuilder::new()
        .text("Start")
        .enabled(true)
        .build();
    let end_item = MenuItemBuilder::new()
        .text("End")
        .enabled(false)
        .build();

    menu.append(&start_item).expect("Couldn't append menu item to menu");
    menu.append(&end_item).expect("Couldn't append menu item to menu");
    menu.append(&folder_item).expect("Couldn't append menu item to menu");
    menu.append(&quit_item).expect("Couldn't append menu item to menu");

    // Throwing away the tray variable disables the tray icon
    let _tray = TrayIconBuilder::new()
        .with_tooltip("Wavr")
        .with_icon(icon)
        .with_menu(Box::new(menu))
        .build()
        .unwrap();

    let _e_loop = event_loop.run_app::<App>(&mut App {
        start_item,
        end_item
    });
}