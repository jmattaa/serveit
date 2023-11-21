mod server;
use server::Server;

fn main() {
    let server = Server::new(4242);
    server.serve();
}
