use crate::gen::context::GenContext;
use crate::gen::simple_type::{get_type_name, simple_type_mapping};
use crate::models::{OpenXmlNamespace, OpenXmlSchema, OpenXmlSchemaType, OpenXmlSchemaTypeAttribute, OpenXmlSchemaTypeChild};
use crate::utils::{escape_snake_case, escape_upper_camel_case};
use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::HashMap;
use syn::{parse2, parse_str, ItemEnum, Type, Variant};

pub fn gen_open_xml_schema(schema: &OpenXmlSchema, context: &GenContext) -> TokenStream {
    let mut token_stream_list: Vec<TokenStream> = vec![];
    let schema_namespace_name = schema.target_namespace.as_str();
    let schema_namespace = context
        .uri_namespace_map
        .get(schema_namespace_name)
        .ok_or(format!("{:?}", schema.target_namespace))
        .unwrap();

    for e in &schema.enums {
        let e_enum_name_ident: Ident = parse_str(&e.name.to_upper_camel_case()).unwrap();

        let mut variants: Vec<Variant> = vec![];
        let mut string_variants: Vec<Variant> = vec![];

        for facet in &e.facets {
            let (variant_ident, lit): (Ident, String) = if facet.name.is_empty() {
                let v = facet.value.to_upper_camel_case();
                let n = facet.value.to_lower_camel_case();
                (parse_str(&escape_upper_camel_case(v.clone())).unwrap(), n)
            } else {
                let v = facet.name.to_upper_camel_case();
                let n = facet.value.to_lower_camel_case();
                (parse_str(&escape_upper_camel_case(v.clone())).unwrap(), n)
            };
            variants.push(
                parse2(quote! {
                    #variant_ident
                }).unwrap(),
            );
            string_variants.push(
                parse2(quote! {
                    #variant_ident = #lit
                }).unwrap(),
            );
        }

        token_stream_list.push(quote! {
            #[derive(Debug, Clone, PartialEq, Eq, Default)]
            pub enum #e_enum_name_ident {
                #[default]
                #( #variants , )*
            }

            crate::__string_enum! {
                #e_enum_name_ident {
                    #( #string_variants , )*
                }
            }
        })
    }


    for t in &schema.types {
        let mut fields: Vec<TokenStream> = vec![];

        let mut child_choice_enum_option: Option<ItemEnum> = None;

        let mut xmlns_derive: Option<TokenStream> = None;

        if t.base_class == "OpenXmlLeafTextElement" {
            for attr in &t.attributes {
                fields.push(gen_attr(attr, schema_namespace, context));
            }

            let (simple_type_name, tag_type, tag_name) = gen_child_type(t, schema_namespace, context);
            if let Some(tag_name) = tag_name {
                fields.push(quote! {
                    #[xml(#tag_type = #tag_name)]
                    pub child: Option<#simple_type_name>,
                });
            } else {
                fields.push(quote! {
                    #[xml(#tag_type)]
                    pub child: #simple_type_name,
                });
            }
        } else if t.base_class == "OpenXmlLeafElement" {
            for attr in &t.attributes {
                fields.push(gen_attr(attr, schema_namespace, context));
            }
        } else if t.base_class == "OpenXmlCompositeElement"
            || t.base_class == "CustomXmlElement"
            || t.base_class == "OpenXmlPartRootElement"
            || t.base_class == "SdtElement"
        {
            if !t.part.is_empty()
                || t.base_class == "OpenXmlPartRootElement"
                || schema.target_namespace == "http://schemas.openxmlformats.org/drawingml/2006/main"
                || schema.target_namespace == "http://schemas.openxmlformats.org/drawingml/2006/picture"
            {
                let xmlns_derive_name_str = format!("{}_xmlns_derive",&t.class_name.to_snake_case());
                let xmlns_derive_name_ident: Ident = parse_str(&xmlns_derive_name_str).unwrap();
                fields.push(quote! {
                    #[xml(attr = "xmlns", with = #xmlns_derive_name_str)]
                    pub xmlns: Option<String>,
                });

                xmlns_derive = Some(quote! {
                    mod #xmlns_derive_name_ident {
                        pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
                            Ok(mode.to_string())
                        }
                        pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
                            Ok(#schema_namespace_name)
                        }
                    }
                });

                fields.push(quote! {
                    #[xml(prefix = "xmlns")]
                    pub xmlns_map: std::collections::HashMap<String, String>,
                });

                fields.push(quote! {
                    #[xml(attr = "mc:Ignorable")]
                    pub mc_ignorable: Option<String>,
                });
            }

            for attr in &t.attributes {
                fields.push(gen_attr(attr, schema_namespace, context));
            }

            if t.is_one_sequence_flatten() {
                let one_sequence_fields = gen_one_sequence_fields(t, schema_namespace, context);

                fields.extend(one_sequence_fields);
            } else {
                let (field_option, enum_option) =
                    gen_children(&t.class_name, &t.children, schema_namespace, context);

                if let Some(field) = field_option {
                    fields.push(field);
                }

                child_choice_enum_option = enum_option;
            }
        } else if t.is_derived {
            let base_class_type = context
                .type_name_type_map
                .get(&t.name[0..t.name.find('/').unwrap() + 1])
                .ok_or(format!("{:?}", t.base_class))
                .unwrap();

            for attr in &t.attributes {
                fields.push(gen_attr(attr, schema_namespace, context));
            }

            for attr in &base_class_type.attributes {
                fields.push(gen_attr(attr, schema_namespace, context));
            }

            if t.is_one_sequence_flatten() && base_class_type.composite_type == "OneSequence" {
                let one_sequence_fields = gen_one_sequence_fields(t, schema_namespace, context);

                fields.extend(one_sequence_fields);
            } else {
                let (field_option, enum_option) =
                    gen_children(&t.class_name, &t.children, schema_namespace, context);

                if let Some(field) = field_option {
                    fields.push(field);
                }

                child_choice_enum_option = enum_option;
            }

            if t.children.is_empty() && base_class_type.base_class == "OpenXmlLeafTextElement" {
                let (simple_type_name, tag_type, tag_name) = gen_child_type(base_class_type, schema_namespace, context);
                if let Some(tag_name) = tag_name {
                    fields.push(quote! {
                        #[xml(#tag_type = #tag_name)]
                        pub child: Option<#simple_type_name>,
                    });
                } else {
                    fields.push(quote! {
                        #[xml(#tag_type)]
                        pub child: #simple_type_name,
                    });
                }
            }
        } else {
            panic!("{:?}", t);
        }

        let struct_name_ident: Ident = parse_str(&t.class_name.to_upper_camel_case()).unwrap();

        let summary_doc = format!(" {}", t.summary);

        let (qualified_doc, qualified_name) = if t.name.ends_with('/') {
            (" When the object is serialized out as xml, it's qualified name is .".to_string(), ".")
        } else {
            let qualified_str = &t.name[t.name.find('/').unwrap() + 1..t.name.len()];

            (format!(
                " When the object is serialized out as xml, it's qualified name is {}.",
                qualified_str
            ), qualified_str)
        };

        if let Some(child_choice_enum) = child_choice_enum_option {
            token_stream_list.push(quote! {
                #[doc = #summary_doc]
                #[doc = #qualified_doc]
                #[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
                #[xml(tag = #qualified_name)]
                pub struct #struct_name_ident {
                  #( #fields )*
                }

                #child_choice_enum
            });
        } else {
            token_stream_list.push(quote! {
                #[doc = #summary_doc]
                #[doc = #qualified_doc]
                #[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
                #[xml(tag = #qualified_name)]
                pub struct #struct_name_ident {
                  #( #fields )*
                }
            });
        }

        if let Some(xmlns_derive) = xmlns_derive {
            token_stream_list.push(quote! {#xmlns_derive});
        }
    }

    quote! {
        #( #token_stream_list )*
    }
}


fn gen_attr(
    attr: &OpenXmlSchemaTypeAttribute,
    schema_namespace: &OpenXmlNamespace,
    context: &GenContext,
) -> TokenStream {
    let attr_rename_ser_str = if attr.q_name.starts_with(':') {
        &attr.q_name[1..attr.q_name.len()]
    } else {
        &attr.q_name
    }.to_string();

    let (attr_name_ident, attr_name): (Ident, String) = if attr.property_name.is_empty() {
        (parse_str(&escape_snake_case(attr.q_name.to_snake_case())).unwrap(), attr_rename_ser_str)
    } else {
        (parse_str(&escape_snake_case(attr.property_name.to_snake_case())).unwrap(), attr_rename_ser_str)
    };

    let (type_ident, tag_type, tag_name): (Type, Type, Option<String>) = if attr.r#type.starts_with("ListValue<") {
        (parse_str("String").unwrap(), parse_str("attr").unwrap(), Some(attr_name))
    } else if attr.r#type.starts_with("EnumValue<") {
        let e_typed_namespace_str =
            &attr.r#type[attr.r#type.find("<").unwrap() + 1..attr.r#type.rfind(".").unwrap()];

        let enum_name = &attr.r#type[attr.r#type.rfind(".").unwrap() + 1..attr.r#type.len() - 1];

        let mut e_prefix = "";

        for typed_namespace in &context.typed_namespaces {
            if e_typed_namespace_str == typed_namespace.namespace {
                let e_schema = context
                    .prefix_schema_map
                    .get(typed_namespace.prefix.as_str())
                    .ok_or(format!("{:?}", typed_namespace))
                    .unwrap();

                for e in &e_schema.enums {
                    if e.name == enum_name {
                        e_prefix = &typed_namespace.prefix;
                    }
                }
            }
        }

        let e_namespace = context
            .prefix_namespace_map
            .get(e_prefix)
            .ok_or(format!("{:?}", e_prefix))
            .unwrap();

        if e_namespace.prefix != schema_namespace.prefix {
            let scheme_mod = context
                .prefix_schema_mod_map
                .get(e_namespace.prefix.as_str())
                .ok_or(format!("{:?}", e_namespace.prefix))
                .unwrap();

            (parse_str(&format!(
                "crate::schemas::{}::{}",
                scheme_mod,
                enum_name.to_upper_camel_case()
            ))
                 .unwrap(), parse_str("attr").unwrap(), Some(attr_name))
        } else {
            (parse_str(&enum_name.to_upper_camel_case()).unwrap(), parse_str("attr").unwrap(), Some(attr_name))
        }
    } else {
        (parse_str(&get_type_name(&attr.r#type)).unwrap(), parse_str("attr").unwrap(), Some(attr_name))
    };

    let mut required = false;

    for validator in &attr.validators {
        if validator.name == "RequiredValidator" {
            required = true;
        }
    }

    let property_comments_doc = format!(" {}", attr.property_comments);

    let qualified_doc = format!(
        " Represents the following attribute in the schema: {}",
        attr.q_name
    );

    if let Some(tag_name) = tag_name {
        if required {
            quote! {
            #[doc = #property_comments_doc]
            #[doc = #qualified_doc]
            #[xml(#tag_type = #tag_name)]
            pub #attr_name_ident: #type_ident,
        }
        } else {
            quote! {
            #[doc = #property_comments_doc]
            #[doc = #qualified_doc]
            #[xml(#tag_type = #tag_name)]
            pub #attr_name_ident: Option<#type_ident>,
        }
        }
    } else {
        if required {
            quote! {
                #[doc = #property_comments_doc]
                #[doc = #qualified_doc]
                #[xml(#tag_type)]
                pub #attr_name_ident: #type_ident,
            }
        } else {
            quote! {
                #[doc = #property_comments_doc]
                #[doc = #qualified_doc]
                #[xml(#tag_type)]
                pub #attr_name_ident: Option<#type_ident>,
            }
        }
    }
}

fn gen_children(
    class_name: &str,
    children: &Vec<OpenXmlSchemaTypeChild>,
    schema_namespace: &OpenXmlNamespace,
    context: &GenContext,
) -> (Option<TokenStream>, Option<ItemEnum>) {
    if children.is_empty() {
        return (None, None);
    }

    let child_choice_enum_ident: Ident =
        parse_str(&format!("{}ChildChoice", class_name.to_upper_camel_case())).unwrap();

    let mut variants: Vec<TokenStream> = vec![];
    let mut tags: Vec<String> = vec![];

    for child in children {
        let child_type = context
            .type_name_type_map
            .get(child.name.as_str())
            .ok_or(format!("{:?}", child.name))
            .unwrap();

        let child_namespace = context
            .type_name_namespace_map
            .get(child.name.as_str())
            .ok_or(format!("{:?}", child.name))
            .unwrap();

        let child_name_list: Vec<&str> = child.name.split('/').collect();

        let child_rename_ser_str = child_name_list
            .last()
            .ok_or(format!("{:?}", child.name))
            .unwrap();

        let child_variant_name_ident: Ident =
            parse_str(&child_rename_ser_str.to_upper_camel_case()).unwrap();

        let child_variant_type: Type = if child_namespace.prefix != schema_namespace.prefix {
            let scheme_mod = context
                .prefix_schema_mod_map
                .get(child_namespace.prefix.as_str())
                .ok_or(format!("{:?}", child_namespace.prefix))
                .unwrap();

            parse_str(&format!(
                "crate::schemas::{}::{}",
                scheme_mod,
                child_type.class_name.to_upper_camel_case()
            ))
                .unwrap()
        } else {
            parse_str(&child_type.class_name.to_upper_camel_case()).unwrap()
        };
        // let tag_name = format!("{}", &child_variant_name_ident).to_lower_camel_case();
        tags.push(child_rename_ser_str.to_string());
        variants.push(quote! {
            #[xml(tag = #child_rename_ser_str)]
            #child_variant_name_ident(#child_variant_type),
        });
    }

    let enum_option = Some(
        parse2(quote! {
            #[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
            pub enum #child_choice_enum_ident {
                #( #variants )*
            }
        })
            .unwrap(),
    );

    let field_option = Some(quote! {
        #[xml( #( child = #tags , )* )]
        pub children: Vec<#child_choice_enum_ident>,
    });

    (field_option, enum_option)
}

fn gen_child_type(
    t: &OpenXmlSchemaType,
    schema_namespace: &OpenXmlNamespace,
    context: &GenContext,
) -> (Type, Type, Option<String>) {
    let name_list: Vec<&str> = t.name.split('/').collect();

    let first_name = name_list.first().ok_or(format!("{:?}", t.name)).unwrap();
    let rename_ser_str = name_list.last().ok_or(format!("{:?}", t.name)).unwrap().to_string();


    if let Some(e) = context.enum_type_enum_map.get(first_name) {
        let e_namespace = context
            .enum_type_namespace_map
            .get(e.r#type.as_str())
            .ok_or(format!("{:?}", e.r#type))
            .unwrap();

        if e_namespace.prefix != schema_namespace.prefix {
            let scheme_mod = context
                .prefix_schema_mod_map
                .get(e_namespace.prefix.as_str())
                .ok_or(format!("{:?}", e_namespace.prefix))
                .unwrap();

            (parse_str(&format!(
                "crate::schemas::{}::{}",
                scheme_mod,
                e.name.to_upper_camel_case()
            ))
                 .unwrap(), parse_str("attr").unwrap(), Some(rename_ser_str))
        } else {
            (parse_str(&e.name.to_upper_camel_case()).unwrap(), parse_str("attr").unwrap(), Some(rename_ser_str))
        }
    } else {
        (parse_str(&get_type_name(simple_type_mapping(first_name)))
             .unwrap(), parse_str("text").unwrap(), None)
    }
}

fn gen_one_sequence_fields(
    t: &OpenXmlSchemaType,
    schema_namespace: &OpenXmlNamespace,
    context: &GenContext,
) -> Vec<TokenStream> {
    let mut fields: Vec<TokenStream> = vec![];

    let mut child_map: HashMap<&str, &OpenXmlSchemaTypeChild> = HashMap::new();

    for child in &t.children {
        child_map.insert(&child.name, child);
    }

    for p in &t.particle.items {
        let child = child_map
            .get(p.name.as_str())
            .ok_or(format!("{:?}", p.name))
            .unwrap();

        let child_type = context
            .type_name_type_map
            .get(child.name.as_str())
            .ok_or(format!("{:?}", child.name))
            .unwrap();

        let child_namespace = context
            .type_name_namespace_map
            .get(child.name.as_str())
            .ok_or(format!("{:?}", child.name))
            .unwrap();

        let child_variant_type: Type = if child_namespace.prefix != schema_namespace.prefix {
            let scheme_mod = context
                .prefix_schema_mod_map
                .get(child_namespace.prefix.as_str())
                .ok_or(format!("{:?}", child_namespace.prefix))
                .unwrap();

            parse_str(&format!(
                "crate::schemas::{}::{}",
                scheme_mod,
                child_type.class_name.to_upper_camel_case()
            ))
                .unwrap()
        } else {
            parse_str(&child_type.class_name.to_upper_camel_case()).unwrap()
        };

        let child_name_list: Vec<&str> = child.name.split('/').collect();

        let child_rename_ser_str = child_name_list
            .last()
            .ok_or(format!("{:?}", child.name))
            .unwrap().to_string();
        let (child_name_ident, tag_type, tag_name): (Ident, Type, String) = if child.property_name.is_empty() {
            (parse_str(&child_rename_ser_str.to_snake_case()).unwrap(), parse_str("child").unwrap(), child_rename_ser_str)
        } else {
            (parse_str(&escape_snake_case(child.property_name.to_snake_case())).unwrap(), parse_str("child").unwrap(), child_rename_ser_str)
        };

        let property_comments = if child.property_comments.is_empty() {
            " _"
        } else {
            &child.property_comments
        };

        if p.occurs.is_empty() {
            fields.push(quote! {
                #[doc = #property_comments]
                #[xml(#tag_type = #tag_name)]
                pub #child_name_ident: #child_variant_type,
            });
        } else if p.occurs[0].min == 0 && p.occurs[0].max == 1 {
            fields.push(quote! {
                #[doc = #property_comments]
                #[xml(#tag_type = #tag_name)]
                pub #child_name_ident: Option<#child_variant_type>,
            });
        } else {
            fields.push(quote! {
                #[doc = #property_comments]
                #[xml(child = #tag_name)]
                pub #child_name_ident: Vec<#child_variant_type>,
            });
        }
    }

    fields
}