use std::time::Duration;
use crossterm::event::{self, Event as CEvent, KeyEvent};
use tokio::sync::mpsc;
use tokio::time::{interval, Interval};

#[derive(Debug, Clone)]
pub enum Event {
    Key(KeyEvent),
    Tick,
}

pub struct Events {
    rx: mpsc::UnboundedReceiver<Event>,
    // keep one sender so the channel stays alive while Events is alive
    _tx: mpsc::UnboundedSender<Event>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<Event>();

        // clone for the input task
        let tx_input = tx.clone();
        tokio::spawn(async move {
            loop {
                if event::poll(Duration::from_millis(100)).unwrap_or(false) {
                    match event::read() {
                        Ok(CEvent::Key(key)) => {
                            let _ = tx_input.send(Event::Key(key));
                        }
                        _ => {}
                    }
                }
                tokio::task::yield_now().await;
            }
        });

        // clone for the tick task
        let tx_tick = tx.clone();
        tokio::spawn(async move {
            let mut ticker: Interval = interval(tick_rate);
            loop {
                ticker.tick().await;
                let _ = tx_tick.send(Event::Tick);
            }
        });

        Self { rx, _tx: tx }
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.rx.recv().await
    }
}