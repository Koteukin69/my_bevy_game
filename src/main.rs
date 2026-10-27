use bevy::prelude::*;

mod flappybird;
mod hello;

use flappybird::FlappyBirdPlugin;
use hello::HelloPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin, FlappyBirdPlugin))
        .run();
}
