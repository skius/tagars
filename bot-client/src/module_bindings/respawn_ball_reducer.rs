// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

use super::respawn_balls_schedule_type::RespawnBallsSchedule;

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct RespawnBallArgs {
    pub schedule: RespawnBallsSchedule,
}

impl From<RespawnBallArgs> for super::Reducer {
    fn from(args: RespawnBallArgs) -> Self {
        Self::RespawnBall {
            schedule: args.schedule,
        }
    }
}

impl __sdk::InModule for RespawnBallArgs {
    type Module = super::RemoteModule;
}

pub struct RespawnBallCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `respawn_ball`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait respawn_ball {
    /// Request that the remote module invoke the reducer `respawn_ball` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_respawn_ball`] callbacks.
    fn respawn_ball(&self, schedule: RespawnBallsSchedule) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `respawn_ball`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`RespawnBallCallbackId`] can be passed to [`Self::remove_on_respawn_ball`]
    /// to cancel the callback.
    fn on_respawn_ball(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &RespawnBallsSchedule) + Send + 'static,
    ) -> RespawnBallCallbackId;
    /// Cancel a callback previously registered by [`Self::on_respawn_ball`],
    /// causing it not to run in the future.
    fn remove_on_respawn_ball(&self, callback: RespawnBallCallbackId);
}

impl respawn_ball for super::RemoteReducers {
    fn respawn_ball(&self, schedule: RespawnBallsSchedule) -> __sdk::Result<()> {
        self.imp
            .call_reducer("respawn_ball", RespawnBallArgs { schedule })
    }
    fn on_respawn_ball(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &RespawnBallsSchedule) + Send + 'static,
    ) -> RespawnBallCallbackId {
        RespawnBallCallbackId(self.imp.on_reducer(
            "respawn_ball",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::RespawnBall { schedule },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, schedule)
            }),
        ))
    }
    fn remove_on_respawn_ball(&self, callback: RespawnBallCallbackId) {
        self.imp.remove_on_reducer("respawn_ball", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `respawn_ball`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_respawn_ball {
    /// Set the call-reducer flags for the reducer `respawn_ball` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn respawn_ball(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_respawn_ball for super::SetReducerFlags {
    fn respawn_ball(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("respawn_ball", flags);
    }
}
