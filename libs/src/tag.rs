use crate::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum NbtTag {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List { tag_type: u8, items: Vec<NbtTag> },
    Compound(HashMap<String, NbtTag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl NbtTag {
    // Create NBT Byte Tags
    pub fn byte(value: i8) -> Self {
        NbtTag::Byte(value)
    }

    // Create NBT Int Tags
    pub fn int(value: i32) -> Self {
        NbtTag::Int(value)
    }

    // Create NBT String Tags
    pub fn string(value: impl Into<String>) -> Self {
        NbtTag::String(value.into())
    }

    // Create NBT Compound Tags
    pub fn compound() -> Self {
        NbtTag::Compound(HashMap::new())
    }

    // Create NBT List Tags
    pub fn list(tag_type: u8) -> Self {
        NbtTag::List {
            tag_type,
            items: Vec::new(),
        }
    }

    // Get NBT type ID (same as TypeScript NbtType enum)
    pub fn type_id(&self) -> u8 {
        match self {
            NbtTag::End => 0,
            NbtTag::Byte(_) => 1,
            NbtTag::Short(_) => 2,
            NbtTag::Int(_) => 3,
            NbtTag::Long(_) => 4,
            NbtTag::Float(_) => 5,
            NbtTag::Double(_) => 6,
            NbtTag::ByteArray(_) => 7,
            NbtTag::String(_) => 8,
            NbtTag::List { .. } => 9,
            NbtTag::Compound(_) => 10,
            NbtTag::IntArray(_) => 11,
            NbtTag::LongArray(_) => 12,
        }
    }

    // Get as number (0 if not numeric)
    pub fn as_number(&self) -> f64 {
        match self {
            NbtTag::Byte(v) => *v as f64,
            NbtTag::Short(v) => *v as f64,
            NbtTag::Int(v) => *v as f64,
            NbtTag::Long(v) => *v as f64,
            NbtTag::Float(v) => *v as f64,
            NbtTag::Double(v) => *v,
            _ => 0.0,
        }
    }

    // Get as string (empty if not string)
    pub fn as_string(&self) -> &str {
        match self {
            NbtTag::String(s) => s,
            _ => "",
        }
    }

    // Get as compound (None if not compound)
    pub fn as_compound(&self) -> Option<&HashMap<String, NbtTag>> {
        match self {
            NbtTag::Compound(map) => Some(map),
            _ => None,
        }
    }

    // Get mutable compound
    pub fn as_compound_mut(&mut self) -> Option<&mut HashMap<String, NbtTag>> {
        match self {
            NbtTag::Compound(map) => Some(map),
            _ => None,
        }
    }

    // Get as list (None if not list)
    pub fn as_list(&self) -> Option<(&u8, &Vec<NbtTag>)> {
        match self {
            NbtTag::List { tag_type, items } => Some((tag_type, items)),
            _ => None,
        }
    }

    // Get mutable list
    pub fn as_list_mut(&mut self) -> Option<(&mut u8, &mut Vec<NbtTag>)> {
        match self {
            NbtTag::List { tag_type, items } => Some((tag_type, items)),
            _ => None,
        }
    }

    // Type checking methods - simple and fast
    pub fn is_number(&self) -> bool {
        matches!(
            self,
            NbtTag::Byte(_)
                | NbtTag::Short(_)
                | NbtTag::Int(_)
                | NbtTag::Long(_)
                | NbtTag::Float(_)
                | NbtTag::Double(_)
        )
    }

    // Check if tag is a string
    pub fn is_string(&self) -> bool {
        matches!(self, NbtTag::String(_))
    }

    // Check if tag is a compound
    pub fn is_compound(&self) -> bool {
        matches!(self, NbtTag::Compound(_))
    }

    // Check if tag is a list
    pub fn is_list(&self) -> bool {
        matches!(self, NbtTag::List { .. })
    }

    // Get nested value by key (compound only)
    pub fn get(&self, key: &str) -> Option<&NbtTag> {
        self.as_compound()?.get(key)
    }

    // Get string value by key
    pub fn get_string(&self, key: &str) -> &str {
        self.get(key).map(|tag| tag.as_string()).unwrap_or("")
    }

    // Get number value by key
    pub fn get_number(&self, key: &str) -> f64 {
        self.get(key).map(|tag| tag.as_number()).unwrap_or(0.0)
    }

    // Get boolean value by key (number != 0)
    pub fn get_bool(&self, key: &str) -> bool {
        self.get_number(key) != 0.0
    }

    // Get compound by key
    pub fn get_compound(&self, key: &str) -> Option<&HashMap<String, NbtTag>> {
        self.get(key)?.as_compound()
    }
}
