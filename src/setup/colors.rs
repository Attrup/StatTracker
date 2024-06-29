use egui::Color32;

/// Struct to easily store the ColorMap for the Silent Assassin status
pub struct ColorMap {
    pub sa_true: Color32,
    pub sa_false: Color32,
}

impl ColorMap {
    /// Standard Green/Red colormap
    pub fn standard_cmap() -> Self {
        ColorMap {
            sa_true: Color32::GREEN,
            sa_false: Color32::RED,
        }
    }

    /// Alternative Blue/Red colormap
    pub fn alt_br_cmap() -> Self {
        ColorMap {
            sa_true: Color32::from_rgb(0, 90, 180),
            sa_false: Color32::from_rgb(220, 50, 32),
        }
    }

    /// Alternative Blue/Orange colormap
    pub fn alt_bo_cmap() -> Self {
        ColorMap {
            sa_true: Color32::from_rgb(12, 123, 220),
            sa_false: Color32::from_rgb(255, 194, 10),
        }
    }

    /// Alternative Blue/Brown colormap
    pub fn alt_bb_cmap() -> Self {
        ColorMap {
            sa_true: Color32::from_rgb(0, 108, 209),
            sa_false: Color32::from_rgb(153, 79, 0),
        }
    }

    /// Alternative Mint/Khaki colormap
    pub fn alt_mk_cmap() -> Self {
        ColorMap {
            sa_true: Color32::from_rgb(64, 76, 166),
            sa_false: Color32::from_rgb(225, 190, 106),
        }
    }
}
