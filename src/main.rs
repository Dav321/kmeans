mod kmeans;

use crate::kmeans::KMeans;
use ggez::ContextBuilder;
use ggez::conf::{Conf, FullscreenType};
use ggez::event::run;

fn main() {
    let mut conf = Conf::new();
    conf.window_setup.title = "KMeans".to_string();
    conf.window_mode.fullscreen_type = FullscreenType::Desktop;
    conf.window_setup.vsync = true;
    let (mut ctx, event_loop) = ContextBuilder::new("kmeans", "David Schmidt")
        .default_conf(conf)
        .build()
        .expect("Could not create Context!");
    let mut kmeans = KMeans::new(&mut ctx);
    kmeans.random(100, 6);
    run(ctx, event_loop, kmeans).expect("Game Crash:");
}
