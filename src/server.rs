pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr,
        }
    }
    // self is like this
    pub fn run(self) {
        println!("Listenin on {}", self.addr)
    }
}

// file server.rs is the same as mod server {}
// by default modules are always private
