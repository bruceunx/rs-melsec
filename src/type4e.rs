use super::db::consts;
use super::db::DataType;
use super::type3e::Type3E;

#[allow(dead_code)]
pub struct Type4E {
    pub type3e: Type3E,
    subheader: u16,
    subheader_serial: u16,
}

#[allow(dead_code)]
impl Type4E {
    pub fn new(host: String, port: u16, plc_type: &'static str) -> Self {
        Type4E {
            type3e: Type3E::new(host, port, plc_type),
            subheader: 0x5400,
            subheader_serial: 0x0000,
        }
    }
    fn set_subheader_serial(&mut self, subheader_serial: u16) -> Result<(), String> {
        if (0..=65535).contains(&subheader_serial) {
            self.subheader_serial = subheader_serial;
            Ok(())
        } else {
            Err("subheader_serial must be 0 <= subheader_serial <= 65535".to_string())
        }
    }
    fn get_response_data_index(&self) -> usize {
        match self.type3e.comm_type {
            consts::COMMTYPE_BINARY => 15,
            consts::COMMTYPE_ASCII => 30,
            _ => panic!("failed to get response data index"),
        }
    }

    fn get_response_status_index(&self) -> usize {
        match self.type3e.comm_type {
            consts::COMMTYPE_BINARY => 13,
            consts::COMMTYPE_ASCII => 26,
            _ => panic!("failed to get response data index"),
        }
    }
    fn build_send_data(&self, request_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut mc_data = Vec::new();

        if self.type3e.comm_type == consts::COMMTYPE_BINARY {
            mc_data.extend_from_slice(&self.subheader.to_be_bytes());
        } else {
            mc_data.extend_from_slice(format!("{:04X}", self.subheader).as_bytes());
        }

        mc_data.extend_from_slice(&self.type3e.encode_value(
            self.subheader_serial as i64,
            DataType::SWORD,
            false,
        )?);
        mc_data.extend_from_slice(&self.type3e.encode_value(0, DataType::SWORD, false)?);
        mc_data.extend_from_slice(&self.type3e.encode_value(
            self.type3e.network as i64,
            DataType::BIT,
            false,
        )?);
        mc_data.extend_from_slice(&self.type3e.encode_value(
            self.type3e.pc as i64,
            DataType::BIT,
            false,
        )?);
        mc_data.extend_from_slice(&self.type3e.encode_value(
            self.type3e.dest_moduleio as i64,
            DataType::SWORD,
            false,
        )?);
        mc_data.extend_from_slice(&self.type3e.encode_value(
            self.type3e.dest_modulesta as i64,
            DataType::BIT,
            false,
        )?);
        mc_data.extend_from_slice(&self.type3e.encode_value(
            (self.type3e._wordsize + request_data.len()) as i64,
            DataType::SWORD,
            false,
        )?);
        mc_data.extend_from_slice(&self.type3e.encode_value(
            self.type3e.timer as i64,
            DataType::SWORD,
            false,
        )?);
        mc_data.extend_from_slice(request_data);

        Ok(mc_data)
    }
}
