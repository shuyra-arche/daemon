#![no_std]
#![no_main]

#[mold::entry]
fn main() -> Result<(), env::Reason> {
    daemon_server::run().map_err(|why| {
        system_support::debug::put(why);
        daemon_server::E_DAEMON
    })
}
