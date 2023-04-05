use std::{sync::mpsc, thread, time::Duration};

// temporary
struct Key {}

enum Event<I> {
    Input(I),
    Tick,
}
/// A small event handler that wrap crossterm input and tick event. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    // Need to be kept around to prevent disposing the sender side.
    _tx: mpsc::Sender<Event<Key>>,
}

impl Events {
    /// Constructs an new instance of `Events` with the default config.
    pub fn new(tick_rate: u64) -> Events {
        Events::with_config(EventConfig {
            tick_rate: Duration::from_millis(tick_rate),
            ..Default::default()
        })
    }

    /// Constructs an new instance of `Events` from given config.
    fn with_config(config: EventConfig) -> Events {
        let (tx, rx) = mpsc::channel();

        let event_tx = tx.clone();
        thread::spawn(move || {
            loop {
                // poll for tick rate duration, if no event, sent tick event.
                if event::poll(config.tick_rate).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        let key = Key::from(key);

                        event_tx.send(Event::Input(key)).unwrap();
                    }
                }

                event_tx.send(Event::Tick).unwrap();
            }
        });

        Events { rx, _tx: tx }
    }

    /// Attempts to read an event.
    /// This function will block the current thread.
    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}
