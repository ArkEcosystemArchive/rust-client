use serde::Deserialize;
use serde::Deserializer;

pub fn deserialize_u64_as_number_or_string<'de, D>(de: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let deser_result: serde_json::Value = Deserialize::deserialize(de)?;

    match deser_result {
        serde_json::Value::Number(ref obj) if obj.is_u64() => Ok(obj.as_u64().unwrap()),
        serde_json::Value::String(ref obj) if !obj.is_empty() => {
            Ok(obj.as_str().parse::<u64>().unwrap())
        }
        _ => Ok(0),
    }
}
