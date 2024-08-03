use byteorder::{BigEndian, ByteOrder, LittleEndian, NativeEndian, ReadBytesExt, WriteBytesExt};
use hex;
use std::error::Error;
use std::io::Cursor;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::db::DataType;
use super::db::{commands, consts, subcommands, DeviceConstants};
use super::err;
use super::tag::{QueryTag, Tag};
use regex::Regex;

fn get_device_type(device: &str) -> Result<String, String> {
    let re = Regex::new(r"\D+").map_err(|_| "Failed to compile regex".to_string())?;
    match re.find(device) {
        Some(mat) => Ok(mat.as_str().to_string()),
        None => Err(format!("Invalid device type \"{}\"", device)),
    }
}

fn get_device_index(device: &str) -> Result<i32, String> {
    let re = Regex::new(r"\d.*").map_err(|_| "Failed to compile regex".to_string())?;
    match re.find(device) {
        Some(mat) => match mat.as_str().parse::<i32>() {
            Ok(index) => Ok(index),
            Err(_) => Err(format!("Failed to parse device index \"{}\"", mat.as_str())),
        },
        None => Err(format!("Invalid device index \"{}\"", device)),
    }
}

struct E3 {
    subheader: u16,
}

struct E4 {
    subheader: u16,
    subheader_serial: u16,
}

enum DeviceType {
    E3(E3),
    E4(E4),
}

impl DeviceType {
    fn get_response_data_index(&self, comm_type: &str) -> usize {
        match self {
            DeviceType::E3(_) => {
                if comm_type == consts::COMMTYPE_BINARY {
                    11
                } else {
                    22
                }
            }
            DeviceType::E4(_) => {
                if comm_type == consts::COMMTYPE_BINARY {
                    15
                } else {
                    30
                }
            }
        }
    }

    fn get_response_status_index(&self, comm_type: &str) -> usize {
        match self {
            DeviceType::E3(_) => {
                if comm_type == consts::COMMTYPE_BINARY {
                    9
                } else {
                    18
                }
            }
            DeviceType::E4(_) => {
                if comm_type == consts::COMMTYPE_BINARY {
                    13
                } else {
                    26
                }
            }
        }
    }

    fn set_subheader_series(&self, subheader_serial: u16) -> Result<DeviceType, &str> {
        match self {
            DeviceType::E4(e3) => {
                if (0..=65555).contains(&subheader_serial) {
                    Ok(DeviceType::E4(E4 {
                        subheader: e3.subheader,
                        subheader_serial,
                    }))
                } else {
                    Err("subheader_serial must be 0 <= subheader_serial <= 65535")
                }
            }
            _ => Err("not implemented"),
        }
    }
}

pub struct Client {
    pub plc_type: &'static str,
    pub comm_type: &'static str,
    pub device_type: DeviceType,
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
impl Client {
    pub fn new(host: String, port: u16, plc_type: &'static str, use_e4: bool) -> Self {
        let device_type = if use_e4 {
            DeviceType::E4(E4 {
                subheader: 0x5400,
                subheader_serial: 0x0000,
            })
        } else {
            DeviceType::E3(E3 { subheader: 0x5000 })
        };

        let mut instance = Client {
            plc_type: consts::Q_SERIES,
            comm_type: consts::COMMTYPE_BINARY,
            device_type,
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

    fn set_subheader_serial(&mut self, subheader_serial: u16) -> Result<(), String> {
        self.device_type = self.device_type.set_subheader_series(subheader_serial)?;
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

        match &self.device_type {
            DeviceType::E3(e3) => {
                if self.comm_type == consts::COMMTYPE_BINARY {
                    let mut buffer = Vec::new();
                    buffer.write_u16::<BigEndian>(e3.subheader)?;
                    mc_data.extend_from_slice(&buffer);
                } else {
                    let subheader_hex = format!("{:04X}", e3.subheader);
                    mc_data.extend_from_slice(subheader_hex.as_bytes());
                }
            }
            DeviceType::E4(e4) => {
                if self.comm_type == consts::COMMTYPE_BINARY {
                    let mut buffer = Vec::new();
                    buffer.write_u16::<BigEndian>(e4.subheader)?;
                    mc_data.extend_from_slice(&buffer);
                } else {
                    let subheader_hex = format!("{:04X}", e4.subheader);
                    mc_data.extend_from_slice(subheader_hex.as_bytes());
                }
                mc_data.extend_from_slice(&self.encode_value(
                    e4.subheader_serial as i64,
                    DataType::SWORD,
                    false,
                )?);
                mc_data.extend_from_slice(&self.encode_value(0, DataType::SWORD, false)?);
            }
        }

        mc_data.extend_from_slice(&self.encode_value(self.network as i64, DataType::BIT, false)?);
        mc_data.extend_from_slice(&self.encode_value(self.pc as i64, DataType::BIT, false)?);
        mc_data.extend_from_slice(&self.encode_value(
            self.dest_moduleio as i64,
            DataType::SWORD,
            false,
        )?);
        mc_data.extend_from_slice(&self.encode_value(
            self.dest_modulesta as i64,
            DataType::BIT,
            false,
        )?);
        mc_data.extend_from_slice(&self.encode_value(
            (self._wordsize + request_data.len() as usize) as i64,
            DataType::SWORD,
            false,
        )?);
        mc_data.extend_from_slice(&self.encode_value(self.timer as i64, DataType::SWORD, false)?);
        mc_data.extend_from_slice(request_data);
        Ok(mc_data)
    }

    fn build_command_data(&self, command: u16, subcommand: u16) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut command_data = Vec::new();
        command_data.extend_from_slice(&self.encode_value(
            command as i64,
            DataType::SWORD,
            false,
        )?);
        command_data.extend_from_slice(&self.encode_value(
            subcommand as i64,
            DataType::SWORD,
            false,
        )?);
        Ok(command_data)
    }

    pub fn encode_value(
        &self,
        value: i64,
        mode: DataType,
        is_signal: bool,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = Vec::new();

        let mode_size = mode.size();
        match *self.endian {
            consts::ENDIAN_LITTLE => match mode_size {
                2 => buffer.write_u8(value as u8)?,
                4 => match is_signal {
                    true => buffer.write_i16::<LittleEndian>(value as i16)?,
                    false => buffer.write_u16::<LittleEndian>(value as u16)?,
                },
                8 => match is_signal {
                    true => buffer.write_i32::<LittleEndian>(value as i32)?,
                    false => buffer.write_u32::<LittleEndian>(value as u32)?,
                },
                _ => return Err("Unsupported data type size".into()),
            },
            consts::ENDIAN_BIG => match mode_size {
                2 => buffer.write_u8(value as u8)?,
                4 => match is_signal {
                    true => buffer.write_i32::<BigEndian>(value as i32)?,
                    false => buffer.write_u32::<BigEndian>(value as u32)?,
                },
                8 => match is_signal {
                    true => buffer.write_i32::<BigEndian>(value as i32)?,
                    false => buffer.write_u32::<BigEndian>(value as u32)?,
                },
                _ => return Err("Unsupported data type size".into()),
            },
            consts::ENDIAN_NATIVE => match mode_size {
                2 => buffer.write_u8(value as u8)?,
                4 => match is_signal {
                    true => buffer.write_i64::<NativeEndian>(value as i64)?,
                    false => buffer.write_u64::<NativeEndian>(value as u64)?,
                },
                8 => match is_signal {
                    true => buffer.write_i64::<NativeEndian>(value as i64)?,
                    false => buffer.write_u64::<NativeEndian>(value as u64)?,
                },
                _ => return Err("Unsupported data type size".into()),
            },
            _ => return Err("Unsupported endianness".into()),
        }

        Ok(buffer)
    }

    fn decode_value(
        &self,
        data: &[u8],
        mode: &DataType,
        is_signed: bool,
    ) -> Result<i64, Box<dyn Error>> {
        let mut bytes = data.to_vec();
        if self.comm_type != consts::COMMTYPE_BINARY {
            bytes = hex::decode(bytes)?;
        }

        let mode_size = mode.size();
        let mut cursor = Cursor::new(bytes);
        let value = match *self.endian {
            consts::ENDIAN_LITTLE => match mode_size {
                2 => cursor.read_u8()? as i64,
                4 => match is_signed {
                    true => cursor.read_i16::<LittleEndian>()? as i64,
                    false => cursor.read_u16::<LittleEndian>()? as i64,
                },
                8 => match is_signed {
                    true => cursor.read_i16::<LittleEndian>()? as i64,
                    false => cursor.read_u16::<LittleEndian>()? as i64,
                },
                _ => return Err("Unsupported data type size".into()),
            },
            consts::ENDIAN_BIG => match mode_size {
                2 => cursor.read_u8()? as i64,
                4 => match is_signed {
                    true => cursor.read_i16::<BigEndian>()? as i64,
                    false => cursor.read_u16::<BigEndian>()? as i64,
                },
                8 => match is_signed {
                    true => cursor.read_i16::<BigEndian>()? as i64,
                    false => cursor.read_u16::<BigEndian>()? as i64,
                },
                _ => return Err("Unsupported data type size".into()),
            },
            consts::ENDIAN_NATIVE => match mode_size {
                2 => cursor.read_u8()? as i64,
                4 => match is_signed {
                    true => cursor.read_i16::<NativeEndian>()? as i64,
                    false => cursor.read_u16::<NativeEndian>()? as i64,
                },
                8 => match is_signed {
                    true => cursor.read_i16::<NativeEndian>()? as i64,
                    false => cursor.read_u16::<NativeEndian>()? as i64,
                },
                _ => return Err("Unsupported data type size".into()),
            },
            _ => return Err("Unsupported endianness".into()),
        };
        Ok(value)
    }

    fn check_mc_error(status: u16) -> Result<(), err::MCError> {
        if status == 0 {
            Ok(())
        } else {
            Err(err::MCError::new(status))
        }
    }

    pub fn batch_read(
        &mut self,
        ref_device: &str,
        read_size: usize,
        data_type: DataType,
        decode: bool,
    ) -> Result<Vec<Tag>, Box<dyn Error>> {
        let data_type_size = data_type.size();
        let device_type = get_device_type(ref_device)?;
        let device_index: i32 = get_device_index(ref_device)?;

        let command = commands::BATCH_READ;
        let subcommand = if data_type == DataType::BIT {
            if self.plc_type == consts::IQR_SERIES {
                subcommands::THREE
            } else {
                subcommands::ONE
            }
        } else {
            if self.plc_type == consts::IQR_SERIES {
                subcommands::TWO
            } else {
                subcommands::ZERO
            }
        };

        let mut request_data = Vec::new();
        request_data.extend(self.build_command_data(command, subcommand)?);
        request_data.extend(self.build_device_data(ref_device)?);
        request_data.extend(self.encode_value(
            (read_size * data_type_size as usize) as i64 / 2,
            DataType::SWORD,
            false,
        )?);
        let send_data = self.build_send_data(&request_data)?;

        self.send(&send_data)?;
        let recv_data = self.recv()?;
        self.check_command_response(&recv_data)?;

        let mut result = Vec::new();
        let mut data_index = self.get_response_data_index();

        if data_type == DataType::BIT {
            if self.comm_type == consts::COMMTYPE_BINARY {
                for index in 0..read_size {
                    data_index = index / 2 + data_index;
                    let bit_value = if decode {
                        let value = recv_data[data_index];
                        if index % 2 == 0 {
                            if (value & (1 << 4)) != 0 {
                                1
                            } else {
                                0
                            }
                        } else {
                            if (value & (1 << 0)) != 0 {
                                1
                            } else {
                                0
                            }
                        }
                    } else {
                        recv_data[data_index] as i32
                    };
                    result.push(Tag {
                        device: format!("{}{}", device_type, device_index + index as i32),
                        value: format!("{}", bit_value).into(),
                        data_type: data_type.clone(),
                    });
                }
            } else {
                for index in 0..read_size {
                    let bit_value = if decode {
                        recv_data[data_index] as i32
                    } else {
                        recv_data[data_index] as i32
                    };
                    result.push(Tag {
                        device: format!("{}{}", device_type, device_index + index as i32),
                        value: format!("{}", bit_value).into(),
                        data_type: data_type.clone(),
                    });
                    data_index += 1;
                }
            }
        } else {
            for index in 0..read_size {
                let value = if decode {
                    let decode_value = self.decode_value(
                        &recv_data[data_index..data_index + data_type_size as usize].to_vec(),
                        &data_type,
                        false,
                    )?;
                    format!("{}", decode_value).to_string()
                } else {
                    let raw_value = &recv_data[data_index..data_index + data_type_size as usize];
                    String::from_utf8(raw_value.to_vec())?
                };
                result.push(Tag {
                    device: format!("{}{}", device_type, device_index + index as i32),
                    value: Some(value),
                    data_type: data_type.clone(),
                });
                data_index += data_type_size as usize;
            }
        }

        Ok(result)
    }

    pub fn batch_write(
        &self,
        ref_device: &str,
        values: Vec<i64>,
        data_type: &DataType,
    ) -> Result<(), Box<dyn Error>> {
        let data_type_size = data_type.size();
        let write_elements = values.len();

        let command = commands::BATCH_WRITE;
        let subcommand = if *data_type == DataType::BIT {
            if self.plc_type == consts::IQR_SERIES {
                subcommands::THREE
            } else {
                subcommands::ONE
            }
        } else {
            if self.plc_type == consts::IQR_SERIES {
                subcommands::TWO
            } else {
                subcommands::ZERO
            }
        };

        let mut request_data = Vec::new();
        request_data.extend(self.build_command_data(command, subcommand)?);
        request_data.extend(self.build_device_data(ref_device)?);
        request_data.extend(self.encode_value(
            (write_elements * data_type_size as usize) as i64 / 2,
            DataType::SWORD,
            false,
        )?);

        if *data_type == DataType::BIT {
            if self.comm_type == consts::COMMTYPE_BINARY {
                let mut bit_data = vec![0; (values.len() + 1) / 2];
                for (index, value) in values.iter().enumerate() {
                    let value = (*value != 0) as u8;
                    let value_index = index / 2;
                    let bit_index = if index % 2 == 0 { 4 } else { 0 };
                    let bit_value = value << bit_index;
                    bit_data[value_index] |= bit_value;
                }
                request_data.extend(bit_data);
            } else {
                for value in values {
                    request_data.extend(value.to_string().into_bytes());
                }
            }
        } else {
            for value in values {
                request_data.extend(self.encode_value(value, data_type.clone(), false)?);
            }
        }

        let send_data = self.build_send_data(&request_data)?;

        self.send(&send_data)?;
        let recv_data = self.recv()?;
        self.check_command_response(&recv_data)?;
        Ok(())
    }

    fn build_device_data(&self, device: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut device_data = Vec::new();

        let device_type = get_device_type(device)?;

        if self.comm_type == consts::COMMTYPE_BINARY {
            let (device_code, device_base) =
                DeviceConstants::get_binary_device_code(self.plc_type, &device_type)?;
            let device_number =
                i32::from_str_radix(&get_device_index(device)?.to_string(), device_base)?;

            if self.plc_type == consts::IQR_SERIES {
                let mut buf = [0u8; 6];
                if *self.endian == consts::ENDIAN_LITTLE {
                    LittleEndian::write_u32(&mut buf, device_number as u32);
                } else {
                    BigEndian::write_u32(&mut buf, device_number as u32);
                }
                device_data.extend_from_slice(&buf[0..4]);
                device_data.extend_from_slice(&buf[4..6]);
            } else {
                let mut buf = [0u8; 4];
                if *self.endian == consts::ENDIAN_LITTLE {
                    LittleEndian::write_u32(&mut buf, device_number as u32);
                } else {
                    BigEndian::write_u32(&mut buf, device_number as u32);
                }
                device_data.extend_from_slice(&buf[0..3]);
                device_data.push(device_code as u8);
            }
        } else {
            let (device_code, device_base) =
                DeviceConstants::get_ascii_device_code(self.plc_type, &device_type)?;
            let device_number = format!(
                "{:06x}",
                i32::from_str_radix(&get_device_index(device)?.to_string(), device_base)?
            );

            device_data.extend_from_slice(device_code.as_bytes());
            device_data.extend_from_slice(device_number.as_bytes());
        }

        Ok(device_data)
    }

    fn check_command_response(&self, recv_data: &[u8]) -> Result<(), err::MCError> {
        let response_status_index = self.get_response_status_index();
        let response_status = self
            .decode_value(
                &recv_data[response_status_index..response_status_index + self._wordsize],
                &DataType::SWORD,
                false,
            )
            .unwrap() as u16;

        Client::check_mc_error(response_status)
    }

    fn read(&self, devices: Vec<QueryTag>) -> Result<Vec<Tag>, Box<dyn Error>> {
        let command = commands::RANDOM_READ;
        let subcommand = if self.plc_type == consts::IQR_SERIES {
            subcommands::TWO
        } else {
            subcommands::ZERO
        };

        let mut words_count = 0;

        for element in &devices {
            let _size = element.data_type.size();
            words_count += _size / 2;
        }

        let mut request_data = Vec::new();
        request_data.extend(self.build_command_data(command, subcommand)?);
        request_data.extend(self.encode_value(words_count as i64, DataType::BIT, false)?);
        request_data.extend(self.encode_value(0, DataType::BIT, false)?);

        for element in &devices {
            let element_size = element.data_type.size() / 2;
            if element_size > 1 {
                let tag_name = &element.device;
                let device_type = get_device_type(tag_name)?;
                let mut device_index = get_device_index(tag_name)?;
                for _ in 0..element_size {
                    let temp_tag_name = format!("{}{}", device_type, device_index);
                    request_data.extend(self.build_device_data(&temp_tag_name)?);
                    device_index += 1;
                }
            } else {
                request_data.extend(self.build_device_data(&element.device)?);
            }
        }

        if words_count < 1 {
            return Ok(Vec::new());
        }

        let send_data = self.build_send_data(&request_data)?;
        self.send(&send_data)?;
        let recv_data = self.recv()?;

        let mut output = Vec::new();
        self.check_command_response(&recv_data)?;

        let mut data_index = self.get_response_data_index();

        for element in devices {
            let size = element.data_type.size();
            let value = self.decode_value(
                &recv_data[data_index..data_index + size as usize],
                &DataType::BIT,
                false,
            )?;

            output.push(Tag {
                device: element.device,
                value: format!("{}", value).into(),
                data_type: element.data_type,
            });

            data_index += size as usize;
        }

        Ok(output)
    }

    fn write(&self, devices: Vec<Tag>) -> Result<(), Box<dyn Error>> {
        let command = commands::RANDOM_WRITE;
        let subcommand = if self.plc_type == consts::IQR_SERIES {
            subcommands::TWO
        } else {
            subcommands::ZERO
        };

        // Get the words equivalent in size
        let mut words_count = 0;
        for element in &devices {
            words_count += element.data_type.size() / 2;
        }

        let mut request_data = Vec::new();
        request_data.extend(self.build_command_data(command, subcommand)?);
        request_data.extend(self.encode_value(words_count as i64, DataType::BIT, false)?);
        request_data.extend(self.encode_value(0, DataType::BIT, false)?);

        for mut element in devices {
            if element.data_type == DataType::BIT {
                match element.value {
                    Some(s) => {
                        let s_vec: Vec<i64> = s
                            .split_whitespace()
                            .filter_map(|part| part.parse::<i64>().ok())
                            .collect();
                        self.batch_write(&element.device, s_vec, &element.data_type)?;
                    }
                    None => continue,
                }
                continue;
            }
            let element_size = element.data_type.size() / 2;
            if (element.data_type == DataType::UWORD || element.data_type == DataType::UDWORD)
                && element.value.clone().unwrap().parse::<i64>().unwrap() < 0
            {
                element.value = format!("-{}", element.value.unwrap()).into();
            }
            if element_size > 1 {
                let tag_name = &element.device;
                let device_type = get_device_type(tag_name)?;
                let mut device_index = get_device_index(tag_name)?;
                let _value = element.value.unwrap().parse::<i64>().unwrap();
                let temp_tag_value = self.encode_value(_value, element.data_type, false)?;
                let mut data_index = 0;
                for _ in 0..element_size {
                    let temp_tag_name = format!("{}{}", device_type, device_index);
                    request_data.extend(self.build_device_data(&temp_tag_name)?);
                    request_data.extend(&temp_tag_value[data_index..data_index + self._wordsize]);
                    data_index += self._wordsize;
                    device_index += 1;
                }
            } else {
                request_data.extend(self.build_device_data(&element.device)?);
                let _value = element.value.unwrap().parse::<i64>().unwrap();
                request_data.extend(&self.encode_value(_value, element.data_type, false)?);
            }
        }

        let send_data = self.build_send_data(&request_data)?;
        self.send(&send_data)?;
        let recv_data = self.recv()?;
        self.check_command_response(&recv_data)?;

        Ok(())
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if let Err(e) = self.close() {
            eprintln!("Error closing connection: {:?}", e);
        }
    }
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Type3E")
            .field("plc_type", &self.plc_type)
            .field("comm_type", &self.comm_type)
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
