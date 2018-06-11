use audiosnap;
use iui::prelude::*;
use iui::controls::{
    Button,
    Slider,
    VerticalBox,
    HorizontalBox,
    Spacer,
    Group};
use std::rc::Rc;
use std::cell::RefCell;
use state::{State,Status};
use std::i16;

fn process(state: &mut State) {
    state.status = Status::Processing;
    state.splits = audiosnap::split(&state.data, state.split_ceil);
    state.status = Status::Ready;
}

pub fn start() {
    // Initialize the UI library
    let ui = UI::init().expect("Couldn't initialize UI library");

    let state = Rc::new(RefCell::new(State::new()));

    // Create a window into which controls can be placed
    let mut win = Window::new(&ui, "Audiosnap", 800, 440, WindowType::HasMenubar);

    // Create a vertical layout to hold the controls
    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);

    // chart
    let mut group_vbox = VerticalBox::new(&ui);
    group_vbox.append(&ui, Spacer::new(&ui), LayoutStrategy::Compact);
    let mut group = Group::new(&ui, "Waveform");
    group.set_child(&ui, group_vbox);

    // chart and controls
    let mut hbox = HorizontalBox::new(&ui);
    hbox.set_padded(&ui, true);

    // Load sample button
    let mut button = Button::new(&ui, "Load..");
    hbox.append(&ui, button.clone(), LayoutStrategy::Stretchy);

    // Split ceil slider
    let mut slider = Slider::new(&ui, 0, i16::MAX as i64);
    hbox.append(&ui, slider.clone(), LayoutStrategy::Stretchy);

    // Put together layouts
    vbox.append(&ui, group, LayoutStrategy::Compact);
    vbox.append(&ui, hbox, LayoutStrategy::Compact);

    win.set_child(&ui, vbox);
    win.show(&ui);

    // Load button handling
    button.on_clicked(&ui, {
        let ui = ui.clone();
        let state = state.clone();
        move |_| {
            if let Some(path) = win.open_file(&ui) {
                let mut s = state.borrow_mut();
                s.file_path = path.into_os_string()
                    .into_string().unwrap_or(String::new());
                // Load file to state
                s.status = Status::Loading;
                s.data = audiosnap::load_file(&s.file_path);
                s.status = Status::Ready;
            }
        }
    });

    // Split ceiling handling
    slider.on_changed(&ui, {
        let state = state.clone();
        move |val| { state.borrow_mut().split_ceil = (val as f32) / (i16::MAX as f32); }
    });

    // Run the application
    ui.main();
}
