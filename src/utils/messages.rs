#![allow(dead_code)]

pub const PING_MSG: &str = "ping";
pub const REGISTER_MSG: &str = "register";
pub const NEW_NODE: &str = "new_node";
pub const END: &str = "-";
pub const ACQUIRE_MSG: &str = "acquire\n";
pub const RELEASE_MSG: &str = "release\n";
pub const NEW_NODE_MSG: &str = "discover\n";
pub const DISCONNECT_MSG: &str = "";
pub const BLOCKCHAIN: &str = "blockchain";
pub const LEADER_DOWN: &str = "leader_down";
pub const LEADER_IS_DOWN: &str = "leader_is_down";
pub const CLOSE: &str = "close";
pub const BLOCKCHAIN_MSG: &str = "blockchain";
pub const WHO_IS_LEADER: &str = "who_is_leader";
pub const I_AM_LEADER: &str = "i_am_leader";

// Mensaje devuelto por el lider cuando esta ok el recibo del dato
pub const ACK_MSG: &str = "ack";
