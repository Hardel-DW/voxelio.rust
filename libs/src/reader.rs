use crate::{NbtError, NbtTag, Result};
use std::collections::{HashMap, HashSet};

/// Endianness for NBT data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    Big,    // Java Edition (default)
    Little, // Bedrock Edition
}

/// Zero-copy NBT reader with streaming capabilities
pub struct NbtReader<'a> {
    data: &'a [u8],
    pub cursor: usize,
    endian: Endian,
}

impl<'a> NbtReader<'a> {
    pub fn new(data: &'a [u8], endian: Endian) -> Self {
        Self {
            data,
            cursor: 0,
            endian,
        }
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.cursor)
    }

    // Basic reading methods
    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8]> {
        if self.remaining() < len {
            return Err(NbtError::UnexpectedEof);
        }
        let slice = &self.data[self.cursor..self.cursor + len];
        self.cursor += len;
        Ok(slice)
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let bytes = self.read_bytes(1)?;
        Ok(bytes[0])
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_u8()? as i8)
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        let bytes = self.read_bytes(2)?;
        Ok(match self.endian {
            Endian::Big => i16::from_be_bytes([bytes[0], bytes[1]]),
            Endian::Little => i16::from_le_bytes([bytes[0], bytes[1]]),
        })
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        let bytes = self.read_bytes(4)?;
        Ok(match self.endian {
            Endian::Big => i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            Endian::Little => i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        })
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        let bytes = self.read_bytes(8)?;
        let array: [u8; 8] = bytes.try_into().unwrap();
        Ok(match self.endian {
            Endian::Big => i64::from_be_bytes(array),
            Endian::Little => i64::from_le_bytes(array),
        })
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        Ok(f32::from_bits(self.read_i32()? as u32))
    }

    pub fn read_f64(&mut self) -> Result<f64> {
        Ok(f64::from_bits(self.read_i64()? as u64))
    }

    pub fn read_string(&mut self) -> Result<String> {
        let len = self.read_i16()? as usize;
        let bytes = self.read_bytes(len)?;
        String::from_utf8(bytes.to_vec())
            .map_err(|e| NbtError::Parse(format!("Invalid UTF-8: {e}")))
    }

    pub fn read_tag(&mut self, tag_type: u8) -> Result<NbtTag> {
        match tag_type {
            0 => Ok(NbtTag::End),
            1 => Ok(NbtTag::Byte(self.read_i8()?)),
            2 => Ok(NbtTag::Short(self.read_i16()?)),
            3 => Ok(NbtTag::Int(self.read_i32()?)),
            4 => Ok(NbtTag::Long(self.read_i64()?)),
            5 => Ok(NbtTag::Float(self.read_f32()?)),
            6 => Ok(NbtTag::Double(self.read_f64()?)),
            7 => self.read_byte_array(),
            8 => Ok(NbtTag::String(self.read_string()?)),
            9 => self.read_list(),
            10 => self.read_compound(),
            11 => self.read_int_array(),
            12 => self.read_long_array(),
            _ => Err(NbtError::InvalidTagType(tag_type)),
        }
    }

    fn read_byte_array(&mut self) -> Result<NbtTag> {
        let len = self.read_i32()? as usize;
        let mut array = Vec::with_capacity(len);
        for _ in 0..len {
            array.push(self.read_i8()?);
        }
        Ok(NbtTag::ByteArray(array))
    }

    fn read_list(&mut self) -> Result<NbtTag> {
        let tag_type = self.read_u8()?;
        let len = self.read_i32()? as usize;
        let mut items = Vec::with_capacity(len);

        for _ in 0..len {
            items.push(self.read_tag(tag_type)?);
        }

        Ok(NbtTag::List { tag_type, items })
    }

    fn read_compound(&mut self) -> Result<NbtTag> {
        let mut map = HashMap::new();

        loop {
            let tag_type = self.read_u8()?;
            if tag_type == 0 {
                break;
            }

            let name = self.read_string()?;
            let value = self.read_tag(tag_type)?;
            map.insert(name, value);
        }

        Ok(NbtTag::Compound(map))
    }

    /// Parse seulement les champs spécifiés pour optimiser la performance
    pub fn read_compound_selective(&mut self, wanted_fields: &[&str]) -> Result<NbtTag> {
        let mut map = HashMap::new();
        let wanted_set: HashSet<&str> = wanted_fields.iter().copied().collect();

        loop {
            let tag_type = self.read_u8()?;
            if tag_type == 0 {
                break;
            }

            let name = self.read_string()?;

            if wanted_set.contains(name.as_str()) {
                let value = self.read_tag(tag_type)?;
                map.insert(name, value);
            } else {
                // Skip ce tag pour économiser du temps
                self.skip_tag(tag_type)?;
            }
        }

        Ok(NbtTag::Compound(map))
    }

    fn read_int_array(&mut self) -> Result<NbtTag> {
        let len = self.read_i32()? as usize;
        let mut array = Vec::with_capacity(len);
        for _ in 0..len {
            array.push(self.read_i32()?);
        }
        Ok(NbtTag::IntArray(array))
    }

    fn read_long_array(&mut self) -> Result<NbtTag> {
        let len = self.read_i32()? as usize;
        let mut array = Vec::with_capacity(len);
        for _ in 0..len {
            array.push(self.read_i64()?);
        }
        Ok(NbtTag::LongArray(array))
    }

    // Streaming methods
    pub fn skip_tag(&mut self, tag_type: u8) -> Result<()> {
        match tag_type {
            0 => {}
            1 => {
                self.cursor += 1;
            }
            2 => {
                self.cursor += 2;
            }
            3 => {
                self.cursor += 4;
            }
            4 => {
                self.cursor += 8;
            }
            5 => {
                self.cursor += 4;
            }
            6 => {
                self.cursor += 8;
            }
            7 => {
                let len = self.read_i32()? as usize;
                self.cursor += len;
            }
            8 => {
                let len = self.read_i16()? as usize;
                self.cursor += len;
            }
            9 => {
                let list_type = self.read_u8()?;
                let len = self.read_i32()? as usize;
                for _ in 0..len {
                    self.skip_tag(list_type)?;
                }
            }
            10 => loop {
                let tag_type = self.read_u8()?;
                if tag_type == 0 {
                    break;
                }
                self.read_string()?;
                self.skip_tag(tag_type)?;
            },
            11 => {
                let len = self.read_i32()? as usize;
                self.cursor += len * 4;
            }
            12 => {
                let len = self.read_i32()? as usize;
                self.cursor += len * 8;
            }
            _ => return Err(NbtError::InvalidTagType(tag_type)),
        }
        Ok(())
    }

    pub fn find_path(&mut self, path: &str) -> Result<Option<NbtTag>> {
        let original_cursor = self.cursor;
        self.cursor = 0;

        let tag_type = self.read_u8()?;
        if tag_type != 10 {
            return Err(NbtError::InvalidTagType(tag_type));
        }

        let _root_name = self.read_string()?;
        let result = self.find_in_compound(path);

        self.cursor = original_cursor;
        result
    }

    fn find_in_compound(&mut self, target_path: &str) -> Result<Option<NbtTag>> {
        loop {
            let tag_type = self.read_u8()?;
            if tag_type == 0 {
                break;
            }

            let name = self.read_string()?;

            if name == target_path {
                return Ok(Some(self.read_tag(tag_type)?));
            }

            self.skip_tag(tag_type)?;
        }

        Ok(None)
    }
}

/// Simple NBT writer
pub struct NbtWriter {
    buffer: Vec<u8>,
    endian: Endian,
}

impl NbtWriter {
    pub fn new(endian: Endian) -> Self {
        Self {
            buffer: Vec::with_capacity(1024),
            endian,
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buffer
    }

    pub fn write_u8(&mut self, value: u8) {
        self.buffer.push(value);
    }

    pub fn write_i8(&mut self, value: i8) {
        self.write_u8(value as u8);
    }

    pub fn write_i16(&mut self, value: i16) {
        let bytes = match self.endian {
            Endian::Big => value.to_be_bytes(),
            Endian::Little => value.to_le_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
    }

    pub fn write_i32(&mut self, value: i32) {
        let bytes = match self.endian {
            Endian::Big => value.to_be_bytes(),
            Endian::Little => value.to_le_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
    }

    pub fn write_i64(&mut self, value: i64) {
        let bytes = match self.endian {
            Endian::Big => value.to_be_bytes(),
            Endian::Little => value.to_le_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
    }

    pub fn write_f32(&mut self, value: f32) {
        self.write_i32(value.to_bits() as i32);
    }

    pub fn write_f64(&mut self, value: f64) {
        self.write_i64(value.to_bits() as i64);
    }

    pub fn write_string(&mut self, value: &str) {
        self.write_i16(value.len() as i16);
        self.buffer.extend_from_slice(value.as_bytes());
    }

    pub fn write_tag(&mut self, tag: &NbtTag) -> Result<()> {
        match tag {
            NbtTag::End => {}
            NbtTag::Byte(v) => self.write_i8(*v),
            NbtTag::Short(v) => self.write_i16(*v),
            NbtTag::Int(v) => self.write_i32(*v),
            NbtTag::Long(v) => self.write_i64(*v),
            NbtTag::Float(v) => self.write_f32(*v),
            NbtTag::Double(v) => self.write_f64(*v),
            NbtTag::ByteArray(array) => self.write_byte_array(array),
            NbtTag::String(s) => self.write_string(s),
            NbtTag::List { tag_type, items } => self.write_list(*tag_type, items),
            NbtTag::Compound(map) => self.write_compound(map),
            NbtTag::IntArray(array) => self.write_int_array(array),
            NbtTag::LongArray(array) => self.write_long_array(array),
        }
        Ok(())
    }

    fn write_byte_array(&mut self, array: &[i8]) {
        self.write_i32(array.len() as i32);
        for &byte in array {
            self.write_i8(byte);
        }
    }

    fn write_list(&mut self, tag_type: u8, items: &[NbtTag]) {
        self.write_u8(tag_type);
        self.write_i32(items.len() as i32);
        for item in items {
            let _ = self.write_tag(item);
        }
    }

    fn write_compound(&mut self, map: &HashMap<String, NbtTag>) {
        for (name, tag) in map {
            self.write_u8(tag.type_id());
            self.write_string(name);
            let _ = self.write_tag(tag);
        }
        self.write_u8(0); // End tag
    }

    fn write_int_array(&mut self, array: &[i32]) {
        self.write_i32(array.len() as i32);
        for &int in array {
            self.write_i32(int);
        }
    }

    fn write_long_array(&mut self, array: &[i64]) {
        self.write_i32(array.len() as i32);
        for &long in array {
            self.write_i64(long);
        }
    }
}
