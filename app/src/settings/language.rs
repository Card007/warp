use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::macros::define_settings_group;
use settings::{RespectUserSyncSetting, SupportedPlatforms, SyncToCloud};

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    settings_value::SettingsValue,
)]
#[serde(rename_all = "snake_case")]
#[schemars(rename_all = "snake_case")]
pub enum UserInterfaceLanguage {
    #[default]
    English,
    Chinese,
}

impl UserInterfaceLanguage {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Chinese => "中文",
        }
    }
}

define_settings_group!(LanguageSettings, settings: [
   user_interface_language: UserInterfaceLanguageSetting {
       type: UserInterfaceLanguage,
       default: UserInterfaceLanguage::English,
       supported_platforms: SupportedPlatforms::ALL,
       sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
       private: false,
       toml_path: "appearance.language",
       description: "The language used by the settings UI.",
   },
]);
