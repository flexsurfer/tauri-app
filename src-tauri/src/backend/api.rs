use super::status_go;

pub struct Api {
    status_go: status_go::StatusGo
}

impl Api {
    pub fn init() -> Result<Api, String> {
        let status_go = status_go::StatusGo::init();
        match status_go {
            Ok(status_go) => Ok(Api { status_go }),
            Err(error) => Err(error),
        }
    }

    pub fn sha3(&self, value: &str) -> String {
        return self.status_go.call_rpc(b"Sha3", value);
    }
}
