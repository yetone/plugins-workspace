// Copyright 2021 Jonas Kruckenberg
// SPDX-License-Identifier: MIT

#[cfg(feature = "system-tray")]
use crate::Tray;
use serde_repr::Deserialize_repr;
#[cfg(feature = "system-tray")]
use tauri::Manager;
use tauri::{PhysicalPosition, PhysicalSize, Result, Runtime, Window, Monitor};

/// Well known window positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize_repr)]
#[repr(u16)]
pub enum Position {
    TopLeft = 0,
    TopRight,
    BottomLeft,
    BottomRight,
    TopCenter,
    BottomCenter,
    LeftCenter,
    RightCenter,
    Center,
    #[cfg(feature = "system-tray")]
    TrayLeft,
    #[cfg(feature = "system-tray")]
    TrayBottomLeft,
    #[cfg(feature = "system-tray")]
    TrayRight,
    #[cfg(feature = "system-tray")]
    TrayBottomRight,
    #[cfg(feature = "system-tray")]
    TrayCenter,
    #[cfg(feature = "system-tray")]
    TrayBottomCenter,
}

/// A [`Window`] extension that provides extra methods related to positioning.
pub trait WindowExt {
    /// Moves the [`Window`] to the given [`Position`] relative to the **current** [`Monitor`]
    ///
    /// # Panics
    /// 
    /// Panics if no monitor can be detected.
    fn move_window(&self, position: Position) -> Result<()>;

    /// Moves the [`Window`] to the given [`Position`] relative to the given [`Monitor`]
    fn move_window_with_monitor(&self, pos: Position, monitor: &Monitor) -> Result<()>;
}

impl<R: Runtime> WindowExt for Window<R> {
    fn move_window_with_monitor(&self, pos: Position, monitor: &Monitor) -> Result<()> {
        use Position::*;

        let monitor_position = monitor.position();
        let monitor_size = PhysicalSize::<i32> {
            width: monitor.size().width as i32,
            height: monitor.size().height as i32,
        };
        let window_size = PhysicalSize::<i32> {
            width: self.outer_size()?.width as i32,
            height: self.outer_size()?.height as i32,
        };
        #[cfg(feature = "system-tray")]
        let (tray_position, tray_size) = self
            .state::<Tray>()
            .0
            .lock()
            .unwrap()
            .map(|(pos, size)| {
                (
                    Some((pos.x as i32, pos.y as i32)),
                    Some((size.width as i32, size.height as i32)),
                )
            })
            .unwrap_or_default();

        let physical_pos = match pos {
            TopLeft => *monitor_position,
            TopRight => PhysicalPosition {
                x: monitor_position.x + (monitor_size.width - window_size.width),
                y: monitor_position.y,
            },
            BottomLeft => PhysicalPosition {
                x: monitor_position.x,
                y: monitor_size.height - (window_size.height - monitor_position.y),
            },
            BottomRight => PhysicalPosition {
                x: monitor_position.x + (monitor_size.width - window_size.width),
                y: monitor_size.height - (window_size.height - monitor_position.y),
            },
            TopCenter => PhysicalPosition {
                x: monitor_position.x + ((monitor_size.width / 2) - (window_size.width / 2)),
                y: monitor_position.y,
            },
            BottomCenter => PhysicalPosition {
                x: monitor_position.x + ((monitor_size.width / 2) - (window_size.width / 2)),
                y: monitor_size.height - (window_size.height - monitor_position.y),
            },
            LeftCenter => PhysicalPosition {
                x: monitor_position.x,
                y: monitor_position.y + (monitor_size.height / 2) - (window_size.height / 2),
            },
            RightCenter => PhysicalPosition {
                x: monitor_position.x + (monitor_size.width - window_size.width),
                y: monitor_position.y + (monitor_size.height / 2) - (window_size.height / 2),
            },
            Center => PhysicalPosition {
                x: monitor_position.x + ((monitor_size.width / 2) - (window_size.width / 2)),
                y: monitor_position.y + (monitor_size.height / 2) - (window_size.height / 2),
            },
            #[cfg(feature = "system-tray")]
            TrayLeft => {
                if let Some((tray_x, tray_y)) = tray_position {
                    PhysicalPosition {
                        x: tray_x,
                        y: tray_y - window_size.height,
                    }
                } else {
                    panic!("tray position not set");
                }
            }
            #[cfg(feature = "system-tray")]
            TrayBottomLeft => {
                if let Some((tray_x, tray_y)) = tray_position {
                    PhysicalPosition {
                        x: tray_x,
                        y: tray_y,
                    }
                } else {
                    panic!("Tray position not set");
                }
            }
            #[cfg(feature = "system-tray")]
            TrayRight => {
                if let (Some((tray_x, tray_y)), Some((tray_width, _))) = (tray_position, tray_size)
                {
                    PhysicalPosition {
                        x: tray_x + tray_width,
                        y: tray_y - window_size.height,
                    }
                } else {
                    panic!("Tray position not set");
                }
            }
            #[cfg(feature = "system-tray")]
            TrayBottomRight => {
                if let (Some((tray_x, tray_y)), Some((tray_width, _))) = (tray_position, tray_size)
                {
                    PhysicalPosition {
                        x: tray_x + tray_width,
                        y: tray_y,
                    }
                } else {
                    panic!("Tray position not set");
                }
            }
            #[cfg(feature = "system-tray")]
            TrayCenter => {
                if let (Some((tray_x, tray_y)), Some((tray_width, _))) = (tray_position, tray_size)
                {
                    PhysicalPosition {
                        x: tray_x + (tray_width / 2) - (window_size.width / 2),
                        y: tray_y - window_size.height,
                    }
                } else {
                    panic!("Tray position not set");
                }
            }
            #[cfg(feature = "system-tray")]
            TrayBottomCenter => {
                if let (Some((tray_x, tray_y)), Some((tray_width, _))) = (tray_position, tray_size)
                {
                    PhysicalPosition {
                        x: tray_x + (tray_width / 2) - (window_size.width / 2),
                        y: tray_y,
                    }
                } else {
                    panic!("Tray position not set");
                }
            }
        };

        self.set_position(tauri::Position::Physical(physical_pos))
    }

    fn move_window(&self, pos: Position) -> Result<()> {
        let monitor = self.current_monitor()?.expect("No monitor detected");

        self.move_window_with_monitor(pos, &monitor)
    }
}
