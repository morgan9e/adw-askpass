mod notification;
mod password;

use std::cell::Cell;
use std::env;
use std::process::ExitCode;
use std::rc::Rc;

use gtk4::prelude::*;
use libadwaita as adw;

enum Mode {
    Password,
    Notification,
}

pub struct Args {
    mode: Mode,
    title: String,
    message: String,
    icon: String,
    ok_label: String,
    cancel_label: String,
    timeout: u32,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            mode: Mode::Password,
            title: "Authentication Required".into(),
            message: "Enter your password to continue".into(),
            icon: "dialog-password-symbolic".into(),
            ok_label: "Unlock".into(),
            cancel_label: "Cancel".into(),
            timeout: 0,
        }
    }
}

impl Args {
    fn parse() -> Self {
        let mut args = Args::default();
        let raw: Vec<String> = env::args().skip(1).collect();
        let mut i = 0;

        while i < raw.len() {
            let a = &raw[i];

            if let Some((key, val)) = a.split_once('=') {
                match key {
                    "--title" => args.title = val.to_string(),
                    "--text" | "--message" => args.message = val.to_string(),
                    "--icon" | "--window-icon" => args.icon = val.to_string(),
                    "--ok-label" => args.ok_label = val.to_string(),
                    "--cancel-label" => args.cancel_label = val.to_string(),
                    "--timeout" => args.timeout = val.parse().unwrap_or(0),
                    _ => {}
                }
                i += 1;
                continue;
            }

            match a.as_str() {
                "--password" | "--modal" => {
                    i += 1;
                    continue;
                }
                "--notification" => {
                    args.mode = Mode::Notification;
                    if args.icon == "dialog-password-symbolic" {
                        args.icon = "dialog-information-symbolic".into();
                    }
                    i += 1;
                    continue;
                }
                _ => {}
            }

            let next = raw.get(i + 1).map(|s| s.as_str());
            let consumed = match a.as_str() {
                "--title" => {
                    if let Some(v) = next { args.title = v.to_string(); true } else { false }
                }
                "--text" | "--message" => {
                    if let Some(v) = next { args.message = v.to_string(); true } else { false }
                }
                "--icon" | "--window-icon" => {
                    if let Some(v) = next { args.icon = v.to_string(); true } else { false }
                }
                "--ok-label" => {
                    if let Some(v) = next { args.ok_label = v.to_string(); true } else { false }
                }
                "--cancel-label" => {
                    if let Some(v) = next { args.cancel_label = v.to_string(); true } else { false }
                }
                "--timeout" => {
                    if let Some(v) = next { args.timeout = v.parse().unwrap_or(0); true } else { false }
                }
                _ => false,
            };

            if !consumed && !a.starts_with('-') {
                args.message = a.to_string();
            }

            i += if consumed { 2 } else { 1 };
        }

        args
    }
}

fn main() -> ExitCode {
    let args = Args::parse();
    let submitted = Rc::new(Cell::new(false));

    let app = adw::Application::new(None, gtk4::gio::ApplicationFlags::default());

    match args.mode {
        Mode::Notification => {
            notification::send(&args);
            ExitCode::SUCCESS
        }
        Mode::Password => {
            let submitted_clone = submitted.clone();
            app.connect_activate(move |app| {
                password::build(app, &args, &submitted_clone);
            });
            app.run_with_args::<&str>(&[]);
            if submitted.get() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
    }
}
