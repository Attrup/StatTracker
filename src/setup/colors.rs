use egui::Color32;

/// Struct to easily store the ColorMaps for the Silent Assassin status
#[derive(PartialEq)]
pub struct ColorMap {
    label: String,
    sa_true: Color32,
    sa_false: Color32,
}

impl ColorMap {
    /// Standard Green/Red colormap
    pub fn gr_cmap() -> Self {
        ColorMap {
            label: String::from("Green / Red"),
            sa_true: Color32::from_rgb(0, 204, 0),
            sa_false: Color32::RED,
        }
    }

    /// Alternative Blue/Red colormap
    pub fn br_cmap() -> Self {
        ColorMap {
            label: String::from("Blue / Red"),
            sa_true: Color32::from_rgb(0, 90, 180),
            sa_false: Color32::from_rgb(220, 50, 32),
        }
    }

    /// Alternative Blue/Orange colormap
    pub fn bo_cmap() -> Self {
        ColorMap {
            label: String::from("Blue / Orange"),
            sa_true: Color32::from_rgb(12, 123, 220),
            sa_false: Color32::from_rgb(255, 194, 10),
        }
    }

    /// Alternative Blue/Brown colormap
    pub fn bb_cmap() -> Self {
        ColorMap {
            label: String::from("Blue / Brown"),
            sa_true: Color32::from_rgb(0, 108, 209),
            sa_false: Color32::from_rgb(153, 79, 0),
        }
    }

    /// Alternative Mint/Khaki colormap
    pub fn mk_cmap() -> Self {
        ColorMap {
            label: String::from("Mint / Khaki"),
            sa_true: Color32::from_rgb(64, 76, 166),
            sa_false: Color32::from_rgb(225, 190, 106),
        }
    }

    // Getters
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
