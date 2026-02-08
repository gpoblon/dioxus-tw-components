use crate::components::icon::*;
use dioxus::prelude::*;
use dioxus_sdk_storage::use_persistent;
use serde::{Deserialize, Serialize};

/// Application theme mode.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
}

impl ThemeMode {
    /// Returns the theme mode as a string (e.g., "dark" or "light").
    pub const fn as_str(&self) -> &'static str {
        match self {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        }
    }

    /// Toggle between light and dark themes.
    pub const fn toggle(self) -> Self {
        match self {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        }
    }

    /// Returns true if this is the dark theme variant.
    pub const fn is_dark(&self) -> bool {
        matches!(self, ThemeMode::Dark)
    }

    /// Returns true if this is the light theme variant.
    pub const fn is_light(&self) -> bool {
        matches!(self, ThemeMode::Light)
    }
}

const STORAGE_KEY: &str = "theme_mode";

/// Hook to access the theme state from context.
/// Returns the signal containing the current theme.
///
/// # Panics
/// Panics if no ThemeProvider is present in the component tree.
pub fn use_theme() -> Signal<ThemeMode> {
    use_context::<Signal<ThemeMode>>()
}

/// Try to access the theme state from context.
/// Returns None if no ThemeProvider is present in the component tree.
pub fn try_use_theme() -> Option<Signal<ThemeMode>> {
    try_use_context::<Signal<ThemeMode>>()
}

#[derive(Clone, PartialEq, Props)]
pub struct ThemeProviderProps {
    children: Element,
}

/// Provider component that manages theme state and makes it available via context.
/// Uses dioxus-sdk-storage for cross-platform persistent storage.
///
/// Wrap your app (or the part that needs theming) with this component.
///
/// # Example
/// ```rust
/// fn App() -> Element {
///     rsx! {
///         ThemeProvider {
///             ThemedRoot {
///                 id: "main",
///                 // Your app content
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    // Use persistent storage for the theme preference
    // This works cross-platform: localStorage on web, file storage on desktop/mobile
    let theme = use_persistent(STORAGE_KEY, ThemeMode::default);

    // Provide the state via context
    let state = use_context_provider(|| Signal::new(*theme.read()));

    // Keep context in sync with persistent storage
    use_effect(move || {
        let mut state = state;
        state.set(*theme.read());
    });

    rsx! {
        {props.children}
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ThemedRootProps {
    children: Element,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

/// A wrapper component that applies theme attributes based on the theme context.
/// Applies the "dark" class and sets `data-theme` attribute.
///
/// # Example
/// ```rust
/// fn App() -> Element {
///     rsx! {
///         ThemeProvider {
///             ThemedRoot {
///                 id: "main",
///                 class: "your-classes",
///                 // Your app content
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn ThemedRoot(mut props: ThemedRootProps) -> Element {
    let theme = use_context::<Signal<ThemeMode>>();
    let is_dark = theme.read().is_dark();
    let theme_str = theme.read().as_str();

    // Add "dark" class if dark mode is enabled
    if is_dark {
        crate::setup_class_attribute(&mut props.attributes, "dark");
    }

    rsx! {
        div {
            "data-theme": theme_str,
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ThemeSwitchProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Optional callback when the theme is toggled.
    #[props(optional)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    children: Element,
}

/// A toggle button that switches between light and dark themes.
/// Must be used within a ThemeProvider.
///
/// Uses dioxus-sdk-storage for cross-platform persistent storage of the theme preference.
///
/// # Example
/// ```rust
/// rsx! {
///     ThemeSwitch {
///         class: "my-button-class",
///     }
/// }
/// ```
#[component]
pub fn ThemeSwitch(mut props: ThemeSwitchProps) -> Element {
    let default_classes = "lightswitch";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    // Use persistent storage for the theme preference
    let mut theme_storage = use_persistent(STORAGE_KEY, ThemeMode::default);

    // Also update the context when we toggle
    let mut theme_context = use_context::<Signal<ThemeMode>>();

    let icon = if theme_storage.read().is_dark() {
        rsx! {
            Icon { icon: Icons::DarkMode }
        }
    } else {
        rsx! {
            Icon { icon: Icons::LightMode }
        }
    };

    rsx! {
        button {
            r#type: "button",
            onclick: move |e| {
                let new_value = theme_storage.read().toggle();
                theme_storage.set(new_value);
                theme_context.set(new_value);
                if let Some(ref handler) = props.onclick {
                    handler.call(e);
                }
            },
            ..props.attributes,
            {icon}
        }
    }
}

// Backwards compatibility aliases
pub use ThemeProvider as DarkThemeProvider;
pub use ThemeSwitch as LightSwitch;
