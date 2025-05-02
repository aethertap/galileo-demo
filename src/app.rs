use eframe::CreationContext;
use egui::{Align, Key, Layout, Modifiers, TextEdit};
use galileo::Map;
use galileo::control::UserEventHandler;
use galileo_egui::EguiMapState;

pub struct MapApp {
    pub map: EguiMapState,
    //pub cmd_state: TextEditState,
    pub cmd: String,
    pub cmd_history: Vec<String>,
}

impl MapApp {
    pub fn new(
        map: Map,
        cc: &CreationContext<'_>,
        handlers: impl IntoIterator<Item = Box<dyn UserEventHandler>>,
    ) -> Self {
        let ctx = cc.egui_ctx.clone();
        let render_state = cc
            .wgpu_render_state
            .clone()
            .expect("failed to get wgpu context");
        Self {
            map: EguiMapState::new(map, ctx, render_state, handlers),
            //cmd_state: TextEditState::default(),
            cmd: String::new(),
            cmd_history: vec![],
        }
    }
}

impl eframe::App for MapApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                Layout::bottom_up(Align::Center).with_cross_justify(true),
                |ui| {
                    let response = ui.add(TextEdit::singleline(&mut self.cmd));
                    self.map.render(ui);
                    let enter_pressed =
                        ctx.input_mut(|i| i.consume_key(Modifiers::default(), Key::Enter));
                    if enter_pressed {
                        if response.lost_focus() {
                            println!("Run command: {:?}", self.cmd);
                            self.cmd_history.push(self.cmd.clone());
                            self.cmd = "".into();
                        } else if let Some(ref c) = self.cmd_history.last() {
                            println!("Repeat command: {c:?}");
                        }
                    }
                    let colon_pressed = ctx.input_mut(|i| {
                        i.consume_key(Modifiers::default().plus(Modifiers::SHIFT), Key::Colon)
                    });
                    if !response.has_focus() && colon_pressed {
                        response.request_focus();
                    }
                },
            );
        });
    }
}
