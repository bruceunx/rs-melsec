use byteorder::{BigEndian, LittleEndian, NativeEndian, WriteBytesExt};
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::db::consts;
use super::db::DataType;

pub struct Type3E {
    pub plc_type: &'static str,
    pub comm_type: &'static str,
    pub subheader: u16,
    pub network: u8,
    pub pc: u8,
    pub dest_moduleio: u16,
    pub dest_modulesta: u8,
    pub timer: u8,
    pub sock_timeout: u64,
    _is_connected: Arc<Mutex<bool>>,
    _sockbufsize: usize,
    _wordsize: usize,
    _debug: bool,
    endian: &'static char,
    host: String,
    port: u16,
    _sock: Option<TcpStream>,
}

#[allow(dead_code)]
impl Type3E {
    pub fn new(host: String, port: u16, plc_type: &'static str) -> Self {
        let mut instance = Type3E {
            plc_type: consts::Q_SERIES,
            comm_type: consts::COMMTYPE_BINARY,
            subheader: 0x5000,
            network: 0,
            pc: 0xFF,
            dest_moduleio: 0x3FF,
            dest_modulesta: 0x0,
            timer: 4,
            sock_timeout: 2,
            _is_connected: Arc::new(Mutex::new(false)),
            _sockbufsize: 4096,
            _wordsize: 2,
            _debug: false,
            endian: &consts::ENDIAN_LITTLE,
            host,
            port,
            _sock: None,
        };

        instance.set_plc_type(plc_type);
        instance
    }

    pub fn set_debug(&mut self, enable: bool) {
        self._debug = enable;
    }

    pub fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let ip_port = format!("{}:{}", self.host, self.port);
        let stream = TcpStream::connect(ip_port)?;
        stream.set_read_timeout(Some(Duration::new(self.sock_timeout, 0)))?;
        stream.set_write_timeout(Some(Duration::new(self.sock_timeout, 0)))?;
        self._sock = Some(stream);
        let mut is_connected = self._is_connected.lock().unwrap();
        *is_connected = true;
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut sock) = self._sock {
            sock.shutdown(std::net::Shutdown::Both)?;
        }
        self._sock = None;
        let mut is_connected = self._is_connected.lock().unwrap();
        *is_connected = false;
        Ok(())
    }

    pub fn send(&self, send_data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if *self._is_connected.lock().unwrap() {
            self._sock.as_ref().unwrap().write_all(send_data)?;
            Ok(())
        } else {
            Err("Socket is not connected. Please use the connect method.".into())
        }
    }

    pub fn recv(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut recv_data = vec![0u8; self._sockbufsize];
        let size = self._sock.as_ref().unwrap().read(&mut recv_data)?;
        recv_data.truncate(size);
        Ok(recv_data)
    }

    fn set_plc_type(&mut self, plc_type: &str) {
        match plc_type {
            "Q" => self.plc_type = consts::Q_SERIES,
            "L" => self.plc_type = consts::L_SERIES,
            "QnA" => self.plc_type = consts::QNA_SERIES,
            "iQ-L" => self.plc_type = consts::IQL_SERIES,
            "iQ-R" => self.plc_type = consts::IQR_SERIES,
            _ => panic!("Failed to set PLC type. Please use 'Q', 'L', 'QnA', 'iQ-L', 'iQ-R'"),
        }
    }

    fn set_comm_type(&mut self, comm_type: &str) {
        match comm_type {
            "binary" => {
                self.comm_type = consts::COMMTYPE_BINARY;
                self._wordsize = 2;
            }
            "ascii" => {
                self.comm_type = consts::COMMTYPE_ASCII;
                self._wordsize = 4;
            }
            _ => panic!("Failed to set communication type. Please use 'binary' or 'ascii'"),
        }
    }

    fn get_response_data_index(&self) -> usize {
        if self.comm_type == consts::COMMTYPE_BINARY {
            11
        } else {
            22
        }
    }

    fn get_response_status_index(&self) -> usize {
        if self.comm_type == consts::COMMTYPE_BINARY {
            9
        } else {
            18
        }
    }

    fn build_send_data(&self, request_data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut mc_data = Vec::new();
        if self.comm_type == consts::COMMTYPE_BINARY {
            let mut buffer = Vec::new();
            buffer.write_u16::<BigEndian>(self.subheader)?;
            mc_data.extend_from_slice(&buffer);
        } else {
            let subheader_hex = format!("{:04X}", self.subheader);
            mc_data.extend_from_slice(subheader_hex.as_bytes());
        }
        // mc_data.extend_from_slice(&self.encode_value(self.network, consts::DT::BIT)?);
        // mc_data.extend_from_slice(&self.encode_value(self.pc, consts::DT::BIT)?);
        // mc_data.extend_from_slice(&self.encode_value(self.dest_moduleio, consts::DT::SWORD)?);
        // mc_data.extend_from_slice(&self.encode_value(self.dest_modulesta, consts::DT::BIT)?);
        // mc_data.extend_from_slice(&self.encode_value(
        //     (self._wordsize + request_data.len() as usize) as u16,
        //     consts::DT::SWORD,
        // )?);
        // mc_data.extend_from_slice(&self.encode_value(self.timer, consts::DT::SWORD)?);
        mc_data.extend_from_slice(request_data);

        Ok(mc_data)
    }

    fn build_command_data(&self, command: u16, subcommand: u16) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut command_data = Vec::new();
        command_data.extend_from_slice(&self.encode_value(command as i64, DataType::SWORD)?);
        command_data.extend_from_slice(&self.encode_value(subcommand as i64, DataType::SWORD)?);
        Ok(command_data)
    }

    fn encode_value(&self, value: i64, mode: DataType) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = Vec::new();

        let mode_size = mode.size();
        match self.endian {
            &consts::ENDIAN_LITTLE => match mode_size {
                2 => buffer.write_u8(value as u8)?,
                4 => buffer.write_u16::<LittleEndian>(value as u16)?,
                8 => buffer.write_u32::<LittleEndian>(value as u32)?,
                _ => return Err("Unsupported data type size".into()),
            },
            &consts::ENDIAN_BIG => match mode_size {
                2 => buffer.write_u8(value as u8)?,
                4 => buffer.write_u16::<BigEndian>(value as u16)?,
                8 => buffer.write_u32::<BigEndian>(value as u32)?,
                _ => return Err("Unsupported data type size".into()),
            },
            &consts::ENDIAN_NATIVE => match mode_size {
                2 => buffer.write_u8(value as u8)?,
                4 => buffer.write_u16::<NativeEndian>(value as u16)?,
                8 => buffer.write_u32::<NativeEndian>(value as u32)?,
                _ => return Err("Unsupported data type size".into()),
            },
            _ => return Err("Unsupported endianness".into()),
        }

        Ok(buffer)
    }
}

impl Drop for Type3E {
    fn drop(&mut self) {
        if let Err(e) = self.close() {
            eprintln!("Error closing connection: {:?}", e);
        }
    }
}

impl std::fmt::Debug for Type3E {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Type3E")
            .field("plc_type", &self.plc_type)
            .field("comm_type", &self.comm_type)
            .field("subheader", &self.subheader)
            .field("network", &self.network)
            .field("pc", &self.pc)
            .field("dest_moduleio", &self.dest_moduleio)
            .field("dest_modulesta", &self.dest_modulesta)
            .field("timer", &self.timer)
            .field("sock_timeout", &self.sock_timeout)
            .field("_is_connected", &self._is_connected)
            .field("_sockbufsize", &self._sockbufsize)
            .field("_wordsize", &self._wordsize)
            .field("_debug", &self._debug)
            .field("endian", &self.endian)
            .field("host", &self.host)
            .field("port", &self.port)
            .finish()
    }
}
