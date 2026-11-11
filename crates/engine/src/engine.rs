use crate::parser::Command;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("Execution error: {0}")]
    Execution(String),
}

pub struct Engine;

impl Engine {
    pub async fn execute(commands: &[Command]) -> Result<usize, EngineError> {
        let mut executed = 0;
        let mut typing_delay = 10; // default 10ms per char

        for cmd in commands {
            match cmd {
                Command::Delay(ms) => {
                    sleep(Duration::from_millis(*ms)).await;
                }
                Command::Speed(ms) => {
                    typing_delay = *ms;
                }
                Command::Move { x, y } => {
                    #[cfg(windows)]
                    unsafe {
                        let _ = windows::Win32::UI::WindowsAndMessaging::SetCursorPos(*x, *y);
                    }
                    #[cfg(not(windows))]
                    {
                        tracing::info!("Mock Move to ({}, {})", x, y);
                    }
                }
                Command::Click { x, y } => {
                    #[cfg(windows)]
                    unsafe {
                        if let (Some(px), Some(py)) = (x, y) {
                            let _ = windows::Win32::UI::WindowsAndMessaging::SetCursorPos(*px, *py);
                        }
                        use windows::Win32::UI::Input::KeyboardAndMouse::*;
                        let inputs = [
                            INPUT {
                                r#type: INPUT_MOUSE,
                                Anonymous: INPUT_0 {
                                    mi: MOUSEINPUT {
                                        dx: 0,
                                        dy: 0,
                                        mouseData: 0,
                                        dwFlags: MOUSEEVENTF_LEFTDOWN,
                                        time: 0,
                                        dwExtraInfo: 0,
                                    }
                                },
                            },
                            INPUT {
                                r#type: INPUT_MOUSE,
                                Anonymous: INPUT_0 {
                                    mi: MOUSEINPUT {
                                        dx: 0,
                                        dy: 0,
                                        mouseData: 0,
                                        dwFlags: MOUSEEVENTF_LEFTUP,
                                        time: 0,
                                        dwExtraInfo: 0,
                                    }
                                },
                            },
                        ];
                        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
                    }
                    #[cfg(not(windows))]
                    {
                        tracing::info!("Mock Click at {:?}", (x, y));
                    }
                }
                Command::Type(text) => {
                    #[cfg(windows)]
                    {
                        for c in text.chars() {
                            let inputs = [
                                windows::Win32::UI::Input::KeyboardAndMouse::INPUT {
                                    r#type: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_KEYBOARD,
                                    Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                                        ki: windows::Win32::UI::Input::KeyboardAndMouse::KEYBDINPUT {
                                            wVk: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY(0),
                                            wScan: c as u16,
                                            dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYEVENTF_UNICODE,
                                            time: 0,
                                            dwExtraInfo: 0,
                                        }
                                    },
                                },
                                windows::Win32::UI::Input::KeyboardAndMouse::INPUT {
                                    r#type: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_KEYBOARD,
                                    Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                                        ki: windows::Win32::UI::Input::KeyboardAndMouse::KEYBDINPUT {
                                            wVk: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY(0),
                                            wScan: c as u16,
                                            dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYEVENTF_UNICODE | windows::Win32::UI::Input::KeyboardAndMouse::KEYEVENTF_KEYUP,
                                            time: 0,
                                            dwExtraInfo: 0,
                                        }
                                    },
                                },
                            ];
                            unsafe {
                                windows::Win32::UI::Input::KeyboardAndMouse::SendInput(&inputs, std::mem::size_of::<windows::Win32::UI::Input::KeyboardAndMouse::INPUT>() as i32);
                            }
                            if typing_delay > 0 {
                                sleep(Duration::from_millis(typing_delay)).await;
                            }
                        }
                    }
                    #[cfg(not(windows))]
                    {
                        tracing::info!("Mock Type: {} (delay: {}ms)", text, typing_delay);
                    }
                }
                Command::Key(code) => {
                    tracing::info!("Key command: {}", code);
                }
                Command::Repeat { count, commands: sub_cmds } => {
                    for _ in 0..*count {
                        Box::pin(Self::execute(sub_cmds)).await?;
                    }
                }
            }
            executed += 1;
        }
        Ok(executed)
    }
}
