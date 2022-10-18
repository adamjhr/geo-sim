use sim_prototype::run;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    pollster::block_on(run());
}
