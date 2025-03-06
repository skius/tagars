// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use super::ball_type::Ball;
use super::rgb_type::Rgb;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `balls`.
///
/// Obtain a handle from the [`BallsTableAccess::balls`] method on [`super::RemoteTables`],
/// like `ctx.db.balls()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.balls().on_insert(...)`.
pub struct BallsTableHandle<'ctx> {
    imp: __sdk::TableHandle<Ball>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `balls`.
///
/// Implemented for [`super::RemoteTables`].
pub trait BallsTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`BallsTableHandle`], which mediates access to the table `balls`.
    fn balls(&self) -> BallsTableHandle<'_>;
}

impl BallsTableAccess for super::RemoteTables {
    fn balls(&self) -> BallsTableHandle<'_> {
        BallsTableHandle {
            imp: self.imp.get_table::<Ball>("balls"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct BallsInsertCallbackId(__sdk::CallbackId);
pub struct BallsDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for BallsTableHandle<'ctx> {
    type Row = Ball;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = Ball> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = BallsInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> BallsInsertCallbackId {
        BallsInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: BallsInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = BallsDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> BallsDeleteCallbackId {
        BallsDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: BallsDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<Ball>("balls");
    _table.add_unique_constraint::<__sdk::Identity>("identity", |row| &row.identity);
}
pub struct BallsUpdateCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::TableWithPrimaryKey for BallsTableHandle<'ctx> {
    type UpdateCallbackId = BallsUpdateCallbackId;

    fn on_update(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row, &Self::Row) + Send + 'static,
    ) -> BallsUpdateCallbackId {
        BallsUpdateCallbackId(self.imp.on_update(Box::new(callback)))
    }

    fn remove_on_update(&self, callback: BallsUpdateCallbackId) {
        self.imp.remove_on_update(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<Ball>> {
    __sdk::TableUpdate::parse_table_update(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<Ball>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}

/// Access to the `identity` unique index on the table `balls`,
/// which allows point queries on the field of the same name
/// via the [`BallsIdentityUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.balls().identity().find(...)`.
pub struct BallsIdentityUnique<'ctx> {
    imp: __sdk::UniqueConstraintHandle<Ball, __sdk::Identity>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> BallsTableHandle<'ctx> {
    /// Get a handle on the `identity` unique index on the table `balls`.
    pub fn identity(&self) -> BallsIdentityUnique<'ctx> {
        BallsIdentityUnique {
            imp: self
                .imp
                .get_unique_constraint::<__sdk::Identity>("identity"),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> BallsIdentityUnique<'ctx> {
    /// Find the subscribed row whose `identity` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &__sdk::Identity) -> Option<Ball> {
        self.imp.find(col_val)
    }
}
