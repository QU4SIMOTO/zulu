use clap::Args;

#[derive(Debug, Args)]
pub struct SdgGet {
    /// The key of the SDG get variable.
    pub key: String,
}

impl SdgGet {
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
    }
}

impl Into<Vec<u8>> for &SdgGet {
    fn into(self) -> Vec<u8> {
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

impl SdgSet {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

impl Into<Vec<u8>> for &SdgSet {
    fn into(self) -> Vec<u8> {
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

impl SdgDo {
    pub fn new(key: impl Into<String>, value: Option<String>) -> Self {
        Self {
            key: key.into(),
            value,
        }
    }
}

impl Into<Vec<u8>> for &SdgDo {
    fn into(self) -> Vec<u8> {
        format!(
            "! U1 do \"{}\" \"{}\"\r\n",
            self.key,
            self.value.as_deref().unwrap_or("")
        )
        .into_bytes()
    }
}
