use clap::Args;

use crate::IntoSgd;

#[derive(Debug, Args)]
pub struct SdgGet {
    /// The key of the SDG get variable.
    pub key: String,
}

impl IntoSgd for SdgGet {
    fn into_sgd(self: Self) -> Vec<u8> {
        format!("! U1 getvar \"{}\"\r\n", self.key).into_bytes()
    }
}

#[derive(Debug, Args)]
pub struct SdgSet {
    /// The key of the SDG set variable.
    pub key: String,
    /// The value to set the key at.
    pub value: String,
}

impl IntoSgd for SdgSet {
    fn into_sgd(self: Self) -> Vec<u8> {
        format!("! U1 setvar \"{}\" \"{}\"\r\n", self.key, self.value).into_bytes()
    }
}

#[derive(Debug, Args)]
pub struct SdgDo {
    /// The name of the operation to perform
    pub key: String,
    #[clap(default_value = "")]
    pub value: Option<String>,
}

impl IntoSgd for SdgDo {
    fn into_sgd(self: Self) -> Vec<u8> {
        format!(
            "! U1 do \"{}\" \"{}\"\r\n",
            self.key,
            self.value.as_deref().unwrap_or("")
        )
        .into_bytes()
    }
}
