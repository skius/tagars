// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

pub mod apply_impulse_reducer;
pub mod ball_type;
pub mod balls_table;
pub mod food_type;
pub mod foods_table;
pub mod identity_connected_reducer;
pub mod identity_disconnected_reducer;
pub mod physics_tick_type;
pub mod physics_ticks_table;
pub mod respawn_ball_reducer;
pub mod respawn_balls_schedule_table;
pub mod respawn_balls_schedule_type;
pub mod rgb_type;
pub mod spawn_food_reducer;
pub mod spawn_food_schedule_type;
pub mod spawn_foods_schedule_table;
pub mod update_balls_reducer;
pub mod update_balls_schedule_table;
pub mod update_balls_schedule_type;

pub use apply_impulse_reducer::{
    ApplyImpulseCallbackId, apply_impulse, set_flags_for_apply_impulse,
};
pub use ball_type::Ball;
pub use balls_table::*;
pub use food_type::Food;
pub use foods_table::*;
pub use identity_connected_reducer::{
    IdentityConnectedCallbackId, identity_connected, set_flags_for_identity_connected,
};
pub use identity_disconnected_reducer::{
    IdentityDisconnectedCallbackId, identity_disconnected, set_flags_for_identity_disconnected,
};
pub use physics_tick_type::PhysicsTick;
pub use physics_ticks_table::*;
pub use respawn_ball_reducer::{RespawnBallCallbackId, respawn_ball, set_flags_for_respawn_ball};
pub use respawn_balls_schedule_table::*;
pub use respawn_balls_schedule_type::RespawnBallsSchedule;
pub use rgb_type::Rgb;
pub use spawn_food_reducer::{SpawnFoodCallbackId, set_flags_for_spawn_food, spawn_food};
pub use spawn_food_schedule_type::SpawnFoodSchedule;
pub use spawn_foods_schedule_table::*;
pub use update_balls_reducer::{UpdateBallsCallbackId, set_flags_for_update_balls, update_balls};
pub use update_balls_schedule_table::*;
pub use update_balls_schedule_type::UpdateBallsSchedule;

#[derive(Clone, PartialEq, Debug)]

/// One of the reducers defined by this module.
///
/// Contained within a [`__sdk::ReducerEvent`] in [`EventContext`]s for reducer events
/// to indicate which reducer caused the event.

pub enum Reducer {
    ApplyImpulse { impulse_x: f64, impulse_y: f64 },
    IdentityConnected,
    IdentityDisconnected,
    RespawnBall { schedule: RespawnBallsSchedule },
    SpawnFood { schedule: SpawnFoodSchedule },
    UpdateBalls { schedule: UpdateBallsSchedule },
}

impl __sdk::InModule for Reducer {
    type Module = RemoteModule;
}

impl __sdk::Reducer for Reducer {
    fn reducer_name(&self) -> &'static str {
        match self {
            Reducer::ApplyImpulse { .. } => "apply_impulse",
            Reducer::IdentityConnected => "identity_connected",
            Reducer::IdentityDisconnected => "identity_disconnected",
            Reducer::RespawnBall { .. } => "respawn_ball",
            Reducer::SpawnFood { .. } => "spawn_food",
            Reducer::UpdateBalls { .. } => "update_balls",
        }
    }
}
impl TryFrom<__ws::ReducerCallInfo<__ws::BsatnFormat>> for Reducer {
    type Error = __sdk::Error;
    fn try_from(value: __ws::ReducerCallInfo<__ws::BsatnFormat>) -> __sdk::Result<Self> {
        match &value.reducer_name[..] {
            "apply_impulse" => Ok(__sdk::parse_reducer_args::<
                apply_impulse_reducer::ApplyImpulseArgs,
            >("apply_impulse", &value.args)?
            .into()),
            "identity_connected" => Ok(__sdk::parse_reducer_args::<
                identity_connected_reducer::IdentityConnectedArgs,
            >("identity_connected", &value.args)?
            .into()),
            "identity_disconnected" => Ok(__sdk::parse_reducer_args::<
                identity_disconnected_reducer::IdentityDisconnectedArgs,
            >("identity_disconnected", &value.args)?
            .into()),
            "respawn_ball" => Ok(
                __sdk::parse_reducer_args::<respawn_ball_reducer::RespawnBallArgs>(
                    "respawn_ball",
                    &value.args,
                )?
                .into(),
            ),
            "spawn_food" => Ok(
                __sdk::parse_reducer_args::<spawn_food_reducer::SpawnFoodArgs>(
                    "spawn_food",
                    &value.args,
                )?
                .into(),
            ),
            "update_balls" => Ok(
                __sdk::parse_reducer_args::<update_balls_reducer::UpdateBallsArgs>(
                    "update_balls",
                    &value.args,
                )?
                .into(),
            ),
            unknown => {
                Err(
                    __sdk::InternalError::unknown_name("reducer", unknown, "ReducerCallInfo")
                        .into(),
                )
            }
        }
    }
}

#[derive(Default)]
#[allow(non_snake_case)]
#[doc(hidden)]
pub struct DbUpdate {
    balls: __sdk::TableUpdate<Ball>,
    foods: __sdk::TableUpdate<Food>,
    physics_ticks: __sdk::TableUpdate<PhysicsTick>,
    respawn_balls_schedule: __sdk::TableUpdate<RespawnBallsSchedule>,
    spawn_foods_schedule: __sdk::TableUpdate<SpawnFoodSchedule>,
    update_balls_schedule: __sdk::TableUpdate<UpdateBallsSchedule>,
}

impl TryFrom<__ws::DatabaseUpdate<__ws::BsatnFormat>> for DbUpdate {
    type Error = __sdk::Error;
    fn try_from(raw: __ws::DatabaseUpdate<__ws::BsatnFormat>) -> Result<Self, Self::Error> {
        let mut db_update = DbUpdate::default();
        for table_update in raw.tables {
            match &table_update.table_name[..] {
                "balls" => db_update.balls = balls_table::parse_table_update(table_update)?,
                "foods" => db_update.foods = foods_table::parse_table_update(table_update)?,
                "physics_ticks" => {
                    db_update.physics_ticks = physics_ticks_table::parse_table_update(table_update)?
                }
                "respawn_balls_schedule" => {
                    db_update.respawn_balls_schedule =
                        respawn_balls_schedule_table::parse_table_update(table_update)?
                }
                "spawn_foods_schedule" => {
                    db_update.spawn_foods_schedule =
                        spawn_foods_schedule_table::parse_table_update(table_update)?
                }
                "update_balls_schedule" => {
                    db_update.update_balls_schedule =
                        update_balls_schedule_table::parse_table_update(table_update)?
                }

                unknown => {
                    return Err(__sdk::InternalError::unknown_name(
                        "table",
                        unknown,
                        "DatabaseUpdate",
                    )
                    .into());
                }
            }
        }
        Ok(db_update)
    }
}

impl __sdk::InModule for DbUpdate {
    type Module = RemoteModule;
}

impl __sdk::DbUpdate for DbUpdate {
    fn apply_to_client_cache(
        &self,
        cache: &mut __sdk::ClientCache<RemoteModule>,
    ) -> AppliedDiff<'_> {
        let mut diff = AppliedDiff::default();

        diff.balls = cache
            .apply_diff_to_table::<Ball>("balls", &self.balls)
            .with_updates_by_pk(|row| &row.identity);
        diff.foods = cache
            .apply_diff_to_table::<Food>("foods", &self.foods)
            .with_updates_by_pk(|row| &row.id);
        diff.physics_ticks = cache
            .apply_diff_to_table::<PhysicsTick>("physics_ticks", &self.physics_ticks)
            .with_updates_by_pk(|row| &row.tick_id);
        diff.respawn_balls_schedule = cache
            .apply_diff_to_table::<RespawnBallsSchedule>(
                "respawn_balls_schedule",
                &self.respawn_balls_schedule,
            )
            .with_updates_by_pk(|row| &row.scheduled_id);
        diff.spawn_foods_schedule = cache
            .apply_diff_to_table::<SpawnFoodSchedule>(
                "spawn_foods_schedule",
                &self.spawn_foods_schedule,
            )
            .with_updates_by_pk(|row| &row.scheduled_id);
        diff.update_balls_schedule = cache
            .apply_diff_to_table::<UpdateBallsSchedule>(
                "update_balls_schedule",
                &self.update_balls_schedule,
            )
            .with_updates_by_pk(|row| &row.scheduled_id);

        diff
    }
}

#[derive(Default)]
#[allow(non_snake_case)]
#[doc(hidden)]
pub struct AppliedDiff<'r> {
    balls: __sdk::TableAppliedDiff<'r, Ball>,
    foods: __sdk::TableAppliedDiff<'r, Food>,
    physics_ticks: __sdk::TableAppliedDiff<'r, PhysicsTick>,
    respawn_balls_schedule: __sdk::TableAppliedDiff<'r, RespawnBallsSchedule>,
    spawn_foods_schedule: __sdk::TableAppliedDiff<'r, SpawnFoodSchedule>,
    update_balls_schedule: __sdk::TableAppliedDiff<'r, UpdateBallsSchedule>,
}

impl __sdk::InModule for AppliedDiff<'_> {
    type Module = RemoteModule;
}

impl<'r> __sdk::AppliedDiff<'r> for AppliedDiff<'r> {
    fn invoke_row_callbacks(
        &self,
        event: &EventContext,
        callbacks: &mut __sdk::DbCallbacks<RemoteModule>,
    ) {
        callbacks.invoke_table_row_callbacks::<Ball>("balls", &self.balls, event);
        callbacks.invoke_table_row_callbacks::<Food>("foods", &self.foods, event);
        callbacks.invoke_table_row_callbacks::<PhysicsTick>(
            "physics_ticks",
            &self.physics_ticks,
            event,
        );
        callbacks.invoke_table_row_callbacks::<RespawnBallsSchedule>(
            "respawn_balls_schedule",
            &self.respawn_balls_schedule,
            event,
        );
        callbacks.invoke_table_row_callbacks::<SpawnFoodSchedule>(
            "spawn_foods_schedule",
            &self.spawn_foods_schedule,
            event,
        );
        callbacks.invoke_table_row_callbacks::<UpdateBallsSchedule>(
            "update_balls_schedule",
            &self.update_balls_schedule,
            event,
        );
    }
}

#[doc(hidden)]
pub struct RemoteModule;

impl __sdk::InModule for RemoteModule {
    type Module = Self;
}

/// The `reducers` field of [`EventContext`] and [`DbConnection`],
/// with methods provided by extension traits for each reducer defined by the module.
pub struct RemoteReducers {
    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::InModule for RemoteReducers {
    type Module = RemoteModule;
}

#[doc(hidden)]
/// The `set_reducer_flags` field of [`DbConnection`],
/// with methods provided by extension traits for each reducer defined by the module.
/// Each method sets the flags for the reducer with the same name.
///
/// This type is currently unstable and may be removed without a major version bump.
pub struct SetReducerFlags {
    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::InModule for SetReducerFlags {
    type Module = RemoteModule;
}

/// The `db` field of [`EventContext`] and [`DbConnection`],
/// with methods provided by extension traits for each table defined by the module.
pub struct RemoteTables {
    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::InModule for RemoteTables {
    type Module = RemoteModule;
}

/// A connection to a remote module, including a materialized view of a subset of the database.
///
/// Connect to a remote module by calling [`DbConnection::builder`]
/// and using the [`__sdk::DbConnectionBuilder`] builder-pattern constructor.
///
/// You must explicitly advance the connection by calling any one of:
///
/// - [`DbConnection::frame_tick`].
/// - [`DbConnection::run_threaded`].
/// - [`DbConnection::run_async`].
/// - [`DbConnection::advance_one_message`].
/// - [`DbConnection::advance_one_message_blocking`].
/// - [`DbConnection::advance_one_message_async`].
///
/// Which of these methods you should call depends on the specific needs of your application,
/// but you must call one of them, or else the connection will never progress.
pub struct DbConnection {
    /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
    pub db: RemoteTables,
    /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
    pub reducers: RemoteReducers,
    #[doc(hidden)]
    /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
    /// via extension traits implemented for [`SetReducerFlags`].
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    pub set_reducer_flags: SetReducerFlags,

    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::InModule for DbConnection {
    type Module = RemoteModule;
}

impl __sdk::DbContext for DbConnection {
    type DbView = RemoteTables;
    type Reducers = RemoteReducers;
    type SetReducerFlags = SetReducerFlags;

    fn db(&self) -> &Self::DbView {
        &self.db
    }
    fn reducers(&self) -> &Self::Reducers {
        &self.reducers
    }
    fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
        &self.set_reducer_flags
    }

    fn is_active(&self) -> bool {
        self.imp.is_active()
    }

    fn disconnect(&self) -> __sdk::Result<()> {
        self.imp.disconnect()
    }

    type SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>;

    fn subscription_builder(&self) -> Self::SubscriptionBuilder {
        __sdk::SubscriptionBuilder::new(&self.imp)
    }

    fn try_identity(&self) -> Option<__sdk::Identity> {
        self.imp.try_identity()
    }
    fn connection_id(&self) -> __sdk::ConnectionId {
        self.imp.connection_id()
    }
}

impl DbConnection {
    /// Builder-pattern constructor for a connection to a remote module.
    ///
    /// See [`__sdk::DbConnectionBuilder`] for required and optional configuration for the new connection.
    pub fn builder() -> __sdk::DbConnectionBuilder<RemoteModule> {
        __sdk::DbConnectionBuilder::new()
    }

    /// If any WebSocket messages are waiting, process one of them.
    ///
    /// Returns `true` if a message was processed, or `false` if the queue is empty.
    /// Callers should invoke this message in a loop until it returns `false`
    /// or for as much time is available to process messages.
    ///
    /// Returns an error if the connection is disconnected.
    /// If the disconnection in question was normal,
    ///  i.e. the result of a call to [`__sdk::DbContext::disconnect`],
    /// the returned error will be downcastable to [`__sdk::DisconnectedError`].
    ///
    /// This is a low-level primitive exposed for power users who need significant control over scheduling.
    /// Most applications should call [`Self::frame_tick`] each frame
    /// to fully exhaust the queue whenever time is available.
    pub fn advance_one_message(&self) -> __sdk::Result<bool> {
        self.imp.advance_one_message()
    }

    /// Process one WebSocket message, potentially blocking the current thread until one is received.
    ///
    /// Returns an error if the connection is disconnected.
    /// If the disconnection in question was normal,
    ///  i.e. the result of a call to [`__sdk::DbContext::disconnect`],
    /// the returned error will be downcastable to [`__sdk::DisconnectedError`].
    ///
    /// This is a low-level primitive exposed for power users who need significant control over scheduling.
    /// Most applications should call [`Self::run_threaded`] to spawn a thread
    /// which advances the connection automatically.
    pub fn advance_one_message_blocking(&self) -> __sdk::Result<()> {
        self.imp.advance_one_message_blocking()
    }

    /// Process one WebSocket message, `await`ing until one is received.
    ///
    /// Returns an error if the connection is disconnected.
    /// If the disconnection in question was normal,
    ///  i.e. the result of a call to [`__sdk::DbContext::disconnect`],
    /// the returned error will be downcastable to [`__sdk::DisconnectedError`].
    ///
    /// This is a low-level primitive exposed for power users who need significant control over scheduling.
    /// Most applications should call [`Self::run_async`] to run an `async` loop
    /// which advances the connection when polled.
    pub async fn advance_one_message_async(&self) -> __sdk::Result<()> {
        self.imp.advance_one_message_async().await
    }

    /// Process all WebSocket messages waiting in the queue,
    /// then return without `await`ing or blocking the current thread.
    pub fn frame_tick(&self) -> __sdk::Result<()> {
        self.imp.frame_tick()
    }

    /// Spawn a thread which processes WebSocket messages as they are received.
    pub fn run_threaded(&self) -> std::thread::JoinHandle<()> {
        self.imp.run_threaded()
    }

    /// Run an `async` loop which processes WebSocket messages when polled.
    pub async fn run_async(&self) -> __sdk::Result<()> {
        self.imp.run_async().await
    }
}

impl __sdk::DbConnection for DbConnection {
    fn new(imp: __sdk::DbContextImpl<RemoteModule>) -> Self {
        Self {
            db: RemoteTables { imp: imp.clone() },
            reducers: RemoteReducers { imp: imp.clone() },
            set_reducer_flags: SetReducerFlags { imp: imp.clone() },
            imp,
        }
    }
}

/// A handle on a subscribed query.
// TODO: Document this better after implementing the new subscription API.
#[derive(Clone)]
pub struct SubscriptionHandle {
    imp: __sdk::SubscriptionHandleImpl<RemoteModule>,
}

impl __sdk::InModule for SubscriptionHandle {
    type Module = RemoteModule;
}

impl __sdk::SubscriptionHandle for SubscriptionHandle {
    fn new(imp: __sdk::SubscriptionHandleImpl<RemoteModule>) -> Self {
        Self { imp }
    }

    /// Returns true if this subscription has been terminated due to an unsubscribe call or an error.
    fn is_ended(&self) -> bool {
        self.imp.is_ended()
    }

    /// Returns true if this subscription has been applied and has not yet been unsubscribed.
    fn is_active(&self) -> bool {
        self.imp.is_active()
    }

    /// Unsubscribe from the query controlled by this `SubscriptionHandle`,
    /// then run `on_end` when its rows are removed from the client cache.
    fn unsubscribe_then(self, on_end: __sdk::OnEndedCallback<RemoteModule>) -> __sdk::Result<()> {
        self.imp.unsubscribe_then(Some(on_end))
    }

    fn unsubscribe(self) -> __sdk::Result<()> {
        self.imp.unsubscribe_then(None)
    }
}

/// Alias trait for a [`__sdk::DbContext`] connected to this module,
/// with that trait's associated types bounded to this module's concrete types.
///
/// Users can use this trait as a boundary on definitions which should accept
/// either a [`DbConnection`] or an [`EventContext`] and operate on either.
pub trait RemoteDbContext:
    __sdk::DbContext<
        DbView = RemoteTables,
        Reducers = RemoteReducers,
        SetReducerFlags = SetReducerFlags,
        SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>,
    >
{
}
impl<
    Ctx: __sdk::DbContext<
            DbView = RemoteTables,
            Reducers = RemoteReducers,
            SetReducerFlags = SetReducerFlags,
            SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>,
        >,
> RemoteDbContext for Ctx
{
}

/// An [`__sdk::DbContext`] augmented with a [`__sdk::Event`],
/// passed to [`__sdk::Table::on_insert`], [`__sdk::Table::on_delete`] and [`__sdk::TableWithPrimaryKey::on_update`] callbacks.
pub struct EventContext {
    /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
    pub db: RemoteTables,
    /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
    pub reducers: RemoteReducers,
    /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
    /// via extension traits implemented for [`SetReducerFlags`].
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    pub set_reducer_flags: SetReducerFlags,
    /// The event which caused these callbacks to run.
    pub event: __sdk::Event<Reducer>,
    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::AbstractEventContext for EventContext {
    type Event = __sdk::Event<Reducer>;
    fn event(&self) -> &Self::Event {
        &self.event
    }
    fn new(imp: __sdk::DbContextImpl<RemoteModule>, event: Self::Event) -> Self {
        Self {
            db: RemoteTables { imp: imp.clone() },
            reducers: RemoteReducers { imp: imp.clone() },
            set_reducer_flags: SetReducerFlags { imp: imp.clone() },
            event,
            imp,
        }
    }
}

impl __sdk::InModule for EventContext {
    type Module = RemoteModule;
}

impl __sdk::DbContext for EventContext {
    type DbView = RemoteTables;
    type Reducers = RemoteReducers;
    type SetReducerFlags = SetReducerFlags;

    fn db(&self) -> &Self::DbView {
        &self.db
    }
    fn reducers(&self) -> &Self::Reducers {
        &self.reducers
    }
    fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
        &self.set_reducer_flags
    }

    fn is_active(&self) -> bool {
        self.imp.is_active()
    }

    fn disconnect(&self) -> __sdk::Result<()> {
        self.imp.disconnect()
    }

    type SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>;

    fn subscription_builder(&self) -> Self::SubscriptionBuilder {
        __sdk::SubscriptionBuilder::new(&self.imp)
    }

    fn try_identity(&self) -> Option<__sdk::Identity> {
        self.imp.try_identity()
    }
    fn connection_id(&self) -> __sdk::ConnectionId {
        self.imp.connection_id()
    }
}

impl __sdk::EventContext for EventContext {}

/// An [`__sdk::DbContext`] augmented with a [`__sdk::ReducerEvent`],
/// passed to on-reducer callbacks.
pub struct ReducerEventContext {
    /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
    pub db: RemoteTables,
    /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
    pub reducers: RemoteReducers,
    /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
    /// via extension traits implemented for [`SetReducerFlags`].
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    pub set_reducer_flags: SetReducerFlags,
    /// The event which caused these callbacks to run.
    pub event: __sdk::ReducerEvent<Reducer>,
    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::AbstractEventContext for ReducerEventContext {
    type Event = __sdk::ReducerEvent<Reducer>;
    fn event(&self) -> &Self::Event {
        &self.event
    }
    fn new(imp: __sdk::DbContextImpl<RemoteModule>, event: Self::Event) -> Self {
        Self {
            db: RemoteTables { imp: imp.clone() },
            reducers: RemoteReducers { imp: imp.clone() },
            set_reducer_flags: SetReducerFlags { imp: imp.clone() },
            event,
            imp,
        }
    }
}

impl __sdk::InModule for ReducerEventContext {
    type Module = RemoteModule;
}

impl __sdk::DbContext for ReducerEventContext {
    type DbView = RemoteTables;
    type Reducers = RemoteReducers;
    type SetReducerFlags = SetReducerFlags;

    fn db(&self) -> &Self::DbView {
        &self.db
    }
    fn reducers(&self) -> &Self::Reducers {
        &self.reducers
    }
    fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
        &self.set_reducer_flags
    }

    fn is_active(&self) -> bool {
        self.imp.is_active()
    }

    fn disconnect(&self) -> __sdk::Result<()> {
        self.imp.disconnect()
    }

    type SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>;

    fn subscription_builder(&self) -> Self::SubscriptionBuilder {
        __sdk::SubscriptionBuilder::new(&self.imp)
    }

    fn try_identity(&self) -> Option<__sdk::Identity> {
        self.imp.try_identity()
    }
    fn connection_id(&self) -> __sdk::ConnectionId {
        self.imp.connection_id()
    }
}

impl __sdk::ReducerEventContext for ReducerEventContext {}

/// An [`__sdk::DbContext`] passed to [`__sdk::SubscriptionBuilder::on_applied`] and [`SubscriptionHandle::unsubscribe_then`] callbacks.
pub struct SubscriptionEventContext {
    /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
    pub db: RemoteTables,
    /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
    pub reducers: RemoteReducers,
    /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
    /// via extension traits implemented for [`SetReducerFlags`].
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    pub set_reducer_flags: SetReducerFlags,
    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::AbstractEventContext for SubscriptionEventContext {
    type Event = ();
    fn event(&self) -> &Self::Event {
        &()
    }
    fn new(imp: __sdk::DbContextImpl<RemoteModule>, _event: Self::Event) -> Self {
        Self {
            db: RemoteTables { imp: imp.clone() },
            reducers: RemoteReducers { imp: imp.clone() },
            set_reducer_flags: SetReducerFlags { imp: imp.clone() },
            imp,
        }
    }
}

impl __sdk::InModule for SubscriptionEventContext {
    type Module = RemoteModule;
}

impl __sdk::DbContext for SubscriptionEventContext {
    type DbView = RemoteTables;
    type Reducers = RemoteReducers;
    type SetReducerFlags = SetReducerFlags;

    fn db(&self) -> &Self::DbView {
        &self.db
    }
    fn reducers(&self) -> &Self::Reducers {
        &self.reducers
    }
    fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
        &self.set_reducer_flags
    }

    fn is_active(&self) -> bool {
        self.imp.is_active()
    }

    fn disconnect(&self) -> __sdk::Result<()> {
        self.imp.disconnect()
    }

    type SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>;

    fn subscription_builder(&self) -> Self::SubscriptionBuilder {
        __sdk::SubscriptionBuilder::new(&self.imp)
    }

    fn try_identity(&self) -> Option<__sdk::Identity> {
        self.imp.try_identity()
    }
    fn connection_id(&self) -> __sdk::ConnectionId {
        self.imp.connection_id()
    }
}

impl __sdk::SubscriptionEventContext for SubscriptionEventContext {}

/// An [`__sdk::DbContext`] augmented with a [`__sdk::Error`],
/// passed to [`__sdk::DbConnectionBuilder::on_disconnect`], [`__sdk::DbConnectionBuilder::on_connect_error`] and [`__sdk::SubscriptionBuilder::on_error`] callbacks.
pub struct ErrorContext {
    /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
    pub db: RemoteTables,
    /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
    pub reducers: RemoteReducers,
    /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
    /// via extension traits implemented for [`SetReducerFlags`].
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    pub set_reducer_flags: SetReducerFlags,
    /// The event which caused these callbacks to run.
    pub event: Option<__sdk::Error>,
    imp: __sdk::DbContextImpl<RemoteModule>,
}

impl __sdk::AbstractEventContext for ErrorContext {
    type Event = Option<__sdk::Error>;
    fn event(&self) -> &Self::Event {
        &self.event
    }
    fn new(imp: __sdk::DbContextImpl<RemoteModule>, event: Self::Event) -> Self {
        Self {
            db: RemoteTables { imp: imp.clone() },
            reducers: RemoteReducers { imp: imp.clone() },
            set_reducer_flags: SetReducerFlags { imp: imp.clone() },
            event,
            imp,
        }
    }
}

impl __sdk::InModule for ErrorContext {
    type Module = RemoteModule;
}

impl __sdk::DbContext for ErrorContext {
    type DbView = RemoteTables;
    type Reducers = RemoteReducers;
    type SetReducerFlags = SetReducerFlags;

    fn db(&self) -> &Self::DbView {
        &self.db
    }
    fn reducers(&self) -> &Self::Reducers {
        &self.reducers
    }
    fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
        &self.set_reducer_flags
    }

    fn is_active(&self) -> bool {
        self.imp.is_active()
    }

    fn disconnect(&self) -> __sdk::Result<()> {
        self.imp.disconnect()
    }

    type SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>;

    fn subscription_builder(&self) -> Self::SubscriptionBuilder {
        __sdk::SubscriptionBuilder::new(&self.imp)
    }

    fn try_identity(&self) -> Option<__sdk::Identity> {
        self.imp.try_identity()
    }
    fn connection_id(&self) -> __sdk::ConnectionId {
        self.imp.connection_id()
    }
}

impl __sdk::ErrorContext for ErrorContext {}

impl __sdk::SpacetimeModule for RemoteModule {
    type DbConnection = DbConnection;
    type EventContext = EventContext;
    type ReducerEventContext = ReducerEventContext;
    type SubscriptionEventContext = SubscriptionEventContext;
    type ErrorContext = ErrorContext;
    type Reducer = Reducer;
    type DbView = RemoteTables;
    type Reducers = RemoteReducers;
    type SetReducerFlags = SetReducerFlags;
    type DbUpdate = DbUpdate;
    type AppliedDiff<'r> = AppliedDiff<'r>;
    type SubscriptionHandle = SubscriptionHandle;

    fn register_tables(client_cache: &mut __sdk::ClientCache<Self>) {
        balls_table::register_table(client_cache);
        foods_table::register_table(client_cache);
        physics_ticks_table::register_table(client_cache);
        respawn_balls_schedule_table::register_table(client_cache);
        spawn_foods_schedule_table::register_table(client_cache);
        update_balls_schedule_table::register_table(client_cache);
    }
}
