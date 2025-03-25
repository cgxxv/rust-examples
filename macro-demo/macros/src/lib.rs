use proc_macro::TokenStream;
use quote::quote;
use syn::{meta::ParseNestedMeta, parse_macro_input, DeriveInput, LitStr, Path};

#[proc_macro_attribute]
pub fn task_param(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let schema_title = name.to_string();
    ast.attrs
        .push(syn::parse_quote!(#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, macros::OrderedFields, macros::FromValue, validator::Validate, schemars::JsonSchema)]));
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

    let gen = if let Some(fields) = fields {
        let mut fields_with_priority = Vec::new();
        for field in fields {
            let priority = field
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident("schemars"))
                .filter_map(|attr| {
                    attr.parse_args_with(|input: syn::parse::ParseStream| -> syn::Result<i64> {
                        let mut priority = 0;

                        while !input.is_empty() {
                            let path = input.call(syn::Path::parse_mod_style)?;

                            if path.is_ident("extend") && input.peek(syn::token::Paren) {
                                let content;
                                syn::parenthesized!(content in input);

                                while !content.is_empty()
                                    && (content.peek(syn::LitStr) || content.peek(syn::Ident))
                                {
                                    let key = if content.peek(syn::LitStr) {
                                        let lit: syn::LitStr = content.parse()?;
                                        lit.value()
                                    } else {
                                        let ident: syn::Ident = content.parse()?;
                                        ident.to_string()
                                    };

                                    if content.peek(syn::Token![=]) {
                                        content.parse::<syn::Token![=]>()?;

                                        if key == "x-priority" {
                                            let lit: syn::LitInt = content.parse()?;
                                            priority = lit.base10_parse::<i64>()?;
                                        }
                                    }
                                }
                            }
                        }

                        Ok(priority)
                    })
                    .ok()
                })
                .next()
                .unwrap_or(0);

            if let Some(ident) = &field.ident {
                fields_with_priority.push((ident.to_string(), priority));
            }
        }

        fields_with_priority.sort_by(|a, b| b.1.cmp(&a.1));

        let prioritized_fields = fields_with_priority.iter().map(|(name, pri)| {
            quote! { (#name.to_string(), #pri) }
        });

        let prioritized_field_names = fields_with_priority.iter().map(|(name, _)| {
            quote! { #name.to_string() }
        });

        quote! {
            impl #name {
                pub fn get_ordered_fields() -> Vec<String> {
                    vec![#(#prioritized_field_names),*]
                }

                pub fn get_fields_with_priority() -> Vec<(String, i64)> {
                    vec![#(#prioritized_fields),*]
                }
            }
        }
    } else {
        quote! {
            impl #name {
                pub fn get_ordered_fields() -> Vec<String> {
                    vec![]
                }

                pub fn get_fields_with_priority() -> Vec<(String, i64)> {
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
