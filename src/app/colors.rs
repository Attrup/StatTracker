use egui::Color32;

/// Struct to easily store the ColorMaps for the Silent Assassin status
#[derive(PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ColorMap {
    label: String,
    sa_true: Color32,
    sa_false: Color32,
}

impl ColorMap {
    /// Get all available colormaps
    pub fn all_cmaps() -> Vec<Self> {
        vec![
            ColorMap::gr_cmap(),
            ColorMap::dgr_cmap(),
            ColorMap::br_cmap(),
            ColorMap::bo_cmap(),
            ColorMap::bb_cmap(),
            ColorMap::mk_cmap(),
        ]
    }

    /// Standard Green/Red colormap
    fn gr_cmap() -> Self {
        ColorMap {
            label: String::from("Green / Red"),
            sa_true: Color32::from_rgb(0, 160, 0),
            sa_false: Color32::RED,
        }
    }

    /// Dark Green/Red colormap
    fn dgr_cmap() -> Self {
        ColorMap {
            label: String::from("Darker Green / Red"),
            sa_true: Color32::DARK_GREEN,
            sa_false: Color32::DARK_RED,
        }
    }

    /// Alternative Blue/Red colormap
    fn br_cmap() -> Self {
        ColorMap {
            label: String::from("Blue / Red"),
            sa_true: Color32::from_rgb(0, 90, 180),
            sa_false: Color32::from_rgb(220, 50, 32),
        }
    }

    /// Alternative Blue/Orange colormap
    fn bo_cmap() -> Self {
        ColorMap {
            label: String::from("Blue / Orange"),
            sa_true: Color32::from_rgb(12, 123, 220),
            sa_false: Color32::from_rgb(255, 194, 10),
        }
    }

    /// Alternative Blue/Brown colormap
    fn bb_cmap() -> Self {
        ColorMap {
            label: String::from("Blue / Brown"),
            sa_true: Color32::from_rgb(0, 108, 209),
            sa_false: Color32::from_rgb(153, 79, 0),
        }
    }

    /// Alternative Mint/Khaki colormap
    fn mk_cmap() -> Self {
        ColorMap {
            label: String::from("Mint / Khaki"),
            sa_true: Color32::from_rgb(64, 176, 166),
            sa_false: Color32::from_rgb(225, 190, 106),
        }
    }

    // Getters
    pub fn get_rating_color(&self, sa_status: bool) -> Color32 {
        if sa_status {
            self.sa_true
        } else {
            self.sa_false
        }
    }

    pub fn default() -> Self {
        ColorMap::gr_cmap()
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }

    pub fn get_sa_true(&self) -> Color32 {
        self.sa_true
    }

    pub fn get_sa_false(&self) -> Color32 {
        self.sa_false
    }
}
