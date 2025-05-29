#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use actix_example_service::{
    sea_orm::{Database, DatabaseConnection},
    Mutation, Query,
};
use actix_files::Files as Fs;
use actix_web::{
    error, get, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Result,
};
use ::middleware::{Permission, Role, SayHi};
use entity::posts;
use listenfd::ListenFd;
use migration::{Migrator, MigratorTrait};
use serde::{Deserialize, Serialize};
use std::env;
use tera::Tera;
const DEFAULT_POSTS_PER_PAGE: u64 = 5;
struct AppState {
    templates: tera::Tera,
    conn: DatabaseConnection,
}
#[automatically_derived]
impl ::core::fmt::Debug for AppState {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "AppState",
            "templates",
            &self.templates,
            "conn",
            &&self.conn,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for AppState {
    #[inline]
    fn clone(&self) -> AppState {
        AppState {
            templates: ::core::clone::Clone::clone(&self.templates),
            conn: ::core::clone::Clone::clone(&self.conn),
        }
    }
}
pub struct Params {
    page: Option<u64>,
    posts_per_page: Option<u64>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Params {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Params",
            "page",
            &self.page,
            "posts_per_page",
            &&self.posts_per_page,
        )
    }
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Params {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "page" => _serde::__private::Ok(__Field::__field0),
                        "posts_per_page" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"page" => _serde::__private::Ok(__Field::__field0),
                        b"posts_per_page" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Params>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Params;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct Params")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        Option<u64>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Params with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        Option<u64>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Params with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(Params {
                        page: __field0,
                        posts_per_page: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<Option<u64>> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<Option<u64>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("page"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<u64>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "posts_per_page",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<u64>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("page")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("posts_per_page")?
                        }
                    };
                    _serde::__private::Ok(Params {
                        page: __field0,
                        posts_per_page: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["page", "posts_per_page"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Params",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Params>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
struct FlashData {
    kind: String,
    message: String,
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for FlashData {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "kind" => _serde::__private::Ok(__Field::__field0),
                        "message" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"kind" => _serde::__private::Ok(__Field::__field0),
                        b"message" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<FlashData>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = FlashData;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct FlashData",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct FlashData with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct FlashData with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(FlashData {
                        kind: __field0,
                        message: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("kind"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "message",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("kind")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("message")?
                        }
                    };
                    _serde::__private::Ok(FlashData {
                        kind: __field0,
                        message: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["kind", "message"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "FlashData",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<FlashData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for FlashData {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "FlashData",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "kind",
                &self.kind,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "message",
                &self.message,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for FlashData {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "FlashData",
            "kind",
            &self.kind,
            "message",
            &&self.message,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for FlashData {
    #[inline]
    fn clone(&self) -> FlashData {
        FlashData {
            kind: ::core::clone::Clone::clone(&self.kind),
            message: ::core::clone::Clone::clone(&self.message),
        }
    }
}
#[allow(non_camel_case_types, missing_docs)]
pub struct list;
impl ::actix_web::dev::HttpServiceFactory for list {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        async fn list(
            req: HttpRequest,
            data: web::Data<AppState>,
        ) -> Result<HttpResponse, Error> {
            let template = &data.templates;
            let conn = &data.conn;
            let params = web::Query::<Params>::from_query(req.query_string()).unwrap();
            let page = params.page.unwrap_or(1);
            let posts_per_page = params.posts_per_page.unwrap_or(DEFAULT_POSTS_PER_PAGE);
            let (posts, num_pages) = Query::find_posts_in_page(
                    conn,
                    page,
                    posts_per_page,
                )
                .await
                .expect("Cannot find posts in page");
            {
                ::std::io::_print(format_args!("=> {0:#?}\n", posts));
            };
            let mut ctx = tera::Context::new();
            ctx.insert("posts", &posts);
            ctx.insert("page", &page);
            ctx.insert("posts_per_page", &posts_per_page);
            ctx.insert("num_pages", &num_pages);
            let body = template
                .render("index.html.tera", &ctx)
                .map_err(|_| error::ErrorInternalServerError("Template error"))?;
            Ok(HttpResponse::Ok().content_type("text/html").body(body))
        }
        let __resource = ::actix_web::Resource::new("/")
            .name("list")
            .guard(::actix_web::guard::Get())
            .wrap(
                SayHi::new(
                    "all",
                    Role::Owner,
                    <[_]>::into_vec(::alloc::boxed::box_new([Permission::All])),
                ),
            )
            .wrap(
                SayHi::new(
                    "list",
                    Role::Saler,
                    <[_]>::into_vec(
                        ::alloc::boxed::box_new([Permission::Check, Permission::Publish]),
                    ),
                ),
            )
            .to(list);
        ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
    }
}
#[allow(non_camel_case_types, missing_docs)]
pub struct new;
impl ::actix_web::dev::HttpServiceFactory for new {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        async fn new(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
            let template = &data.templates;
            let ctx = tera::Context::new();
            let body = template
                .render("new.html.tera", &ctx)
                .map_err(|_| error::ErrorInternalServerError("Template error"))?;
            Ok(HttpResponse::Ok().content_type("text/html").body(body))
        }
        let __resource = ::actix_web::Resource::new("/new")
            .name("new")
            .guard(::actix_web::guard::Get())
            .wrap(
                SayHi::new(
                    "new",
                    Role::Manager,
                    <[_]>::into_vec(::alloc::boxed::box_new([Permission::Publish])),
                ),
            )
            .to(new);
        ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
    }
}
#[allow(non_camel_case_types, missing_docs)]
pub struct create;
impl ::actix_web::dev::HttpServiceFactory for create {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        async fn create(
            data: web::Data<AppState>,
            post_form: web::Form<posts::Model>,
        ) -> Result<HttpResponse, Error> {
            let conn = &data.conn;
            let form = post_form.into_inner();
            Mutation::create_post(conn, form).await.expect("could not insert post");
            Ok(HttpResponse::Found().append_header(("location", "/")).finish())
        }
        let __resource = ::actix_web::Resource::new("/")
            .name("create")
            .guard(::actix_web::guard::Post())
            .wrap(
                SayHi::new(
                    "create",
                    Role::Owner,
                    <[_]>::into_vec(::alloc::boxed::box_new([Permission::All])),
                ),
            )
            .to(create);
        ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
    }
}
#[allow(non_camel_case_types, missing_docs)]
pub struct edit;
impl ::actix_web::dev::HttpServiceFactory for edit {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        async fn edit(
            data: web::Data<AppState>,
            id: web::Path<i32>,
        ) -> Result<HttpResponse, Error> {
            let conn = &data.conn;
            let template = &data.templates;
            let id = id.into_inner();
            let post: Option<posts::Model> = Query::find_post_by_id(conn, id)
                .await
                .expect("could not find post");
            let mut ctx = tera::Context::new();
            let body = match post {
                Some(post) => {
                    ctx.insert("post", &post);
                    template
                        .render("edit.html.tera", &ctx)
                        .map_err(|_| error::ErrorInternalServerError("Template error"))
                }
                None => {
                    ctx.insert(
                        "uri",
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(format_args!("/{0}", id));
                            res
                        }),
                    );
                    template
                        .render("error/404.html.tera", &ctx)
                        .map_err(|_| error::ErrorInternalServerError("Template error"))
                }
            };
            Ok(HttpResponse::Ok().content_type("text/html").body(body?))
        }
        let __resource = ::actix_web::Resource::new(r#"/{id:\d+}"#)
            .name("edit")
            .guard(::actix_web::guard::Get())
            .wrap(
                SayHi::new(
                    "edit",
                    Role::Manager,
                    <[_]>::into_vec(::alloc::boxed::box_new([Permission::Publish])),
                ),
            )
            .to(edit);
        ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
    }
}
#[allow(non_camel_case_types, missing_docs)]
pub struct update;
impl ::actix_web::dev::HttpServiceFactory for update {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        async fn update(
            data: web::Data<AppState>,
            id: web::Path<i32>,
            post_form: web::Form<posts::Model>,
        ) -> Result<HttpResponse, Error> {
            let conn = &data.conn;
            let form = post_form.into_inner();
            let id = id.into_inner();
            Mutation::update_post_by_id(conn, id, form)
                .await
                .expect("could not edit post");
            Ok(HttpResponse::Found().append_header(("location", "/")).finish())
        }
        let __resource = ::actix_web::Resource::new("/{id}")
            .name("update")
            .guard(::actix_web::guard::Post())
            .wrap(
                SayHi::new(
                    "update",
                    Role::Manager,
                    <[_]>::into_vec(::alloc::boxed::box_new([Permission::Publish])),
                ),
            )
            .to(update);
        ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
    }
}
#[allow(non_camel_case_types, missing_docs)]
pub struct delete;
impl ::actix_web::dev::HttpServiceFactory for delete {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        async fn delete(
            data: web::Data<AppState>,
            id: web::Path<i32>,
        ) -> Result<HttpResponse, Error> {
            let conn = &data.conn;
            let id = id.into_inner();
            Mutation::delete_post(conn, id).await.expect("could not delete post");
            Ok(HttpResponse::Found().append_header(("location", "/")).finish())
        }
        let __resource = ::actix_web::Resource::new("/delete/{id}")
            .name("delete")
            .guard(::actix_web::guard::Post())
            .wrap(
                SayHi::new(
                    "delete",
                    Role::Owner,
                    <[_]>::into_vec(::alloc::boxed::box_new([Permission::All])),
                ),
            )
            .to(delete);
        ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
    }
}
async fn not_found(
    data: web::Data<AppState>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("uri", request.uri().path());
    let template = &data.templates;
    let body = template
        .render("error/404.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
fn start() -> std::io::Result<()> {
    <::actix_web::rt::System>::new()
        .block_on(async move {
            {
                std::env::set_var("RUST_LOG", "debug");
                tracing_subscriber::fmt::init();
                dotenvy::dotenv().ok();
                let db_url = env::var("DATABASE_URL")
                    .expect("DATABASE_URL is not set in .env file");
                let host = env::var("HOST").expect("HOST is not set in .env file");
                let port = env::var("PORT").expect("PORT is not set in .env file");
                let server_url = ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(format_args!("{0}:{1}", host, port));
                    res
                });
                let conn = Database::connect(&db_url).await.unwrap();
                Migrator::up(&conn, None).await.unwrap();
                let templates = Tera::new(
                        "/Users/zzq/work/rust-examples/sea-orm-demo/api/templates/**/*",
                    )
                    .unwrap();
                let state = AppState { templates, conn };
                let mut listenfd = ListenFd::from_env();
                let mut server = HttpServer::new(move || {
                    App::new()
                        .service(Fs::new("/static", "./api/static"))
                        .app_data(web::Data::new(state.clone()))
                        .wrap(middleware::Logger::default())
                        .default_service(web::route().to(not_found))
                        .configure(init)
                });
                server = match listenfd.take_tcp_listener(0)? {
                    Some(listener) => server.listen(listener)?,
                    None => server.bind(&server_url)?,
                };
                {
                    ::std::io::_print(
                        format_args!("Starting server at {0}\n", server_url),
                    );
                };
                server.run().await?;
                Ok(())
            }
        })
}
fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(new);
    cfg.service(create);
    cfg.service(edit);
    cfg.service(update);
    cfg.service(delete);
}
pub fn main() {
    let result = start();
    if let Some(err) = result.err() {
        {
            ::std::io::_print(format_args!("Error: {0}\n", err));
        };
    }
}
