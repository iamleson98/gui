//! Accessibility system — ARIA-like roles, labels, and hints.
//!
//! Provides semantic metadata for assistive technologies.
//! Each widget can declare its role, label, and state.

use crate::core::Id;
use std::collections::HashMap;

/// ARIA-like widget roles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Role {
    Button,
    Checkbox,
    Slider,
    TextBox,
    List,
    ListItem,
    Tab,
    TabPanel,
    Menu,
    MenuItem,
    Dialog,
    Alert,
    Heading,
    Link,
    Image,
    Separator,
    ProgressBar,
    SpinButton,
    Tree,
    TreeItem,
    ComboBox,
    Option,
    RadioGroup,
    Radio,
    Label,
    None,
}

impl Role {
    /// Whether this role is interactive (can receive focus).
    pub fn is_interactive(self) -> bool {
        matches!(self,
            Role::Button | Role::Checkbox | Role::Slider | Role::TextBox |
            Role::Tab | Role::MenuItem | Role::Link |
            Role::SpinButton | Role::TreeItem | Role::ComboBox |
            Role::Option | Role::Radio
        )
    }

    /// Whether this role represents a container.
    pub fn is_container(self) -> bool {
        matches!(self,
            Role::List | Role::TabPanel | Role::Menu | Role::Dialog |
            Role::Tree | Role::RadioGroup
        )
    }
}

/// Accessible state flags.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AriaStates {
    pub focused: bool,
    pub disabled: bool,
    pub checked: bool,
    pub expanded: bool,
    pub selected: bool,
    pub hidden: bool,
    pub busy: bool,
    pub invalid: bool,
    pub required: bool,
    pub readonly: bool,
}

/// Accessibility metadata for a single widget.
#[derive(Clone, Debug, Default)]
pub struct AccessibilityInfo {
    pub role: Option<Role>,
    pub label: Option<String>,
    pub description: Option<String>,
    pub hint: Option<String>,
    pub value: Option<String>,
    pub states: AriaStates,
    /// Minimum/maximum for range widgets (slider, progress).
    pub value_min: Option<f32>,
    pub value_max: Option<f32>,
    pub value_now: Option<f32>,
    /// For live regions — announces changes to screen readers.
    pub live: LiveRegion,
}

/// Live region politeness levels.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LiveRegion {
    #[default]
    Off,
    Polite,
    Assertive,
}

/// Manages accessibility tree across all widgets.
pub struct AccessibilityTree {
    nodes: HashMap<Id, AccessibilityInfo>,
    /// Announcements to be read by screen readers.
    announcements: Vec<String>,
}

impl Default for AccessibilityTree {
    fn default() -> Self {
        Self {
            nodes: HashMap::new(),
            announcements: Vec::new(),
        }
    }
}

impl AccessibilityTree {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register or update accessibility info for a widget.
    pub fn set(&mut self, id: Id, info: AccessibilityInfo) {
        self.nodes.insert(id, info);
    }

    /// Get accessibility info for a widget.
    pub fn get(&self, id: Id) -> Option<&AccessibilityInfo> {
        self.nodes.get(&id)
    }

    /// Update a specific field.
    pub fn set_label(&mut self, id: Id, label: impl Into<String>) {
        self.nodes.entry(id).or_default().label = Some(label.into());
    }

    pub fn set_role(&mut self, id: Id, role: Role) {
        self.nodes.entry(id).or_default().role = Some(role);
    }

    pub fn set_states(&mut self, id: Id, states: AriaStates) {
        self.nodes.entry(id).or_default().states = states;
    }

    /// Announce a message to screen readers.
    pub fn announce(&mut self, message: impl Into<String>) {
        self.announcements.push(message.into());
    }

    /// Drain pending announcements.
    pub fn drain_announcements(&mut self) -> Vec<String> {
        std::mem::take(&mut self.announcements)
    }

    /// Find all widgets with a given role.
    pub fn find_by_role(&self, role: Role) -> Vec<Id> {
        self.nodes
            .iter()
            .filter(|(_, info)| info.role == Some(role))
            .map(|(id, _)| *id)
            .collect()
    }

    /// Get all focusable widgets (interactive + not disabled + not hidden).
    pub fn focusable_widgets(&self) -> Vec<Id> {
        self.nodes
            .iter()
            .filter(|(_, info)| {
                info.role.map_or(false, |r| r.is_interactive())
                    && !info.states.disabled
                    && !info.states.hidden
            })
            .map(|(id, _)| *id)
            .collect()
    }

    /// Number of registered nodes.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

/// Builder for AccessibilityInfo.
pub struct AccessibilityBuilder {
    info: AccessibilityInfo,
}

impl AccessibilityBuilder {
    pub fn new() -> Self {
        Self { info: AccessibilityInfo::default() }
    }

    pub fn role(mut self, role: Role) -> Self {
        self.info.role = Some(role);
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.info.label = Some(label.into());
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.info.description = Some(desc.into());
        self
    }

    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.info.hint = Some(hint.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.info.value = Some(value.into());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.info.states.disabled = true;
        self
    }

    pub fn checked(mut self) -> Self {
        self.info.states.checked = true;
        self
    }

    pub fn expanded(mut self) -> Self {
        self.info.states.expanded = true;
        self
    }

    pub fn live(mut self, live: LiveRegion) -> Self {
        self.info.live = live;
        self
    }

    pub fn value_range(mut self, min: f32, max: f32, now: f32) -> Self {
        self.info.value_min = Some(min);
        self.info.value_max = Some(max);
        self.info.value_now = Some(now);
        self
    }

    pub fn build(self) -> AccessibilityInfo {
        self.info
    }
}

impl Default for AccessibilityBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_interactive() {
        assert!(Role::Button.is_interactive());
        assert!(Role::TextBox.is_interactive());
        assert!(!Role::Heading.is_interactive());
        assert!(!Role::None.is_interactive());
    }

    #[test]
    fn role_container() {
        assert!(Role::List.is_container());
        assert!(Role::Dialog.is_container());
        assert!(!Role::Button.is_container());
    }

    #[test]
    fn tree_set_get() {
        let mut tree = AccessibilityTree::new();
        let id = Id::new("btn1");
        let info = AccessibilityBuilder::new()
            .role(Role::Button)
            .label("Submit")
            .build();
        tree.set(id, info);
        assert_eq!(tree.get(id).unwrap().role, Some(Role::Button));
        assert_eq!(tree.get(id).unwrap().label.as_deref(), Some("Submit"));
    }

    #[test]
    fn tree_find_by_role() {
        let mut tree = AccessibilityTree::new();
        tree.set_role(Id::new("a"), Role::Button);
        tree.set_role(Id::new("b"), Role::Label);
        tree.set_role(Id::new("c"), Role::Button);
        let buttons = tree.find_by_role(Role::Button);
        assert_eq!(buttons.len(), 2);
    }

    #[test]
    fn tree_focusable_widgets() {
        let mut tree = AccessibilityTree::new();
        let mut info1 = AccessibilityBuilder::new().role(Role::Button).build();
        let mut info2 = AccessibilityBuilder::new().role(Role::Button).disabled().build();
        let info3 = AccessibilityBuilder::new().role(Role::Heading).build();
        tree.set(Id::new("a"), info1);
        tree.set(Id::new("b"), info2);
        tree.set(Id::new("c"), info3);
        let focusable = tree.focusable_widgets();
        assert_eq!(focusable.len(), 1);
    }

    #[test]
    fn announce_and_drain() {
        let mut tree = AccessibilityTree::new();
        tree.announce("Page loaded");
        tree.announce("3 items found");
        let msgs = tree.drain_announcements();
        assert_eq!(msgs.len(), 2);
        assert!(tree.drain_announcements().is_empty());
    }

    #[test]
    fn builder_value_range() {
        let info = AccessibilityBuilder::new()
            .role(Role::Slider)
            .label("Volume")
            .value_range(0.0, 100.0, 50.0)
            .build();
        assert_eq!(info.value_min, Some(0.0));
        assert_eq!(info.value_max, Some(100.0));
        assert_eq!(info.value_now, Some(50.0));
    }
}
