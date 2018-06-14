use audiosnap;
use ui_sys::{
    uiControl,
    uiArea,
    uiAreaHandler,
    uiDrawContext,
    uiAreaDrawParams,
    uiAreaMouseEvent,
    uiAreaKeyEvent,
    uiNewArea,
};
use iui::prelude::*;
use iui::draw::{
    FillMode,
    Brush,
    SolidBrush,
    DrawContext,
    Path
};
use iui::controls::{
    Control,
    Button,
    Slider,
    VerticalBox,
    HorizontalBox,
    Spacer,
    Group};
use std::rc::Rc;
use std::cell::RefCell;
use std::os::raw::c_int;
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
    let area = unsafe{uiNewArea(newAreaHandler().as_ptr())};
    let draw_context = unsafe{DrawContext::from_ui_draw_context(area as *mut uiDrawContext)};

    let c_area = unsafe{Control::from_ui_control(area as *mut uiControl)};
    let mut group_vbox = VerticalBox::new(&ui);
    group_vbox.append(&ui, c_area, LayoutStrategy::Compact);
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
                s.status = Status::Processing;
                draw_context.fill(
                    &ui,
                    &data_path(&ui,&s.data),
                    &Brush::Solid(
                        SolidBrush{
                            r: 0.0,
                            g: 0.0,
                            b: 1.0,
                            a: 1.0,
                        })
                    );
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

fn newAreaHandler() -> RefCell<uiAreaHandler> {
    RefCell::new(uiAreaHandler {
        Draw: handler_draw,
        MouseEvent: handler_mouse_event,
        MouseCrossed: handler_mouse_crossed,
        DragBroken: handler_drag_broken,
        KeyEvent: handler_key_event 
    })
}

extern "C" 
fn handler_draw (this: *mut uiAreaHandler,
                    area: *mut uiArea,
                    draw_params: *mut uiAreaDrawParams) {
    // nothing
}

extern "C" 
fn handler_mouse_event(this: *mut uiAreaHandler,
                                  area: *mut uiArea,
                                  mouse_event: *mut uiAreaMouseEvent){
    // nothing
}

extern "C" 
fn handler_mouse_crossed(this: *mut uiAreaHandler,
                                  area: *mut uiArea,
                                  left: c_int){
}

extern "C" 
fn handler_drag_broken(this: *mut uiAreaHandler,
                                  area: *mut uiArea){
}

extern "C" 
fn handler_key_event(this: *mut uiAreaHandler,
                  area: *mut uiArea,
                  key_event: *mut uiAreaKeyEvent) -> c_int {
    unsafe {(*key_event).Up}
}

fn data_path(ui: &UI, data: &Vec<i16>) -> Path {
    let mut p = Path::new(ui, FillMode::Winding);
    p.end(ui);
    p
}
