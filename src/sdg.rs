use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum SdgCommand {
    Get {
        /// The key of the SDG get variable.
        key: String,
    },
    Set {
        /// The key of the SDG get variable.
        key: String,
        /// The value to set the key at.
        value: String,
    },
    Do {
        /// The name of the operation to perform
        key: String,
        #[clap(default_value = "")]
        value: Option<String>,
    },
}

impl std::fmt::Display for SdgCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get { key } => {
                write!(f, "! U1 getvar \"{key}\"\r\n")
            }
            Self::Set { key, value } => {
                write!(f, "! U1 setvar \"{key}\" \"{value}\"\r\n")
            }
            Self::Do { key, value } => {
                write!(
                    f,
                    "! U1 do \"{key}\" \"{}\"\r\n",
                    value.as_deref().unwrap_or("")
                )
            }
        }
    }
}
