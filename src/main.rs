mod kmeans;

use ggez::conf::{Conf, FullscreenType};
use ggez::ContextBuilder;
use ggez::event::run;
use crate::kmeans::KMeans;

fn main() {
    let mut conf = Conf::new();
    conf.window_setup.title = "KMeans".to_string();
    conf.window_mode.fullscreen_type = FullscreenType::Desktop;
    let (mut ctx, event_loop) = ContextBuilder::new("kmeans", "David Schmidt")
        .default_conf(conf)
        .build()
        .expect("Could not create Context!");
    let mut kmeans = KMeans::new(&mut ctx);
    kmeans.random(100, 6);
    run(ctx, event_loop, kmeans);
}
