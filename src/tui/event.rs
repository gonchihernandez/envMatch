use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Resize,
}

pub struct EventHandler {
    _tx: mpsc::UnboundedSender<Event>,
    rx: mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let _tx = tx.clone();

        tokio::spawn(async move {
            loop {
                if event::poll(tick_rate).unwrap() {
                    match event::read().unwrap() {
                        CrosstermEvent::Key(key_event) => {
                            tx.send(Event::Key(key_event)).unwrap();
                        }
                        CrosstermEvent::Resize(_, _) => {
                            tx.send(Event::Resize).unwrap();
                        }
                        _ => {}
                    }
                }
                tx.send(Event::Tick).unwrap();
            }
        });

        Self { _tx, rx }
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.rx.recv().await
    }
}
