use eframe::egui;

pub struct MainUi {
    word: Option<String>,
    definition: Option<String>,
    user_answer: String,
}

impl MainUi {
    pub fn new() -> Self {
        Self {
            word: None,
            definition: None,
            user_answer: String::new(),
        }
    }
}

impl MainUi {
    pub fn set_word(&mut self, word: String) {
    }
}

impl eframe::App for MainUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("Single word mode");

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.user_answer)
                    .labelled_by(ui.label(self.word).id)
            });

            if ui.button("Submit").clicked() {
            }

            ui.label(self.words[self.current_pair].to_owned());


            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            // if ui.button("Click each year").clicked() {
            //     self.age += 1;
            // }

            // ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
