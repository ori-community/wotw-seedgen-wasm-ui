use wasm_bindgen::prelude::*;

use num_enum::TryFromPrimitive;

use wotw_seedgen::header::Header;
use wotw_seedgen::header::HeaderDocumentation as SeedgenHeaderDocumentation;
use wotw_seedgen::header::ParameterInfo as SeedgenParameterInfo;
use wotw_seedgen::header::ParameterDefault as SeedgenParameterDefault;

use wasm_bindgen_helper_macros::*;

ts_enum! {
    #[wasm_bindgen]
    #[derive(TryFromPrimitive)]
    #[repr(u8)]
    /// Annotations providing meta information about how to treat the header
    pub enum Annotation {
        /// Hide this header from the user, it is only to be used internally through includes
        Hide,
    }
}

wrapper_list! {
    #[wasm_bindgen]
    pub struct __AnnotationList {
        inner: IntoIter<Annotation>,
    }
}

/// Returns the annotations of a given header syntax
/// 
/// This will only parse the minimum amount required to know the annotations
#[wasm_bindgen]
pub fn parse_annotations(header: &str) -> Result<AnnotationArray, String> {
    let annotations = Header::parse_annotations(header)?
        .into_iter()
        .map(|annotation| (annotation as u8).try_into().unwrap())
        .collect::<Vec<_>>();

    Ok(__AnnotationList::from(annotations).into_js_array())
}

#[wasm_bindgen]
pub struct HeaderDocumentation {
    /// Brief name, this may never exceed one line
    /// 
    /// `undefined` if not provided by the header
    #[wasm_bindgen(getter_with_clone)]
    pub name: Option<String>,
    /// Extended description
    /// 
    /// `undefined` if not provided by the header
    #[wasm_bindgen(getter_with_clone)]
    pub description: Option<String>,
}

impl From<SeedgenHeaderDocumentation> for HeaderDocumentation {
    fn from(header_documentation: SeedgenHeaderDocumentation) -> Self {
        let SeedgenHeaderDocumentation { name, description } = header_documentation;
        Self { name, description }
    }
}

/// Returns the name and description of a given header syntax
/// 
/// This will only parse the minimum amount required to know the documentation
#[wasm_bindgen]
pub fn parse_documentation(header: &str) -> HeaderDocumentation {
    Header::parse_documentation(header).into()
}

wrapper_list! {
    pub struct __ParameterList {
        inner: IntoIter<Parameter>,
    }
}

#[wasm_bindgen]
pub struct Parameter {
    #[wasm_bindgen(getter_with_clone)]
    pub identifier: String,
    parameter_type: ParameterType,
    #[wasm_bindgen(getter_with_clone)]
    pub default_value: String,
    #[wasm_bindgen(getter_with_clone)]
    pub documentation: Option<String>,
}
#[wasm_bindgen]
impl Parameter {
    #[wasm_bindgen(getter, js_name = "type")]
    pub fn parameter_type(&self) -> ParameterTypeEnum {
        self.parameter_type.into_js_enum()
    }
}

impl From<SeedgenParameterInfo> for Parameter {
    fn from(parameter_info: SeedgenParameterInfo) -> Self {
        let SeedgenParameterInfo { identifier, default, documentation } = parameter_info;
        let default_value = default.to_string();
        let parameter_type = default.into();
        Self { identifier, parameter_type, default_value, documentation }
    }
}

ts_enum! {
    #[wasm_bindgen]
    #[derive(Clone, Copy)]
    pub enum ParameterType {
        Bool,
        Int,
        Float,
        String,
    }
}

impl From<SeedgenParameterDefault> for ParameterType {
    fn from(default: SeedgenParameterDefault) -> ParameterType {
        match default {
            SeedgenParameterDefault::Bool(_) => ParameterType::Bool,
            SeedgenParameterDefault::Int(_) => ParameterType::Int,
            SeedgenParameterDefault::Float(_) => ParameterType::Float,
            SeedgenParameterDefault::String(_) => ParameterType::String,
        }
    }
}

/// Returns the parameters present in the header, including their names and default values
/// 
/// This will parse any parameter lines to read their relevant values, but skip parsing anything else
#[wasm_bindgen]
pub fn parse_parameters(header: &str) -> ParameterArray {
    let parameters = Header::parse_parameters(header)
        .into_iter()
        .map(Parameter::from)
        .collect::<Vec<_>>();

    __ParameterList::from(parameters).into_js_array()
}
