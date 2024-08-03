use super::db::consts;

pub trait DeviceInfo {
    fn get_response_data_index(&self, comm_type: &str) -> usize;
    fn get_response_status_index(&self, comm_type: &str) -> usize;
    fn get_subheader(&self) -> u16;
    fn get_subheader_serial(&self) -> u16;
    fn set_subheader_series(&mut self, subheader_serial: u16) {
        println!(
            "not need to set subheader_series {} for E3",
            subheader_serial
        );
    }
}

pub(crate) struct E3 {
    pub subheader: u16,
}

impl DeviceInfo for E3 {
    fn get_response_data_index(&self, comm_type: &str) -> usize {
        if comm_type == consts::COMMTYPE_BINARY {
            11
        } else {
            22
        }
    }
    fn get_response_status_index(&self, comm_type: &str) -> usize {
        if comm_type == consts::COMMTYPE_BINARY {
            9
        } else {
            18
        }
    }
    fn get_subheader(&self) -> u16 {
        self.subheader
    }
    fn get_subheader_serial(&self) -> u16 {
        0
    }
}

pub(crate) struct E4 {
    pub subheader: u16,
    pub subheader_serial: u16,
}

impl DeviceInfo for E4 {
    fn get_response_data_index(&self, comm_type: &str) -> usize {
        if comm_type == consts::COMMTYPE_BINARY {
            15
        } else {
            30
        }
    }
    fn get_response_status_index(&self, comm_type: &str) -> usize {
        if comm_type == consts::COMMTYPE_BINARY {
            13
        } else {
            26
        }
    }

    fn set_subheader_series(&mut self, subheader_serial: u16) {
        if (0..=65535).contains(&subheader_serial) {
            self.subheader_serial = subheader_serial;
        } else {
            println!("Invalid subheader_serial {}", subheader_serial);
        }
    }

    fn get_subheader(&self) -> u16 {
        self.subheader
    }
    fn get_subheader_serial(&self) -> u16 {
        self.subheader_serial
    }
}
