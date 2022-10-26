use bevy::prelude::*;

use crate::args::GameArgs;
use clap::Parser;

pub struct ArgParsePlugin;

impl Plugin for ArgParsePlugin {
    fn build(&self, app: &mut App) {
        let a = GameArgs::parse();
        //TODO: Get the arguments to network.rs so it can do stuff
    }
}
