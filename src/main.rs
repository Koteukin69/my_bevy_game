use bevy::prelude::*;

mod flappybird;

use flappybird::FlappyBirdPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FlappyBirdPlugin))
        .run();
}
