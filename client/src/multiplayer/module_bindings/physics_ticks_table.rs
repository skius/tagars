// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use super::physics_tick_type::PhysicsTick;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `physics_ticks`.
///
/// Obtain a handle from the [`PhysicsTicksTableAccess::physics_ticks`] method on [`super::RemoteTables`],
/// like `ctx.db.physics_ticks()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.physics_ticks().on_insert(...)`.
pub struct PhysicsTicksTableHandle<'ctx> {
    imp: __sdk::TableHandle<PhysicsTick>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `physics_ticks`.
///
/// Implemented for [`super::RemoteTables`].
pub trait PhysicsTicksTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`PhysicsTicksTableHandle`], which mediates access to the table `physics_ticks`.
    fn physics_ticks(&self) -> PhysicsTicksTableHandle<'_>;
}

impl PhysicsTicksTableAccess for super::RemoteTables {
    fn physics_ticks(&self) -> PhysicsTicksTableHandle<'_> {
        PhysicsTicksTableHandle {
            imp: self.imp.get_table::<PhysicsTick>("physics_ticks"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct PhysicsTicksInsertCallbackId(__sdk::CallbackId);
pub struct PhysicsTicksDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for PhysicsTicksTableHandle<'ctx> {
    type Row = PhysicsTick;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = PhysicsTick> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = PhysicsTicksInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> PhysicsTicksInsertCallbackId {
        PhysicsTicksInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: PhysicsTicksInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = PhysicsTicksDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> PhysicsTicksDeleteCallbackId {
        PhysicsTicksDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: PhysicsTicksDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<PhysicsTick>("physics_ticks");
    _table.add_unique_constraint::<u64>("tick_id", |row| &row.tick_id);
}
pub struct PhysicsTicksUpdateCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::TableWithPrimaryKey for PhysicsTicksTableHandle<'ctx> {
    type UpdateCallbackId = PhysicsTicksUpdateCallbackId;

    fn on_update(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row, &Self::Row) + Send + 'static,
    ) -> PhysicsTicksUpdateCallbackId {
        PhysicsTicksUpdateCallbackId(self.imp.on_update(Box::new(callback)))
    }

    fn remove_on_update(&self, callback: PhysicsTicksUpdateCallbackId) {
        self.imp.remove_on_update(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<PhysicsTick>> {
    __sdk::TableUpdate::parse_table_update(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<PhysicsTick>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}

/// Access to the `tick_id` unique index on the table `physics_ticks`,
/// which allows point queries on the field of the same name
/// via the [`PhysicsTicksTickIdUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.physics_ticks().tick_id().find(...)`.
pub struct PhysicsTicksTickIdUnique<'ctx> {
    imp: __sdk::UniqueConstraintHandle<PhysicsTick, u64>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> PhysicsTicksTableHandle<'ctx> {
    /// Get a handle on the `tick_id` unique index on the table `physics_ticks`.
    pub fn tick_id(&self) -> PhysicsTicksTickIdUnique<'ctx> {
        PhysicsTicksTickIdUnique {
            imp: self.imp.get_unique_constraint::<u64>("tick_id"),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> PhysicsTicksTickIdUnique<'ctx> {
    /// Find the subscribed row whose `tick_id` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &u64) -> Option<PhysicsTick> {
        self.imp.find(col_val)
    }
}
