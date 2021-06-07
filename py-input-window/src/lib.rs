use std::{cell::Cell, collections::HashSet, u32, usize};

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use winit::{
    event::{DeviceEvent, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

trait LogExceptionAndPass {
    fn log_and_pass(&self, py: Python);
}

impl<T> LogExceptionAndPass for PyResult<T> {
    fn log_and_pass(&self, py: Python) {
        match self {
            Ok(_) => {}
            Err(err) => {
                err.print(py);
            }
        }
    }
}

/// Display the main window
#[pyfunction(pass_module)]
fn main_window(module: &PyModule, py: Python, handler: &PyAny) -> PyResult<()> {
    // let mut event_loop = EventLoop::new();
    // let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut pressed_keys = HashSet::<u32>::new();

    let foo: &PyCell<KeycodeTable> = module.getattr("_keycode_table")?.downcast()?;
    let table_ref = foo.borrow();
    let keycode_table: &KeycodeTable = &*table_ref;
    // let table = keycode_table.table;

    let handler_py = handler.into_py(py);

    py.allow_threads(move || -> PyResult<()> {
        let mut event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let loop_result: Cell<PyResult<()>> = Cell::new(Ok(()));
    
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
    
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *control_flow = ControlFlow::Exit,
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { input, .. },
                    window_id,
                } if window_id == window.id() => {
                    if let Some(code) = input.virtual_keycode {
                        let code_int = code as u32;
                        match (pressed_keys.contains(&code_int), input.state) {
                            (true, winit::event::ElementState::Pressed) => {}
                            (true, winit::event::ElementState::Released) => {
                                pressed_keys.remove(&code_int);
                                Python::with_gil(|py| {
                                    let handler = handler_py.clone().into_ref(py);
                                    handler.call1((input.scancode, keycode_table.get(code_int), false)).log_and_pass(py);
                                });
                            }
                            (false, winit::event::ElementState::Pressed) => {
                                pressed_keys.insert(code_int);
                                Python::with_gil(|py| {
                                    let handler = handler_py.clone().into_ref(py);
                                    handler.call1((input.scancode, keycode_table.get(code_int), true)).log_and_pass(py);
                                });
                            }
                            (false, winit::event::ElementState::Released) => {}
                        }
                    }
                }
                Event::MainEventsCleared => {
                    let signals_result = Python::with_gil(|py| -> PyResult<()> { py.check_signals() });
                    if let Err(err) = signals_result {
                        loop_result.set(Err(err));
                        *control_flow = ControlFlow::Exit;
                    }
                },
                Event::DeviceEvent {
                    event: DeviceEvent::Key(keyboard_input),
                    device_id: _,
                } => {
                    println!("device event {:?}", keyboard_input);
                }
                _ => (),
            }
        });
        Ok(())
    })
}

const KEYCODE_MAPPING: &[(&str, u32)] = &[
    ("Key1", VirtualKeyCode::Key1 as u32),
    ("Key2", VirtualKeyCode::Key2 as u32),
    ("Key3", VirtualKeyCode::Key3 as u32),
    ("Key4", VirtualKeyCode::Key4 as u32),
    ("Key5", VirtualKeyCode::Key5 as u32),
    ("Key6", VirtualKeyCode::Key6 as u32),
    ("Key7", VirtualKeyCode::Key7 as u32),
    ("Key8", VirtualKeyCode::Key8 as u32),
    ("Key9", VirtualKeyCode::Key9 as u32),
    ("Key0", VirtualKeyCode::Key0 as u32),
    ("A", VirtualKeyCode::A as u32),
    ("B", VirtualKeyCode::B as u32),
    ("C", VirtualKeyCode::C as u32),
    ("D", VirtualKeyCode::D as u32),
    ("E", VirtualKeyCode::E as u32),
    ("F", VirtualKeyCode::F as u32),
    ("G", VirtualKeyCode::G as u32),
    ("H", VirtualKeyCode::H as u32),
    ("I", VirtualKeyCode::I as u32),
    ("J", VirtualKeyCode::J as u32),
    ("K", VirtualKeyCode::K as u32),
    ("L", VirtualKeyCode::L as u32),
    ("M", VirtualKeyCode::M as u32),
    ("N", VirtualKeyCode::N as u32),
    ("O", VirtualKeyCode::O as u32),
    ("P", VirtualKeyCode::P as u32),
    ("Q", VirtualKeyCode::Q as u32),
    ("R", VirtualKeyCode::R as u32),
    ("S", VirtualKeyCode::S as u32),
    ("T", VirtualKeyCode::T as u32),
    ("U", VirtualKeyCode::U as u32),
    ("V", VirtualKeyCode::V as u32),
    ("W", VirtualKeyCode::W as u32),
    ("X", VirtualKeyCode::X as u32),
    ("Y", VirtualKeyCode::Y as u32),
    ("Z", VirtualKeyCode::Z as u32),
    ("Escape", VirtualKeyCode::Escape as u32),
    ("F1", VirtualKeyCode::F1 as u32),
    ("F2", VirtualKeyCode::F2 as u32),
    ("F3", VirtualKeyCode::F3 as u32),
    ("F4", VirtualKeyCode::F4 as u32),
    ("F5", VirtualKeyCode::F5 as u32),
    ("F6", VirtualKeyCode::F6 as u32),
    ("F7", VirtualKeyCode::F7 as u32),
    ("F8", VirtualKeyCode::F8 as u32),
    ("F9", VirtualKeyCode::F9 as u32),
    ("F10", VirtualKeyCode::F10 as u32),
    ("F11", VirtualKeyCode::F11 as u32),
    ("F12", VirtualKeyCode::F12 as u32),
    ("F13", VirtualKeyCode::F13 as u32),
    ("F14", VirtualKeyCode::F14 as u32),
    ("F15", VirtualKeyCode::F15 as u32),
    ("F16", VirtualKeyCode::F16 as u32),
    ("F17", VirtualKeyCode::F17 as u32),
    ("F18", VirtualKeyCode::F18 as u32),
    ("F19", VirtualKeyCode::F19 as u32),
    ("F20", VirtualKeyCode::F20 as u32),
    ("F21", VirtualKeyCode::F21 as u32),
    ("F22", VirtualKeyCode::F22 as u32),
    ("F23", VirtualKeyCode::F23 as u32),
    ("F24", VirtualKeyCode::F24 as u32),
    ("Snapshot", VirtualKeyCode::Snapshot as u32),
    ("Scroll", VirtualKeyCode::Scroll as u32),
    ("Pause", VirtualKeyCode::Pause as u32),
    ("Insert", VirtualKeyCode::Insert as u32),
    ("Home", VirtualKeyCode::Home as u32),
    ("Delete", VirtualKeyCode::Delete as u32),
    ("End", VirtualKeyCode::End as u32),
    ("PageDown", VirtualKeyCode::PageDown as u32),
    ("PageUp", VirtualKeyCode::PageUp as u32),
    ("Left", VirtualKeyCode::Left as u32),
    ("Up", VirtualKeyCode::Up as u32),
    ("Right", VirtualKeyCode::Right as u32),
    ("Down", VirtualKeyCode::Down as u32),
    ("Back", VirtualKeyCode::Back as u32),
    ("Return", VirtualKeyCode::Return as u32),
    ("Space", VirtualKeyCode::Space as u32),
    ("Compose", VirtualKeyCode::Compose as u32),
    ("Caret", VirtualKeyCode::Caret as u32),
    ("Numlock", VirtualKeyCode::Numlock as u32),
    ("Numpad0", VirtualKeyCode::Numpad0 as u32),
    ("Numpad1", VirtualKeyCode::Numpad1 as u32),
    ("Numpad2", VirtualKeyCode::Numpad2 as u32),
    ("Numpad3", VirtualKeyCode::Numpad3 as u32),
    ("Numpad4", VirtualKeyCode::Numpad4 as u32),
    ("Numpad5", VirtualKeyCode::Numpad5 as u32),
    ("Numpad6", VirtualKeyCode::Numpad6 as u32),
    ("Numpad7", VirtualKeyCode::Numpad7 as u32),
    ("Numpad8", VirtualKeyCode::Numpad8 as u32),
    ("Numpad9", VirtualKeyCode::Numpad9 as u32),
    ("NumpadAdd", VirtualKeyCode::NumpadAdd as u32),
    ("NumpadDivide", VirtualKeyCode::NumpadDivide as u32),
    ("NumpadDecimal", VirtualKeyCode::NumpadDecimal as u32),
    ("NumpadComma", VirtualKeyCode::NumpadComma as u32),
    ("NumpadEnter", VirtualKeyCode::NumpadEnter as u32),
    ("NumpadEquals", VirtualKeyCode::NumpadEquals as u32),
    ("NumpadMultiply", VirtualKeyCode::NumpadMultiply as u32),
    ("NumpadSubtract", VirtualKeyCode::NumpadSubtract as u32),
    ("AbntC1", VirtualKeyCode::AbntC1 as u32),
    ("AbntC2", VirtualKeyCode::AbntC2 as u32),
    ("Apostrophe", VirtualKeyCode::Apostrophe as u32),
    ("Apps", VirtualKeyCode::Apps as u32),
    ("Asterisk", VirtualKeyCode::Asterisk as u32),
    ("At", VirtualKeyCode::At as u32),
    ("Ax", VirtualKeyCode::Ax as u32),
    ("Backslash", VirtualKeyCode::Backslash as u32),
    ("Calculator", VirtualKeyCode::Calculator as u32),
    ("Capital", VirtualKeyCode::Capital as u32),
    ("Colon", VirtualKeyCode::Colon as u32),
    ("Comma", VirtualKeyCode::Comma as u32),
    ("Convert", VirtualKeyCode::Convert as u32),
    ("Equals", VirtualKeyCode::Equals as u32),
    ("Grave", VirtualKeyCode::Grave as u32),
    ("Kana", VirtualKeyCode::Kana as u32),
    ("Kanji", VirtualKeyCode::Kanji as u32),
    ("LAlt", VirtualKeyCode::LAlt as u32),
    ("LBracket", VirtualKeyCode::LBracket as u32),
    ("LControl", VirtualKeyCode::LControl as u32),
    ("LShift", VirtualKeyCode::LShift as u32),
    ("LWin", VirtualKeyCode::LWin as u32),
    ("Mail", VirtualKeyCode::Mail as u32),
    ("MediaSelect", VirtualKeyCode::MediaSelect as u32),
    ("MediaStop", VirtualKeyCode::MediaStop as u32),
    ("Minus", VirtualKeyCode::Minus as u32),
    ("Mute", VirtualKeyCode::Mute as u32),
    ("MyComputer", VirtualKeyCode::MyComputer as u32),
    ("NavigateForward", VirtualKeyCode::NavigateForward as u32),
    ("NavigateBackward", VirtualKeyCode::NavigateBackward as u32),
    ("NextTrack", VirtualKeyCode::NextTrack as u32),
    ("NoConvert", VirtualKeyCode::NoConvert as u32),
    ("OEM102", VirtualKeyCode::OEM102 as u32),
    ("Period", VirtualKeyCode::Period as u32),
    ("PlayPause", VirtualKeyCode::PlayPause as u32),
    ("Plus", VirtualKeyCode::Plus as u32),
    ("Power", VirtualKeyCode::Power as u32),
    ("PrevTrack", VirtualKeyCode::PrevTrack as u32),
    ("RAlt", VirtualKeyCode::RAlt as u32),
    ("RBracket", VirtualKeyCode::RBracket as u32),
    ("RControl", VirtualKeyCode::RControl as u32),
    ("RShift", VirtualKeyCode::RShift as u32),
    ("RWin", VirtualKeyCode::RWin as u32),
    ("Semicolon", VirtualKeyCode::Semicolon as u32),
    ("Slash", VirtualKeyCode::Slash as u32),
    ("Sleep", VirtualKeyCode::Sleep as u32),
    ("Stop", VirtualKeyCode::Stop as u32),
    ("Sysrq", VirtualKeyCode::Sysrq as u32),
    ("Tab", VirtualKeyCode::Tab as u32),
    ("Underline", VirtualKeyCode::Underline as u32),
    ("Unlabeled", VirtualKeyCode::Unlabeled as u32),
    ("VolumeDown", VirtualKeyCode::VolumeDown as u32),
    ("VolumeUp", VirtualKeyCode::VolumeUp as u32),
    ("Wake", VirtualKeyCode::Wake as u32),
    ("WebBack", VirtualKeyCode::WebBack as u32),
    ("WebFavorites", VirtualKeyCode::WebFavorites as u32),
    ("WebForward", VirtualKeyCode::WebForward as u32),
    ("WebHome", VirtualKeyCode::WebHome as u32),
    ("WebRefresh", VirtualKeyCode::WebRefresh as u32),
    ("WebSearch", VirtualKeyCode::WebSearch as u32),
    ("WebStop", VirtualKeyCode::WebStop as u32),
    ("Yen", VirtualKeyCode::Yen as u32),
    ("Copy", VirtualKeyCode::Copy as u32),
    ("Paste", VirtualKeyCode::Paste as u32),
    ("Cut", VirtualKeyCode::Cut as u32),
];

const KEYCODE_COUNT: usize = KEYCODE_MAPPING.len();

#[pyclass]
struct KeycodeTable {
    table: Vec<Py<PyAny>>,
}

impl KeycodeTable {
    pub fn get<'a>(&self, i: u32) -> Py<PyAny> {
        self.table.get(i as usize).unwrap().clone()
    }
}

fn define_keycode_enum(py: Python) -> PyResult<(&PyAny, &PyAny)> {
    let enum_module = PyModule::import(py, "enum")?;
    let enum_class = enum_module.getattr("IntEnum")?;

    use pyo3::types::IntoPyDict;
    let keycode_enum = enum_class.call1(("KeyCode", KEYCODE_MAPPING.into_py_dict(py)))?;

    let keycode_table = PyCell::new(py, KeycodeTable {
        table: Vec::new()
    }).unwrap();
    let table = &mut keycode_table.borrow_mut().table;
    for i in 0..KEYCODE_COUNT {
        let enum_member = keycode_enum.call1((i,))?;
        table.push(enum_member.into_py(py));
    }

    Ok((keycode_enum, &keycode_table))
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_input_window(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(main_window, m)?)?;
    let (keycode_enum, keycode_table) = define_keycode_enum(py)?;
    m.add("KeyCode", keycode_enum)?;
    m.add("_keycode_table", keycode_table)?;
    Ok(())
}
