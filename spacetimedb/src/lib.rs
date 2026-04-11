use spacetimedb::{reducer, table, Identity, ReducerContext, Table, Timestamp};

#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    init_message(ctx);
}

#[reducer(client_connected)]
pub fn on_connected(ctx: &ReducerContext) {
    add_cursor(ctx);
}

#[reducer(client_disconnected)]
pub fn on_disconnected(ctx: &ReducerContext) {
    remove_cursor(ctx);
}

/* --- Message --- */
#[table(accessor = message, public)]
pub struct Message {
    #[primary_key]
    id: u8,
    message: String,
    timestamp: Timestamp,
    author: Identity,
}

#[table(accessor = message_position, public)]
pub struct MessagePosition {
    #[primary_key]
    id: u8,
    x: i32,
    y: i32,
}

pub fn init_message(ctx: &ReducerContext) {
    ctx.db.message().insert(Message {
        id: 0,
        message: "Try changing me".into(),
        timestamp: ctx.timestamp,
        author: ctx.identity(),
    });
    ctx.db
        .message_position()
        .insert(MessagePosition { id: 0, x: 0, y: 0 });
}

#[reducer]
pub fn update_message(ctx: &ReducerContext, message: String, timestamp: Timestamp) {
    ctx.db.message().id().update(Message {
        id: 0,
        message,
        timestamp,
        author: ctx.sender(),
    });
}
#[reducer]
pub fn update_message_position(ctx: &ReducerContext, x: i32, y: i32) {
    ctx.db
        .message_position()
        .id()
        .update(MessagePosition { id: 0, x, y });
}

/* --- Cursor --- */
#[table(accessor = cursor, public)]
#[derive(Debug, Default)]
pub struct Cursor {
    #[primary_key]
    id: Identity,
    x: i32,
    y: i32,
}

pub fn add_cursor(ctx: &ReducerContext) {
    let id = ctx.sender();
    ctx.db.cursor().insert(Cursor {
        id,
        ..Default::default()
    });
}

pub fn remove_cursor(ctx: &ReducerContext) {
    let id = ctx.sender();
    ctx.db.cursor().id().delete(id);
}

#[reducer]
pub fn update_position(ctx: &ReducerContext, x: i32, y: i32) {
    ctx.db.cursor().id().update(Cursor {
        id: ctx.sender(),
        x,
        y,
    });
}
