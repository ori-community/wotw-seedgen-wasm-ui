use wasm_bindgen::prelude::*;

use wotw_seedgen::header::Annotation as SeedgenAnnotation;
use wotw_seedgen::header::Header;
use wotw_seedgen::header::HeaderDocumentation as SeedgenHeaderDocumentation;
use wotw_seedgen::header::ParameterDefault as SeedgenParameterDefault;
use wotw_seedgen::header::ParameterInfo as SeedgenParameterInfo;

use wasm_bindgen_helper_macros::*;

/// Meta information contained in a header's annotations
#[wasm_bindgen]
#[derive(Default)]
pub struct Annotations {
    /// Hide this header from the user, it is only to be used internally through includes
    pub hide: bool,
    /// Put this header into a category with other, similar headers
    #[wasm_bindgen(getter_with_clone)]
    pub category: Option<String>,
}
impl From<Vec<SeedgenAnnotation>> for Annotations {
    fn from(annotation_vec: Vec<SeedgenAnnotation>) -> Annotations {
        let mut annotations = Annotations::default();
        for annotation in annotation_vec {
            match annotation {
                SeedgenAnnotation::Hide => annotations.hide = true,
                SeedgenAnnotation::Category(category) => annotations.category = Some(category),
            }
        }
        annotations
    }
}

/// Returns the annotations of a given header syntax
///
/// This will only parse the minimum amount required to know the annotations
#[wasm_bindgen]
pub fn parse_annotations(header: &str) -> Result<Annotations, String> {
    Header::parse_annotations(header).map(Annotations::from)
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
    pub parameter_type: ParameterType,
    #[wasm_bindgen(getter_with_clone)]
    pub default_value: String,
    #[wasm_bindgen(getter_with_clone)]
    pub documentation: Option<String>,
}

impl From<SeedgenParameterInfo> for Parameter {
    fn from(parameter_info: SeedgenParameterInfo) -> Self {
        let SeedgenParameterInfo {
            identifier,
            default,
            documentation,
        } = parameter_info;
        let default_value = default.to_string();
        let parameter_type = default.into();
        Self {
            identifier,
            parameter_type,
            default_value,
            documentation,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum ParameterType {
    Bool,
    Int,
    Float,
    String,
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
