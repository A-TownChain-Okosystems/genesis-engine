//! Structured validator for canonical Genesis Franchise Factory .atc specifications.
//!
//! This intentionally does not use regular expressions. The contract is split into
//! metadata parsing, lexical analysis and semantic validation so malformed source
//! cannot accidentally satisfy a textual pattern.

use std::collections::BTreeSet;

const COPYRIGHT_PREFIX: &str = "Copyright (c) 2026";
const DEFAULT_FORBIDDEN_IMPORTS: &[&str] = &["chronicles"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtcField { pub name: String, pub ty: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtcStruct { pub name: String, pub fields: Vec<AtcField> }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtcEnum { pub name: String, pub variants: Vec<String> }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtcFunction { pub name: String, pub params: Vec<AtcField>, pub return_type: Option<String> }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtcSpec {
    pub ad_id: u16, pub title: String, pub filename: String, pub imports: Vec<String>,
    pub structs: Vec<String>, pub enums: Vec<String>, pub functions: Vec<String>,
    pub struct_definitions: Vec<AtcStruct>, pub enum_definitions: Vec<AtcEnum>,
    pub function_definitions: Vec<AtcFunction>,
}

impl AtcSpec {
    pub fn has_implementation_surface(&self) -> bool {
        !self.structs.is_empty() && !self.functions.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecError {
    EmptySource,
    InvalidFilename,
    MissingAdHeader,
    InvalidAdHeader,
    InvalidAdId,
    MissingCopyright,
    ForbiddenDependency { import: String, forbidden: String },
    UnexpectedToken { offset: usize, token: String },
    ExpectedIdentifier { offset: usize, context: &'static str },
    DuplicateDeclaration { kind: &'static str, name: String },
    MissingImplementationSurface,
    MissingType { offset: usize, context: &'static str },
    InvalidFunctionSignature { offset: usize },
    InvalidStructBody { offset: usize },
    AdFilenameMismatch { header: u16, filename: u16 },
    DuplicateAdId(u16),
    NonCanonicalAdRange { ad_id: u16 },
    MissingCanonicalAdId { ad_id: u16 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenKind {
    Ident(String),
    String(String),
    Number(String),
    Symbol(char),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    kind: TokenKind,
    offset: usize,
}

fn lex(source: &str) -> Result<Vec<Token>, SpecError> {
    let bytes = source.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;

    while i < bytes.len() {
        match bytes[i] {
            b' ' | b'\t' | b'\r' | b'\n' => i += 1,
            b'/' if bytes.get(i + 1) == Some(&b'/') => {
                i += 2;
                while i < bytes.len() && bytes[i] != b'\n' { i += 1; }
            }
            b'/' if bytes.get(i + 1) == Some(&b'*') => {
                let start = i;
                i += 2;
                let mut closed = false;
                while i + 1 < bytes.len() {
                    if bytes[i] == b'*' && bytes[i + 1] == b'/' {
                        i += 2;
                        closed = true;
                        break;
                    }
                    i += 1;
                }
                if !closed {
                    return Err(SpecError::UnexpectedToken {
                        offset: start,
                        token: "unterminated block comment".into(),
                    });
                }
            }
            b'"' => {
                let start = i;
                i += 1;
                let mut value = String::new();
                let mut closed = false;
                while i < bytes.len() {
                    match bytes[i] {
                        b'\\' if i + 1 < bytes.len() => {
                            value.push(bytes[i + 1] as char);
                            i += 2;
                        }
                        b'"' => {
                            i += 1;
                            closed = true;
                            break;
                        }
                        c => {
                            value.push(c as char);
                            i += 1;
                        }
                    }
                }
                if !closed {
                    return Err(SpecError::UnexpectedToken {
                        offset: start,
                        token: "unterminated string".into(),
                    });
                }
                out.push(Token { kind: TokenKind::String(value), offset: start });
            }
            c if (c as char).is_ascii_alphabetic() || c == b'_' => {
                let start = i;
                i += 1;
                while i < bytes.len()
                    && ((bytes[i] as char).is_ascii_alphanumeric() || bytes[i] == b'_')
                {
                    i += 1;
                }
                out.push(Token {
                    kind: TokenKind::Ident(source[start..i].to_string()),
                    offset: start,
                });
            }
            c if (c as char).is_ascii_digit() => {
                let start = i;
                i += 1;
                while i < bytes.len() && (bytes[i] as char).is_ascii_digit() { i += 1; }
                out.push(Token {
                    kind: TokenKind::Number(source[start..i].to_string()),
                    offset: start,
                });
            }
            c => {
                out.push(Token { kind: TokenKind::Symbol(c as char), offset: i });
                i += 1;
            }
        }
    }

    Ok(out)
}

fn parse_ad_header(source: &str) -> Result<(u16, String), SpecError> {
    for line in source.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("//") { continue; }
        let body = trimmed[2..].trim();
        let Some(rest) = body.strip_prefix("AD-") else { continue; };

        let digits_len = rest.bytes().take_while(u8::is_ascii_digit).count();
        if digits_len == 0 {
            return Err(SpecError::InvalidAdHeader);
        }

        let id = rest[..digits_len]
            .parse::<u16>()
            .map_err(|_| SpecError::InvalidAdId)?;
        let title = rest[digits_len..]
            .trim_start_matches([' ', '\t', '—', '-'])
            .trim();

        if title.is_empty() {
            return Err(SpecError::InvalidAdHeader);
        }
        return Ok((id, title.to_string()));
    }
    Err(SpecError::MissingAdHeader)
}

fn has_copyright(source: &str) -> bool {
    source.lines().any(|line| line.trim_start_matches('/').trim().starts_with(COPYRIGHT_PREFIX))
}

fn filename_ad_id(filename: &str) -> Result<u16, SpecError> {
    if !filename.ends_with(".atc") {
        return Err(SpecError::InvalidFilename);
    }
    let stem = &filename[..filename.len() - 4];
    let marker = "_ad";
    let Some(pos) = stem.rfind(marker) else {
        return Err(SpecError::InvalidFilename);
    };
    let digits = &stem[pos + marker.len()..];
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err(SpecError::InvalidFilename);
    }
    digits.parse::<u16>().map_err(|_| SpecError::InvalidFilename)
}

fn expect_ident(tokens: &[Token], index: usize, context: &'static str) -> Result<String, SpecError> {
    match tokens.get(index).map(|t| &t.kind) {
        Some(TokenKind::Ident(name)) => Ok(name.clone()),
        _ => Err(SpecError::ExpectedIdentifier {
            offset: tokens.get(index).map_or(0, |t| t.offset),
            context,
        }),
    }
}

fn parse_import(tokens: &[Token], mut i: usize) -> Result<(String, usize), SpecError> {
    let path = match tokens.get(i).map(|t| &t.kind) {
        Some(TokenKind::String(value)) => {
            i += 1;
            value.clone()
        }
        _ => {
            return Err(SpecError::UnexpectedToken {
                offset: tokens.get(i).map_or(0, |t| t.offset),
                token: "import requires a string path".into(),
            });
        }
    };

    while i < tokens.len() {
        match &tokens[i].kind {
            TokenKind::Ident(name) if name == "as" => {
                let _ = expect_ident(tokens, i + 1, "import alias")?;
                i += 2;
            }
            TokenKind::Ident(name)
                if matches!(name.as_str(), "import" | "struct" | "enum" | "pub" | "type" | "event") =>
            {
                break;
            }
            _ => i += 1,
        }
    }
    Ok((path, i))
}

fn consume_type(tokens: &[Token], mut i: usize) -> Result<(String, usize), SpecError> {
    let offset = tokens.get(i).map_or(0, |t| t.offset);
    if !matches!(tokens.get(i).map(|t| &t.kind), Some(TokenKind::Ident(_))) {
        return Err(SpecError::MissingType { offset, context: "type" });
    }
    let mut ty = String::new();
    while let Some(t) = tokens.get(i) {
        match &t.kind {
            TokenKind::Ident(v) | TokenKind::Number(v) => { ty.push_str(v); i += 1; }
            TokenKind::Symbol(c) if matches!(c, '<' | '>' | '[' | ']' | ',') => { ty.push(*c); i += 1; }
            _ => break,
        }
    }
    Ok((ty, i))
}

fn parse_struct_definition(tokens: &[Token], mut i: usize) -> Result<(AtcStruct, usize), SpecError> {
    let name = expect_ident(tokens, i + 1, "struct name")?;
    i += 2;
    if !matches!(tokens.get(i).map(|t| &t.kind), Some(TokenKind::Symbol('{'))) {
        return Err(SpecError::InvalidStructBody { offset: tokens.get(i).map_or(0, |t| t.offset) });
    }
    i += 1;
    let mut fields = Vec::new();
    while i < tokens.len() && !matches!(tokens[i].kind, TokenKind::Symbol('}')) {
        let field = expect_ident(tokens, i, "struct field name")?;
        if !matches!(tokens.get(i + 1).map(|t| &t.kind), Some(TokenKind::Symbol(':'))) {
            return Err(SpecError::InvalidStructBody { offset: tokens.get(i).map_or(0, |t| t.offset) });
        }
        let (ty, next) = consume_type(tokens, i + 2)?;
        fields.push(AtcField { name: field, ty });
        i = next;
        if matches!(tokens.get(i).map(|t| &t.kind), Some(TokenKind::Symbol(',')) | Some(TokenKind::Symbol(';'))) { i += 1; }
    }
    if !matches!(tokens.get(i).map(|t| &t.kind), Some(TokenKind::Symbol('}'))) {
        return Err(SpecError::InvalidStructBody { offset: tokens.get(i).map_or(0, |t| t.offset) });
    }
    Ok((AtcStruct { name, fields }, i + 1))
}

fn parse_enum_definition(tokens: &[Token], mut i: usize) -> Result<(AtcEnum, usize), SpecError> {
    let name = expect_ident(tokens, i + 1, "enum name")?;
    i += 2;
    if !matches!(tokens.get(i).map(|t| &t.kind), Some(TokenKind::Symbol('{'))) {
        return Err(SpecError::UnexpectedToken { offset: tokens.get(i).map_or(0, |t| t.offset), token: "enum requires body".into() });
    }
    i += 1;
    let mut variants = Vec::new();
    while i < tokens.len() && !matches!(tokens[i].kind, TokenKind::Symbol('}')) {
        variants.push(expect_ident(tokens, i, "enum variant")?);
        i += 1;
        if matches!(tokens.get(i).map(|t| &t.kind), Some(TokenKind::Symbol(',')) | Some(TokenKind::Symbol(';'))) { i += 1; }
    }
    if !matches!(tokens.get(i).map(|t| &t.kind), Some(TokenKind::Symbol('}'))) {
        return Err(SpecError::UnexpectedToken { offset: tokens.get(i).map_or(0, |t| t.offset), token: "unterminated enum".into() });
    }
    Ok((AtcEnum { name, variants }, i + 1))
}

fn parse_function_definition(tokens: &[Token], mut i: usize) -> Result<(AtcFunction, usize), SpecError> {
    let name = expect_ident(tokens, i + 2, "public function name")?;
    i += 3;
    if !matches!(tokens.get(i).map(|t| &t.kind), Some(TokenKind::Symbol('('))) {
        return Err(SpecError::InvalidFunctionSignature { offset: tokens.get(i).map_or(0, |t| t.offset) });
    }
    i += 1;
    let mut params = Vec::new();
    while i < tokens.len() && !matches!(tokens[i].kind, TokenKind::Symbol(')')) {
        let param = expect_ident(tokens, i, "function parameter name")?;
        if !matches!(tokens.get(i + 1).map(|t| &t.kind), Some(TokenKind::Symbol(':'))) {
            return Err(SpecError::InvalidFunctionSignature { offset: tokens.get(i).map_or(0, |t| t.offset) });
        }
        let (ty, next) = consume_type(tokens, i + 2)?;
        params.push(AtcField { name: param, ty });
        i = next;
        if matches!(tokens.get(i).map(|t| &t.kind), Some(TokenKind::Symbol(','))) { i += 1; }
    }
    if !matches!(tokens.get(i).map(|t| &t.kind), Some(TokenKind::Symbol(')'))) {
        return Err(SpecError::InvalidFunctionSignature { offset: tokens.get(i).map_or(0, |t| t.offset) });
    }
    i += 1;
    let return_type = if matches!(tokens.get(i).map(|t| &t.kind), Some(TokenKind::Symbol('-')))
        && matches!(tokens.get(i + 1).map(|t| &t.kind), Some(TokenKind::Symbol('>'))) {
        let (ty, next) = consume_type(tokens, i + 2)?;
        i = next;
        Some(ty)
    } else { None };
    Ok((AtcFunction { name, params, return_type }, i))
}

fn collect_surface(tokens: &[Token]) -> Result<(Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<AtcStruct>, Vec<AtcEnum>, Vec<AtcFunction>), SpecError> {
    let mut imports = Vec::new(); let mut structs = Vec::new(); let mut enums = Vec::new(); let mut functions = Vec::new();
    let mut struct_definitions = Vec::new(); let mut enum_definitions = Vec::new(); let mut function_definitions = Vec::new();
    let mut seen_structs = BTreeSet::new(); let mut seen_enums = BTreeSet::new(); let mut seen_functions = BTreeSet::new();
    let mut i = 0;
    while i < tokens.len() {
        let TokenKind::Ident(keyword) = &tokens[i].kind else { i += 1; continue };
        match keyword.as_str() {
            "import" => { let (path, next) = parse_import(tokens, i + 1)?; imports.push(path); i = next; }
            "struct" => { let (def, next) = parse_struct_definition(tokens, i)?; if !seen_structs.insert(def.name.clone()) { return Err(SpecError::DuplicateDeclaration { kind: "struct", name: def.name }); } structs.push(def.name.clone()); struct_definitions.push(def); i = next; }
            "enum" => { let (def, next) = parse_enum_definition(tokens, i)?; if !seen_enums.insert(def.name.clone()) { return Err(SpecError::DuplicateDeclaration { kind: "enum", name: def.name }); } enums.push(def.name.clone()); enum_definitions.push(def); i = next; }
            "pub" if matches!(tokens.get(i + 1).map(|t| &t.kind), Some(TokenKind::Ident(v)) if v == "fn") => {
                let (def, next) = parse_function_definition(tokens, i)?; if !seen_functions.insert(def.name.clone()) { return Err(SpecError::DuplicateDeclaration { kind: "function", name: def.name }); } functions.push(def.name.clone()); function_definitions.push(def); i = next;
            }
            _ => i += 1,
        }
    }
    Ok((imports, structs, enums, functions, struct_definitions, enum_definitions, function_definitions))
}

pub fn parse_spec(filename: &str, source: &str) -> Result<AtcSpec, SpecError> {
    if source.trim().is_empty() {
        return Err(SpecError::EmptySource);
    }

    let (ad_id, title) = parse_ad_header(source)?;
    if !has_copyright(source) {
        return Err(SpecError::MissingCopyright);
    }

    let filename_id = filename_ad_id(filename)?;
    if filename_id != ad_id {
        return Err(SpecError::AdFilenameMismatch { header: ad_id, filename: filename_id });
    }

    let tokens = lex(source)?;
    let (imports, structs, enums, functions, struct_definitions, enum_definitions, function_definitions) = collect_surface(&tokens)?;

    for import in &imports {
        for forbidden in DEFAULT_FORBIDDEN_IMPORTS {
            if import.to_ascii_lowercase().contains(&forbidden.to_ascii_lowercase()) {
                return Err(SpecError::ForbiddenDependency {
                    import: import.clone(),
                    forbidden: (*forbidden).into(),
                });
            }
        }
    }

    let spec = AtcSpec { ad_id, title, filename: filename.into(), imports, structs, enums, functions, struct_definitions, enum_definitions, function_definitions };
    if !spec.has_implementation_surface() {
        return Err(SpecError::MissingImplementationSurface);
    }
    Ok(spec)
}

pub fn validate_spec_set<'a, I>(specs: I) -> Result<Vec<AtcSpec>, SpecError>
where
    I: IntoIterator<Item = (&'a str, &'a str)>,
{
    let mut parsed = Vec::new();
    let mut ids = BTreeSet::new();

    for (filename, source) in specs {
        let spec = parse_spec(filename, source)?;
        if !ids.insert(spec.ad_id) {
            return Err(SpecError::DuplicateAdId(spec.ad_id));
        }
        if !(20..=43).contains(&spec.ad_id) {
            return Err(SpecError::NonCanonicalAdRange { ad_id: spec.ad_id });
        }
        parsed.push(spec);
    }

    parsed.sort_by_key(|s| s.ad_id);
    Ok(parsed)
}

pub fn validate_canonical_spec_set<'a, I>(specs: I) -> Result<Vec<AtcSpec>, SpecError>
where
    I: IntoIterator<Item = (&'a str, &'a str)>,
{
    let parsed = validate_spec_set(specs)?;
    for ad_id in 20..=43 {
        if !parsed.iter().any(|spec| spec.ad_id == ad_id) {
            return Err(SpecError::MissingCanonicalAdId { ad_id });
        }
    }
    if parsed.len() != 24 {
        return Err(SpecError::MissingCanonicalAdId { ad_id: 43 });
    }
    Ok(parsed)
}


#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = r#"// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// AD-20 — Test Factory
import "std/crypto.atc" as Crypto
struct Core { value: UInt64 }
enum Status { Ready }
pub fn init_gff() -> Core { return Core { value: 0 } }
pub fn run(core: Core) -> Bool { return true }
"#;

    #[test]
    fn parses_structured_surface_without_regex() {
        let spec = parse_spec("test_factory_ad20.atc", VALID).unwrap();
        assert_eq!(spec.ad_id, 20);
        assert_eq!(spec.title, "Test Factory");
        assert_eq!(spec.imports, vec!["std/crypto.atc"]);
        assert_eq!(spec.structs, vec!["Core"]);
        assert_eq!(spec.enums, vec!["Status"]);
        assert_eq!(spec.functions, vec!["init_gff", "run"]);
        assert!(spec.has_implementation_surface());
        assert_eq!(spec.struct_definitions[0].fields[0].ty, "UInt64");
        assert_eq!(spec.function_definitions[0].return_type.as_deref(), Some("Core"));
    }

    #[test]
    fn captures_typed_fields_and_function_parameters() {
        let source = VALID.replace(
            "struct Core { value: UInt64 }",
            "struct Core { values: List<UInt256>, state: Status }"
        ).replace(
            "pub fn init_gff() -> Core { return Core { value: 0 } }",
            "pub fn init_gff(input: UInt256, state: Status) -> Core { return Core { values: [], state: state } }"
        );
        let spec = parse_spec("typed_factory_ad20.atc", &source).unwrap();
        assert_eq!(spec.struct_definitions[0].fields[0].ty, "List<UInt256>");
        assert_eq!(spec.struct_definitions[0].fields[1].name, "state");
        assert_eq!(spec.function_definitions[0].params[0].ty, "UInt256");
        assert_eq!(spec.function_definitions[0].params[1].ty, "Status");
        assert_eq!(spec.function_definitions[0].return_type.as_deref(), Some("Core"));
    }

    #[test]
    fn rejects_missing_struct_field_type() {
        let source = VALID.replace("value: UInt64", "value:");
        assert!(matches!(
            parse_spec("invalid_field_ad20.atc", &source),
            Err(SpecError::MissingType { context: "type", .. })
        ));
    }

    #[test]
    fn rejects_invalid_function_signature() {
        let source = VALID.replace("pub fn run(core: Core) -> Bool", "pub fn run(core Core) -> Bool");
        assert!(matches!(
            parse_spec("invalid_fn_ad20.atc", &source),
            Err(SpecError::InvalidFunctionSignature { .. })
        ));
    }

    #[test]
    fn rejects_header_filename_mismatch() {
        assert_eq!(
            parse_spec("test_factory_ad21.atc", VALID),
            Err(SpecError::AdFilenameMismatch { header: 20, filename: 21 })
        );
    }

    #[test]
    fn rejects_forbidden_dependency() {
        let source = VALID.replace("std/crypto.atc", "std/chronicles.atc");
        assert!(matches!(
            parse_spec("test_factory_ad20.atc", &source),
            Err(SpecError::ForbiddenDependency { .. })
        ));
    }

    #[test]
    fn rejects_missing_surface() {
        let source = "// Copyright (c) 2026 Michael Wroblewski\n// AD-20 — Test Factory\n";
        assert_eq!(
            parse_spec("test_factory_ad20.atc", source),
            Err(SpecError::MissingImplementationSurface)
        );
    }

    #[test]
    fn validates_ad_set_and_duplicate_ids() {
        let a = ("a_ad20.atc", VALID);
        let b = ("b_ad20.atc", VALID);
        assert_eq!(
            validate_spec_set([a, b]),
            Err(SpecError::DuplicateAdId(20))
        );
    }

    #[test]
    fn rejects_incomplete_canonical_set() {
        assert_eq!(
            validate_canonical_spec_set([("test_factory_ad20.atc", VALID)]),
            Err(SpecError::MissingCanonicalAdId { ad_id: 21 })
        );
    }

    #[test]
    fn accepts_complete_canonical_ad_set() {
        let sources: Vec<(String, String)> = (20..=43).map(|id| {
            (format!("test_factory_ad{id}.atc"), VALID.replace("AD-20", &format!("AD-{id}")))
        }).collect();
        let refs: Vec<(&str, &str)> = sources.iter().map(|(f, s)| (f.as_str(), s.as_str())).collect();
        let parsed = validate_canonical_spec_set(refs).unwrap();
        assert_eq!(parsed.len(), 24);
        assert_eq!(parsed.first().unwrap().ad_id, 20);
        assert_eq!(parsed.last().unwrap().ad_id, 43);
    }

    #[test]
    fn enforces_canonical_ad_range() {
        let source = VALID.replace("AD-20", "AD-44");
        assert_eq!(
            validate_spec_set([("test_factory_ad44.atc", &source)]),
            Err(SpecError::NonCanonicalAdRange { ad_id: 44 })
        );
    }
}
