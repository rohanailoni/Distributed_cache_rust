use crate::server::sync_server;
use crate::server::async_server;
mod server { pub mod sync_server;pub mod async_server; }
fn main() {
    unsafe {
        async_server::run_async_tcp_server();
    }

}




