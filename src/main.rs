//! Entry point — wires crates together, stays thin.

use innovator::auth::Session;
use innovator::domains;
use innovator::workspace::{AppShell, WorkspaceRegistry};
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let event_loop = EventLoop::new().expect("event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let db = infinite_db::InfiniteDb::open("innovator.db").expect("open innovator.db");

    let mut registry = WorkspaceRegistry::new();
    domains::register_all(&mut registry);

    let session = Session::guest();

    let mut app = AppShell::new(db, registry, session);
    event_loop.run_app(&mut app).expect("run app");
}
