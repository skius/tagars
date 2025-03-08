mod module_bindings;

use clap::Parser;
use module_bindings::*;
use rand::Rng;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Instant;
use spacetimedb_sdk::{
    DbContext, Error, Event, Identity, ScheduleAt, Status, Table, TableWithPrimaryKey,
    TimeDuration, credentials,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The server to connect to.
    #[clap(short, long, default_value = "http://localhost:3000")]
    server: String,

    /// The amount of bots to spawn
    #[clap(short, long, default_value = "1")]
    bots: u32,
    
    /// Run multithreaded?
    #[clap(short, long)]
    multithreaded: bool,
    
    /// Run reducers?
    #[clap(short, long)]
    reducers: bool,
}

fn main() {
    let args = Args::parse();

    
    if !args.multithreaded {
        run_single_threaded(args);
    } else {
        let url = args.server;
        for _ in 0..args.bots {
            let url = url.clone();
            thread::spawn(|| {
                multiplayer_loop(url);
            });
        }

        loop {
            thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}

fn run_single_threaded(args: Args) {
    let url = args.server;
    let num = args.bots;
    
    let mut ctxs = Vec::new();
    for _ in 0..num {
        let ctx = connect_to_db(url.clone());
        ctxs.push(ctx);
    }

    let sleep_duration = std::time::Duration::from_millis(300);
    
    let mut rng = rand::rng();
    loop {
        for ctx in &ctxs {
            // pick a ranom direction on the unit circle
            let angle = rand::random::<f64>() * 2.0 * std::f64::consts::PI;
            // pick a random magnitude
            let magnitude = rng.random::<f64>() * 3.0;
            let impulse_x = angle.cos() * magnitude;
            let impulse_y = angle.sin() * magnitude;

            if args.reducers {
                ctx.reducers.apply_impulse(impulse_x, impulse_y).unwrap();
            }
            ctx.frame_tick().unwrap();
        }
        thread::sleep(sleep_duration);
    }
}

fn multiplayer_loop(url: String) {
    // Connect to the database
    let ctx = connect_to_db(url);

    // Register callbacks to run in response to database events.
    register_callbacks(&ctx);

    // Subscribe to SQL queries in order to construct a local partial replica of the database.
    subscribe_to_tables(&ctx);

    // Spawn a thread, where the connection will process messages and invoke callbacks.
    ctx.run_threaded();

    let mut rng = rand::thread_rng();
    let sleep_duration = std::time::Duration::from_millis(1000);

    // Handle input
    loop {
        // let wait_until = std::time::Instant::now() + sleep_duration;
        // while Instant::now() < wait_until {
        //     // spinloop
        // }
        thread::sleep(sleep_duration);
        
        
        // pick a ranom direction on the unit circle
        let angle = rand::random::<f64>() * 2.0 * std::f64::consts::PI;
        // pick a random magnitude
        let magnitude = rng.random::<f64>() * 3.0;
        let impulse_x = angle.cos() * magnitude;
        let impulse_y = angle.sin() * magnitude;

        ctx.reducers.apply_impulse(impulse_x, impulse_y).unwrap()
    }
}

/// The database name we chose when we published our module.
const DB_NAME: &str = "tagars";

/// Load credentials from a file and connect to the database.
fn connect_to_db(url: String) -> DbConnection {
    DbConnection::builder()
        // Register our `on_connect` callback, which will save our auth token.
        .on_connect(on_connected)
        // Register our `on_connect_error` callback, which will print a message, then exit the process.
        .on_connect_error(on_connect_error)
        // Our `on_disconnect` callback, which will print a message, then exit the process.
        .on_disconnect(on_disconnected)
        // Set the database name we chose when we called `spacetime publish`.
        .with_module_name(DB_NAME)
        // Set the URI of the SpacetimeDB host that's running our database.
        .with_uri(url)
        // Finalize configuration and connect!
        .build()
        .expect("Failed to connect")
}

/// Register all the callbacks our app will use to respond to database events.
fn register_callbacks(ctx: &DbConnection) {}

/// Register subscriptions for all rows of both tables.
fn subscribe_to_tables(ctx: &DbConnection) {
    // ctx.subscription_builder()
    //     .subscribe(["SELECT * FROM balls"]);
}

/// Our `on_connect` callback: save our credentials to a file.
fn on_connected(_ctx: &DbConnection, _identity: Identity, token: &str) {}

fn on_connect_error(_ctx: &ErrorContext, err: Error) {
    panic!("Failed to connect: {:?}", err);
}

fn on_disconnected(_ctx: &ErrorContext, err: Option<Error>) {
    panic!("Disconnected: {:?}", err);
}
