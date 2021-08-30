use raw_window_handle::RawWindowHandle;

use crate::ProgressIndicatorState;

mod unity;
mod xapps;

pub struct TaskbarIndicator {
    xapps: Option<xapps::Manager>,
    unity: Option<unity::Manager>,
    progress: f64,
    progress_visible: bool,
    needs_attention: bool,
}

impl TaskbarIndicator {
    pub fn new(window: RawWindowHandle) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            xapps: xapps::Manager::new(window),
            unity: None,
            progress: 0.0,
            progress_visible: false,
            needs_attention: false,
        })
    }

    pub fn unity_app_uri(mut self, uri: impl AsRef<str>) -> Self {
        let _ = self.set_unity_app_uri(uri);
        self
    }

    pub fn set_unity_app_uri(&mut self, uri: impl AsRef<str>) -> Result<(), dbus::Error> {
        let mut unity = unity::Manager::new(uri.as_ref().to_owned())?;
        unity.set_progress(self.progress);
        unity.set_progress_visible(self.progress_visible);
        unity.needs_attention(self.needs_attention);
        self.unity.replace(unity);
        Ok(())
    }

    pub fn set_progress(&mut self, progress: f64) {
        self.set_progress_state(ProgressIndicatorState::Normal);
        let progress = progress.clamp(0.0, 1.0);
        self.progress = progress;
        if let Some(ref mut xapps) = self.xapps {
            xapps.set_progress(progress);
        }
        if let Some(ref mut unity) = self.unity {
            unity.set_progress(progress);
        }
    }

    pub fn set_progress_state(&mut self, state: ProgressIndicatorState) {
        let visible = match state {
            ProgressIndicatorState::NoProgress | ProgressIndicatorState::Indeterminate => false,
            _ => true,
        };
        self.progress_visible = visible;
        if let Some(ref mut xapps) = self.xapps {
            xapps.set_progress_visible(visible);
        }
        if let Some(ref mut unity) = self.unity {
            unity.set_progress_visible(visible);
        }
    }

    pub fn needs_attention(&mut self, needs_attention: bool) {
        self.needs_attention = needs_attention;
        if let Some(ref mut xapps) = self.xapps {
            xapps.needs_attention(needs_attention);
        }
        if let Some(ref mut unity) = self.unity {
            unity.needs_attention(needs_attention);
        }
    }
}
