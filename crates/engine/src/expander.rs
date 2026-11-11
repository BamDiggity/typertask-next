use std::sync::Mutex;
use std::collections::HashMap;
use tracing::{info, error};

#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowsHookExW, UnhookWindowsHookEx, CallNextHookEx,
    HHOOK, WH_KEYBOARD_LL, KBDLLHOOKSTRUCT, WM_KEYDOWN
};
#[cfg(windows)]
use windows::Win32::Foundation::{LRESULT, WPARAM, LPARAM, HWND};
#[cfg(windows)]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT,
    KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VK_BACK, VIRTUAL_KEY
};

lazy_static::lazy_static! {
    static ref EXPANSIONS: Mutex<HashMap<String, String>> = {
        let mut m = HashMap::new();
        m.insert(";email".into(), "nicholas.jacobsen@go/src/cmd/go/testdata/mod/example.com_ambiguous_a_b_v0.0.0-empty.txt".into());
        m.insert(";sig".into(), "Best regards,\nSystems Architecture & Automation".into());
        Mutex::new(m)
    };
    static ref BUFFER: Mutex<String> = Mutex::new(String::new());
}

pub struct TextExpander;

impl TextExpander {
    pub fn new() -> Self {
        Self
    }

    pub fn start_hook(&self) {
        #[cfg(windows)]
        {
            info!("Starting Win32 WH_KEYBOARD_LL global hook for text expansion");
            std::thread::spawn(|| {
                unsafe {
                    let hook = SetWindowsHookExW(
                        WH_KEYBOARD_LL,
                        Some(keyboard_hook_proc),
                        None,
                        0,
                    );

                    if hook.is_err() {
                        error!("Failed to install WH_KEYBOARD_LL hook!");
                        return;
                    }

                    // Message loop required to keep global low-level hook alive
                    let mut msg = windows::Win32::UI::WindowsAndMessaging::MSG::default();
                    while windows::Win32::UI::WindowsAndMessaging::GetMessageW(&mut msg, HWND(std::ptr::null_mut()), 0, 0).as_bool() {
                        let _ = windows::Win32::UI::WindowsAndMessaging::TranslateMessage(&msg);
                        windows::Win32::UI::WindowsAndMessaging::DispatchMessageW(&msg);
                    }

                    let _ = UnhookWindowsHookEx(hook.unwrap());
                }
            });
        }
        #[cfg(not(windows))]
        {
            info!("Mock TextExpander hook started");
        }
    }
}

#[cfg(windows)]
unsafe extern "system" fn keyboard_hook_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code >= 0 && w_param.0 == WM_KEYDOWN as usize {
        let kb_struct = *(l_param.0 as *const KBDLLHOOKSTRUCT);
        let vk_code = kb_struct.vkCode as u8;

        // Convert virtual key to char (basic ASCII handling for abbreviations)
        if (vk_code >= b'A' && vk_code <= b'Z') || vk_code == b';' as u8 || vk_code == b'_' as u8 {
            let mut buf = BUFFER.lock().unwrap();
            let ch = if vk_code >= b'A' && vk_code <= b'Z' {
                (vk_code + 32) as char // lowercase normalize for triggers
            } else {
                vk_code as char
            };
            buf.push(ch);
            if buf.len() > 32 {
                buf.remove(0);
            }

            // Check dictionary triggers
            let expansions = EXPANSIONS.lock().unwrap();
            for (trigger, replacement) in expansions.iter() {
                if buf.ends_with(trigger) {
                    info!("Trigger matched: {}", trigger);
                    
                    // 1. Send backspaces to delete trigger text
                    let backspaces = trigger.len();
                    drop(buf); // release lock before synthetic input injection
                    
                    TextExpander::inject_backspaces(backspaces);
                    TextExpander::inject_text(replacement);

                    let mut buf_reset = BUFFER.lock().unwrap();
                    buf_reset.clear();
                    break;
                }
            }
        }
    }

    CallNextHookEx(HHOOK(std::ptr::null_mut()), n_code, w_param, l_param)
}

#[cfg(windows)]
impl TextExpander {
    unsafe fn inject_backspaces(count: usize) {
        let mut inputs = Vec::new();
        for _ in 0..count {
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_BACK,
                        wScan: 0,
                        dwFlags: KEYBD_EVENT_FLAGS(0),
                        time: 0,
                        dwExtraInfo: 0,
                    }
                }
            });
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_BACK,
                        wScan: 0,
                        dwFlags: KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    }
                }
            });
        }
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }

    unsafe fn inject_text(text: &str) {
        let mut inputs = Vec::new();
        for c in text.chars() {
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: c as u16,
                        dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYEVENTF_UNICODE,
                        time: 0,
                        dwExtraInfo: 0,
                    }
                }
            });
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: c as u16,
                        dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    }
                }
            });
        }
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}
