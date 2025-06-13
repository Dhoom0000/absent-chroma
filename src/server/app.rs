use std::time::Duration;
use bevy::{app::ScheduleRunnerPlugin, prelude::*};

pub fn start() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.))))
        .add_systems(Update, print_hello)
        .run();
}

fn print_hello(time: Res<Time>) {
    println!("{:?} Hello World!",time.delta());
}
