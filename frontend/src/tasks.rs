use std::{
    sync::mpsc::{Receiver, Sender, TryRecvError},
    time::Duration,
};

use crate::app::{Event, Task};

pub struct TaskRequester {
    pub send: Sender<Task>,
    pub recv: Receiver<Event>,
}

pub struct TaskHandler {
    send: Sender<Event>,
    recv: Receiver<Task>,
}

pub fn create_manager() -> (TaskRequester, TaskHandler) {
    let (req_tx, req_rx) = std::sync::mpsc::channel();
    let (handle_tx, handle_rx) = std::sync::mpsc::channel();

    let requester = TaskRequester {
        send: req_tx,
        recv: handle_rx,
    };

    let handler = TaskHandler {
        send: handle_tx,
        recv: req_rx,
    };

    (requester, handler)
}

impl TaskRequester {
    pub fn run(&self, task: Task) {
        self.send.send(task).unwrap();
    }

    pub fn next(&self) -> Option<Event> {
        self.recv
            .try_recv()
            .map_err(|e| assert!(matches!(e, TryRecvError::Empty)))
            .ok()
    }
}

impl TaskHandler {
    pub async fn run(&mut self, egui_ctx: egui::Context) {
        log::info!("Task handler started.");

        self.send.send(Event::TaskHandlerInitialised).unwrap();
        egui_ctx.request_repaint();

        loop {
            if let Ok(mut task) = self.recv.try_recv() {
                let send = self.send.clone();
                let ctx = egui_ctx.clone();
                n0_future::task::spawn(async move {
                    let event = task.run().await;
                    if !matches!(event, Event::None) {
                        send.send(event).unwrap();
                        ctx.request_repaint();
                    }
                });
            } else {
                n0_future::time::sleep(Duration::from_millis(20)).await;
            }
        }
    }
}
