use std::clone;

use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_json::json;
use serde_json::Map;
use serde_json::Number;
use serde_json::Value;

#[cfg(test)]
use pretty_assertions::assert_eq;

// Exercise 1 — Get string field
// Goal: Extract a string field safely.
pub fn get_string_field(value: &Value, field: &str) -> Option<String> {
    value.get(field)?.as_str().map(|e| e.to_string())
}

#[test]
fn test_get_string_field() {
    let v: Value = serde_json::from_str(r#"{"name":"Alice","age":30}"#).unwrap();
    assert_eq!(get_string_field(&v, "name"), Some("Alice".to_string()));
    assert_eq!(get_string_field(&v, "age"), None);
    assert_eq!(get_string_field(&v, "missing"), None);
}

// Exercise 2 — Increment numeric field
// Goal: Mutate a JSON object safely.
pub fn increment_field(value: &mut Value, field: &str) {
    if let Some(Value::Number(num)) = value.get_mut(field) {
        if let Some(n) = num.as_i64() {
            *num = Number::from(n + 1);
        } else if let Some(n) = num.as_f64() {
            *num = Number::from_f64(n + 1.0).unwrap();
        }
    }
}

#[test]
fn test_increment_field() {
    let mut v: Value = serde_json::from_str(r#"{"count":1}"#).unwrap();
    increment_field(&mut v, "count");
    assert_eq!(v["count"], 2);
}

// Exercise 3 — Count boolean true
// Goal: Work with arrays of objects.
pub fn count_true(entries: &[Value], field: &str) -> usize {
    entries
        .iter()
        .filter(|value| value.get(field).and_then(Value::as_bool) == Some(true))
        .count()
}

#[test]
fn test_count_true() {
    let entries: Vec<Value> = serde_json::from_str(
        r#"
        [
            {"active": true},
            {"active": false},
            {"active": true}
        ]
    "#,
    )
    .unwrap();

    assert_eq!(count_true(&entries, "active"), 2);
}

// Exercise 4 — Extract numeric IDs
// Goal: Filter + map.
pub fn extract_ids(entries: &[Value]) -> Vec<u64> {
    entries
        .iter()
        .filter_map(|value| value.get("id").and_then(|e| e.as_u64()))
        .collect()
}

#[test]
fn test_extract_ids() {
    let entries: Vec<Value> = serde_json::from_str(
        r#"
        [
            {"id": 1},
            {"id": 2},
            {"name": "no id"}
        ]
    "#,
    )
    .unwrap();

    assert_eq!(extract_ids(&entries), vec![1, 2]);
}

// Exercise 5 — Match exact field value
// Goal: Compare serde_json::Value directly.
pub fn filter_equals(entries: Vec<Value>, field: &str, value: Value) -> Vec<Value> {
    entries
        .into_iter()
        .filter(|v| v.get(field) == Some(&value))
        .collect()
}

#[test]
fn test_filter_equals() {
    let entries: Vec<Value> = serde_json::from_str(
        r#"
        [
            {"name":"Alice"},
            {"name":"Bob"},
            {"name":"Alice"}
        ]
    "#,
    )
    .unwrap();

    let result = filter_equals(entries, "name", json!("Alice"));
    assert_eq!(result.len(), 2);
}

// Exercise 6 — Add derived field
// Goal: Build new JSON objects.
pub fn add_full_name(entries: Vec<Value>) -> Vec<Value> {
    entries
        .into_iter()
        .filter_map(|mut entry| {
            let full_name = format!(
                "{} {}",
                entry.get("first")?.as_str()?,
                entry.get("last")?.as_str()?
            );

            entry
                .as_object_mut()
                .and_then(|object| object.insert("full_name".to_string(), json!(full_name)));

            Some(entry)
        })
        .collect()
}

#[test]
fn test_add_full_name() {
    let entries: Vec<Value> = serde_json::from_str(
        r#"
        [
            {"first":"John","last":"Doe"}
        ]
    "#,
    )
    .unwrap();

    let result = add_full_name(entries);
    assert_eq!(result[0]["full_name"], json!("John Doe"));
}

// Exercise 7 — Deserialize into struct
// Goal: Move from Value to typed struct.
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct User {
    pub id: u64,
    pub active: bool,
}

pub fn active_users(json_input: &str) -> String {
    if let Ok(users) = serde_json::from_str::<Vec<User>>(json_input) {
        json!(users
            .iter()
            .filter(|user| user.active)
            .collect::<Vec<&User>>())
        .to_string()
    } else {
        String::new()
    }
}

#[test]
fn test_active_users() {
    let input = r#"
        [
            {"id":1,"active":true},
            {"id":2,"active":false}
        ]
    "#;

    let output = active_users(input);
    assert!(output.contains("\"id\":1"));
    assert!(!output.contains("\"id\":2"));
}

// Exercise 8 — Optional & default fields
// Goal: Use Option and #[serde(default)].
#[derive(Deserialize, Default)]
pub struct Config {
    pub name: String,
    #[serde(default)]
    pub enabled: bool,
    pub timeout: Option<u64>,
}

pub fn parse_config(input: &str) -> Config {
    let config: Config = from_str(input).unwrap_or_default();
    config
}

#[test]
fn test_parse_config() {
    let input = r#"{"name":"app"}"#;
    let cfg = parse_config(input);

    assert_eq!(cfg.enabled, false);
    assert_eq!(cfg.timeout, None);
}

// Exercise 9 — Dot-path access
// Goal: Navigate nested JSON.
pub fn get_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    path.split(".").fold(Some(value), |a, e| a?.get(e))
}

#[test]
fn test_get_path() {
    let v: Value = serde_json::from_str(
        r#"
        {"user":{"profile":{"email":"a@b.com"}}}
    "#,
    )
    .unwrap();

    assert_eq!(
        get_path(&v, "user.profile.email"),
        Some(&Value::String("a@b.com".to_string()))
    );
}

// Exercise 10 — Recursive merge
// Goal: Deep JSON manipulation.
pub fn merge(a: &mut Value, b: Value) {
    match (a, b) {
        (Value::Object(a), Value::Object(b)) => {
            for (k, v) in b {
                match a.get_mut(&k) {
                    None => {
                        a.insert(k.to_string(), v);
                    }
                    Some(value) => merge(value, v),
                }
            }
        }
        (a, b) => *a = b,
    }
}

#[test]
fn test_merge() {
    let mut a: Value = serde_json::from_str(
        r#"
        {"user":{"name":"Alice","age":30}}
    "#,
    )
    .unwrap();

    let b: Value = serde_json::from_str(
        r#"
        {"user":{"age":31,"city":"London"}}
    "#,
    )
    .unwrap();

    merge(&mut a, b);

    assert_eq!(a["user"]["age"], 31);
    assert_eq!(a["user"]["city"], "London");
}
