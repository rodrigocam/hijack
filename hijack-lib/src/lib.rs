#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

pub mod server;

#[cfg(test)]
mod tests {
    use crate::server;

    #[test]
    fn it_works() {
        let server = server::Server::new();
        server.run();
    }
}
