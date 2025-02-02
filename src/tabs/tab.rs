// Copyright (C) 2022 Lily Lyons
//
// This file is part of Luminol.
//
// Luminol is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Luminol is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Luminol.  If not, see <http://www.gnu.org/licenses/>.

use std::{cell::RefCell, hash::Hash};

use crate::UpdateInfo;

/// The tree type;
type Tree<T> = egui_dock::Tree<T>;

/// Helper struct for tabs.
pub struct Tabs<T> {
    tree: RefCell<Tree<T>>,
    id: egui::Id,
}

impl<T> Tabs<T>
where
    T: Tab,
{
    /// Create a new Tab viewer without any tabs.
    pub fn new(id: impl Hash, tabs: Vec<T>) -> Self {
        Self {
            id: egui::Id::new(id),
            tree: Tree::new(tabs).into(),
        }
    }

    /// Display all tabs.
    pub fn ui(&self, ui: &mut egui::Ui, info: &'static UpdateInfo) {
        ui.group(|ui| {
            egui_dock::DockArea::new(&mut self.tree.borrow_mut())
                .id(self.id)
                .show_inside(
                    ui,
                    &mut TabViewer {
                        info,
                        marker: std::marker::PhantomData,
                    },
                );
        });
    }

    /// Add a tab.
    pub fn add_tab(&self, tab: T) {
        let mut tree = self.tree.borrow_mut();
        for n in tree.iter() {
            if let egui_dock::Node::Leaf { tabs, .. } = n {
                if tabs.iter().any(|t| t.name() == tab.name()) {
                    return;
                }
            }
        }
        tree.push_to_focused_leaf(tab);
    }

    /// Clean tabs by if they need the filesystem.
    pub fn clean_tabs<F: FnMut(&mut T) -> bool>(&self, mut f: F) {
        let mut tree = self.tree.borrow_mut();
        for node in tree.iter_mut() {
            if let egui_dock::Node::Leaf { tabs, .. } = node {
                tabs.drain_filter(&mut f);
            }
        }
    }

    /// Returns the name of the focused tab.
    pub fn focused_name(&self) -> Option<String> {
        let mut tree = self.tree.borrow_mut();
        tree.find_active().map(|(_, t)| t.name())
    }

    /// The discord rpc text to display.
    #[cfg(feature = "discord-rpc")]
    pub fn discord_display(&self) -> String {
        let mut tree = self.tree.borrow_mut();
        if let Some((_, tab)) = tree.find_active() {
            tab.discord_display()
        } else {
            "No tab open".to_string()
        }
    }
}

struct TabViewer<T: Tab> {
    info: &'static UpdateInfo,

    // we don't actually own any types of T, but we use them in TabViewer
    // *const is used here to avoid needing lifetimes and to indicate to the drop checker that we don't own any types of T
    marker: std::marker::PhantomData<*const T>,
}

impl<T> egui_dock::TabViewer for TabViewer<T>
where
    T: Tab,
{
    type Tab = T;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.name().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.show(ui, self.info);
    }

    fn force_close(&mut self, tab: &mut Self::Tab) -> bool {
        tab.force_close()
    }
}

/// A tab trait.
pub trait Tab {
    /// The name of the tab.
    fn name(&self) -> String;

    /// Show this tab.
    fn show(&mut self, ui: &mut egui::Ui, info: &'static UpdateInfo);

    /// Does this tab need the filesystem?
    fn requires_filesystem(&self) -> bool {
        false
    }

    /// Does this tab need to be closed?
    fn force_close(&mut self) -> bool {
        false
    }

    /// The discord rpc text to display for this tab.
    #[cfg(feature = "discord-rpc")]
    fn discord_display(&self) -> String {
        "Idling".to_string()
    }
}

impl Tab for Box<dyn Tab> {
    fn force_close(&mut self) -> bool {
        self.as_mut().force_close()
    }

    fn name(&self) -> String {
        self.as_ref().name()
    }

    fn requires_filesystem(&self) -> bool {
        self.as_ref().requires_filesystem()
    }

    fn show(&mut self, ui: &mut egui::Ui, info: &'static UpdateInfo) {
        self.as_mut().show(ui, info)
    }
}
