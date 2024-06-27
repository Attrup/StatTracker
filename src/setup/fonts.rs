/// Add monospace font to existing font families to fonts and set as default for monospace
pub fn load_monospace_font(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "Monofonto".to_owned(),
        egui::FontData::from_static(include_bytes!("../../data/monofonto rg.otf")),
    );

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "Monofonto".to_owned());

    ctx.set_fonts(fonts);
}