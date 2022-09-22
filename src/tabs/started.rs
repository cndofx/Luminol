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

pub struct Started {}

impl Started {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::tab::Tab for Started {
    fn name(&self) -> String {
        "Get Started".to_string()
    }

    fn show(&mut self, ui: &mut egui::Ui, _info: &crate::UpdateInfo<'_>) {
        ui.centered_and_justified(|ui| {
            ui.heading("Luminol");
        });
    }
}
