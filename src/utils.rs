use serde::{de::IntoDeserializer, Deserialize, Serialize};

// https://github.com/serde-rs/serde/issues/1425#issuecomment-462282398
pub fn deserialize_empty_string_as_none<'de, D, T>(de: D) -> core::result::Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_deref();
    match opt {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}
