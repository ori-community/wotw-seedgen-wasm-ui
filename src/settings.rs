
use wasm_bindgen::prelude::*;

use wotw_seedgen::settings::GameSettings as SeedgenGameSettings;
use wotw_seedgen::preset::GamePreset as SeedgenGamePreset;
use wotw_seedgen::settings::WorldSettings as SeedgenWorldSettings;
use wotw_seedgen::preset::WorldPreset as SeedgenWorldPreset;

use crate::files::JsFileAccess;

/// A representation of all the relevant settings when generating a seed
/// 
/// Using the same settings will result in generating the same seed (unless the used header files change)
#[wasm_bindgen]
pub struct GameSettings(SeedgenGameSettings);
#[wasm_bindgen]
impl GameSettings {
    /// Returns the default `GameSettings`
    /// 
    /// When using this function, the string used to seed the rng will be randomly generated
    pub fn default() -> Self { Self(SeedgenGameSettings::default()) }

    /// Parse the `GameSettings` from json
    /// 
    /// @throws {string} if the input fails to deserialize
    #[wasm_bindgen(js_name = "fromJson")]
    pub fn from_json(json: &str) -> Result<GameSettings, String> {
        SeedgenGameSettings::parse(json)
            .map(GameSettings)
            .map_err(|err| err.to_string())
    }
    /// Serialize the `Settings` into json
    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> String {
        self.0.to_json()
    }

    /// Apply a `GamePreset`
    /// 
    /// This follows various rules to retain all unrelated parts of the existing Settings:
    /// - Any `undefined` values of the preset will be ignored
    /// - `Array`s will be appended to the current contents
    /// - Other values will be overwritten
    /// - If the number of worlds matches, the preset will be applied to each world per index
    /// - If only one world is in the preset, but multiple in the existing settings, the preset is applied to all worlds
    /// - If multiple worlds are in the preset, but only one in the existing settings, the existing settings will be copied for all worlds, then the preset will be applied per index
    /// - If multiple worlds are in both and their number does not match, throws
    /// - Nested presets will be applied before the parent preset
    /// 
    /// @throws {string} if included presets cannot be found using the provided `fileaccess` or the world counts are incompatible
    #[wasm_bindgen(js_name = "applyPreset")]
    pub fn apply_preset(&mut self, preset: GamePreset, file_access: &JsFileAccess) -> Result<(), String> {
        self.0.apply_preset(preset.0, file_access).map_err(|err| err.to_string())
    }
}

/// Seed settings bound to a specific world of a seed
/// 
/// See the [Multiplayer wiki page](https://wiki.orirando.com/features/multiplayer) for an explanation of worlds
#[wasm_bindgen]
pub struct WorldSettings(SeedgenWorldSettings);
#[wasm_bindgen]
impl WorldSettings {
    /// Returns the default Settings
    pub fn default() -> Self { Self(SeedgenWorldSettings::default()) }

    /// Parse the `WorldSettings` from json
    /// 
    /// @throws {string} if the input fails to deserialize
    #[wasm_bindgen(js_name = "fromJson")]
    pub fn from_json(json: &str) -> Result<WorldSettings, String> {
        SeedgenWorldSettings::parse(json)
            .map(WorldSettings)
            .map_err(|err| err.to_string())
    }
    /// Serialize the `WorldSettings` into json
    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> String {
        self.0.to_json()
    }

    /// Apply the settings from a WorldPreset
    /// 
    /// This follows various rules to retain all unrelated parts of the existing Settings:
    /// - Any undefined values of the preset will be ignored
    /// - Arrays will be appended to the current contents
    /// - Other values will be overwritten
    /// - Nested presets will be applied before the parent preset
    /// 
    /// @throws {string} if included presets cannot be found using the provided `fileaccess`
    #[wasm_bindgen(js_name = "applyWorldPreset")]
    pub fn apply_world_preset(&mut self, preset: WorldPreset, file_access: &JsFileAccess) -> Result<(), String> {
        self.0.apply_world_preset(preset.0, file_access).map_err(|err| err.to_string())
    }
}

/// A collection of settings that can be applied to existing settings
/// 
/// Use `GameSettings.apply_preset` to apply a `GamePreset` to existing `GameSettings`
#[wasm_bindgen]
pub struct GamePreset(SeedgenGamePreset);
#[wasm_bindgen]
impl GamePreset {
    /// Parse a `GamePreset` from json
    /// 
    /// @throws {string} if the input fails to deserialize
    #[wasm_bindgen(js_name = "fromJson")]
    pub fn from_json(json: &str) -> Result<GamePreset, String> {
        SeedgenGamePreset::parse(json)
            .map(GamePreset)
            .map_err(|err| err.to_string())
    }
    /// Serialize the `GamePreset` into json
    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> String {
        self.0.to_json()
    }
}

/// A collection of settings that can be applied to one world of the existing settings
/// 
/// Use `WorldSettings.apply_world_preset` to apply a `WorldPreset` to existing `WorldSettings`
#[wasm_bindgen]
pub struct WorldPreset(SeedgenWorldPreset);
#[wasm_bindgen]
impl WorldPreset {
    /// Parse a `WorldPreset` from json
    /// 
    /// @throws {string} if the input fails to deserialize
    #[wasm_bindgen(js_name = "fromJson")]
    pub fn from_json(json: &str) -> Result<WorldPreset, String> {
        SeedgenWorldPreset::parse(json)
            .map(WorldPreset)
            .map_err(|err| err.to_string())
    }
    /// Serialize the `WorldPreset` into json
    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> String {
        self.0.to_json()
    }
}
