use crate::SpudTypes;

fn append_le_bytes<T: AsRef<[u8]>>(bytes: T, writer: &mut Vec<u8>) {
    writer.extend_from_slice(bytes.as_ref());
}

pub trait SpudAddNumber {
    fn get_spud_type_tag(&self) -> SpudTypes;
    fn write_spud_bytes(&self, writer: &mut Vec<u8>);
}

impl SpudAddNumber for i8 {
    fn get_spud_type_tag(&self) -> SpudTypes {
        SpudTypes::I8
    }

    fn write_spud_bytes(&self, writer: &mut Vec<u8>) {
        append_le_bytes(self.to_le_bytes(), writer);
    }
}

impl SpudAddNumber for i16 {
    fn get_spud_type_tag(&self) -> SpudTypes {
        SpudTypes::I16
    }

    fn write_spud_bytes(&self, writer: &mut Vec<u8>) {
        append_le_bytes(self.to_le_bytes(), writer);
    }
}

impl SpudAddNumber for i32 {
    fn get_spud_type_tag(&self) -> SpudTypes {
        SpudTypes::I32
    }

    fn write_spud_bytes(&self, writer: &mut Vec<u8>) {
        append_le_bytes(self.to_le_bytes(), writer);
    }
}

impl SpudAddNumber for i64 {
    fn get_spud_type_tag(&self) -> SpudTypes {
        SpudTypes::I64
    }

    fn write_spud_bytes(&self, writer: &mut Vec<u8>) {
        append_le_bytes(self.to_le_bytes(), writer);
    }
}

impl SpudAddNumber for u8 {
    fn get_spud_type_tag(&self) -> SpudTypes {
        SpudTypes::U8
    }

    fn write_spud_bytes(&self, writer: &mut Vec<u8>) {
        append_le_bytes(self.to_le_bytes(), writer);
    }
}

impl SpudAddNumber for u16 {
    fn get_spud_type_tag(&self) -> SpudTypes {
        SpudTypes::U16
    }

    fn write_spud_bytes(&self, writer: &mut Vec<u8>) {
        append_le_bytes(self.to_le_bytes(), writer);
    }
}

impl SpudAddNumber for u32 {
    fn get_spud_type_tag(&self) -> SpudTypes {
        SpudTypes::U32
    }

    fn write_spud_bytes(&self, writer: &mut Vec<u8>) {
        append_le_bytes(self.to_le_bytes(), writer);
    }
}

impl SpudAddNumber for u64 {
    fn get_spud_type_tag(&self) -> SpudTypes {
        SpudTypes::U64
    }

    fn write_spud_bytes(&self, writer: &mut Vec<u8>) {
        append_le_bytes(self.to_le_bytes(), writer);
    }
}

impl SpudAddNumber for f32 {
    fn get_spud_type_tag(&self) -> SpudTypes {
        SpudTypes::F32
    }

    fn write_spud_bytes(&self, writer: &mut Vec<u8>) {
        append_le_bytes(self.to_le_bytes(), writer);
    }
}

impl SpudAddNumber for f64 {
    fn get_spud_type_tag(&self) -> SpudTypes {
        SpudTypes::F64
    }

    fn write_spud_bytes(&self, writer: &mut Vec<u8>) {
        append_le_bytes(self.to_le_bytes(), writer);
    }
}
