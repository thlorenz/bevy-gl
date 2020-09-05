use bevy::prelude::*;

// *******
// Components {

struct Id(String);

// } Components
// *******

// *******
// Entities {

#[derive(Bundle)]
struct EntityWithId {
    id: Id,
}

// } Entities
// *******

fn setup(mut commands: Commands) {
    commands
        .spawn(EntityWithId {
            id: Id("uno".to_string()),
        })
        .spawn(EntityWithId {
            id: Id("dos".to_string()),
        });
    eprintln!("## Ran startup system");
}

fn first(comp: &Id) {
    eprintln!("## first with Id('{}')", comp.0);
}

fn pre_pre_update(comp: &Id) {
    eprintln!("## pre_pre_update with Id('{}')", comp.0);
}

fn pre_update(comp: &Id) {
    eprintln!("## pre_update with Id('{}')", comp.0);
}

fn update(comp: &Id) {
    eprintln!("## update with Id('{}')", comp.0);
}

fn post_update(comp: &Id) {
    eprintln!("## post_update with Id('{}')", comp.0);
}

fn last(comp: &Id) {
    eprintln!("## last with Id('{}')", comp.0);
}

fn main() {
    // Note: that it doesn't matter in which order we register stage systems with the builder
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        // TODO: startup stages don't seem to work properly ATM
        // .add_system_to_stage(startup_stage::STARTUP, startup.system())
        .add_system_to_stage(stage::LAST, last.system())
        .add_system_to_stage(stage::FIRST, first.system())
        .add_system_to_stage(stage::PRE_UPDATE, pre_update.system())
        .add_system_to_stage(stage::POST_UPDATE, post_update.system())
        .add_system_to_stage(stage::UPDATE, update.system())
        .add_stage_before(stage::PRE_UPDATE, "pre_pre_update")
        .add_system_to_stage("pre_pre_update", pre_pre_update.system())
        .run();
}
