use std::time::Duration;

use n0_future::TryFutureExt;

use crate::tasks::{self, TaskHandler, TaskRequester};

pub enum Event {
    None,
    TaskHandlerInitialised,
    Pong(Duration, reqwest::Result<String>),
}

pub enum Task {
    Ping,
}

impl Task {
    pub async fn run(&mut self) -> Event {
        match self {
            Task::Ping => {
                let start = std::time::Instant::now();
                let pong = Self::run_ping().await;
                let fin = std::time::Instant::now();
                let dur = fin.duration_since(start);
                return Event::Pong(dur, pong);
            }
        }

        Event::None
    }

    async fn run_ping() -> reqwest::Result<String> {
        reqwest::get("http://127.0.0.1:8080/api/ping")
            .await?
            .text()
            .await
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    #[serde(skip)]
    handler_initialised: bool,
    #[serde(skip)]
    tasks: Option<TaskRequester>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            tasks: None,
            handler_initialised: false,
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    pub fn get_task_handler(&mut self) -> TaskHandler {
        assert!(self.tasks.is_none(), "Already has a task requester.");

        let (req, handler) = tasks::create_manager();
        self.tasks = Some(req);
        handler
    }

    pub fn with_task_requester(&mut self, req: TaskRequester) {
        assert!(self.tasks.is_none(), "Already has a task requester.");
        self.tasks = Some(req);
    }

    fn handle_events(&mut self) {
        while let Some(event) = self.tasks.as_mut().unwrap().next() {
            self.handle_event(event);
        }
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::None => {}
            Event::TaskHandlerInitialised => self.handler_initialised = true,
        }
    }

    fn send_task(&mut self, task: Task) {
        self.tasks
            .as_mut()
            .unwrap()
            .send
            .send(task)
            .expect("Couldn't send task.");
    }
}

impl eframe::App for App {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_events();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(if self.handler_initialised {
                "Task handler initialised."
            } else {
                "Task handler has not yet responded..."
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
