use wasm_bindgen::prelude::*;

use wotw_seedgen::preset::UniversePreset as SeedgenUniversePreset;
use wotw_seedgen::preset::WorldPreset as SeedgenWorldPreset;
use wotw_seedgen::settings::UniverseSettings as SeedgenUniverseSettings;
use wotw_seedgen::settings::WorldSettings as SeedgenWorldSettings;

use crate::files::JsFileAccess;

/// A representation of all the relevant settings when generating a seed
///
/// Using the same settings will result in generating the same seed (unless the used header files change)
#[wasm_bindgen]
pub struct UniverseSettings(SeedgenUniverseSettings);
#[wasm_bindgen]
impl UniverseSettings {
    /// Returns the default `UniverseSettings`
    ///
    /// When using this function, the string used to seed the rng will be randomly generated
    pub fn default() -> Self {
        Self(SeedgenUniverseSettings::default())
    }

    /// Parse the `UniverseSettings` from json
    ///
    /// @throws {string} if the input fails to deserialize
    #[wasm_bindgen(js_name = "fromJson")]
    pub fn from_json(json: &str) -> Result<UniverseSettings, String> {
        SeedgenUniverseSettings::parse(json)
            .map(UniverseSettings)
            .map_err(|err| err.to_string())
    }
    /// Serialize the `Settings` into json
    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> String {
        self.0.to_json()
    }

    /// Apply a `UniversePreset`
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
    pub fn apply_preset(
        &mut self,
        preset: UniversePreset,
        file_access: &JsFileAccess,
    ) -> Result<(), String> {
        self.0
            .apply_preset(preset.0, file_access)
            .map_err(|err| err.to_string())
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
    pub fn default() -> Self {
        Self(SeedgenWorldSettings::default())
    }

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
    pub fn apply_world_preset(
        &mut self,
        preset: WorldPreset,
        file_access: &JsFileAccess,
    ) -> Result<(), String> {
        self.0
            .apply_world_preset(preset.0, file_access)
            .map_err(|err| err.to_string())
    }
}

/// A collection of settings that can be applied to existing settings
///
/// Use `UniverseSettings.apply_preset` to apply a `UniversePreset` to existing `UniverseSettings`
#[wasm_bindgen]
pub struct UniversePreset(SeedgenUniversePreset);
#[wasm_bindgen]
impl UniversePreset {
    /// Parse a `UniversePreset` from json
    ///
    /// @throws {string} if the input fails to deserialize
    #[wasm_bindgen(js_name = "fromJson")]
    pub fn from_json(json: &str) -> Result<UniversePreset, String> {
        SeedgenUniversePreset::parse(json)
            .map(UniversePreset)
            .map_err(|err| err.to_string())
    }
    /// Serialize the `UniversePreset` into json
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
