//! Kinetics reaction simulation binary.
//! Compute the change in concentration for a given set of reactions and initial concentrations.

use arctk::{
    args,
    chem::{Concentrations, Reactor, ReactorLinker},
    data::Table,
    file::{Load, Save},
    ord::{Link, Register},
    util::{
        banner::{section, title},
        dir,
    },
};
use arctk_attr::input;
use ndarray::Array1;
use std::{env::current_dir, path::PathBuf};

/// Parameter builder structure.
#[input]
pub struct Parameters {
    /// Initial concentrations.
    pub concs: Concentrations,
    /// Reactions.
    pub reactor: ReactorLinker,
    /// Total integration time [s].
    pub time: f64,
    /// Number of intermediate dumps.
    pub dumps: usize,
}

fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    title(term_width, "FLASK");

    section(term_width, "Initialisation");
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    let cwd = current_dir().expect("Failed to determine current working directory.");
    // let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
    let (in_dir, out_dir) = dir::io_dirs(Some(cwd.clone()), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");

    section(term_width, "Input");
    let params =
        Parameters::load(&in_dir.join(params_path)).expect("Failed to load parameters file.");

    section(term_width, "Building");
    let time = params.time;
    let dumps = params.dumps;

    section(term_width, "Registration");
    let mut names = params.concs.requires();
    names.append(&mut params.reactor.requires());
    let specs = Register::new(names);

    section(term_width, "Linking");
    let concs = params
        .concs
        .link(specs.set())
        .expect("Species link failure.");
    let reactor = params
        .reactor
        .link(specs.set())
        .expect("Species link failure.");

    section(term_width, "Simulation");
    let data = simulation(&specs, &reactor, concs, time, dumps);

    section(term_width, "Output");
    data.save(&out_dir.join("concs.csv"))
        .expect("Failed to save output data.");

    section(term_width, "Finished");
}

/// Run the simulation.
#[inline]
fn simulation(
    specs: &Register,
    reactor: &Reactor,
    mut concs: Array1<f64>,
    time: f64,
    dumps: usize,
) -> Table<f64> {
    debug_assert!(time > 0.0);

    let steps = dumps + 1;
    let dt = time / steps as f64;

    let mut data = Vec::with_capacity(dumps + 1);
    for _ in 0..steps {
        concs = evolve_rk4(&reactor, concs, dt);
        data.push(concs.to_vec());
    }

    Table::new(
        specs
            .names_list()
            .into_iter()
            .map(|x| format!("{}", x))
            .collect(),
        data,
    )
}

/// Evolve forward the given amount of time.
#[allow(dead_code)]
#[inline]
#[must_use]
fn evolve_euler(reactor: &Reactor, mut concs: Array1<f64>, time: f64) -> Array1<f64> {
    debug_assert!(time > 0.0);

    let n = 100;
    let dt = time / n as f64;

    for _ in 0..n {
        concs += &(&reactor.deltas(&concs) * dt);
    }

    concs
}

/// Evolve forward the given amount of time.
#[inline]
#[must_use]
fn evolve_rk4(reactor: &Reactor, mut concs: Array1<f64>, time: f64) -> Array1<f64> {
    debug_assert!(time > 0.0);

    let n = 100;
    let dt = time / n as f64;
    let half_dt = dt * 0.5;
    let sixth_dt = dt / 6.0;

    let mut k1;
    let mut k2;
    let mut k3;
    let mut k4;
    for _ in 0..n {
        k1 = reactor.deltas(&concs);
        k2 = reactor.deltas(&(&concs + &(&k1 * half_dt)));
        k3 = reactor.deltas(&(&concs + &(&k2 * half_dt)));
        k4 = reactor.deltas(&(&concs + &(&k3 * dt)));

        concs += &(&(&k1 + &(2.0 * (k2 + k3)) + &k4) * sixth_dt);
    }

    concs
}

/// Evolve forward the given amount of time.
/// But this is broken version.
#[allow(dead_code)]
#[inline]
#[must_use]
fn evolve_rk4_broken(reactor: &Reactor, mut concs: Array1<f64>, time: f64) -> Array1<f64> {
    debug_assert!(time > 0.0);

    let n = 100;
    let dt = time / n as f64;
    let half_dt = dt * 0.5;

    for _ in 0..n {
        let k1 = reactor.deltas(&concs);
        let k2 = reactor.deltas(&(&concs + &(k1 * half_dt)));
        let k3 = reactor.deltas(&(&concs + &(k2 * half_dt)));
        let k4 = reactor.deltas(&(&concs + &(k3 * dt)));

        concs += &(&k4 * dt);
    }

    concs
}
