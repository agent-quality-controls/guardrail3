use garde::Validate;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RawStructInput {
    pub name: String,
}

#[derive(Deserialize)]
pub enum RawEnumInput {
    Payload(String),
}

pub struct ManualInput {
    pub name: String,
}

impl<'de> Deserialize<'de> for ManualInput {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self)
    }
}

#[derive(Deserialize, Validate)]
pub struct MissingFieldConstraint {
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct Payload {
    #[garde(length(min = 1))]
    pub value: String,
}

#[derive(Deserialize, Validate)]
pub struct MissingDive {
    pub payload: Payload,
}

fn validate_title(_value: &str, _ctx: &()) -> garde::Result {
    Ok(())
}

#[derive(Deserialize, Validate)]
pub struct MissingContext {
    #[garde(custom(validate_title(ctx)))]
    pub title: String,
}

pub fn query_as_probe() {
    sqlx::query_as!(ManualInput, "select 1");
}
