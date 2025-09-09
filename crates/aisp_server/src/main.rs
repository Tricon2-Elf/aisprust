use crate::net::vce_server::VceServer;
use crate::servers::area_server::VceAreaServer;
use crate::servers::auth_server::VceAuthServer;
use crate::servers::msg_server::VceMsgServer;

pub mod compression;
pub mod crypt;
pub mod net;
pub mod servers;

use argp::FromArgs;
#[derive(FromArgs, Debug)]
#[argp(description = "top level args")]
struct TopLevelArgs {
    // #[argp(positional, description="Game install directory")]
    // path: String,
    #[argp(
        option,
        default = "false",
        short = 'e',
        description = "Enable encryption (this allows you to not use launcher)"
    )]
    encryption: bool,
}

#[tokio::main]
async fn main() {
    let args: TopLevelArgs = argp::parse_args_or_exit(argp::DEFAULT);
    println!("{:#?}", args);

    let mut auth = VceAuthServer::new("0.0.0.0", 50050, args.encryption);
    let mut msg = VceMsgServer::new("0.0.0.0", 50052, args.encryption);
    let mut area = VceAreaServer::new("0.0.0.0", 50054, args.encryption);

    loop {
        auth.tick();
        msg.tick();
        area.tick();
    }
}
