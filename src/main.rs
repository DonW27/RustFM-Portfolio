/*
Donald Whitehead
CS-339R-601 Fall 2023
Portfolio Project

Rust File Manager

Main - Handles all the framework for building the UI, the callbacks between Slint and Rust,
and runs the main program.

icons provided by By GNOME Project, CC BY-SA 3.0 us, https://commons.wikimedia.org/w/index.php?curid=4339610
*/
use rustfm::{get_dir_conts, FSComponent};
use slint::{Image, Model};
use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

// UI builder marcro
slint::slint! {
    struct FSComp {
        name: string,
        icon: image,
        highlighted: bool,
        dir: bool,
    }

    import { HorizontalBox , ListView, LineEdit, Button, StandardButton} from "std-widgets.slint";
// the UI element for an entry in the file browser
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

    // UI element for the path box
    component PathBox inherits LineEdit {
        in property <string> path;
        font-size: 14px;
        placeholder-text: path;
    }

    // UI element for any dialot text input
    component InputBox inherits LineEdit {
        in property <string> rename;
        font-size: 14px;
        placeholder-text: rename;
    }

    // UI element for the root window
    export component RustFM inherits Window {

        // root variables
        private property <bool> is-highlight;
        in property <[FSComp]> panel_1;
        in property <image> rename;
        in property <image> back;
        in property <image> copy;
        in property <image> move;
        in property <image> ndir;
        in property <image> del;
        in-out property <string> cdw;

        // root functions
        callback reset_highlight;
        callback go_back;
        callback go_to_path(string);
        callback open_entry;
        callback rename_entry(string);
        callback copy_entry;
        callback move_entry(string);
        callback new_dir(string);
        callback delete_entry;
        callback show_invalid_path_popup;
        callback show_delete_diag_popup;
        callback show_rename_popup;
        show_invalid_path_popup => { invalid-path.show(); }

        // root parameters
        title: "RustFM: " + cdw;
        preferred-height: 800px;
        preferred-width: 800px;
        VerticalLayout {
            // top UI element
            HorizontalBox {
                alignment: start;
                PathBox {
                    width: parent.width - 65px;
                    path: "Enter Path";
                    accepted => { root.go-to-path(self.text) }
                }
                Button {
                    icon: back;
                    clicked => { root.go-back() }
                }
            }
            // main browser window
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
                                    root.open-entry();
                                } else {
                                    root.reset-highlight();
                                    data.highlighted = !data.highlighted;
                                }
                        }}
                    }
                }
            }
            // bottom UI element
            HorizontalBox {
                alignment: start;
                Button {
                    icon: rename;
                    clicked => { rename-dialog.show() }
                }
                Button {
                    icon: copy;
                    clicked => { root.copy_entry() }
                }
                Button {
                    icon: move;
                    clicked => { move-dialog.show() }
                }
                Button {
                    icon: ndir;
                    clicked => { new-dir-dialog.show() }
                }
                Button {
                    icon: del;
                    clicked => { delete-diag.show() }
                }
            }
        }

        // Invalid Path message - happens when entering an invalid path into the path field
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

        // Dialog popup for prompting file deletion
        delete_diag := PopupWindow {
            x: 10px;
            y: 660px;
            width: min(confirm_delete_layout.preferred-width, root.width - 80px);

            Rectangle {
                background: root.background;
                border-color: confirm_delete_text.color;
                border-width: 1px;
            }

            confirm_delete_layout := Dialog {
                height:100%; width: 100%;

                confirm_delete_text := Text {
                    text: "Are you sure you want to delete this file?";
                    wrap: word-wrap;
                }
                Button {
                    text: "Delete";
                    dialog-button-role: action;
                    clicked => { root.delete-entry(); }
                }
                StandardButton { kind: cancel; }
            }
        }

        // Dialog popup for prompting a entry rename
        rename_dialog := PopupWindow {
            x: 10px;
            y: 600px;
            width: root.width - 400px;
            height: confirm-rename-layout.preferred-height + 100px;
            close-on-click: false;

            Rectangle {
                background: root.background;
                border-color: confirm_rename_text.color;
                border-width: 1px;
            }

            InputBox {
                width: parent.width - 65px;
                rename: "Enter New Name";
                accepted => {
                    root.rename-entry(self.text);
                    rename-dialog.close();
                }
            }
            confirm_rename_layout := Dialog {
                height:100%; width: 100%;

                confirm_rename_text := Text {
                    text: "New file name:";
                    wrap: word-wrap;
                }
                Button {
                    text: "cancel";
                    height: 25px;
                    dialog-button-role: action;
                    clicked => { rename-dialog.close(); }
                }
            }
        }

        // Dialog popup for prompting an entry move
        move_dialog := PopupWindow {
            x: 10px;
            y: 600px;
            width: root.width - 400px;
            height: confirm-move-layout.preferred-height + 100px;
            close-on-click: false;

            Rectangle {
                background: root.background;
                border-color: confirm_move_text.color;
                border-width: 1px;
            }

            InputBox {
                width: parent.width - 65px;
                rename: "Enter Destination Path";
                accepted => {
                    root.move-entry(self.text);
                    rename-dialog.close();
                }
            }
            confirm_move_layout := Dialog {
                height:100%; width: 100%;

                confirm_move_text := Text {
                    text: "Enter path of destination folder:";
                    wrap: word-wrap;
                }
                Button {
                    text: "cancel";
                    height: 25px;
                    dialog-button-role: action;
                    clicked => { rename-dialog.close(); }
                }
            }
        }

        // Dialog popup for prompting a new directory
        new_dir_dialog := PopupWindow {
            x: 10px;
            y: 600px;
            width: root.width - 400px;
            height: confirm-new-dir-layout.preferred-height + 100px;
            close-on-click: false;

            Rectangle {
                background: root.background;
                border-color: confirm_new_dir_text.color;
                border-width: 1px;
            }

            InputBox {
                width: parent.width - 65px;
                rename: "Enter New Directory";
                accepted => {
                    root.new_dir(self.text);
                    new-dir-dialog.close();
                }
            }
            confirm_new_dir_layout := Dialog {
                height:100%; width: 100%;

                confirm_new_dir_text := Text {
                    text: "Enter path of destination folder:";
                    wrap: word-wrap;
                }
                Button {
                    text: "cancel";
                    height: 25px;
                    dialog-button-role: action;
                    clicked => { new-dir-dialog.close(); }
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
            dir: i.dir,
        };
        ui.push(temp)
    }
    ui
}

// checks the type of selection and chooses which way to open it
fn open_selection(p: Vec<FSComp>) -> Option<Vec<FSComp>> {
    for i in p {
        if i.highlighted {
            if i.dir {
                let cd = get_dir_conts(Path::new(i.name.as_str())).unwrap();
                env::set_current_dir(Path::new(i.name.as_str())).unwrap();
                return Some(build_list(cd));
            } else {
                let _ = open::that(i.name.as_str());
                return None;
            }
        }
    }
    None
}

// renames a selected file or directory
fn rename_selection(p: Vec<FSComp>, s: String) -> Option<Vec<FSComp>> {
    for i in p {
        if i.highlighted {
            let file = Path::new(i.name.as_str());
            fs::rename(file, s).expect("Rename Failed somehow!");
            let cd = get_dir_conts(&env::current_dir().unwrap()).unwrap();
            return Some(build_list(cd));
        }
    }
    None
}

// copies a selected file, placing it in the same directory. Directories aren't supported.
fn copy_selection(p: Vec<FSComp>) -> Option<Vec<FSComp>> {
    for i in p {
        if i.highlighted {
            if i.dir {
                return None;
            } else {
                let file = Path::new(i.name.as_str());
                let mut cp = i.name.as_str().to_string();
                // allows to make multipule copies
                while Path::new(&cp).exists() {
                    cp = format!("copy_{}", cp);
                }
                fs::copy(file, cp).expect("Copy Failed somehow!");
                let cd = get_dir_conts(&env::current_dir().unwrap()).unwrap();
                return Some(build_list(cd));
            }
        }
    }
    None
}

// moves a selected file or directory to another location, only works on the same mounting point
// if path does not exist, I'd of liked an error but Slint doesn't seem to want to allow multipule popups
fn move_selection(p: Vec<FSComp>, s: String) -> Option<Vec<FSComp>> {
    for i in p {
        if i.highlighted {
            let file = Path::new(i.name.as_str());
            let dest = Path::new(&s);
            if dest.exists() {
                let fp = s + i.name.as_str();
                fs::rename(file, fp).expect("Move Failed somehow!");
                let cd = get_dir_conts(&env::current_dir().unwrap()).unwrap();
                return Some(build_list(cd));
            }
        }
    }
    None
}

// creates a new directory in the current folder with specified name
// if already exists, I'd of liked a more elegant error but Slint doesn't seem to want to allow multipule popups
fn new_directory(s: String) -> Option<Vec<FSComp>> {
    let dest = Path::new(&s);
    if !dest.exists() {
        fs::create_dir(s).expect("new directory failed somehow!");
        let cd = get_dir_conts(&env::current_dir().unwrap()).unwrap();
        return Some(build_list(cd));
    }
    None
}

// deletes a selected file. Directories are not supported for good reason,
// I don't want someone removing half of their hard drive..
fn delete_selection(p: Vec<FSComp>) -> Option<Vec<FSComp>> {
    for i in p {
        if i.highlighted {
            if i.dir {
                return None;
            } else {
                let file = &Path::new(i.name.as_str());
                fs::remove_file(file).expect("File cannot be deleted!");
                let cd = get_dir_conts(&env::current_dir().unwrap()).unwrap();
                return Some(build_list(cd));
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get arguments for a valid directory, if none is supplied then starts in current directory
    let args: Vec<String> = env::args().collect();
    let cdw = if args.len() >= 2 {
        env::set_current_dir(Path::new(&args[1]))?;
        PathBuf::from(&args[1])
    } else {
        env::current_dir()?
    };

    // Build the initial window
    let dir_list = get_dir_conts(cdw.as_path())?;
    let rust_fm: RustFM = RustFM::new()?;
    rust_fm.set_cdw(cdw.to_str().unwrap().into());
    rust_fm.set_rename(Image::load_from_path(Path::new("icons/rename.png")).unwrap());
    rust_fm.set_back(Image::load_from_path(Path::new("icons/previous.png")).unwrap());
    rust_fm.set_copy(Image::load_from_path(Path::new("icons/copy.png")).unwrap());
    rust_fm.set_move(Image::load_from_path(Path::new("icons/move.png")).unwrap());
    rust_fm.set_ndir(Image::load_from_path(Path::new("icons/folder-new.png")).unwrap());
    rust_fm.set_del(Image::load_from_path(Path::new("icons/delete.png")).unwrap());
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
    rust_fm.on_open_entry(move || {
        let rust_fm = rust_fm_weak.unwrap();
        let p: Vec<FSComp> = rust_fm.get_panel_1().iter().collect();

        if let Some(x) = open_selection(p) {
            let pm = std::rc::Rc::new(slint::VecModel::from(x));
            rust_fm.set_panel_1(pm.clone().into());
        }
    });

    // Callback closure that goes back one directory, unless in root directory
    let rust_fm_weak = rust_fm.as_weak();
    rust_fm.on_go_back(move || {
        let rust_fm = rust_fm_weak.unwrap();
        let mut cd = env::current_dir().unwrap();
        if cd.pop() {
            let parent: Vec<FSComponent> = get_dir_conts(cd.as_path()).unwrap();
            let pm = std::rc::Rc::new(slint::VecModel::from(build_list(parent)));
            rust_fm.set_panel_1(pm.clone().into());
            env::set_current_dir(Path::new(&cd)).unwrap();
        }
    });

    // Callback closure that goes to the directory specified in path LineEdit, gives error dialog if invalid path
    let rust_fm_weak = rust_fm.as_weak();
    rust_fm.on_go_to_path(move |p| {
        let rust_fm = rust_fm_weak.unwrap();
        let path: String = p.into();
        let path = Path::new(&path);
        if path.exists() {
            let cd: Vec<FSComponent> = get_dir_conts(path).unwrap();
            let pm = std::rc::Rc::new(slint::VecModel::from(build_list(cd)));
            rust_fm.set_panel_1(pm.clone().into());
            env::set_current_dir(Path::new(&path)).unwrap();
        } else {
            rust_fm.invoke_show_invalid_path_popup();
        }
    });

    // Callback closure that queries if user for a new name of a file
    let rust_fm_weak = rust_fm.as_weak();
    rust_fm.on_rename_entry(move |s| {
        let rust_fm = rust_fm_weak.unwrap();
        let p: Vec<FSComp> = rust_fm.get_panel_1().iter().collect();
        if let Some(x) = rename_selection(p, s.into()) {
            let pm = std::rc::Rc::new(slint::VecModel::from(x));
            rust_fm.set_panel_1(pm.clone().into());
        }
    });

    // Callback closure that initiates a copy file on the selected file
    let rust_fm_weak = rust_fm.as_weak();
    rust_fm.on_copy_entry(move || {
        let rust_fm = rust_fm_weak.unwrap();
        let p: Vec<FSComp> = rust_fm.get_panel_1().iter().collect();
        if let Some(x) = copy_selection(p) {
            let pm = std::rc::Rc::new(slint::VecModel::from(x));
            rust_fm.set_panel_1(pm.clone().into());
        }
    });

    // Callback closure that queries if user for a path to move a selected file
    let rust_fm_weak = rust_fm.as_weak();
    rust_fm.on_move_entry(move |s| {
        let rust_fm = rust_fm_weak.unwrap();
        let p: Vec<FSComp> = rust_fm.get_panel_1().iter().collect();
        if let Some(x) = move_selection(p, s.into()) {
            let pm = std::rc::Rc::new(slint::VecModel::from(x));
            rust_fm.set_panel_1(pm.clone().into());
        }
    });

    // Callback closure that queries the user for the name of a new directory
    let rust_fm_weak = rust_fm.as_weak();
    rust_fm.on_new_dir(move |s| {
        let rust_fm = rust_fm_weak.unwrap();
        if let Some(x) = new_directory(s.into()) {
            let pm = std::rc::Rc::new(slint::VecModel::from(x));
            rust_fm.set_panel_1(pm.clone().into());
        }
    });

    // Callback closure that queries if user wants to delete a file before either deleting or canceling the operation
    let rust_fm_weak = rust_fm.as_weak();
    rust_fm.on_delete_entry(move || {
        let rust_fm = rust_fm_weak.unwrap();
        let p: Vec<FSComp> = rust_fm.get_panel_1().iter().collect();
        if let Some(x) = delete_selection(p) {
            let pm = std::rc::Rc::new(slint::VecModel::from(x));
            rust_fm.set_panel_1(pm.clone().into());
        }
    });

    // Start the UI
    rust_fm.run()?;
    Ok(())
}
