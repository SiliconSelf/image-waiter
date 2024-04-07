//! Actor that provides images to other threads
//!
//! This actor precomuptes all RNG operations for faster response times to
//! requests

use flume::{Receiver, Sender};

/// How big the bounded channel should be
const CHANNEL_SIZE: usize = 200;

/// A representation of a single image that can be served
#[derive(Debug)]
pub(crate) struct Image {
    /// The url to the image
    pub(crate) url: String,
}

/// An actor to provide images on demand
pub(crate) struct ImageActor {
    /// The flume::Receiver for taking computed responses on demand
    rx: Receiver<Image>,
}

impl ImageActor {
    /// Create a new image actor
    pub(crate) fn new() -> Self {
        let (tx, rx) = flume::bounded(CHANNEL_SIZE);
        std::thread::spawn(move || rng_thread(&tx));
        Self {
            rx,
        }
    }

    /// Get an image
    pub(crate) fn get(&self) -> Image {
        self.rx.recv().expect("Image actor got disconnected from RNG thread")
    }
}

/// The background RNG thread that precomputes response data
fn rng_thread(sender: &Sender<Image>) {
    // let mut rng_thread = rand::thread_rng();

    loop {
        while !sender.is_full() {
            sender
                .send(Image {
                    url: "OwO".to_owned(),
                })
                .expect("RNG thread got disconnected from Image actor");
        }
    }
}
