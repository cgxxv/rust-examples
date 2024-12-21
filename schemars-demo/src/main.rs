use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, Expr};

// #[proc_macro]
// pub fn call_function(input: TokenStream) -> TokenStream {
//     let expr: Expr = parse_macro_input!(input);
//     if let syn::Expr::Call(syn::ExprCall { func, args: _, .. }) = expr {
//         if let syn::Expr::Path(syn::ExprPath { path, .. }) = &*func {
//             let last_segment = path.segments.last().unwrap();
//             let function_name = last_segment.ident.to_string();
//             // 这里假设要调用的函数在当前模块下，实际可能需要更完善的模块路径处理等
//             let expanded = quote! {
//                 ::your_module_name::#function_name()
//             };
//             return TokenStream::from(expanded);
//         }
//     }
//     TokenStream::new()
// }

// `int_as_string` and `bool_as_string` use the schema for `String`.
#[derive(Default, Deserialize, Serialize, JsonSchema)]
pub struct MyStruct {
    // #[serde(default = "eight", with = "as_string")]
    #[schemars(with = "String", required)]
    pub int_as_string: i32,

    // #[serde(default = "eight")]
    pub int_normal: i32,

    // #[serde(default, with = "as_string")]
    #[schemars(schema_with = "make_custom_schema2")]
    #[schemars(extend("simple" = "string"))]
    #[schemars(extend("x-simple" = "string", "complex" = {"array": [1, 2, 3]}))]
    pub bool_as_string: bool,

    #[serde(default)]
    pub bool_normal: bool,
}

// fn make_custom_schema() -> impl Fn(&mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema
// {
//     move |generator| {
//         let mut schema = String::json_schema(generator);
//         if let schemars::schema::Schema::Object(ref mut schema_obj) = schema {
//             schema_obj.extensions.insert("foo".into(), "string".into());
//         }
//         schema
//     }
// }

// fn make_custom_schema2(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
//     let mut schema = String::json_schema(generator);
//     if let schemars::schema::Schema::Object(ref mut schema_obj) = schema {
//         schema_obj.extensions.insert("foo".into(), "string".into());
//     }
//     schema
// }

fn make_custom_schema2(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    let mut schema = String::json_schema(generator);
    schema.ensure_object().insert("foo".into(), "string".into());
    schema
}

fn eight() -> i32 {
    8
}

// This module serializes values as strings
mod as_string {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: std::fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: std::str::FromStr,
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        string
            .parse()
            .map_err(|_| D::Error::custom("Input was not valid"))
    }
}

fn main() {
    let schema = schema_for!(MyStruct);
    println!("{schema:#?}");
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
