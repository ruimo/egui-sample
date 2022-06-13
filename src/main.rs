use eframe::egui::{self, Event};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for e in ctx.input().events.iter() {
            match *e {
                Event::PointerButton { pos: _, button: _, pressed, modifiers } => {
                    if pressed {
                        if modifiers.shift {
                            println!("shift on");
                        } else {
                            println!("shift off");
                        }
                    }
                },
                _ => {},
            }
        }
    }
}
