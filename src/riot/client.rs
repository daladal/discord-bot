pub struct RiotClient {
    api_key: String,
}

impl RiotClient {
    pub fn new(api_key: String) -> Self {
        RiotClient {
            api_key,
        }
    }
}
