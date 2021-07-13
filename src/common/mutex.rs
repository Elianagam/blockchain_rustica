use super::socket::Socket;

pub struct Mutex;

impl Mutex {
    pub fn acquire(&mut self, socket: Socket) {
        socket.write("adquire\n".to_string());
    }

    pub fn release(&mut self, socket: Socket) {
        socket.write("release\n".to_string());
    }
}

pub struct Node;
