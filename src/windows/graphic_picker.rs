// Copyright (C) 2023 Lily Lyons
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
use super::window::Window;
use crate::{load_image_software, UpdateInfo};
use egui_extras::RetainedImage;
use poll_promise::Promise;
use std::path::Path;

pub struct Graphic {
    pub name: String,
    pub image: RetainedImage,
}

pub struct GraphicPicker {
    icons: Vec<Graphic>,
    selected_icon: usize,
}

impl GraphicPicker {
    #[must_use]
    pub fn new(icons: Vec<String>, info: &'static UpdateInfo) -> Self {
        let mut retained_images = Vec::new();

        for icon_path in icons {
            let icon_path = icon_path;
            let split = icon_path.split('.').collect::<Vec<&str>>();

            let icon_path = String::from(split[0]);

            let image = match Promise::spawn_local(load_image_software(
                format!("Graphics/Icons/{}", icon_path.clone()),
                info,
            ))
            .block_and_take()
            {
                Ok(ri) => ri,
                Err(why) => {
                    info.toasts
                        .error(format!("Cannot load `{icon_path}` icon: {why}"));
                    continue;
                }
            };
            retained_images.push(Graphic {
                name: icon_path,
                image,
            });
        }

        Self {
            icons: retained_images,
            selected_icon: 0,
        }
    }

    pub fn set_active_icon(&mut self, active_icon_index: usize) {
        if active_icon_index < self.icons.len() {
            self.selected_icon = active_icon_index;
        }
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        open: &mut bool,
        _info: &'static crate::UpdateInfo,
        graphic_icon: &mut String,
    ) {
        egui::Window::new("Graphic Picker")
            .id(egui::Id::new("icon_picker"))
            .resize(|res| res.min_width(480.))
            .open(open)
            .show(ctx, |ui| {
                egui::SidePanel::left(egui::Id::new("item_picker_sidebar")).show_inside(ui, |ui| {
                    egui::ScrollArea::both().max_height(600.).show_rows(
                        ui,
                        ui.text_style_height(&egui::TextStyle::Body),
                        self.icons.len(),
                        |ui, rows| {
                            for (id, icon) in self
                                .icons
                                .iter()
                                .enumerate()
                                .filter(|(ele, _)| rows.contains(ele))
                            {
                                ui.selectable_value(&mut self.selected_icon, id, icon.name.clone());
                                *graphic_icon =
                                    self.icons.get(self.selected_icon).unwrap().name.clone();
                            }
                        },
                    );
                });

                let icon = &self.icons[self.selected_icon];
                icon.image.show_scaled(ui, 3.);
            });
    }
}
