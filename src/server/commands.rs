pub enum CommandsEnum {
    JoinCluster {},
    LeaveCluster {},
    GetClusterState {},
    GetKeysForBucket {
        key: String,
    },
}


pub struct CmdResponse {
}

impl CmdResponse {
    fn serialize(&self) -> String {
        return String::new();
    }
}

