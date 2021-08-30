use std::mem;

windows::include_bindings!();
use self::Windows::Win32::{
    Foundation::HWND,
    System::{
        Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED},
        Diagnostics::Debug::{FlashWindowEx, FLASHWINFO, FLASHW_STOP, FLASHW_TIMER, FLASHW_TRAY},
    },
    UI::Shell::{
        ITaskbarList3, TaskbarList, TBPF_ERROR, TBPF_INDETERMINATE, TBPF_NOPROGRESS, TBPF_NORMAL,
        TBPF_PAUSED,
    },
};
use raw_window_handle::RawWindowHandle;

use crate::ProgressIndicatorState;

const MAX_PROGRESS: u64 = 100_000;

pub struct TaskbarIndicator {
    hwnd: HWND,
    taskbar: ITaskbarList3,
    progress: u64,
}

impl TaskbarIndicator {
    pub fn new(window: RawWindowHandle) -> Result<Self, Box<dyn std::error::Error>> {
        let hwnd = match window {
            RawWindowHandle::Windows(handle) => HWND(handle.hwnd as isize),
            h @ _ => unimplemented!("{:?}", h),
        };
        unsafe {
            // Intialize COM library if it is not already done
            let _ = CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED);
        }
        let taskbar: ITaskbarList3 = unsafe { CoCreateInstance(&TaskbarList, None, CLSCTX_ALL)? };
        Ok(Self {
            hwnd,
            taskbar,
            progress: 0,
        })
    }

    fn update_progress(&self) {
        unsafe {
            self.taskbar
                .SetProgressValue(self.hwnd, self.progress, MAX_PROGRESS)
                .unwrap();
        }
    }

    pub fn set_progress(&mut self, progress: f64) {
        let progress = (progress.clamp(0.0, 1.0) * MAX_PROGRESS as f64) as u64;
        if self.progress != progress {
            self.progress = progress;
            self.update_progress();
        }
    }

    pub fn set_progress_state(&mut self, state: ProgressIndicatorState) {
        let flag = match state {
            ProgressIndicatorState::NoProgress => TBPF_NOPROGRESS,
            ProgressIndicatorState::Indeterminate => TBPF_INDETERMINATE,
            ProgressIndicatorState::Normal => TBPF_NORMAL,
            ProgressIndicatorState::Paused => TBPF_PAUSED,
            ProgressIndicatorState::Error => TBPF_ERROR,
        };
        unsafe {
            self.taskbar.SetProgressState(self.hwnd, flag).unwrap();
        }
        if matches!(
            state,
            ProgressIndicatorState::Normal
                | ProgressIndicatorState::Paused
                | ProgressIndicatorState::Error
        ) {
            self.update_progress();
        }
    }

    pub fn needs_attention(&mut self, needs_attention: bool) {
        let flags = match needs_attention {
            true => FLASHW_TIMER | FLASHW_TRAY,
            false => FLASHW_STOP,
        };
        let mut params = FLASHWINFO {
            cbSize: mem::size_of::<FLASHWINFO>() as u32,
            hwnd: self.hwnd,
            dwFlags: flags,
            uCount: 0,
            dwTimeout: 0,
        };
        unsafe {
            FlashWindowEx(&mut params);
        }
    }
}
