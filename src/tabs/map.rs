use crate::components::tilemap::Tilemap;

pub struct Map {
    pub id: i32,
    pub name: String,
    pub selected_layer: usize,
    pub tilemap: Tilemap,
}

impl Map {
    pub fn new(id: i32, name: String) -> Self {
        Self {
            id,
            name,
            selected_layer: 0,
            tilemap: Tilemap::new(),
        }
    }
}

impl super::tab::Tab for Map {
    fn name(&self) -> String {
        format!("Map {}: {}", self.id, self.name)
    }

    #[allow(unused_variables, unused_mut)]
    fn show(&mut self, ui: &mut egui::Ui, info: &crate::UpdateInfo<'_>) {
        // Load the map if it isn't loaded.
        let mut map = info.data_cache.load_map(info.filesystem, self.id);
        let tilesets = info.data_cache.tilesets();
        // We subtract 1 because RMXP is stupid and pads arrays with nil to start at 1.
        let tileset = &tilesets.as_ref().expect("Tilesets not loaded")[map.tileset_id as usize - 1];

        // Display the toolbar.
        self.toolbar(ui, &mut map);

        // Display the tilepicker.
        egui::SidePanel::left(format!("map_{}_tilepicker", self.id)).show_inside(ui, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {});
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.tilemap
                .ui(ui, &mut map, self.id, &tileset.tileset_name, info)
        });
    }
}
