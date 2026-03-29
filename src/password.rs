use std::cell::Cell;
use std::io::Write;
use std::rc::Rc;

use gtk4 as gtk;
use gtk4::glib;
use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;

use crate::Args;

pub fn build(app: &adw::Application, args: &Args, submitted: &Rc<Cell<bool>>) {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .default_width(340)
        .resizable(false)
        .build();

    // Close on Escape
    let esc = gtk::ShortcutController::new();
    let win_ref = window.clone();
    esc.add_shortcut(gtk::Shortcut::new(
        gtk::ShortcutTrigger::parse_string("Escape"),
        Some(gtk::CallbackAction::new(move |_, _| {
            win_ref.close();
            glib::Propagation::Stop
        })),
    ));
    window.add_controller(esc);

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(14)
        .margin_top(32)
        .margin_bottom(24)
        .margin_start(28)
        .margin_end(28)
        .build();
    window.set_content(Some(&vbox));

    let icon = gtk::Image::builder()
        .icon_name(&args.icon)
        .pixel_size(48)
        .margin_bottom(4)
        .css_classes(["dim-label"])
        .build();
    vbox.append(&icon);

    let title_label = gtk::Label::builder()
        .label(&args.title)
        .css_classes(["title-4"])
        .build();
    vbox.append(&title_label);

    let msg_label = gtk::Label::builder()
        .label(&args.message)
        .css_classes(["dim-label", "body"])
        .wrap(true)
        .margin_bottom(4)
        .justify(gtk::Justification::Center)
        .build();
    vbox.append(&msg_label);

    let list_box = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .css_classes(["boxed-list"])
        .margin_bottom(4)
        .build();
    vbox.append(&list_box);

    let entry = adw::PasswordEntryRow::builder().title("Password").build();
    list_box.append(&entry);

    let btn_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(12)
        .homogeneous(true)
        .build();
    vbox.append(&btn_box);

    let cancel_btn = gtk::Button::builder()
        .label(&args.cancel_label)
        .css_classes(["pill"])
        .build();
    let win_ref = window.clone();
    cancel_btn.connect_clicked(move |_| win_ref.close());
    btn_box.append(&cancel_btn);

    let ok_btn = gtk::Button::builder()
        .label(&args.ok_label)
        .css_classes(["pill", "suggested-action"])
        .build();
    btn_box.append(&ok_btn);

    let submit = {
        let entry = entry.clone();
        let window = window.clone();
        let submitted = submitted.clone();
        move || {
            let pw: String = entry.text().into();
            if !pw.is_empty() {
                let _ = std::io::stdout().write_all(pw.as_bytes());
                let _ = std::io::stdout().flush();
                submitted.set(true);
                window.close();
            }
        }
    };

    let submit_clone = submit.clone();
    ok_btn.connect_clicked(move |_| submit_clone());
    entry.connect_entry_activated(move |_| submit());

    if args.timeout > 0 {
        let win_ref = window.clone();
        glib::timeout_add_seconds_local_once(args.timeout, move || {
            win_ref.close();
        });
    }

    window.present();

    let entry_ref = entry.clone();
    glib::idle_add_local_once(move || {
        entry_ref.grab_focus();
    });
}
