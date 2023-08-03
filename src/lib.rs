pub mod mess {
    use std::{
        sync::OnceLock,
        collections::HashMap,
    };
    use thiserror::Error;
    use include_dir::{include_dir, Dir};


    static mut HASHES: OnceLock<HashMap<&str, String>> = OnceLock::new();
    static MESS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/message");

    #[derive(Debug, Error)]
    pub enum MessageError {
        #[error("the localization system is not initialized")]
        LocalizeNotInitialized,
        #[error("the localization system is already initialized")]
        LocalizationInitialized,
        #[error("the localized messages contain a UTF8 error")]
        LocalizationEncoding,
        #[error("the file(s) could not be found for region `{0}`")]
        MissingLocalization(String),
        #[error("the label `{0}` could not be found")]
        MissingLabel(String),
        #[error("a JSON deserialization error happened: {0}")]
        DeserializeError(#[from] std::io::Error),
    }

    fn map_languages<'a>(region: impl AsRef<str> + 'a) -> Option<&'a str> {
        match region.as_ref() {
            "eu/eues" => Some("us/uses"),
            "us/usfr" => Some("eu/eufr"),
            _ => None,
        }
    }

    pub fn initialize(region: impl AsRef<str>) -> Result<(), MessageError> {
        let region = region.as_ref().to_lowercase();

        println!("Region: {}", &region);

        // Default to reading the US localization, as it is the officially endorsed one.
        let us_loc = MESS_DIR.get_file("us/usen/table.json")
            .ok_or(MessageError::MissingLocalization(region.to_string()))?
            .contents_utf8()
            .ok_or(MessageError::LocalizationEncoding)?;

        // Populate the hashmap with the labels
        let mut hashes: HashMap<&str, String> = serde_json::from_str(us_loc)
            .map_err(|err| MessageError::DeserializeError(err.into()))?;

        // Process the labels for the user's region. Make sure to not read the US localization again, since we already did so.
        if region != "us/usen" || region != "eu/euen" {
            // It's not that big of a deal if the user's region is missing, in that case the US localization is already loaded.
            if let Some(loc) = MESS_DIR.get_file(format!("{}/table.json", map_languages(&region).unwrap_or(&region))) {
                let regional_loc = loc.contents_utf8()
                    .ok_or(MessageError::LocalizationEncoding)?;

                let iter: HashMap<&str, String> = serde_json::from_str(regional_loc)
                .map_err(|err| MessageError::DeserializeError(err.into()))?;

                // Extend the hashmap with the new localization. Existing labels will be overwritten by the ones from the localization.
                hashes.extend(iter);
            }
        }

        unsafe {
            let _ = HASHES.take();
            HASHES.set(hashes).map_err(|_| MessageError::LocalizationInitialized)
        }
    }

    /// Helper method to acquire the hashmap so it automatically checks if it is initialized.
    fn instance<'a>() -> Result<&'a HashMap<&'a str, String>, MessageError> {
        unsafe { HASHES.get().ok_or(MessageError::LocalizeNotInitialized) }
    }

    /// Get a localized message by label.
    /// 
    /// If the label is not found, it will be printed instead.
    pub fn get<'a>(label: impl AsRef<str> + 'a) -> String {
        instance().unwrap()
            .get(label.as_ref())
            .unwrap_or(&label.as_ref().to_string()) // Replace the missing text by the label we tried to get
            .to_owned() // Turn the &String into a String
    }

    /// Get a localized message by label.
    /// 
    /// If the label is not found, an error will be returned.
    pub fn try_get<'a>(label: impl AsRef<str> + 'a) -> Result<String, MessageError> {
        instance().unwrap().get(label.as_ref())
            .ok_or(MessageError::MissingLabel(label.as_ref().to_string())) // Report that the label couldn't be found, and which one
            .map(|text| text.to_owned()) // Turn the &String into a String
    }
}