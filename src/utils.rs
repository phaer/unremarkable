use uuid::{Uuid, uuid};
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

pub fn deserialize_parent<'de, D>(de: D) -> core::result::Result<Option<Uuid>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_deref();
    match opt {
        None | Some("") => Ok(None),
        Some("trash") => Ok(Some(uuid!("urn:uuid:00000000-0000-0000-0000-000000000000"))),
        Some(s) => Uuid::deserialize(s.into_deserializer()).map(Some),
    }
}
