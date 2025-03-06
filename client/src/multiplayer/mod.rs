mod module_bindings;

use std::sync::mpsc::{Receiver, Sender};
use module_bindings::*;

use spacetimedb_sdk::{credentials, DbContext, Error, Event, Identity, ScheduleAt, Status, Table, TableWithPrimaryKey, TimeDuration};

pub use module_bindings::Ball;




#[derive(Debug)]
pub enum ReceiveMessage {
    NewBall(Ball),
    UpdateBall(Ball),
    DeleteBall(Identity),
    OurIdentity(Identity),
}

pub enum SendMessage {
    Impulse(f64, f64)
}



/// Connect to the server at the given URL.
///
/// Returns a receiver for incoming messages and a sender for outgoing messages.
pub fn connect_to(url: String) -> anyhow::Result<(Receiver<ReceiveMessage>, Sender<SendMessage>)> {
    let (receive_tx, receive_rx) = std::sync::mpsc::channel();
    let (send_tx, send_rx) = std::sync::mpsc::channel();



    std::thread::spawn(move || {
        multiplayer_loop(url, receive_tx, send_rx);
    });
    Ok((receive_rx, send_tx))
}

fn multiplayer_loop(url: String, receive_tx: Sender<ReceiveMessage>, send_rx: Receiver<SendMessage>) {
    // Connect to the database
    let ctx = connect_to_db(url);

    // Register callbacks to run in response to database events.
    register_callbacks(&ctx, receive_tx.clone());

    // Subscribe to SQL queries in order to construct a local partial replica of the database.
    subscribe_to_tables(&ctx);

    // Spawn a thread, where the connection will process messages and invoke callbacks.
    ctx.run_threaded();

    while ctx.try_identity().is_none() {
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    receive_tx.send(ReceiveMessage::OurIdentity(ctx.identity())).unwrap();

    // Handle input
    loop {
        match send_rx.recv() {
            Ok(SendMessage::Impulse(x, y)) => {
                ctx.reducers.apply_impulse(x, y).unwrap()
            }
            Err(_) => break,
        }
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
        // If the user has previously connected, we'll have saved a token in the `on_connect` callback.
        // In that case, we'll load it and pass it to `with_token`,
        // so we can re-authenticate as the same `Identity`.
        .with_token(creds_store().load().expect("Error loading credentials"))
        // Set the database name we chose when we called `spacetime publish`.
        .with_module_name(DB_NAME)
        // Set the URI of the SpacetimeDB host that's running our database.
        .with_uri(url)
        // Finalize configuration and connect!
        .build()
        .expect("Failed to connect")
}

/// Register all the callbacks our app will use to respond to database events.
fn register_callbacks(ctx: &DbConnection, tx: Sender<ReceiveMessage>) {
    {
        let tx = tx.clone();
        ctx.db.balls().on_insert(move |ctx, ball| {
            tx.send(ReceiveMessage::NewBall(ball.clone())).unwrap();
        });
    }

    {
        let tx = tx.clone();
        ctx.db.balls().on_update(move |ctx, old_ball, new_ball| {
            tx.send(ReceiveMessage::UpdateBall(new_ball.clone())).unwrap();
        });
    }

    {
        let tx = tx.clone();
        ctx.db.balls().on_delete(move |ctx, ball| {
            tx.send(ReceiveMessage::DeleteBall(ball.identity)).unwrap();
        });
    }

    // TODO: callbacks for reducers
}

/// Register subscriptions for all rows of both tables.
fn subscribe_to_tables(ctx: &DbConnection) {
    ctx.subscription_builder()
        .subscribe(["SELECT * FROM balls"]);
}

fn creds_store() -> credentials::File {
    let rand_num = rand::random::<u64>();
    let filename = format!("{}-{}", DB_NAME, rand_num);
    credentials::File::new(filename)
}

/// Our `on_connect` callback: save our credentials to a file.
fn on_connected(_ctx: &DbConnection, _identity: Identity, token: &str) {
    if let Err(e) = creds_store().save(token) {
        eprintln!("Failed to save credentials: {:?}", e);
    }
}

fn on_connect_error(_ctx: &ErrorContext, err: Error) {
    panic!("Failed to connect: {:?}", err);
}

fn on_disconnected(_ctx: &ErrorContext, err: Option<Error>) {
    panic!("Disconnected: {:?}", err);
}