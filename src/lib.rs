//! If you are using this crate on Linux with Unity support I would recommend to initialize `TaskbarIndicator`
//! at application start because if you application exited without clean its `TaskbarIndicator` the launcher
//! would remember the indicator state when the user opens your app by second time. In that case if you
//! initialize `TaskbarIndicator` your app indicator state will be cleaned.

use raw_window_handle::RawWindowHandle;

#[cfg(all(unix, not(target_os = "macos")))]
#[path = "linux/mod.rs"]
mod platform;
#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod platform;

#[cfg(not(any(all(unix, not(target_os = "macos")), target_os = "windows")))]
compile_error!("Platform not supported");

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProgressIndicatorState {
    /// The progress indicator is not visible.
    NoProgress,
    /// Only available on Windows: Express a progress in an activity without specific the proportion of such progress.
    Indeterminate,
    Normal,
    /// Only available for Windows, currently has no effect on other platforms and would be equivalent to the `Normal` variant.
    Paused,
    /// Only available for Windows, currently has no effect on other platforms and would be equivalent to the `Normal` variant.
    Error,
}

pub struct TaskbarInterface {
    platform: platform::TaskbarIndicator,
}

impl TaskbarInterface {
    pub fn new(window: RawWindowHandle) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            platform: platform::TaskbarIndicator::new(window)?,
        })
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    /// Refer to `set_unity_app_uri`.
    pub fn unity_app_uri(
        mut self,
        uri: impl AsRef<str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        self.platform = self.platform.unity_app_uri(uri)?;
        self
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    /// For the majority of the desktops that looks like had made an attempt to adopt the Unity
    /// launcher protocol for indicators, which is not great. Therefore will need to specific a
    /// "Unity app uri" for the said desktops that is a path to the `.desktop` file from where
    /// your application was called. This uri has the next format: `application://$desktop_file_id`
    /// so for example for Firefox it would be `application://firefox.desktop`.
    pub fn set_unity_app_uri(&mut self, uri: impl AsRef<str>) -> Result<(), dbus::Error> {
        self.platform.set_unity_app_uri(uri)
    }

    /// Changes the indicate progress proportion 0.0-1.0, as a note, this will set the progress indicator state
    /// to `Normal` if it is in `NoProgress` or `Indeterminate`.
    pub fn set_progress(&mut self, progress: f64) -> Result<(), Box<dyn std::error::Error>> {
        self.platform.set_progress(progress)
    }

    /// Changes the progress indicator state.
    pub fn set_progress_state(
        &mut self,
        state: ProgressIndicatorState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.platform.set_progress_state(state)
    }

    /// Hightlights the app in the taskbar. Doubt to platform limitations if this is enable on Windows and
    /// the app window is focused in then this will be disable automatically and you will need to enable it again.
    pub fn needs_attention(
        &mut self,
        needs_attention: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.platform.needs_attention(needs_attention)
    }
}

unsafe impl Send for TaskbarInterface {}
unsafe impl Sync for TaskbarInterface {}
