use bevy::prelude::*;
use std::fmt;

const PRINT_STATE: bool = true;

// *******
// State {
#[derive(Default, Debug)]
struct State {
    startup_calls: u32,
    no_comps_calls: u32,
    named_calls: u32,
    rotated_calls: u32,
    // Either rotated only or also rotating (we don't know)
    rotation_calls: u32,
    rotating_calls: u32,
}

impl State {
    fn total_calls(&self) -> u32 {
        self.startup_calls
            + self.no_comps_calls
            + self.named_calls
            + self.rotated_calls
            + self.rotation_calls
            + self.rotating_calls
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if PRINT_STATE {
            f.write_str(&format!(
                r#"
    State:
      startup calls:  {}
      no comps calls: {}
      named calls:    {}
      rotated calls:  {}
      rotation calls: {}
      rotating calls: {}
      ------------------
      total calls:    {}
            "#,
                self.startup_calls,
                self.no_comps_calls,
                self.named_calls,
                self.rotated_calls,
                self.rotation_calls,
                self.rotating_calls,
                self.total_calls()
            ))
        } else {
            f.write_str("")
        }
    }
}
// } State
// *******

// *******
// Components {
struct Name(String);
struct RotationRadians(f32);
struct Rotating;
// } Components
// *******

// *******
// Entities {

#[derive(Bundle)]
struct NamedBunny {
    name: Name,
}

#[derive(Bundle)]
struct RotatedNamedBunny {
    name: Name,
    rotation: RotationRadians,
}

#[derive(Bundle)]
struct RotatingNamedBunny {
    name: Name,
    rotation: RotationRadians,
    // This property is not used  but just distinguishes this as rotating vs. rotated
    _rotating: Rotating,
}

// } Entities
// *******

// *******
// Normal Systems {
fn no_comps_system(mut state: ResMut<State>) {
    state.no_comps_calls += 1;
    eprintln!("## Ran no comps system:\n{}", *state);
}

fn named_system(mut state: ResMut<State>, name: &Name) {
    state.named_calls += 1;
    eprintln!("## Ran named system: with Name('{}')\n{}", name.0, *state);
}

// NOTE: that we cannot apply With/Without clauses via the For-Each system but need
// to do this in a query system.
// As a result this matches both rotated and rotating bunnies.
//
// @see rotated_named_query_system below.
// @see rotating_named_query_system below.
fn rotation_system(mut state: ResMut<State>, rotation: &RotationRadians) {
    state.rotation_calls += 1;
    eprintln!(
        "## Ran rotation system: with RotationRadians({})\n{}",
        rotation.0, *state
    );
}

fn rotation_named_system(mut state: ResMut<State>, rotation: &RotationRadians, name: &Name) {
    state.rotation_calls += 1;
    state.named_calls += 1;
    eprintln!(
        "## Ran rotation named system: with RotationRadians({}) Name('{}')\n{}",
        rotation.0, name.0, *state
    );
}

// Matches named bunnies that have a RotationRadians component but not a Rotating component
fn rotated_named_query_system(
    mut state: ResMut<State>,
    mut query: Query<Without<Rotating, (&RotationRadians, &Name)>>,
) {
    for (rotation, name) in &mut query.iter() {
        state.rotated_calls += 1;
        state.named_calls += 1;
        eprintln!(
            "## Ran rotated named query iteration: with RotationRadians({}) Name('{}')\n{}",
            rotation.0, name.0, *state
        );
    }
}

// Matches named bunnies that have a RotationRadians component and also a Rotating component
fn rotating_named_query_system(
    mut state: ResMut<State>,
    mut query: Query<With<Rotating, (&mut RotationRadians, &Name)>>,
) {
    for (mut rotation, name) in &mut query.iter() {
        state.rotating_calls += 1;
        state.named_calls += 1;
        rotation.0 += 1.0;
        eprintln!(
            "## Ran rotating named query iteration: with RotationRadians({}) Name('{}')\n{}",
            rotation.0, name.0, *state
        );
    }
}
// } Normal Systems
// *******

// *******
// Startup Systems {
fn startup_system(mut commands: Commands, mut state: ResMut<State>) {
    state.startup_calls += 1;

    commands
        .spawn(NamedBunny {
            name: Name("bugs bunny".to_string()),
        })
        .spawn(RotatedNamedBunny {
            name: Name("rotated bugs bunny".to_string()),
            rotation: RotationRadians(90_f32.to_radians()),
        })
        .spawn(RotatedNamedBunny {
            name: Name("rotated easter bunny".to_string()),
            rotation: RotationRadians(180_f32.to_radians()),
        })
        .spawn(RotatingNamedBunny {
            name: Name("rotating dizzy bunny".to_string()),
            rotation: RotationRadians(0_f32.to_radians()),
            _rotating: Rotating,
        });
    eprintln!("## Ran startup system:\n{}", *state);
}
// } Startup
// *******

fn main() {
    // We intentionally don't add an event loop here to just run through the systems once.
    App::build()
        .init_resource::<State>()
        .add_startup_system(startup_system.system())
        .add_system(no_comps_system.system())
        .add_system(named_system.system())
        .add_system(rotation_system.system())
        .add_system(rotation_named_system.system())
        .add_system(rotated_named_query_system.system())
        .add_system(rotating_named_query_system.system())
        .run();
}
