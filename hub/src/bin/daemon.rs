use crate::config;
use crate::server::Server;

fn main() {
    // TODO additional params from args
    let conf_file = "config.toml";
    let conf = config::load(conf_file);
    let server = Server::new(&conf);
    server.run();
    // TODO gracefull shutdown
}