//! Actor that provides images to other threads
//!
//! This actor precomuptes all RNG operations for faster response times to
//! requests

use std::time::Duration;

use flume::{Receiver, Sender};
use rand::Rng;

use crate::{configuration::APP_CONFIG, database_actor::DatabaseActor};

/// An actor to provide images on demand
pub(crate) struct ImageActor {
    /// The flume::Receiver for taking computed responses on demand
    rx: Receiver<Image>,
}

impl ImageActor {
    /// Create a new image actor
    pub(crate) fn new(db_actor: &DatabaseActor) -> Self {
        let (tx, rx) = flume::bounded(APP_CONFIG.get_channel_size());
        let images: Vec<Image> =
            db_actor.get_images().iter().map(Image::from).collect();
        std::thread::spawn(move || rng_thread(&tx, &images));
        Self {
            rx,
        }
    }

    /// Get an image
    pub(crate) fn get(&self) -> Image {
        self.rx.recv().expect("Image actor got disconnected from RNG thread")
    }
}

/// An in-memory representation of an Image from the database
pub(crate) struct Image {
    /// The serial id of the image
    pub(crate) id: usize,
    /// The URL of the image in the CDN
    pub(crate) url: String,
}

impl From<&crate::models::Image> for Image {
    fn from(value: &crate::models::Image) -> Self {
        Self {
            // We know this isn't going to lose a sign. The serial type in
            // postgres is never negative.
            #[allow(clippy::cast_sign_loss)]
            #[allow(clippy::as_conversions)]
            id: value.id as usize,
            url: value.url.clone(),
        }
    }
}

/// The background RNG thread that precomputes response data
fn rng_thread(sender: &Sender<Image>, source: &[Image]) {
    let source_size = source.len();
    let mut rng_thread = rand::thread_rng();

    loop {
        while !sender.is_full() {
            let selection = rng_thread.gen_range(0..source_size);
            let image = Image {
                id: selection,
                url: "OwO".to_owned(),
            };
            sender
                .send(image)
                .expect("RNG thread got disconnected from Image actor");
        }
        // Sleep here to avoid making a busy thread
        std::thread::sleep(Duration::from_millis(10));
    }
}
