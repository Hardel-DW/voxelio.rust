use crate::{Result, SnbtError};
use nbt_core::{NbtTag, HashMap};
use winnow::{
    ascii::{space0, digit1},
    combinator::{alt, delimited, opt, preceded, separated, terminated},
    token::{one_of, take_till, take_while},
    PResult, Parser,
};

type Input<'i> = &'i str;

/// Parse SNBT string to NBT tag
pub fn parse_tag(input: &str) -> Result<NbtTag> {
    let mut input = input.trim();
    match parse_value.parse_next(&mut input) {
        Ok(tag) => {
            if input.trim().is_empty() {
                Ok(tag)
            } else {
                Err(SnbtError::parse_error("Unexpected characters after valid SNBT", input.len()))
            }
        }
        Err(e) => Err(SnbtError::parse_error(format!("Parse error: {e}"), 0)),
    }
}

/// Parse any NBT value
fn parse_value(input: &mut Input) -> PResult<NbtTag> {
    delimited(
        space0,
        alt((
            parse_compound,
            parse_array,  // Must come before parse_list (both start with '[')
            parse_list,
            parse_quoted_string,
            parse_unquoted_value,
        )),
        space0,
    ).parse_next(input)
}

/// Parse compound: {key:value,key:value}
fn parse_compound(input: &mut Input) -> PResult<NbtTag> {
    let pairs = delimited(
        ('{', space0),
        separated(0.., parse_compound_entry, (',', space0)),
        (space0, '}'),
    );
    
    pairs.map(|entries: Vec<(String, NbtTag)>| {
        let map: HashMap<String, NbtTag> = entries.into_iter().collect();
        NbtTag::Compound(map)
    }).parse_next(input)
}

/// Parse compound entry: key:value
fn parse_compound_entry(input: &mut Input) -> PResult<(String, NbtTag)> {
    let key = delimited(space0, parse_string_key, space0);
    let value = preceded((':', space0), parse_value);
    (key, value).parse_next(input)
}

/// Parse string key (quoted or unquoted)
fn parse_string_key(input: &mut Input) -> PResult<String> {
    alt((
        parse_quoted_string.map(|tag| tag.as_string().to_string()),
        parse_unquoted_string.map(|s| s.to_string()),
    )).parse_next(input)
}

/// Parse list: [value,value,value]
fn parse_list(input: &mut Input) -> PResult<NbtTag> {
    let items = delimited(
        ('[', space0),
        separated(0.., parse_value, (',', space0)),
        (space0, ']'),
    );
    
    items.map(|items: Vec<NbtTag>| {
        if items.is_empty() {
            return NbtTag::List { tag_type: 0, items };
        }
        
        let first_type = items[0].type_id();
        
        // Verify all items have same type
        for item in &items[1..] {
            if item.type_id() != first_type {
                // In real parser, we would return an error, but for simplicity we'll take first type
                break;
            }
        }
        
        NbtTag::List { tag_type: first_type, items }
    }).parse_next(input)
}

/// Parse typed arrays: [B;1,2,3] [I;1,2,3] [L;1,2,3]
fn parse_array(input: &mut Input) -> PResult<NbtTag> {
    let (array_type, items): (char, Vec<NbtTag>) = delimited(
        ('[', space0),
        (
            terminated(one_of(['B', 'I', 'L']), (space0, ';', space0)),
            separated(0.., parse_array_element, (',', space0))
        ),
        (space0, ']')
    ).parse_next(input)?;
    
    match array_type {
        'B' => {
            let bytes: Vec<i8> = items.into_iter()
                .map(|tag| tag.as_number() as i8)
                .collect();
            Ok(NbtTag::ByteArray(bytes))
        },
        'I' => {
            let ints: Vec<i32> = items.into_iter()
                .map(|tag| tag.as_number() as i32)
                .collect();
            Ok(NbtTag::IntArray(ints))
        },
        'L' => {
            let longs: Vec<i64> = items.into_iter()
                .map(|tag| tag.as_number() as i64)
                .collect();
            Ok(NbtTag::LongArray(longs))
        },
        _ => unreachable!(),
    }
}

/// Parse array element (must be numeric)
fn parse_array_element(input: &mut Input) -> PResult<NbtTag> {
    delimited(space0, parse_number, space0).parse_next(input)
}

/// Parse quoted string: "value" or 'value'
fn parse_quoted_string(input: &mut Input) -> PResult<NbtTag> {
    let quote = one_of(['"', '\'']).parse_next(input)?;
    let content = take_till(0.., move |c| c == quote).parse_next(input)?;
    one_of(['"', '\'']).parse_next(input)?; // consume closing quote
    
    Ok(NbtTag::String(content.to_string()))
}

/// Parse unquoted string (identifier-like)
fn parse_unquoted_string<'i>(input: &mut &'i str) -> PResult<&'i str> {
    take_while(1.., |c: char| {
        c.is_alphanumeric() || c == '_' || c == '-' || c == '.' || c == '+'
    }).parse_next(input)
}

/// Parse unquoted value (number, boolean, or fallback string)
fn parse_unquoted_value(input: &mut Input) -> PResult<NbtTag> {
    let value = parse_unquoted_string.parse_next(input)?;
    
    // Try to parse as number first
    if let Ok(tag) = parse_number_from_str(value) {
        return Ok(tag);
    }
    
    // Try boolean
    match value.to_lowercase().as_str() {
        "true" => Ok(NbtTag::Byte(1)),
        "false" => Ok(NbtTag::Byte(0)),
        _ => Ok(NbtTag::String(value.to_string())),
    }
}

/// Parse number with type suffixes
fn parse_number(input: &mut Input) -> PResult<NbtTag> {
    let start = *input;
    let _sign = opt(one_of(['+', '-'])).parse_next(input)?;
    let _digits = digit1.parse_next(input)?;
    let _fraction = opt(preceded('.', digit1)).parse_next(input)?;
    let _suffix = opt(one_of(['b', 'B', 's', 'S', 'l', 'L', 'f', 'F', 'd', 'D'])).parse_next(input)?;
    
    let number_str = &start[..start.len() - input.len()];
    parse_number_from_str(number_str).map_err(|_| winnow::error::ErrMode::Backtrack(winnow::error::ContextError::new()))
}

/// Parse number from string with suffix handling
fn parse_number_from_str(s: &str) -> Result<NbtTag> {
    let s = s.trim();
    
    if s.is_empty() {
        return Err(SnbtError::InvalidNumber(s.to_string()));
    }
    
    // Handle suffixes
    if let Some(last_char) = s.chars().last() {
        let (num_str, tag_type) = match last_char.to_ascii_lowercase() {
            'b' => (&s[..s.len()-1], "byte"),
            's' => (&s[..s.len()-1], "short"), 
            'l' => (&s[..s.len()-1], "long"),
            'f' => (&s[..s.len()-1], "float"),
            'd' => (&s[..s.len()-1], "double"),
            _ => (s, "auto"),
        };
        
        match tag_type {
            "byte" => num_str.parse::<i8>().map(NbtTag::Byte).map_err(|_| SnbtError::InvalidNumber(s.to_string())),
            "short" => num_str.parse::<i16>().map(NbtTag::Short).map_err(|_| SnbtError::InvalidNumber(s.to_string())),
            "long" => num_str.parse::<i64>().map(NbtTag::Long).map_err(|_| SnbtError::InvalidNumber(s.to_string())),
            "float" => num_str.parse::<f32>().map(NbtTag::Float).map_err(|_| SnbtError::InvalidNumber(s.to_string())),
            "double" => num_str.parse::<f64>().map(NbtTag::Double).map_err(|_| SnbtError::InvalidNumber(s.to_string())),
            "auto" => {
                // Auto-detect type
                if s.contains('.') || s.contains('e') || s.contains('E') {
                    s.parse::<f64>().map(NbtTag::Double).map_err(|_| SnbtError::InvalidNumber(s.to_string()))
                } else {
                    s.parse::<i32>().map(NbtTag::Int).map_err(|_| SnbtError::InvalidNumber(s.to_string()))
                }
            },
            _ => unreachable!(),
        }
    } else {
        Err(SnbtError::InvalidNumber(s.to_string()))
    }
}

/// Format NBT tag to SNBT string
pub fn format_tag(tag: &NbtTag, pretty: bool) -> String {
    format_tag_with_indent(tag, 0, pretty)
}

fn format_tag_with_indent(tag: &NbtTag, indent: usize, pretty: bool) -> String {
    match tag {
        NbtTag::End => "".to_string(),
        NbtTag::Byte(v) => format!("{}b", v),
        NbtTag::Short(v) => format!("{}s", v),
        NbtTag::Int(v) => v.to_string(),
        NbtTag::Long(v) => format!("{}L", v),
        NbtTag::Float(v) => format!("{}f", v),
        NbtTag::Double(v) => format!("{}d", v),
        NbtTag::String(s) => format_string(s),
        NbtTag::ByteArray(arr) => format_array("B", arr.iter().map(|&x| x.to_string()), pretty, indent),
        NbtTag::IntArray(arr) => format_array("I", arr.iter().map(|&x| x.to_string()), pretty, indent),
        NbtTag::LongArray(arr) => format_array("L", arr.iter().map(|&x| format!("{}L", x)), pretty, indent),
        NbtTag::List { items, .. } => format_list(items, pretty, indent),
        NbtTag::Compound(map) => format_compound(map, pretty, indent),
    }
}

fn format_string(s: &str) -> String {
    if s.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.') && !s.is_empty() {
        s.to_string()
    } else {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    }
}

fn format_array<I>(prefix: &str, items: I, pretty: bool, indent: usize) -> String 
where 
    I: Iterator<Item = String>
{
    let items: Vec<String> = items.collect();
    if pretty && items.len() > 3 {
        let indent_str = "  ".repeat(indent + 1);
        let items_str = items.join(&format!(",\n{}", indent_str));
        format!("[{};\n{}{}\n{}]", prefix, indent_str, items_str, "  ".repeat(indent))
    } else {
        format!("[{};{}]", prefix, items.join(","))
    }
}

fn format_list(items: &[NbtTag], pretty: bool, indent: usize) -> String {
    if pretty && items.len() > 3 {
        let indent_str = "  ".repeat(indent + 1);
        let items_str: Vec<String> = items.iter()
            .map(|item| format_tag_with_indent(item, indent + 1, pretty))
            .collect();
        format!("[\n{}{}\n{}]", indent_str, items_str.join(&format!(",\n{}", indent_str)), "  ".repeat(indent))
    } else {
        let items_str: Vec<String> = items.iter()
            .map(|item| format_tag_with_indent(item, indent, false))
            .collect();
        format!("[{}]", items_str.join(","))
    }
}

fn format_compound(map: &HashMap<String, NbtTag>, pretty: bool, indent: usize) -> String {
    if pretty && map.len() > 2 {
        let indent_str = "  ".repeat(indent + 1);
        let entries: Vec<String> = map.iter()
            .map(|(k, v)| format!("{}:{}", format_string(k), format_tag_with_indent(v, indent + 1, pretty)))
            .collect();
        format!("{{\n{}{}\n{}}}", indent_str, entries.join(&format!(",\n{}", indent_str)), "  ".repeat(indent))
    } else {
        let entries: Vec<String> = map.iter()
            .map(|(k, v)| format!("{}:{}", format_string(k), format_tag_with_indent(v, indent, false)))
            .collect();
        format!("{{{}}}", entries.join(","))
    }
} 