/*
Donald Whitehead
CS-339R-601 Fall 2023
Portfolio Project: First checkin

Rust File Manager

Main - Handles all the framework for building the UI, the callbacks between Slint and Rust,
and runs the main program. 
*/

use rustfm::{get_dir_conts, FSComponent};
use std::{path::{PathBuf, Path},env, error::Error};
use slint::Model;
use open;

// UI builder marcro
slint::slint! {
    struct FSComp {
        name: string,
        icon: image,
        highlighted: bool,
        dir: bool,
    }

    import { HorizontalBox , ListView, LineEdit, Button, StandardButton} from "std-widgets.slint";
component Entry inherits Rectangle {
        callback clicked;
        in-out property <string> name;
        in property <image> icon;
        in property <bool> highlight;
        in property <bool> dir;
        border-width: 4px;
        border-color: ta.has-hover? cyan : transparent;
        border-radius: 10px;
        background: ta.pressed ? cyan : highlight ? cyan : transparent;
        HorizontalBox {
            Rectangle {
                height: 48px;
                width: 48px;
                Image {
                    source: icon;
                    width: parent.width;
                    height: parent.height;
                }
            }
            Text { text: name; vertical-alignment: TextVerticalAlignment.center; }
        }
        ta := TouchArea {
                clicked => { root.clicked(); }
            }
    }

    component PathBox inherits LineEdit {
        in property <string> path;
        font-size: 14px;
        placeholder-text: path;
    }

    export component RustFM inherits Window {
        private property <bool> is-highlight;
        in property <[FSComp]> panel_1;
        in-out property <string> cdw;
        callback reset_highlight;
        callback go_back;
        callback go_to_path(string);
        callback open_entry(string);
        callback show_invalid_path_popup;
        title: "RustFM: " + cdw;
        preferred-height: 800px;
        preferred-width: 800px;
        VerticalLayout {
            HorizontalBox {
                alignment: start;
                PathBox {
                    width: parent.width - 60px;
                    path: "Enter Path";
                    accepted => { root.go-to-path(self.text) }
                }
                Button {
                    text: "<-";
                    clicked => { root.go-back() }
                }
            }
            GridLayout {
                spacing: 1px;
                Rectangle {
                    border-width: 1px;
                    border-color: grey;
                    ListView {
                        for data in panel_1: Entry {
                            name: data.name;
                            icon: data.icon;
                            highlight: data.highlighted;
                            dir: data.dir;
                            clicked => {
                                if (!root.is-highlight) {
                                    data.highlighted = !data.highlighted;
                                    root.is-highlight = true;
                                } else if (data.highlighted) {
                                    root.open-entry(data.name);
                                } else {
                                    root.reset-highlight();
                                    data.highlighted = !data.highlighted;
                                }
                        }}
                    }
                }
            }
        }
        show_invalid_path_popup => { invalid-path.show(); }

        invalid_path := PopupWindow {
            x: 10px;
            y: 40px;
            width: min(confirm_popup_layout.preferred-width, root.width - 80px);

            Rectangle {
                background: root.background;
                border-color: confirm_popup_text.color;
                border-width: 1px;
            }  

            confirm_popup_layout := Dialog {
                height:100%; width: 100%;

                confirm_popup_text := Text {
                    text: "The path entered was invalid.";
                    wrap: word-wrap;
                }
            }
        }
    }
}

// Converts a Vec<FSComponent> native to rust into an [FSComp] from Slint
fn build_list(fs: Vec<FSComponent>) -> Vec<FSComp> {
    let mut ui: Vec<FSComp> = Vec::new();
    for i in fs {
        let temp = FSComp {
            name: i.name.clone().into(),
            icon: i.icon,
            highlighted: false,
            dir: i.dir
        };
        ui.push(temp)
    }
    ui
}

// checks the type of selection and chooses which way to open it
fn open_selection(p: Vec<FSComp>, f: String) -> Option<Vec<FSComp>> {
    for i in p {
        if i.name == f.clone() {
            if i.dir {
                let cd = get_dir_conts(&Path::new(&f)).unwrap();
                env::set_current_dir(&Path::new(&f)).unwrap();
                return Some(build_list(cd));
            } else {
                let _ = open::that(f.clone());
                return None;
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>>{
    // Get arguments for a valid directory, if none is supplied then starts in current directory
    let args: Vec<String> = env::args().collect(); 
    let cdw = if args.len() >= 2 {
        env::set_current_dir(&Path::new(&args[1]))?;
        PathBuf::from(&args[1])
    } else {
        env::current_dir()?
    };

    // Build the initial window
    let dir_list = get_dir_conts(cdw.as_path())?;
    let rust_fm: RustFM = RustFM::new()?;
    rust_fm.set_cdw(cdw.to_str().unwrap().into());
    let panel = build_list(dir_list);
    let panel_model = std::rc::Rc::new(slint::VecModel::from(panel));
    rust_fm.set_panel_1(panel_model.into());

    // Callback closure that deselects anything selected before selecting another item
    let rust_fm_weak = rust_fm.as_weak();
    rust_fm.on_reset_highlight(move || {
        let rust_fm = rust_fm_weak.unwrap();
        let p: Vec<FSComp> = rust_fm.get_panel_1().iter().collect();
        let pm = std::rc::Rc::new(slint::VecModel::from(p));
        let entries: Vec<_> = pm.iter().enumerate().collect();
        for (idx, mut ent) in entries {
            let pm = pm.clone();
            ent.highlighted = false;
            pm.set_row_data(idx, ent);
            rust_fm.set_panel_1(pm.into());
        }
    });

    // Callback closure that opens a file or directory
    let rust_fm_weak = rust_fm.as_weak();
    rust_fm.on_open_entry(move |name| {
        let rust_fm = rust_fm_weak.unwrap();
        let p: Vec<FSComp> = rust_fm.get_panel_1().iter().collect();
        match open_selection(p, name.into()) {
            Some(x) => {
                let pm = std::rc::Rc::new(slint::VecModel::from(x));
                rust_fm.set_panel_1(pm.clone().into());
            },
            None => {}
        }
    });

    // Callback closure that goes back one directory, unless in root directory
    let rust_fm_weak = rust_fm.as_weak();
    rust_fm.on_go_back(move || {
        let rust_fm = rust_fm_weak.unwrap();
        let mut cd = env::current_dir().unwrap();
        if cd.pop() {
            let parent: Vec<FSComponent> = get_dir_conts(&cd.as_path()).unwrap();            
            let pm = std::rc::Rc::new(slint::VecModel::from(build_list(parent)));
            rust_fm.set_panel_1(pm.clone().into());
            env::set_current_dir(&Path::new(&cd)).unwrap();
        }
    });

    // Callback closure that goes back one directory, unless in root directory
    let rust_fm_weak = rust_fm.as_weak();
    rust_fm.on_go_to_path(move |p| {
        let rust_fm = rust_fm_weak.unwrap();
        let path: String = p.into();
        let path = Path::new(&path);
        if path.exists() {
            let cd: Vec<FSComponent> = get_dir_conts(&path).unwrap();            
            let pm = std::rc::Rc::new(slint::VecModel::from(build_list(cd)));
            rust_fm.set_panel_1(pm.clone().into());
            env::set_current_dir(&Path::new(&path)).unwrap();
        } else {
            rust_fm.invoke_show_invalid_path_popup();
        }
    });

    // Start the UI
    rust_fm.run()?;
    Ok(())
}
