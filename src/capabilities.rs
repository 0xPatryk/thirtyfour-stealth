use thirtyfour::{ChromeCapabilities, ChromiumLikeCapabilities, DesiredCapabilities};

/// Default capabilities that are used.
///
/// Default values are:
/// ```compile_fail
/// no_sandbox: true,
/// disable_dev_shm_usage: true,
/// window_size: (1920, 1080),
/// user_agent: USER_AGENT, // this is a constant accessible from root of the crate
/// hide_chrome_is_being_controlled: true,
/// disable_search_engine_choice_screen: true,
/// window_position: None
/// headless: false,
/// ```
pub struct DefaultCapabilitiesBuilder<'a> {
    no_sandbox: bool,
    disable_dev_shm_usage: bool,
    window_size: (u32, u32),
    user_agent: Option<&'a str>,
    hide_chrome_is_being_controlled: bool,
    disable_search_engine_choice_screen: bool,
    window_position: Option<(i32, i32)>,
    headless: bool,
}

impl<'a> DefaultCapabilitiesBuilder<'a> {
    /// Constructs [`ChromeCapabilities`] from builder.
    pub fn into_chrome_caps(self) -> ChromeCapabilities {
        let mut caps = DesiredCapabilities::chrome();
        if self.no_sandbox {
            caps.set_no_sandbox().unwrap();
        }

        if self.disable_dev_shm_usage {
            caps.set_disable_dev_shm_usage().unwrap();
        }

        caps.add_arg("--disable-blink-features=AutomationControlled")
            .unwrap();

        caps.add_arg(&format!(
            "window-size={},{}",
            self.window_size.0, self.window_size.1
        ))
        .unwrap();

        if let Some(user_agent) = self.user_agent {
            let user_agent = format!("user-agent={}", user_agent);
            caps.add_arg(&user_agent).unwrap();
        }

        if self.hide_chrome_is_being_controlled {
            caps.add_arg("disable-infobars").unwrap();
            caps.add_exclude_switch("enable-automation").unwrap();
        }

        if self.disable_search_engine_choice_screen {
            caps.add_arg("--disable-search-engine-choice-screen")
                .unwrap();
        }

        if let Some((x, y)) = self.window_position {
            caps.add_arg(&format!("--window-position={},{}", x, y))
                .unwrap();
        }

        if self.headless {
            caps.add_arg("--headless=new").unwrap();
        }
        caps
    }

    /// Sets the window position offscreen.
    pub fn put_window_offscreen(mut self) -> Self {
        self.window_position = Some((-32_000, -32_000));
        self
    }

    /// Construct new builder.
    ///
    /// Default values are:
    /// ```compile_fail
    /// no_sandbox: true,
    /// disable_dev_shm_usage: true,
    /// window_size: (1920, 1080),
    /// user_agent: None,
    /// hide_chrome_is_being_controlled: true,
    /// disable_search_engine_choice_screen: true,
    /// window_position: None
    /// headless: false,
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_no_sandbox(mut self, no_sandbox: bool) -> Self {
        self.no_sandbox = no_sandbox;
        self
    }

    pub fn set_disable_dev_shm_usage(mut self, disable_dev_shm_usage: bool) -> Self {
        self.disable_dev_shm_usage = disable_dev_shm_usage;
        self
    }

    pub fn set_window_size(mut self, width: u32, height: u32) -> Self {
        self.window_size = (width, height);
        self
    }

    pub fn set_user_agent(mut self, user_agent: &'a str) -> Self {
        self.user_agent = Some(user_agent);
        self
    }

    pub fn set_hide_chrome_is_being_controlled(
        mut self,
        hide_chrome_is_being_controlled: bool,
    ) -> Self {
        self.hide_chrome_is_being_controlled = hide_chrome_is_being_controlled;
        self
    }

    pub fn set_disable_search_engine_choice_screen(
        mut self,
        disable_search_engine_choice_screen: bool,
    ) -> Self {
        self.disable_search_engine_choice_screen = disable_search_engine_choice_screen;
        self
    }

    pub fn set_window_position(mut self, window_position: Option<(i32, i32)>) -> Self {
        self.window_position = window_position;
        self
    }

    /// Don't forget to set user-agent when using this option!
    pub fn set_headless(mut self, headless: bool) -> Self {
        self.headless = headless;
        self
    }
}

impl<'a> Default for DefaultCapabilitiesBuilder<'a> {
    fn default() -> Self {
        Self {
            no_sandbox: true,
            disable_dev_shm_usage: true,
            window_size: (1920, 1080),
            user_agent: None,
            hide_chrome_is_being_controlled: true,
            disable_search_engine_choice_screen: true,
            window_position: None,
            headless: false,
        }
    }
}

impl<'a> From<DefaultCapabilitiesBuilder<'a>> for ChromeCapabilities {
    fn from(value: DefaultCapabilitiesBuilder<'a>) -> Self {
        value.into_chrome_caps()
    }
}
