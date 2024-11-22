
mod router;
mod server;
mod handler;

fn main() {

    let server = server::Server::new("127.0.0.1:3000");
    server.run();
}
