#![allow(unused)]

use std::{fs::File, io::Write};

use darling::FromField;
use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use quote::ToTokens;
use syn::Attribute;
use syn::{meta::ParseNestedMeta, parse_macro_input, DeriveInput, LitStr, Meta, MetaList, Path};

#[proc_macro_attribute]
pub fn task_param(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let schema_title = name.to_string();
    ast.attrs
        .push(syn::parse_quote!(#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, macros::XPriorityExtractor, macros::OrderedFields, macros::FromValue, validator::Validate, schemars::JsonSchema)]));
    ast.attrs
        .push(syn::parse_quote!(#[serde(rename_all = "camelCase")]));
    ast.attrs
        .push(syn::parse_quote!(#[schemars(title = #schema_title)]));

    // set default custom field property for struct fields
    if let syn::Data::Struct(ref mut data_struct) = ast.data {
        if let syn::Fields::Named(ref mut fields_name) = data_struct.fields {
            for field in fields_name.named.iter_mut() {
                field.attrs.push(syn::parse_quote!(
                    #[schemars(extend("x-decorator" = "FormItem", "x-decorator-props" = { "layout": "vertical" }, "x-component" = "Input"))]
                ));
            }
        }
    }

    let gen = quote! {
        impl #name {
            pub fn json_schema() -> schemars::Schema {
                schemars::schema_for!(#name)
            }
        }
    };

    let output = quote! {
        #ast
        #gen
    };
    output.into()
}

fn process_tokens(tokens: &proc_macro2::TokenStream) -> Vec<String> {
    let mut result = Vec::new();

    for token in tokens.clone() {
        match token {
            proc_macro2::TokenTree::Ident(ident) => {
                result.push(format!("Identifier: {}", ident));
            }
            proc_macro2::TokenTree::Group(group) => {
                result.push(format!("Group delimiter: {:?}", group.delimiter()));
                result.extend(process_tokens(&group.stream()));
            }
            proc_macro2::TokenTree::Punct(punct) => {
                result.push(format!("Punctuation: {}", punct.as_char()));
            }
            proc_macro2::TokenTree::Literal(lit) => {
                result.push(format!("Literal: {}", lit));
            }
        }
    }
    result
}

// fn extract_x_priority(attrs: &[Attribute]) -> Option<u64> {
//     for attr in attrs {
//         if let Ok(Meta::List(MetaList {
//             ref path,
//             ref nested,
//             ..
//         })) = attr.parse_meta()
//         {
//             if path.is_ident("schemars") {
//                 for meta in nested {
//                     if let ParseNestedMeta::Meta(Meta::List(MetaList { path, nested, .. })) = meta {
//                         if path.is_ident("extend") {
//                             for meta in nested {
//                                 if let ParseNestedMeta::Meta(Meta::NameValue(name_value)) = meta {
//                                     if name_value.path.is_ident("x-priority") {
//                                         if let syn::Lit::Int(lit_int) = &name_value.lit {
//                                             if let Ok(value) = lit_int.base10_parse::<u64>() {
//                                                 return Some(value);
//                                             }
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     None
// }

#[derive(Debug, FromMeta, Default)]
struct ExtendAttr {
    #[darling(rename = "x-priority", default)]
    x_priority: i32, // 默认会设置为0
}

#[derive(Debug, FromField)]
#[darling(attributes(schemars))]
struct FieldInfo {
    ident: Option<syn::Ident>,
    #[darling(default)]
    schemars: Option<SchemarsAttr>,
}

#[derive(Debug, FromMeta, Default)]
struct SchemarsAttr {
    #[darling(default)]
    extend: Option<ExtendAttr>, // 直接解析为ExtendAttr
}

#[proc_macro_derive(XPriorityExtractor)]
pub fn derive_x_priority(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = input.data
    {
        fields
    } else {
        panic!("Only works on structs with named fields");
    };

    let mut output = Vec::new();

    for field in fields.named.iter() {
        let field_info = match FieldInfo::from_field(field) {
            Ok(info) => info,
            Err(e) => panic!("Error parsing field: {}", e),
        };

        // let mut priority = 0;

        // if let Some(schemars_attr) = field_info.schemars {
        //     if let Some(extend_attrs) = schemars_attr.extend {
        //         priority = extend_attrs.x_priority;
        //     }
        // }

        let priority = field_info
            .schemars
            .and_then(|s| s.extend)
            .map(|e| e.x_priority)
            .unwrap_or(0); // 默认值为0

        if let Some(ident) = field_info.ident {
            output.push((ident.to_string(), priority));
        }
    }

    let struct_name = &input.ident;
    let output_json = serde_json::to_string(&output).unwrap();

    quote! {
        impl #struct_name {
            pub fn x_priorities() -> &'static str {
                #output_json
            }
        }
    }
    .into()
}

fn parse_extend_attr(s: &str) -> Option<ExtendAttr> {
    // 简单解析 `"x-priority" = 1` 格式
    let parts: Vec<&str> = s.split('=').map(|s| s.trim()).collect();
    if parts.len() == 2 && parts[0].contains("x-priority") {
        if let Ok(priority) = parts[1].trim().parse::<i32>() {
            return Some(ExtendAttr {
                x_priority: priority,
            });
        }
    }
    None
}

// Auto generate get ordered fields method
#[proc_macro_derive(OrderedFields)]
pub fn get_ordered_fields(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    // let schema_title = name.to_string();
    // ast.attrs
    //     .push(syn::parse_quote!(#[derive(schemars::JsonSchema)]));
    // ast.attrs
    //     .push(syn::parse_quote!(#[schemars(title = "")]));
    // fileds of CommonTask
    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => Some(named),
        _ => None,
    };

    let temp_path = "get_ordered_fields.txt";
    let mut file = File::create(temp_path).unwrap();
    file.write_fmt(format_args!("Fields: {:#?}", fields))
        .unwrap();
    file.sync_all().unwrap();

    let temp_path = "get_ordered_fields-info.txt";
    let mut file_field = File::create(temp_path).unwrap();

    let temp_path = "get_ordered_fields-attr.txt";
    let mut file = File::create(temp_path).unwrap();

    let temp_path = "get_ordered_fields-list.txt";
    let mut list_fields = File::create(temp_path).unwrap();

    let gen = if let Some(fields) = fields {
        let mut field_names = Vec::new();
        for field in fields {
            if let Some(ident) = &field.ident {
                file_field.write_fmt(format_args!("ident: {:#?}", ident));
                field_names.push(ident.to_string());
            }
            file_field.write_all("\n".as_bytes()).unwrap();

            for attr in &field.attrs {
                if let Meta::List(list) = &attr.meta {
                    // let tokens = syn::parse_quote!(list.tokens);
                    let tokens = process_tokens(&list.tokens);
                    file.write_fmt(format_args!("syn::Meta::List.tokens: {:#?}", tokens));
                    file.write_all("\n".as_bytes()).unwrap();
                }
            }

            // file_field
            //     .write_fmt(format_args!("attrs: {:#?}", field.attrs))
            //     .unwrap();
            // file_field.write_all("\n".as_bytes()).unwrap();
            // file_field
            //     .write_fmt(format_args!("mutability: {:#?}", field.mutability))
            //     .unwrap();
            // file_field.write_all("\n".as_bytes()).unwrap();
            // file_field
            //     .write_fmt(format_args!("colon_token: {:#?}", field.colon_token))
            //     .unwrap();
            // file_field.write_all("\n".as_bytes()).unwrap();
            // file_field
            //     .write_fmt(format_args!("colon_token: {:#?}", field.ty))
            //     .unwrap();
            // file_field.write_all("\n".as_bytes()).unwrap();
        }
        list_fields
            .write_fmt(format_args!("{:#?}", field_names))
            .unwrap();
        quote! {
            impl #name {
                pub fn get_ordered_fields() -> Vec<String> {
                    vec![#(#field_names.to_string()),*]
                }
            }
        }
    } else {
        quote! {
            impl #name {
                pub fn get_ordered_fields() -> Vec<String> {
                    vec![]
                }
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(FromValue)]
pub fn from_value(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let gen = quote! {
        impl #name {
            pub fn from_value(values: &[serde_json::Value]) -> Result<Vec<Self>> {
                use std::collections::HashSet;

                let mut params = Vec::new();
                for v in values {
                    params.push(serde_json::from_value(v.clone())?);
                }
                Ok(params)
            }
        }
    };
    gen.into()
}

#[derive(Default, Debug)]
struct RegisterTaskInput {
    name: Option<LitStr>,
    param: Option<Path>,
    // param_fixes: Option<Ident>,
    is_product: bool,
    task_type: Option<Path>,
    website: Option<LitStr>,
}

impl RegisterTaskInput {
    fn parse(&mut self, meta: ParseNestedMeta) -> syn::parse::Result<()> {
        if meta.path.is_ident("name") {
            self.name = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("product") {
            self.is_product = true;
        } else if meta.path.is_ident("task_type") {
            self.task_type = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("website") {
            self.website = Some(meta.value()?.parse()?);
        // } else if meta.path.is_ident("param_fixes") {
        //     self.param_fixes = Some(meta.value()?.parse()?);
        } else {
            self.param = Some(meta.path);
        }
        Ok(())
    }
}

// Register task to registry
#[proc_macro_attribute]
pub fn register_task(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut attr = RegisterTaskInput::default();
    let parser = syn::meta::parser(|meta| attr.parse(meta));

    parse_macro_input!(attrs with parser);

    // let ast: DeriveInput = syn::parse(input.clone()).unwrap();
    // let struct_name = &ast.ident;

    if let RegisterTaskInput {
        name: Some(name),
        param: Some(param),
        // param_fixes,
        is_product,
        task_type: Some(task_type),
        website,
    } = attr
    {
        // let level = if is_product {
        //     quote! { TaskLevel::Product }
        // } else {
        //     quote! { TaskLevel::Dev }
        // };
        // let param_update = if is_hot_update {
        //     quote! {
        //         impl crate::HotUpdate for #struct_name {
        //             fn get_channel(&self) -> Option<async_channel::Sender<serde_json::Value>> {
        //                 Some(self.inner.param_update_sx.clone())
        //             }
        //         }
        //     }
        // } else {
        //     quote! { impl crate::HotUpdate for #struct_name {} }
        // };
        // let website = website.unwrap_or_else(|| LitStr::new("", Span::call_site()));
        // let fixes = if let Some(param_fixes) = param_fixes {
        //     quote! {Some(&#param_fixes)}
        // } else {
        //     quote! {None}
        // };
        let mut gen: TokenStream = quote! {
            // inventory::submit! (TaskDescription {
            //         task_name: #name,
            //         fields: #param::get_ordered_fields,
            //         task_type: #task_type,
            //         level: #level,
            //         website: #website,
            //         json_schema: #param::json_schema,
            //         // new_task_fn: |p| {
            //         //     Box::pin(#struct_name::new_tasks_with_values(p))
            //         // },
            //         // resources: #struct_name::resources,
            //         // param_fixes: #fixes,
            //     });
        }
        .into();
        gen.extend(input);
        gen
    } else {
        panic!("invalid register task attribute")
    }
}
