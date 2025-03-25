#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::{collections::HashSet, str::FromStr};
use macros::{register_task, task_param};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
#[serde(rename_all = "camelCase")]
#[schemars(title = "MelonParam")]
pub struct MelonParam {
    #[validate(email)]
    #[schemars(extend("x-priority" = 1))]
    #[schemars(
        extend(
            "x-decorator" = "FormItem",
            "x-decorator-props" = {"layout":"vertical"},
            "x-component" = "Input"
        )
    )]
    pub user_name: String,
    #[validate(email)]
    #[schemars(extend("x-priority" = 500))]
    #[schemars(
        extend(
            "x-decorator" = "FormItem",
            "x-decorator-props" = {"layout":"vertical"},
            "x-component" = "Input"
        )
    )]
    pub pay_email: String,
    #[schemars(
        extend(
            "x-decorator" = "FormItem",
            "x-decorator-props" = {"layout":"vertical"},
            "x-component" = "Input"
        )
    )]
    pub product_id: String,
    #[schemars(
        extend(
            "x-decorator" = "FormItem",
            "x-decorator-props" = {"layout":"vertical"},
            "x-component" = "Input"
        )
    )]
    pub pay_type: String,
    #[schemars(
        extend(
            "x-decorator" = "FormItem",
            "x-decorator-props" = {"layout":"vertical"},
            "x-component" = "Input"
        )
    )]
    pub auto_pay: Option<String>,
    #[schemars(
        extend(
            "x-decorator" = "FormItem",
            "x-decorator-props" = {"layout":"vertical"},
            "x-component" = "Input"
        )
    )]
    pub schedule_no: String,
    #[schemars(with = "String")]
    #[schemars(
        extend(
            "x-decorator" = "FormItem",
            "x-decorator-props" = {"layout":"vertical"},
            "x-component" = "Input"
        )
    )]
    pub transfer_owner: Option<String>,
    #[schemars(
        extend(
            "x-decorator" = "FormItem",
            "x-decorator-props" = {"layout":"vertical"},
            "x-component" = "Input"
        )
    )]
    pub mode: String,
    #[schemars(
        extend(
            "x-decorator" = "FormItem",
            "x-decorator-props" = {"layout":"vertical"},
            "x-component" = "Input"
        )
    )]
    pub before_second: u64,
    #[cfg(not(any(env = "product", env = "beta")))]
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[schemars(
        extend(
            "x-decorator" = "FormItem",
            "x-decorator-props" = {"layout":"vertical"},
            "x-component" = "Input"
        )
    )]
    pub queue_count: u64,
    #[cfg(not(any(env = "product", env = "beta")))]
    #[schemars(extend("x-priority" = 900))]
    #[schemars(
        extend(
            "x-decorator" = "FormItem",
            "x-decorator-props" = {"layout":"vertical"},
            "x-component" = "Input"
        )
    )]
    pub send_restock: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for MelonParam {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "user_name",
            "pay_email",
            "product_id",
            "pay_type",
            "auto_pay",
            "schedule_no",
            "transfer_owner",
            "mode",
            "before_second",
            "queue_count",
            "send_restock",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &self.user_name,
            &self.pay_email,
            &self.product_id,
            &self.pay_type,
            &self.auto_pay,
            &self.schedule_no,
            &self.transfer_owner,
            &self.mode,
            &self.before_second,
            &self.queue_count,
            &&self.send_restock,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "MelonParam",
            names,
            values,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for MelonParam {
    #[inline]
    fn clone(&self) -> MelonParam {
        MelonParam {
            user_name: ::core::clone::Clone::clone(&self.user_name),
            pay_email: ::core::clone::Clone::clone(&self.pay_email),
            product_id: ::core::clone::Clone::clone(&self.product_id),
            pay_type: ::core::clone::Clone::clone(&self.pay_type),
            auto_pay: ::core::clone::Clone::clone(&self.auto_pay),
            schedule_no: ::core::clone::Clone::clone(&self.schedule_no),
            transfer_owner: ::core::clone::Clone::clone(&self.transfer_owner),
            mode: ::core::clone::Clone::clone(&self.mode),
            before_second: ::core::clone::Clone::clone(&self.before_second),
            queue_count: ::core::clone::Clone::clone(&self.queue_count),
            send_restock: ::core::clone::Clone::clone(&self.send_restock),
        }
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
    impl _serde::Serialize for MelonParam {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "MelonParam",
                false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "userName",
                &self.user_name,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "payEmail",
                &self.pay_email,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "productId",
                &self.product_id,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "payType",
                &self.pay_type,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "autoPay",
                &self.auto_pay,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "scheduleNo",
                &self.schedule_no,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "transferOwner",
                &self.transfer_owner,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "mode",
                &self.mode,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "beforeSecond",
                &self.before_second,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "queueCount",
                &self.queue_count,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "sendRestock",
                &self.send_restock,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
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
    impl<'de> _serde::Deserialize<'de> for MelonParam {
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
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
                __field8,
                __field9,
                __field10,
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
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        4u64 => _serde::__private::Ok(__Field::__field4),
                        5u64 => _serde::__private::Ok(__Field::__field5),
                        6u64 => _serde::__private::Ok(__Field::__field6),
                        7u64 => _serde::__private::Ok(__Field::__field7),
                        8u64 => _serde::__private::Ok(__Field::__field8),
                        9u64 => _serde::__private::Ok(__Field::__field9),
                        10u64 => _serde::__private::Ok(__Field::__field10),
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
                        "userName" => _serde::__private::Ok(__Field::__field0),
                        "payEmail" => _serde::__private::Ok(__Field::__field1),
                        "productId" => _serde::__private::Ok(__Field::__field2),
                        "payType" => _serde::__private::Ok(__Field::__field3),
                        "autoPay" => _serde::__private::Ok(__Field::__field4),
                        "scheduleNo" => _serde::__private::Ok(__Field::__field5),
                        "transferOwner" => _serde::__private::Ok(__Field::__field6),
                        "mode" => _serde::__private::Ok(__Field::__field7),
                        "beforeSecond" => _serde::__private::Ok(__Field::__field8),
                        "queueCount" => _serde::__private::Ok(__Field::__field9),
                        "sendRestock" => _serde::__private::Ok(__Field::__field10),
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
                        b"userName" => _serde::__private::Ok(__Field::__field0),
                        b"payEmail" => _serde::__private::Ok(__Field::__field1),
                        b"productId" => _serde::__private::Ok(__Field::__field2),
                        b"payType" => _serde::__private::Ok(__Field::__field3),
                        b"autoPay" => _serde::__private::Ok(__Field::__field4),
                        b"scheduleNo" => _serde::__private::Ok(__Field::__field5),
                        b"transferOwner" => _serde::__private::Ok(__Field::__field6),
                        b"mode" => _serde::__private::Ok(__Field::__field7),
                        b"beforeSecond" => _serde::__private::Ok(__Field::__field8),
                        b"queueCount" => _serde::__private::Ok(__Field::__field9),
                        b"sendRestock" => _serde::__private::Ok(__Field::__field10),
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
                marker: _serde::__private::PhantomData<MelonParam>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = MelonParam;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct MelonParam",
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
                                    &"struct MelonParam with 11 elements",
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
                                    &"struct MelonParam with 11 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct MelonParam with 11 elements",
                                ),
                            );
                        }
                    };
                    let __field3 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct MelonParam with 11 elements",
                                ),
                            );
                        }
                    };
                    let __field4 = match _serde::de::SeqAccess::next_element::<
                        Option<String>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    4usize,
                                    &"struct MelonParam with 11 elements",
                                ),
                            );
                        }
                    };
                    let __field5 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    5usize,
                                    &"struct MelonParam with 11 elements",
                                ),
                            );
                        }
                    };
                    let __field6 = match _serde::de::SeqAccess::next_element::<
                        Option<String>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    6usize,
                                    &"struct MelonParam with 11 elements",
                                ),
                            );
                        }
                    };
                    let __field7 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    7usize,
                                    &"struct MelonParam with 11 elements",
                                ),
                            );
                        }
                    };
                    let __field8 = match _serde::de::SeqAccess::next_element::<
                        u64,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    8usize,
                                    &"struct MelonParam with 11 elements",
                                ),
                            );
                        }
                    };
                    let __field9 = match {
                        #[doc(hidden)]
                        struct __DeserializeWith<'de> {
                            value: u64,
                            phantom: _serde::__private::PhantomData<MelonParam>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::__private::Ok(__DeserializeWith {
                                    value: deserialize_string_to_number(__deserializer)?,
                                    phantom: _serde::__private::PhantomData,
                                    lifetime: _serde::__private::PhantomData,
                                })
                            }
                        }
                        _serde::__private::Option::map(
                            _serde::de::SeqAccess::next_element::<
                                __DeserializeWith<'de>,
                            >(&mut __seq)?,
                            |__wrap| __wrap.value,
                        )
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    9usize,
                                    &"struct MelonParam with 11 elements",
                                ),
                            );
                        }
                    };
                    let __field10 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    10usize,
                                    &"struct MelonParam with 11 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(MelonParam {
                        user_name: __field0,
                        pay_email: __field1,
                        product_id: __field2,
                        pay_type: __field3,
                        auto_pay: __field4,
                        schedule_no: __field5,
                        transfer_owner: __field6,
                        mode: __field7,
                        before_second: __field8,
                        queue_count: __field9,
                        send_restock: __field10,
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
                    let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field4: _serde::__private::Option<Option<String>> = _serde::__private::None;
                    let mut __field5: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field6: _serde::__private::Option<Option<String>> = _serde::__private::None;
                    let mut __field7: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field8: _serde::__private::Option<u64> = _serde::__private::None;
                    let mut __field9: _serde::__private::Option<u64> = _serde::__private::None;
                    let mut __field10: _serde::__private::Option<String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userName",
                                        ),
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
                                            "payEmail",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "productId",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field3 => {
                                if _serde::__private::Option::is_some(&__field3) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "payType",
                                        ),
                                    );
                                }
                                __field3 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field4 => {
                                if _serde::__private::Option::is_some(&__field4) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "autoPay",
                                        ),
                                    );
                                }
                                __field4 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field5 => {
                                if _serde::__private::Option::is_some(&__field5) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "scheduleNo",
                                        ),
                                    );
                                }
                                __field5 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field6 => {
                                if _serde::__private::Option::is_some(&__field6) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transferOwner",
                                        ),
                                    );
                                }
                                __field6 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field7 => {
                                if _serde::__private::Option::is_some(&__field7) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("mode"),
                                    );
                                }
                                __field7 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field8 => {
                                if _serde::__private::Option::is_some(&__field8) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "beforeSecond",
                                        ),
                                    );
                                }
                                __field8 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<u64>(&mut __map)?,
                                );
                            }
                            __Field::__field9 => {
                                if _serde::__private::Option::is_some(&__field9) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "queueCount",
                                        ),
                                    );
                                }
                                __field9 = _serde::__private::Some({
                                    #[doc(hidden)]
                                    struct __DeserializeWith<'de> {
                                        value: u64,
                                        phantom: _serde::__private::PhantomData<MelonParam>,
                                        lifetime: _serde::__private::PhantomData<&'de ()>,
                                    }
                                    #[automatically_derived]
                                    impl<'de> _serde::Deserialize<'de>
                                    for __DeserializeWith<'de> {
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> _serde::__private::Result<Self, __D::Error>
                                        where
                                            __D: _serde::Deserializer<'de>,
                                        {
                                            _serde::__private::Ok(__DeserializeWith {
                                                value: deserialize_string_to_number(__deserializer)?,
                                                phantom: _serde::__private::PhantomData,
                                                lifetime: _serde::__private::PhantomData,
                                            })
                                        }
                                    }
                                    match _serde::de::MapAccess::next_value::<
                                        __DeserializeWith<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__wrapper) => __wrapper.value,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                });
                            }
                            __Field::__field10 => {
                                if _serde::__private::Option::is_some(&__field10) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sendRestock",
                                        ),
                                    );
                                }
                                __field10 = _serde::__private::Some(
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
                            _serde::__private::de::missing_field("userName")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("payEmail")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("productId")?
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private::Some(__field3) => __field3,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("payType")?
                        }
                    };
                    let __field4 = match __field4 {
                        _serde::__private::Some(__field4) => __field4,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("autoPay")?
                        }
                    };
                    let __field5 = match __field5 {
                        _serde::__private::Some(__field5) => __field5,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("scheduleNo")?
                        }
                    };
                    let __field6 = match __field6 {
                        _serde::__private::Some(__field6) => __field6,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("transferOwner")?
                        }
                    };
                    let __field7 = match __field7 {
                        _serde::__private::Some(__field7) => __field7,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("mode")?
                        }
                    };
                    let __field8 = match __field8 {
                        _serde::__private::Some(__field8) => __field8,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("beforeSecond")?
                        }
                    };
                    let __field9 = match __field9 {
                        _serde::__private::Some(__field9) => __field9,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "queueCount",
                                ),
                            );
                        }
                    };
                    let __field10 = match __field10 {
                        _serde::__private::Some(__field10) => __field10,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("sendRestock")?
                        }
                    };
                    _serde::__private::Ok(MelonParam {
                        user_name: __field0,
                        pay_email: __field1,
                        product_id: __field2,
                        pay_type: __field3,
                        auto_pay: __field4,
                        schedule_no: __field5,
                        transfer_owner: __field6,
                        mode: __field7,
                        before_second: __field8,
                        queue_count: __field9,
                        send_restock: __field10,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &[
                "userName",
                "payEmail",
                "productId",
                "payType",
                "autoPay",
                "scheduleNo",
                "transferOwner",
                "mode",
                "beforeSecond",
                "queueCount",
                "sendRestock",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "MelonParam",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<MelonParam>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::default::Default for MelonParam {
    #[inline]
    fn default() -> MelonParam {
        MelonParam {
            user_name: ::core::default::Default::default(),
            pay_email: ::core::default::Default::default(),
            product_id: ::core::default::Default::default(),
            pay_type: ::core::default::Default::default(),
            auto_pay: ::core::default::Default::default(),
            schedule_no: ::core::default::Default::default(),
            transfer_owner: ::core::default::Default::default(),
            mode: ::core::default::Default::default(),
            before_second: ::core::default::Default::default(),
            queue_count: ::core::default::Default::default(),
            send_restock: ::core::default::Default::default(),
        }
    }
}
impl MelonParam {
    pub fn get_ordered_fields() -> Vec<String> {
        <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                "send_restock".to_string(),
                "pay_email".to_string(),
                "user_name".to_string(),
                "product_id".to_string(),
                "pay_type".to_string(),
                "auto_pay".to_string(),
                "schedule_no".to_string(),
                "transfer_owner".to_string(),
                "mode".to_string(),
                "before_second".to_string(),
                "queue_count".to_string(),
            ]),
        )
    }
    pub fn get_fields_with_priority() -> Vec<(String, i64)> {
        <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                ("send_restock".to_string(), 900i64),
                ("pay_email".to_string(), 500i64),
                ("user_name".to_string(), 1i64),
                ("product_id".to_string(), 0i64),
                ("pay_type".to_string(), 0i64),
                ("auto_pay".to_string(), 0i64),
                ("schedule_no".to_string(), 0i64),
                ("transfer_owner".to_string(), 0i64),
                ("mode".to_string(), 0i64),
                ("before_second".to_string(), 0i64),
                ("queue_count".to_string(), 0i64),
            ]),
        )
    }
}
impl MelonParam {
    pub fn from_value(values: &[serde_json::Value]) -> Result<Vec<Self>> {
        use std::collections::HashSet;
        let mut params = Vec::new();
        for v in values {
            params.push(serde_json::from_value(v.clone())?);
        }
        Ok(params)
    }
}
impl ::validator::Validate for MelonParam {
    fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
        use ::validator::ValidateArgs;
        self.validate_with_args(())
    }
}
impl<'v_a> ::validator::ValidateArgs<'v_a> for MelonParam {
    type Args = ();
    fn validate_with_args(
        &self,
        args: Self::Args,
    ) -> ::std::result::Result<(), ::validator::ValidationErrors> {
        use ::validator::ValidateEmail;
        let mut errors = ::validator::ValidationErrors::new();
        if !self.user_name.validate_email() {
            let mut err = ::validator::ValidationError::new("email");
            err.add_param(::std::borrow::Cow::from("value"), &self.user_name);
            errors.add("user_name", err);
        }
        if !self.pay_email.validate_email() {
            let mut err = ::validator::ValidationError::new("email");
            err.add_param(::std::borrow::Cow::from("value"), &self.pay_email);
            errors.add("pay_email", err);
        }
        if errors.is_empty() {
            ::std::result::Result::Ok(())
        } else {
            ::std::result::Result::Err(errors)
        }
    }
}
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for MelonParam {
        fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
            schemars::_private::alloc::borrow::Cow::Borrowed("MelonParam")
        }
        fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
            schemars::_private::alloc::borrow::Cow::Borrowed("macro_demo::MelonParam")
        }
        fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
            {
                let mut schema = ::schemars::Schema::try_from(
                        ::serde_json::Value::Object({
                            let mut object = ::serde_json::Map::new();
                            let _ = object
                                .insert(
                                    ("type").into(),
                                    ::serde_json::to_value(&"object").unwrap(),
                                );
                            object
                        }),
                    )
                    .unwrap();
                {
                    schemars::_private::insert_object_property(
                        &mut schema,
                        "userName",
                        if generator.contract().is_serialize() {
                            false
                        } else {
                            false
                                || (!false
                                    && <String as schemars::JsonSchema>::_schemars_private_is_option())
                        },
                        {
                            let mut schema = generator.subschema_for::<String>();
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-priority",
                                ::serde_json::to_value(&1).unwrap(),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator",
                                ::serde_json::to_value(&"FormItem").unwrap(),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator-props",
                                ::serde_json::Value::Object({
                                    let mut object = ::serde_json::Map::new();
                                    let _ = object
                                        .insert(
                                            ("layout").into(),
                                            ::serde_json::to_value(&"vertical").unwrap(),
                                        );
                                    object
                                }),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-component",
                                ::serde_json::to_value(&"Input").unwrap(),
                            );
                            (&mut schema)
                                .ensure_object()
                                .insert("format".into(), "email".into());
                            schema
                        },
                    );
                }
                {
                    schemars::_private::insert_object_property(
                        &mut schema,
                        "payEmail",
                        if generator.contract().is_serialize() {
                            false
                        } else {
                            false
                                || (!false
                                    && <String as schemars::JsonSchema>::_schemars_private_is_option())
                        },
                        {
                            let mut schema = generator.subschema_for::<String>();
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-priority",
                                ::serde_json::to_value(&500).unwrap(),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator",
                                ::serde_json::to_value(&"FormItem").unwrap(),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator-props",
                                ::serde_json::Value::Object({
                                    let mut object = ::serde_json::Map::new();
                                    let _ = object
                                        .insert(
                                            ("layout").into(),
                                            ::serde_json::to_value(&"vertical").unwrap(),
                                        );
                                    object
                                }),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-component",
                                ::serde_json::to_value(&"Input").unwrap(),
                            );
                            (&mut schema)
                                .ensure_object()
                                .insert("format".into(), "email".into());
                            schema
                        },
                    );
                }
                {
                    schemars::_private::insert_object_property(
                        &mut schema,
                        "productId",
                        if generator.contract().is_serialize() {
                            false
                        } else {
                            false
                                || (!false
                                    && <String as schemars::JsonSchema>::_schemars_private_is_option())
                        },
                        {
                            let mut schema = generator.subschema_for::<String>();
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator",
                                ::serde_json::to_value(&"FormItem").unwrap(),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator-props",
                                ::serde_json::Value::Object({
                                    let mut object = ::serde_json::Map::new();
                                    let _ = object
                                        .insert(
                                            ("layout").into(),
                                            ::serde_json::to_value(&"vertical").unwrap(),
                                        );
                                    object
                                }),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-component",
                                ::serde_json::to_value(&"Input").unwrap(),
                            );
                            schema
                        },
                    );
                }
                {
                    schemars::_private::insert_object_property(
                        &mut schema,
                        "payType",
                        if generator.contract().is_serialize() {
                            false
                        } else {
                            false
                                || (!false
                                    && <String as schemars::JsonSchema>::_schemars_private_is_option())
                        },
                        {
                            let mut schema = generator.subschema_for::<String>();
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator",
                                ::serde_json::to_value(&"FormItem").unwrap(),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator-props",
                                ::serde_json::Value::Object({
                                    let mut object = ::serde_json::Map::new();
                                    let _ = object
                                        .insert(
                                            ("layout").into(),
                                            ::serde_json::to_value(&"vertical").unwrap(),
                                        );
                                    object
                                }),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-component",
                                ::serde_json::to_value(&"Input").unwrap(),
                            );
                            schema
                        },
                    );
                }
                {
                    schemars::_private::insert_object_property(
                        &mut schema,
                        "autoPay",
                        if generator.contract().is_serialize() {
                            false
                        } else {
                            false
                                || (!false
                                    && <Option<
                                        String,
                                    > as schemars::JsonSchema>::_schemars_private_is_option())
                        },
                        {
                            let mut schema = generator.subschema_for::<Option<String>>();
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator",
                                ::serde_json::to_value(&"FormItem").unwrap(),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator-props",
                                ::serde_json::Value::Object({
                                    let mut object = ::serde_json::Map::new();
                                    let _ = object
                                        .insert(
                                            ("layout").into(),
                                            ::serde_json::to_value(&"vertical").unwrap(),
                                        );
                                    object
                                }),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-component",
                                ::serde_json::to_value(&"Input").unwrap(),
                            );
                            schema
                        },
                    );
                }
                {
                    schemars::_private::insert_object_property(
                        &mut schema,
                        "scheduleNo",
                        if generator.contract().is_serialize() {
                            false
                        } else {
                            false
                                || (!false
                                    && <String as schemars::JsonSchema>::_schemars_private_is_option())
                        },
                        {
                            let mut schema = generator.subschema_for::<String>();
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator",
                                ::serde_json::to_value(&"FormItem").unwrap(),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator-props",
                                ::serde_json::Value::Object({
                                    let mut object = ::serde_json::Map::new();
                                    let _ = object
                                        .insert(
                                            ("layout").into(),
                                            ::serde_json::to_value(&"vertical").unwrap(),
                                        );
                                    object
                                }),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-component",
                                ::serde_json::to_value(&"Input").unwrap(),
                            );
                            schema
                        },
                    );
                }
                {
                    schemars::_private::insert_object_property(
                        &mut schema,
                        "transferOwner",
                        if generator.contract().is_serialize() {
                            false
                        } else {
                            false
                                || (!false
                                    && <String as schemars::JsonSchema>::_schemars_private_is_option())
                        },
                        {
                            let mut schema = generator.subschema_for::<String>();
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator",
                                ::serde_json::to_value(&"FormItem").unwrap(),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator-props",
                                ::serde_json::Value::Object({
                                    let mut object = ::serde_json::Map::new();
                                    let _ = object
                                        .insert(
                                            ("layout").into(),
                                            ::serde_json::to_value(&"vertical").unwrap(),
                                        );
                                    object
                                }),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-component",
                                ::serde_json::to_value(&"Input").unwrap(),
                            );
                            schema
                        },
                    );
                }
                {
                    schemars::_private::insert_object_property(
                        &mut schema,
                        "mode",
                        if generator.contract().is_serialize() {
                            false
                        } else {
                            false
                                || (!false
                                    && <String as schemars::JsonSchema>::_schemars_private_is_option())
                        },
                        {
                            let mut schema = generator.subschema_for::<String>();
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator",
                                ::serde_json::to_value(&"FormItem").unwrap(),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator-props",
                                ::serde_json::Value::Object({
                                    let mut object = ::serde_json::Map::new();
                                    let _ = object
                                        .insert(
                                            ("layout").into(),
                                            ::serde_json::to_value(&"vertical").unwrap(),
                                        );
                                    object
                                }),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-component",
                                ::serde_json::to_value(&"Input").unwrap(),
                            );
                            schema
                        },
                    );
                }
                {
                    schemars::_private::insert_object_property(
                        &mut schema,
                        "beforeSecond",
                        if generator.contract().is_serialize() {
                            false
                        } else {
                            false
                                || (!false
                                    && <u64 as schemars::JsonSchema>::_schemars_private_is_option())
                        },
                        {
                            let mut schema = generator.subschema_for::<u64>();
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator",
                                ::serde_json::to_value(&"FormItem").unwrap(),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator-props",
                                ::serde_json::Value::Object({
                                    let mut object = ::serde_json::Map::new();
                                    let _ = object
                                        .insert(
                                            ("layout").into(),
                                            ::serde_json::to_value(&"vertical").unwrap(),
                                        );
                                    object
                                }),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-component",
                                ::serde_json::to_value(&"Input").unwrap(),
                            );
                            schema
                        },
                    );
                }
                {
                    schemars::_private::insert_object_property(
                        &mut schema,
                        "queueCount",
                        if generator.contract().is_serialize() {
                            false
                        } else {
                            false
                                || (!false
                                    && <u64 as schemars::JsonSchema>::_schemars_private_is_option())
                        },
                        {
                            let mut schema = generator.subschema_for::<u64>();
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator",
                                ::serde_json::to_value(&"FormItem").unwrap(),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator-props",
                                ::serde_json::Value::Object({
                                    let mut object = ::serde_json::Map::new();
                                    let _ = object
                                        .insert(
                                            ("layout").into(),
                                            ::serde_json::to_value(&"vertical").unwrap(),
                                        );
                                    object
                                }),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-component",
                                ::serde_json::to_value(&"Input").unwrap(),
                            );
                            schema
                        },
                    );
                }
                {
                    schemars::_private::insert_object_property(
                        &mut schema,
                        "sendRestock",
                        if generator.contract().is_serialize() {
                            false
                        } else {
                            false
                                || (!false
                                    && <String as schemars::JsonSchema>::_schemars_private_is_option())
                        },
                        {
                            let mut schema = generator.subschema_for::<String>();
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-priority",
                                ::serde_json::to_value(&900).unwrap(),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator",
                                ::serde_json::to_value(&"FormItem").unwrap(),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-decorator-props",
                                ::serde_json::Value::Object({
                                    let mut object = ::serde_json::Map::new();
                                    let _ = object
                                        .insert(
                                            ("layout").into(),
                                            ::serde_json::to_value(&"vertical").unwrap(),
                                        );
                                    object
                                }),
                            );
                            schemars::_private::insert_metadata_property(
                                &mut schema,
                                "x-component",
                                ::serde_json::to_value(&"Input").unwrap(),
                            );
                            schema
                        },
                    );
                }
                schemars::_private::insert_metadata_property_if_nonempty(
                    &mut schema,
                    "title",
                    "MelonParam",
                );
                schema
            }
        }
    }
};
impl MelonParam {
    pub fn json_schema() -> schemars::Schema {
        ::schemars::SchemaGenerator::default().into_root_schema_for::<MelonParam>()
    }
}
pub struct MelonTask {
    pub param: MelonParam,
    pub inicis_domain: String,
}
impl MelonTask {
    pub async fn new_tasks_with_values(_values: Vec<Value>) -> Result<()> {
        Ok(())
    }
}
pub struct TaskDescription {
    pub task_name: &'static str,
    pub fields: fn() -> Vec<String>,
    pub task_type: TaskType,
    pub website: &'static str,
    pub json_schema: fn() -> schemars::Schema,
    #[allow(dead_code)]
    pub level: TaskLevel,
}
#[automatically_derived]
impl ::core::clone::Clone for TaskDescription {
    #[inline]
    fn clone(&self) -> TaskDescription {
        TaskDescription {
            task_name: ::core::clone::Clone::clone(&self.task_name),
            fields: ::core::clone::Clone::clone(&self.fields),
            task_type: ::core::clone::Clone::clone(&self.task_type),
            website: ::core::clone::Clone::clone(&self.website),
            json_schema: ::core::clone::Clone::clone(&self.json_schema),
            level: ::core::clone::Clone::clone(&self.level),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for TaskDescription {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "task_name",
            "fields",
            "task_type",
            "website",
            "json_schema",
            "level",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &self.task_name,
            &self.fields,
            &self.task_type,
            &self.website,
            &self.json_schema,
            &&self.level,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "TaskDescription",
            names,
            values,
        )
    }
}
pub enum TaskLevel {
    Dev,
    Product,
}
#[automatically_derived]
impl ::core::fmt::Debug for TaskLevel {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                TaskLevel::Dev => "Dev",
                TaskLevel::Product => "Product",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for TaskLevel {
    #[inline]
    fn clone(&self) -> TaskLevel {
        match self {
            TaskLevel::Dev => TaskLevel::Dev,
            TaskLevel::Product => TaskLevel::Product,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for TaskLevel {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for TaskLevel {}
#[automatically_derived]
impl ::core::cmp::PartialEq for TaskLevel {
    #[inline]
    fn eq(&self, other: &TaskLevel) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
pub enum TaskType {
    Waiting,
    Queue,
    Aco,
    Login,
    Register,
}
#[automatically_derived]
impl ::core::fmt::Debug for TaskType {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                TaskType::Waiting => "Waiting",
                TaskType::Queue => "Queue",
                TaskType::Aco => "Aco",
                TaskType::Login => "Login",
                TaskType::Register => "Register",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for TaskType {
    #[inline]
    fn clone(&self) -> TaskType {
        match self {
            TaskType::Waiting => TaskType::Waiting,
            TaskType::Queue => TaskType::Queue,
            TaskType::Aco => TaskType::Aco,
            TaskType::Login => TaskType::Login,
            TaskType::Register => TaskType::Register,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for TaskType {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for TaskType {}
#[automatically_derived]
impl ::core::cmp::PartialEq for TaskType {
    #[inline]
    fn eq(&self, other: &TaskType) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
///An iterator over the variants of [TaskType]
#[allow(missing_copy_implementations)]
pub struct TaskTypeIter {
    idx: usize,
    back_idx: usize,
    marker: ::core::marker::PhantomData<()>,
}
impl ::core::fmt::Debug for TaskTypeIter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TaskTypeIter").field("len", &self.len()).finish()
    }
}
impl TaskTypeIter {
    fn get(&self, idx: usize) -> ::core::option::Option<TaskType> {
        match idx {
            0usize => ::core::option::Option::Some(TaskType::Waiting),
            1usize => ::core::option::Option::Some(TaskType::Queue),
            2usize => ::core::option::Option::Some(TaskType::Aco),
            3usize => ::core::option::Option::Some(TaskType::Login),
            4usize => ::core::option::Option::Some(TaskType::Register),
            _ => ::core::option::Option::None,
        }
    }
}
impl ::strum::IntoEnumIterator for TaskType {
    type Iterator = TaskTypeIter;
    fn iter() -> TaskTypeIter {
        TaskTypeIter {
            idx: 0,
            back_idx: 0,
            marker: ::core::marker::PhantomData,
        }
    }
}
impl Iterator for TaskTypeIter {
    type Item = TaskType;
    fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item> {
        self.nth(0)
    }
    fn size_hint(&self) -> (usize, ::core::option::Option<usize>) {
        let t = if self.idx + self.back_idx >= 5usize {
            0
        } else {
            5usize - self.idx - self.back_idx
        };
        (t, Some(t))
    }
    fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item> {
        let idx = self.idx + n + 1;
        if idx + self.back_idx > 5usize {
            self.idx = 5usize;
            ::core::option::Option::None
        } else {
            self.idx = idx;
            TaskTypeIter::get(self, idx - 1)
        }
    }
}
impl ExactSizeIterator for TaskTypeIter {
    fn len(&self) -> usize {
        self.size_hint().0
    }
}
impl DoubleEndedIterator for TaskTypeIter {
    fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item> {
        let back_idx = self.back_idx + 1;
        if self.idx + back_idx > 5usize {
            self.back_idx = 5usize;
            ::core::option::Option::None
        } else {
            self.back_idx = back_idx;
            TaskTypeIter::get(self, 5usize - self.back_idx)
        }
    }
}
impl ::core::iter::FusedIterator for TaskTypeIter {}
impl Clone for TaskTypeIter {
    fn clone(&self) -> TaskTypeIter {
        TaskTypeIter {
            idx: self.idx,
            back_idx: self.back_idx,
            marker: self.marker.clone(),
        }
    }
}
impl ::core::fmt::Display for TaskType {
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter,
    ) -> ::core::result::Result<(), ::core::fmt::Error> {
        match *self {
            TaskType::Waiting => ::core::fmt::Display::fmt("Waiting", f),
            TaskType::Queue => ::core::fmt::Display::fmt("Queue", f),
            TaskType::Aco => ::core::fmt::Display::fmt("Aco", f),
            TaskType::Login => ::core::fmt::Display::fmt("Login", f),
            TaskType::Register => ::core::fmt::Display::fmt("Register", f),
        }
    }
}
pub struct Keywords {
    pub raw: String,
    pub keywords: Vec<String>,
    pub accurate_keywords: Vec<String>,
    pub number_keywords: HashSet<i64>,
    pub negative_keywords: Vec<String>,
    pub relative_number_keywords: Vec<(i64, i64)>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Keywords {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "raw",
            "keywords",
            "accurate_keywords",
            "number_keywords",
            "negative_keywords",
            "relative_number_keywords",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &self.raw,
            &self.keywords,
            &self.accurate_keywords,
            &self.number_keywords,
            &self.negative_keywords,
            &&self.relative_number_keywords,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(f, "Keywords", names, values)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Keywords {
    #[inline]
    fn clone(&self) -> Keywords {
        Keywords {
            raw: ::core::clone::Clone::clone(&self.raw),
            keywords: ::core::clone::Clone::clone(&self.keywords),
            accurate_keywords: ::core::clone::Clone::clone(&self.accurate_keywords),
            number_keywords: ::core::clone::Clone::clone(&self.number_keywords),
            negative_keywords: ::core::clone::Clone::clone(&self.negative_keywords),
            relative_number_keywords: ::core::clone::Clone::clone(
                &self.relative_number_keywords,
            ),
        }
    }
}
#[automatically_derived]
impl ::core::default::Default for Keywords {
    #[inline]
    fn default() -> Keywords {
        Keywords {
            raw: ::core::default::Default::default(),
            keywords: ::core::default::Default::default(),
            accurate_keywords: ::core::default::Default::default(),
            number_keywords: ::core::default::Default::default(),
            negative_keywords: ::core::default::Default::default(),
            relative_number_keywords: ::core::default::Default::default(),
        }
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
    impl<'de> _serde::Deserialize<'de> for Keywords {
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
                __field2,
                __field3,
                __field4,
                __field5,
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
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        4u64 => _serde::__private::Ok(__Field::__field4),
                        5u64 => _serde::__private::Ok(__Field::__field5),
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
                        "raw" => _serde::__private::Ok(__Field::__field0),
                        "keywords" => _serde::__private::Ok(__Field::__field1),
                        "accurate_keywords" => _serde::__private::Ok(__Field::__field2),
                        "number_keywords" => _serde::__private::Ok(__Field::__field3),
                        "negative_keywords" => _serde::__private::Ok(__Field::__field4),
                        "relative_number_keywords" => {
                            _serde::__private::Ok(__Field::__field5)
                        }
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
                        b"raw" => _serde::__private::Ok(__Field::__field0),
                        b"keywords" => _serde::__private::Ok(__Field::__field1),
                        b"accurate_keywords" => _serde::__private::Ok(__Field::__field2),
                        b"number_keywords" => _serde::__private::Ok(__Field::__field3),
                        b"negative_keywords" => _serde::__private::Ok(__Field::__field4),
                        b"relative_number_keywords" => {
                            _serde::__private::Ok(__Field::__field5)
                        }
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
                marker: _serde::__private::PhantomData<Keywords>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Keywords;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct Keywords",
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
                                    &"struct Keywords with 6 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        Vec<String>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Keywords with 6 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        Vec<String>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Keywords with 6 elements",
                                ),
                            );
                        }
                    };
                    let __field3 = match _serde::de::SeqAccess::next_element::<
                        HashSet<i64>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct Keywords with 6 elements",
                                ),
                            );
                        }
                    };
                    let __field4 = match _serde::de::SeqAccess::next_element::<
                        Vec<String>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    4usize,
                                    &"struct Keywords with 6 elements",
                                ),
                            );
                        }
                    };
                    let __field5 = match _serde::de::SeqAccess::next_element::<
                        Vec<(i64, i64)>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    5usize,
                                    &"struct Keywords with 6 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(Keywords {
                        raw: __field0,
                        keywords: __field1,
                        accurate_keywords: __field2,
                        number_keywords: __field3,
                        negative_keywords: __field4,
                        relative_number_keywords: __field5,
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
                    let mut __field1: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                    let mut __field3: _serde::__private::Option<HashSet<i64>> = _serde::__private::None;
                    let mut __field4: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                    let mut __field5: _serde::__private::Option<Vec<(i64, i64)>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("raw"),
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
                                            "keywords",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Vec<String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "accurate_keywords",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Vec<String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field3 => {
                                if _serde::__private::Option::is_some(&__field3) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "number_keywords",
                                        ),
                                    );
                                }
                                __field3 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        HashSet<i64>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field4 => {
                                if _serde::__private::Option::is_some(&__field4) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "negative_keywords",
                                        ),
                                    );
                                }
                                __field4 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Vec<String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field5 => {
                                if _serde::__private::Option::is_some(&__field5) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "relative_number_keywords",
                                        ),
                                    );
                                }
                                __field5 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Vec<(i64, i64)>,
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
                            _serde::__private::de::missing_field("raw")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("keywords")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("accurate_keywords")?
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private::Some(__field3) => __field3,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("number_keywords")?
                        }
                    };
                    let __field4 = match __field4 {
                        _serde::__private::Some(__field4) => __field4,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("negative_keywords")?
                        }
                    };
                    let __field5 = match __field5 {
                        _serde::__private::Some(__field5) => __field5,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field(
                                "relative_number_keywords",
                            )?
                        }
                    };
                    _serde::__private::Ok(Keywords {
                        raw: __field0,
                        keywords: __field1,
                        accurate_keywords: __field2,
                        number_keywords: __field3,
                        negative_keywords: __field4,
                        relative_number_keywords: __field5,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &[
                "raw",
                "keywords",
                "accurate_keywords",
                "number_keywords",
                "negative_keywords",
                "relative_number_keywords",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Keywords",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Keywords>,
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
    impl _serde::Serialize for Keywords {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "Keywords",
                false as usize + 1 + 1 + 1 + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "raw",
                &self.raw,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "keywords",
                &self.keywords,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "accurate_keywords",
                &self.accurate_keywords,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "number_keywords",
                &self.number_keywords,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "negative_keywords",
                &self.negative_keywords,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "relative_number_keywords",
                &self.relative_number_keywords,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Keywords {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Keywords {
    #[inline]
    fn eq(&self, other: &Keywords) -> bool {
        self.raw == other.raw && self.keywords == other.keywords
            && self.accurate_keywords == other.accurate_keywords
            && self.number_keywords == other.number_keywords
            && self.negative_keywords == other.negative_keywords
            && self.relative_number_keywords == other.relative_number_keywords
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Keywords {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<String>;
        let _: ::core::cmp::AssertParamIsEq<Vec<String>>;
        let _: ::core::cmp::AssertParamIsEq<Vec<String>>;
        let _: ::core::cmp::AssertParamIsEq<HashSet<i64>>;
        let _: ::core::cmp::AssertParamIsEq<Vec<String>>;
        let _: ::core::cmp::AssertParamIsEq<Vec<(i64, i64)>>;
    }
}
impl Keywords {
    pub fn new(keywords_string: String) -> Self {
        ::core::panicking::panic("not implemented")
    }
}
pub fn deserialize_string_to_number<'de, D, T>(
    deserializer: D,
) -> std::result::Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    <T as FromStr>::Err: std::fmt::Display,
{
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Number(T),
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
        impl<'de, T> _serde::Deserialize<'de> for StringOrInt<T>
        where
            T: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                let __content = <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                    __deserializer,
                )?;
                let __deserializer = _serde::__private::de::ContentRefDeserializer::<
                    __D::Error,
                >::new(&__content);
                if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                    <String as _serde::Deserialize>::deserialize(__deserializer),
                    StringOrInt::String,
                ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                    <T as _serde::Deserialize>::deserialize(__deserializer),
                    StringOrInt::Number,
                ) {
                    return _serde::__private::Ok(__ok);
                }
                _serde::__private::Err(
                    _serde::de::Error::custom(
                        "data did not match any variant of untagged enum StringOrInt",
                    ),
                )
            }
        }
    };
    match StringOrInt::<T>::deserialize(deserializer)? {
        StringOrInt::String(s) => {
            let s = s.trim();
            s.parse::<T>()
                .map_err(|e| {
                    serde::de::Error::custom(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("{0}, {1} can not parse to number", e, s),
                            );
                            res
                        }),
                    )
                })
        }
        StringOrInt::Number(i) => Ok(i),
    }
}
pub fn deserialize_keyword_from_string<'de, D>(
    deserializer: D,
) -> std::result::Result<Keywords, D::Error>
where
    D: Deserializer<'de>,
{
    match Value::deserialize(deserializer)? {
        Value::String(s) => Ok(Keywords::new(s)),
        Value::Null => Ok(Keywords::default()),
        _ => {
            Err(
                serde::de::Error::custom(
                    "field format error, can not parsed to keywords",
                ),
            )
        }
    }
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Sync + Send>>;
fn main() {
    {
        ::std::io::_print(format_args!("Hello, world!\n"));
    };
    let ordered_fields = MelonParam::get_ordered_fields();
    {
        ::std::io::_print(format_args!("=> {0:#?}\n", ordered_fields));
    };
    let fields_with_priority = MelonParam::get_fields_with_priority();
    {
        ::std::io::_print(format_args!("=> {0:#?}\n", fields_with_priority));
    };
}
