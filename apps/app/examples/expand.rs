#![feature(print_internals)]
#![feature(panic_internals)]
#![feature(alloc)]
#![feature(fmt_helpers_for_derive)]
#![allow(warnings, unused)]
#![feature(hint_must_use)]
#![feature(liballoc_internals)]
// controller AppController []
// >>Push: Global("app") -- [None]
//  CMETA: []
// >>Push: Module("AppModule") -- [None]
// >>Push: Service("AppController") -- [Some(String("AppModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
//  CMETA: ["ControllerPath"]
// service_derive "AppController"
// >>Push: Handler("get_hello_world") -- [Some(String("AppModule"))]
//  CMETA: ["datasets::role::Role::User"]
//  CMETA: ["RouterName"]
//  CMETA: ["RouterMethod"]
//  CMETA: ["RouterPath"]
// route_derive "get_hello_world"
// route_derive is_tuple true
// << Pop: Some(Handler("get_hello_world")) ["RouterMethod", "RouterPath", "handler", "datasets::role::Role::User", "RouterName", "ControllerPath", "ServiceType", "ServiceName", "service", "module", "global"]

// << Pop: Some(Service("AppController")) ["ControllerPath", "ServiceType", "ServiceName", "service", "module", "global"]

// >>Push: Service("AppService") -- [Some(String("AppModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "AppService"
// module "AppModule"
// << Pop: Some(Service("AppService")) ["service", "ServiceType", "ServiceName", "module", "global"]

// >>Push: Service("DownlogEntity") -- [Some(String("AppModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "DownlogEntity"
// << Pop: Some(Service("DownlogEntity")) ["ServiceName", "ServiceType", "service", "module", "global"]

// >>Push: Service("ResourceEntity") -- [Some(String("AppModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "ResourceEntity"
// << Pop: Some(Service("ResourceEntity")) ["service", "ServiceName", "ServiceType", "module", "global"]

// >>Push: Service("RoomEntity") -- [Some(String("AppModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "RoomEntity"
// << Pop: Some(Service("RoomEntity")) ["service", "ServiceType", "ServiceName", "module", "global"]

// >>Push: Service("UserEntity") -- [Some(String("AppModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "UserEntity"
// << Pop: Some(Service("UserEntity")) ["ServiceName", "service", "ServiceType", "module", "global"]

// >>Push: Service("UserExtraEntity") -- [Some(String("AppModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "UserExtraEntity"
// controller UserController []
// << Pop: Some(Service("UserExtraEntity")) ["service", "ServiceName", "ServiceType", "module", "global"]

// << Pop: Some(Module("AppModule")) ["module", "global"]

// >>Push: Module("UserModule") -- [None]
// >>Push: Service("UserController") -- [Some(String("UserModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
//  CMETA: ["ControllerPath"]
// service_derive "UserController"
// >>Push: Handler("register") -- [Some(String("UserModule"))]
//  CMETA: ["disable_auto_json"]
//  CMETA: ["RouterIn", "RouterOut"]
//  CMETA: ["RouterName"]
//  CMETA: ["RouterMethod"]
//  CMETA: ["RouterPath"]
// route_derive "register"
// route_derive is_tuple false
// << Pop: Some(Handler("register")) ["RouterName", "RouterIn", "handler", "RouterMethod", "RouterPath", "disable_auto_json", "RouterOut", "ServiceType", "service", "ServiceName", "ControllerPath", "module", "global"]

// << Pop: Some(Service("UserController")) ["ServiceType", "service", "ServiceName", "ControllerPath", "module", "global"]

// >>Push: Service("UserService") -- [Some(String("UserModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "UserService"
// module "UserModule"
// << Pop: Some(Service("UserService")) ["ServiceName", "service", "ServiceType", "module", "global"]

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod app {
    use nidrs::macros::module;
    use nidrs_diesel::{DieselModule, DieselOptions, PostgresPoolManager};
    pub mod controller {
        use std::collections::HashMap;
        use macros::user;
        use nidrs::externs::axum::{extract::Query, response::AppendHeaders, Json};
        use nidrs::macros::{controller, get, post};
        use nidrs::{Inject, Meta};
        use crate::AppResult;
        use super::{dto::Status, service::AppService};
        pub struct AppController {
            app_service: Inject<AppService>,
        }
        #[automatically_derived]
        impl ::core::default::Default for AppController {
            #[inline]
            fn default() -> AppController {
                AppController {
                    app_service: ::core::default::Default::default(),
                }
            }
        }
        impl nidrs::Controller for AppController {}
        impl nidrs::Service for AppController {
            fn inject(
                &self,
                ctx: nidrs::ModuleCtx,
                module_name: &str,
            ) -> nidrs::ModuleCtx {
                let service = ctx.get_service::<AppService>(&module_name, "AppService");
                self.app_service.inject(service.clone());
                ctx
            }
        }
        impl nidrs::ImplMeta for AppController {
            fn __meta() -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set_data(nidrs::datasets::ControllerPath::from(""));
                meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                meta.set_data(nidrs::datasets::ServiceName::from("AppController"));
                meta.set("service", "AppController");
                meta.set("module", "AppModule");
                meta.set("global", "app");
                meta
            }
        }
        impl AppController {
            pub async fn get_hello_world(
                &self,
                meta: Meta,
                Query(q): Query<HashMap<String, String>>,
            ) -> AppResult<(AppendHeaders<[(String, String); 2]>, Status)> {
                {
                    ::std::io::_print(format_args!("Query {0:?}\n", q));
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Meta {0:?}\n",
                            meta.get_data::<datasets::role::Role>(),
                        ),
                    );
                };
                Ok((
                    AppendHeaders([
                        ("X-Custom-Header".to_string(), "hello".to_string()),
                        ("X-Custom-Header".to_string(), "world".to_string()),
                    ]),
                    Status {
                        db: "ok".to_string(),
                        redis: "ok".to_string(),
                    },
                ))
            }
            pub fn __meta_get_hello_world(&self) -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set_data(nidrs::datasets::RouterMethod::from("get"));
                meta.set_data(nidrs::datasets::RouterPath::from("/hello"));
                meta.set("handler", "get_hello_world");
                meta.set_data(datasets::role::Role::User);
                meta.set_data(nidrs::datasets::RouterName::from("get_hello_world"));
                meta.set_data(nidrs::datasets::ControllerPath::from(""));
                meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                meta.set_data(nidrs::datasets::ServiceName::from("AppController"));
                meta.set("service", "AppController");
                meta.set("module", "AppModule");
                meta.set("global", "app");
                meta
            }
            pub fn __route_get_hello_world(
                &self,
                mut ctx: nidrs::ModuleCtx,
            ) -> nidrs::ModuleCtx {
                use nidrs::externs::axum;
                use axum::response::IntoResponse;
                use nidrs::externs::axum::{extract::Query, Json};
                use nidrs::externs::meta::{InnerMeta, Meta};
                use nidrs::Interceptor;
                use serde_json::Value;
                let mut meta = self.__meta_get_hello_world();
                let router_info = ctx.get_router_full(&meta);
                if let Err(e) = router_info {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("[{0}] {1:?}", "__route_get_hello_world", e),
                        );
                    };
                }
                let full_path = router_info.unwrap();
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Registering router \'{0} {1}\'.\n",
                            "get".to_uppercase(),
                            full_path,
                        ),
                    );
                };
                meta.set_data(nidrs::datasets::RouterFullPath(full_path.clone()));
                let meta = Meta::new(meta);
                let module_name = meta.get::<&str>("module").unwrap();
                let controller_name = meta
                    .get_data::<nidrs::datasets::ServiceName>()
                    .unwrap()
                    .value();
                let t_controller = ctx
                    .get_controller::<Self>(module_name, controller_name);
                let router = nidrs::externs::axum::Router::new()
                    .route(
                        &full_path,
                        nidrs::externs::axum::routing::get(|p0, p1| async move {
                            let r = t_controller.get_hello_world(p0, p1).await;
                            r
                        }),
                    )
                    .route_layer(nidrs::externs::axum::Extension(meta.clone()));
                ctx.routers.push(nidrs::MetaRouter::new(router, meta));
                ctx
            }
        }
    }
    pub mod dto {
        use nidrs::externs::axum::{
            body::Body, http::{header, StatusCode},
            response::{IntoResponse, Response},
        };
        use nidrs::externs::serde::{Deserialize, Serialize};
        use nidrs::externs::serde_json;
        pub struct Status {
            pub db: String,
            pub redis: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Status {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Status",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "db",
                        &self.db,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "redis",
                        &self.redis,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Status {
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
                                "db" => _serde::__private::Ok(__Field::__field0),
                                "redis" => _serde::__private::Ok(__Field::__field1),
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
                                b"db" => _serde::__private::Ok(__Field::__field0),
                                b"redis" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
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
                        marker: _serde::__private::PhantomData<Status>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Status;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Status",
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
                                            &"struct Status with 2 elements",
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
                                            &"struct Status with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Status {
                                db: __field0,
                                redis: __field1,
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
                                                <__A::Error as _serde::de::Error>::duplicate_field("db"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("redis"),
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
                                    _serde::__private::de::missing_field("db")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("redis")?
                                }
                            };
                            _serde::__private::Ok(Status {
                                db: __field0,
                                redis: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["db", "redis"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Status",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Status>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Status {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Status",
                    "db",
                    &self.db,
                    "redis",
                    &&self.redis,
                )
            }
        }
        impl IntoResponse for Status {
            fn into_response(self) -> Response {
                let json_body = match serde_json::to_string(&self) {
                    Ok(json) => json,
                    Err(_) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body("Internal server error".into())
                            .unwrap();
                    }
                };
                let res: Response<Body> = Response::builder()
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(json_body.into())
                    .unwrap();
                res
            }
        }
    }
    pub mod exception {
        pub enum AppException {
            ServiceException(String),
        }
    }
    pub mod service {
        use nidrs::macros::injectable;
        use nidrs::Inject;
        pub struct AppService {}
        #[automatically_derived]
        impl ::core::default::Default for AppService {
            #[inline]
            fn default() -> AppService {
                AppService {}
            }
        }
        impl nidrs::Service for AppService {
            fn inject(
                &self,
                ctx: nidrs::ModuleCtx,
                module_name: &str,
            ) -> nidrs::ModuleCtx {
                ctx
            }
        }
        impl nidrs::ImplMeta for AppService {
            fn __meta() -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set("service", "AppService");
                meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                meta.set_data(nidrs::datasets::ServiceName::from("AppService"));
                meta.set("module", "AppModule");
                meta.set("global", "app");
                meta
            }
        }
        impl AppService {}
    }
    use crate::modules::user::UserModule;
    use controller::AppController;
    use service::AppService;
    pub struct AppModule;
    #[automatically_derived]
    impl ::core::default::Default for AppModule {
        #[inline]
        fn default() -> AppModule {
            AppModule {}
        }
    }
    impl nidrs::Module for AppModule {
        fn init(self, mut ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
            use nidrs::{
                Service, Controller, Interceptor, InterCtx, InterceptorHandler,
                ModuleCtx, StateCtx, ImplMeta,
            };
            if ctx.modules.contains_key("AppModule") {
                return ctx;
            }
            {
                ::std::io::_print(
                    format_args!(
                        "{0} ",
                        nidrs_extern::colored::Colorize::green("[nidrs]"),
                    ),
                );
            };
            {
                ::std::io::_print(
                    format_args!("Registering module {0}.\n", "AppModule"),
                );
            };
            ctx.modules.insert("AppModule".to_string(), Box::new(self));
            ctx.imports
                .insert(
                    "AppModule".to_string(),
                    Vec::from(["DieselModule".to_string(), "UserModule".to_string()]),
                );
            ctx.append_exports("AppModule", Vec::from(["AppService"]), false);
            if ctx
                .register_controller(
                    "AppModule",
                    "AppController",
                    Box::new(std::sync::Arc::new(controller::AppController::default())),
                )
            {
                let t_controller = ctx
                    .get_controller::<
                        controller::AppController,
                    >("AppModule", "AppController");
                ctx = t_controller.__route_get_hello_world(ctx);
            }
            let svc = std::sync::Arc::new(AppService::default());
            ctx.register_service("AppModule", "AppService", Box::new(svc));
            let mut dyn_module = DieselModule::for_root(DieselOptions {
                driver: PostgresPoolManager::new(std::env::var("DATABASE_URL").unwrap()),
            });
            let mut dyn_module_wrap = dyn_module.module.take().unwrap();
            let mut dyn_module_services = dyn_module.services;
            dyn_module_services
                .drain()
                .for_each(|(k, v)| {
                    ctx.register_service("DieselModule", &k, v);
                });
            let mut dyn_module_exports = dyn_module.exports;
            ctx.append_exports(
                "DieselModule",
                dyn_module_exports,
                nidrs::get_meta_by_type::<DieselModule>()
                    .get_data::<nidrs::datasets::Global>()
                    .unwrap_or(&nidrs::datasets::Global(false))
                    .value(),
            );
            let mut ctx = dyn_module_wrap.init(ctx);
            let mut ctx = UserModule::default().init(ctx);
            let t = ctx.get_service::<AppService>("AppModule", "AppService");
            {
                ::std::io::_print(
                    format_args!(
                        "{0} ",
                        nidrs_extern::colored::Colorize::green("[nidrs]"),
                    ),
                );
            };
            {
                ::std::io::_print(
                    format_args!("Injecting {0}::{1}.\n", "AppModule", "AppService"),
                );
            };
            let ctx = t.inject(ctx, &"AppModule");
            let t = ctx.get_controller::<AppController>("AppModule", "AppController");
            {
                ::std::io::_print(
                    format_args!(
                        "{0} ",
                        nidrs_extern::colored::Colorize::green("[nidrs]"),
                    ),
                );
            };
            {
                ::std::io::_print(
                    format_args!("Injecting {0}::{1}.\n", "AppModule", "AppController"),
                );
            };
            let ctx = t.inject(ctx, &"AppModule");
            ctx
        }
        fn destroy(&self, ctx: &nidrs::ModuleCtx) {
            {
                ::std::io::_print(
                    format_args!(
                        "{0} ",
                        nidrs_extern::colored::Colorize::green("[nidrs]"),
                    ),
                );
            };
            {
                ::std::io::_print(format_args!("Destroying module {0}.\n", "AppModule"));
            };
        }
    }
    impl nidrs::ImplMeta for AppModule {
        fn __meta() -> nidrs::InnerMeta {
            let mut meta = nidrs::InnerMeta::new();
            meta.set("service", "AppService");
            meta.set_data(nidrs::datasets::ServiceType::from("Service"));
            meta.set_data(nidrs::datasets::ServiceName::from("AppService"));
            meta.set("module", "AppModule");
            meta.set("global", "app");
            meta
        }
    }
}
mod models {
    pub mod dao {
        pub mod downlogs {
            use crate::models::schema::downlogs;
            use chrono::NaiveDateTime;
            use nidrs::{injectable, openapi::schema, AppResult, Inject};
            use nidrs_diesel::{PoolManager, PostgresPoolManager};
            use serde::{Deserialize, Serialize};
            use diesel::{connection::LoadConnection, prelude::*};
            #[diesel(table_name = downlogs)]
            #[diesel(check_for_backend(diesel::pg::Pg))]
            pub struct Downlog {
                pub id: i32,
                pub resource_id: i32,
                pub user_id: i32,
                pub status: i32,
                pub created_at: NaiveDateTime,
            }
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::expression::Selectable;
                impl<__DB: diesel::backend::Backend> Selectable<__DB> for Downlog {
                    type SelectExpression = (
                        downlogs::id,
                        downlogs::resource_id,
                        downlogs::user_id,
                        downlogs::status,
                        downlogs::created_at,
                    );
                    fn construct_selection() -> Self::SelectExpression {
                        (
                            downlogs::id,
                            downlogs::resource_id,
                            downlogs::user_id,
                            downlogs::status,
                            downlogs::created_at,
                        )
                    }
                }
                fn _check_field_compatibility<__DB: diesel::backend::Backend>()
                where
                    i32: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<downlogs::id>,
                        diesel::pg::Pg,
                    >,
                    i32: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<downlogs::resource_id>,
                        diesel::pg::Pg,
                    >,
                    i32: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<downlogs::user_id>,
                        diesel::pg::Pg,
                    >,
                    i32: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<downlogs::status>,
                        diesel::pg::Pg,
                    >,
                    NaiveDateTime: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<downlogs::created_at>,
                        diesel::pg::Pg,
                    >,
                {}
            };
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::deserialize::{self, FromStaticSqlRow, Queryable};
                use diesel::row::{Row as _, Field as _};
                use std::convert::TryInto;
                impl<
                    __DB: diesel::backend::Backend,
                    __ST0,
                    __ST1,
                    __ST2,
                    __ST3,
                    __ST4,
                > Queryable<(__ST0, __ST1, __ST2, __ST3, __ST4), __DB> for Downlog
                where
                    (
                        i32,
                        i32,
                        i32,
                        i32,
                        NaiveDateTime,
                    ): FromStaticSqlRow<(__ST0, __ST1, __ST2, __ST3, __ST4), __DB>,
                {
                    type Row = (i32, i32, i32, i32, NaiveDateTime);
                    fn build(row: Self::Row) -> deserialize::Result<Self> {
                        Ok(Self {
                            id: row.0.try_into()?,
                            resource_id: row.1.try_into()?,
                            user_id: row.2.try_into()?,
                            status: row.3.try_into()?,
                            created_at: row.4.try_into()?,
                        })
                    }
                }
            };
            #[automatically_derived]
            impl ::core::fmt::Debug for Downlog {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field5_finish(
                        f,
                        "Downlog",
                        "id",
                        &self.id,
                        "resource_id",
                        &self.resource_id,
                        "user_id",
                        &self.user_id,
                        "status",
                        &self.status,
                        "created_at",
                        &&self.created_at,
                    )
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for Downlog {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Downlog",
                            false as usize + 1 + 1 + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "id",
                            &self.id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "resource_id",
                            &self.resource_id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "user_id",
                            &self.user_id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "status",
                            &self.status,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "created_at",
                            &self.created_at,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            impl utoipa::IntoParams for Downlog {
                fn into_params(
                    parameter_in_provider: impl Fn(
                    ) -> Option<utoipa::openapi::path::ParameterIn>,
                ) -> Vec<utoipa::openapi::path::Parameter> {
                    [
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("id")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::Integer)
                                        .format(
                                            Some(
                                                utoipa::openapi::SchemaFormat::KnownFormat(
                                                    utoipa::openapi::KnownFormat::Int32,
                                                ),
                                            ),
                                        ),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("resource_id")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::Integer)
                                        .format(
                                            Some(
                                                utoipa::openapi::SchemaFormat::KnownFormat(
                                                    utoipa::openapi::KnownFormat::Int32,
                                                ),
                                            ),
                                        ),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("user_id")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::Integer)
                                        .format(
                                            Some(
                                                utoipa::openapi::SchemaFormat::KnownFormat(
                                                    utoipa::openapi::KnownFormat::Int32,
                                                ),
                                            ),
                                        ),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("status")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::Integer)
                                        .format(
                                            Some(
                                                utoipa::openapi::SchemaFormat::KnownFormat(
                                                    utoipa::openapi::KnownFormat::Int32,
                                                ),
                                            ),
                                        ),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("created_at")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::Ref::from_schema_name("NaiveDateTime"),
                                ),
                            )
                            .build(),
                    ]
                        .to_vec()
                }
            }
            impl<'__s> utoipa::ToSchema<'__s> for Downlog {
                fn schema() -> (
                    &'__s str,
                    utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
                ) {
                    (
                        "Downlog",
                        utoipa::openapi::ObjectBuilder::new()
                            .property(
                                "id",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Integer)
                                    .format(
                                        Some(
                                            utoipa::openapi::SchemaFormat::KnownFormat(
                                                utoipa::openapi::KnownFormat::Int32,
                                            ),
                                        ),
                                    ),
                            )
                            .required("id")
                            .property(
                                "resource_id",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Integer)
                                    .format(
                                        Some(
                                            utoipa::openapi::SchemaFormat::KnownFormat(
                                                utoipa::openapi::KnownFormat::Int32,
                                            ),
                                        ),
                                    ),
                            )
                            .required("resource_id")
                            .property(
                                "user_id",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Integer)
                                    .format(
                                        Some(
                                            utoipa::openapi::SchemaFormat::KnownFormat(
                                                utoipa::openapi::KnownFormat::Int32,
                                            ),
                                        ),
                                    ),
                            )
                            .required("user_id")
                            .property(
                                "status",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Integer)
                                    .format(
                                        Some(
                                            utoipa::openapi::SchemaFormat::KnownFormat(
                                                utoipa::openapi::KnownFormat::Int32,
                                            ),
                                        ),
                                    ),
                            )
                            .required("status")
                            .property(
                                "created_at",
                                utoipa::openapi::Ref::from_schema_name("NaiveDateTime"),
                            )
                            .required("created_at")
                            .into(),
                    )
                }
            }
            impl nidrs::openapi::ToParamDto for Downlog {
                fn to_param_dto(
                    dto_type: nidrs::openapi::ParamDtoIn,
                ) -> nidrs::openapi::ParamDto {
                    use nidrs::openapi::utoipa::IntoParams;
                    use nidrs::openapi::utoipa::ToSchema;
                    match dto_type {
                        nidrs::openapi::ParamDtoIn::Param(p) => {
                            nidrs::openapi::ParamDto::ParamList(
                                Self::into_params(|| Some(p.clone())),
                            )
                        }
                        nidrs::openapi::ParamDtoIn::Body => {
                            nidrs::openapi::ParamDto::BodySchema(Self::schema())
                        }
                    }
                }
            }
            #[diesel(table_name = downlogs)]
            pub struct CreateDownlog {
                pub resource_id: i32,
                pub user_id: i32,
                pub status: i32,
            }
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::insertable::Insertable;
                use diesel::internal::derives::insertable::UndecoratedInsertRecord;
                use diesel::prelude::*;
                #[allow(unused_qualifications)]
                impl Insertable<downlogs::table> for CreateDownlog {
                    type Values = <(
                        std::option::Option<diesel::dsl::Eq<downlogs::resource_id, i32>>,
                        std::option::Option<diesel::dsl::Eq<downlogs::user_id, i32>>,
                        std::option::Option<diesel::dsl::Eq<downlogs::status, i32>>,
                    ) as Insertable<downlogs::table>>::Values;
                    fn values(
                        self,
                    ) -> <(
                        std::option::Option<diesel::dsl::Eq<downlogs::resource_id, i32>>,
                        std::option::Option<diesel::dsl::Eq<downlogs::user_id, i32>>,
                        std::option::Option<diesel::dsl::Eq<downlogs::status, i32>>,
                    ) as Insertable<downlogs::table>>::Values {
                        (
                            std::option::Option::Some(
                                downlogs::resource_id.eq(self.resource_id),
                            ),
                            std::option::Option::Some(
                                downlogs::user_id.eq(self.user_id),
                            ),
                            std::option::Option::Some(downlogs::status.eq(self.status)),
                        )
                            .values()
                    }
                }
                #[allow(unused_qualifications)]
                impl<'insert> Insertable<downlogs::table> for &'insert CreateDownlog {
                    type Values = <(
                        std::option::Option<
                            diesel::dsl::Eq<downlogs::resource_id, &'insert i32>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<downlogs::user_id, &'insert i32>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<downlogs::status, &'insert i32>,
                        >,
                    ) as Insertable<downlogs::table>>::Values;
                    fn values(
                        self,
                    ) -> <(
                        std::option::Option<
                            diesel::dsl::Eq<downlogs::resource_id, &'insert i32>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<downlogs::user_id, &'insert i32>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<downlogs::status, &'insert i32>,
                        >,
                    ) as Insertable<downlogs::table>>::Values {
                        (
                            std::option::Option::Some(
                                downlogs::resource_id.eq(&self.resource_id),
                            ),
                            std::option::Option::Some(
                                downlogs::user_id.eq(&self.user_id),
                            ),
                            std::option::Option::Some(downlogs::status.eq(&self.status)),
                        )
                            .values()
                    }
                }
                impl UndecoratedInsertRecord<downlogs::table> for CreateDownlog {}
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for CreateDownlog {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "CreateDownlog",
                            false as usize + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "resource_id",
                            &self.resource_id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "user_id",
                            &self.user_id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "status",
                            &self.status,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for CreateDownlog {
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
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
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
                                    "resource_id" => _serde::__private::Ok(__Field::__field0),
                                    "user_id" => _serde::__private::Ok(__Field::__field1),
                                    "status" => _serde::__private::Ok(__Field::__field2),
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
                                    b"resource_id" => _serde::__private::Ok(__Field::__field0),
                                    b"user_id" => _serde::__private::Ok(__Field::__field1),
                                    b"status" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
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
                            marker: _serde::__private::PhantomData<CreateDownlog>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = CreateDownlog;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct CreateDownlog",
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
                                    i32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct CreateDownlog with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    i32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct CreateDownlog with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    i32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct CreateDownlog with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(CreateDownlog {
                                    resource_id: __field0,
                                    user_id: __field1,
                                    status: __field2,
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
                                let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<i32> = _serde::__private::None;
                                let mut __field2: _serde::__private::Option<i32> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "resource_id",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "user_id",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("status"),
                                                );
                                            }
                                            __field2 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
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
                                        _serde::__private::de::missing_field("resource_id")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("user_id")?
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("status")?
                                    }
                                };
                                _serde::__private::Ok(CreateDownlog {
                                    resource_id: __field0,
                                    user_id: __field1,
                                    status: __field2,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "resource_id",
                            "user_id",
                            "status",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "CreateDownlog",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<CreateDownlog>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            pub struct DownlogEntity {
                pool: Inject<PostgresPoolManager>,
            }
            #[automatically_derived]
            impl ::core::default::Default for DownlogEntity {
                #[inline]
                fn default() -> DownlogEntity {
                    DownlogEntity {
                        pool: ::core::default::Default::default(),
                    }
                }
            }
            impl nidrs::Service for DownlogEntity {
                fn inject(
                    &self,
                    ctx: nidrs::ModuleCtx,
                    module_name: &str,
                ) -> nidrs::ModuleCtx {
                    let service = ctx
                        .get_service::<
                            PostgresPoolManager,
                        >(&module_name, "PostgresPoolManager");
                    self.pool.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for DownlogEntity {
                fn __meta() -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set_data(nidrs::datasets::ServiceName::from("DownlogEntity"));
                    meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                    meta.set("service", "DownlogEntity");
                    meta.set("module", "AppModule");
                    meta.set("global", "app");
                    meta
                }
            }
            impl DownlogEntity {
                pub async fn all(&self) -> AppResult<Vec<Downlog>> {
                    self.pool
                        .query(|mut conn| downlogs::table.load::<Downlog>(&mut conn))
                        .await
                }
                pub async fn create(
                    &self,
                    new_downlog: CreateDownlog,
                ) -> AppResult<usize> {
                    self.pool
                        .query(move |mut conn| {
                            diesel::insert_into(downlogs::table)
                                .values(&new_downlog)
                                .execute(&mut conn)
                        })
                        .await
                }
                pub async fn update_status(
                    &self,
                    id: i32,
                    status: i32,
                ) -> AppResult<usize> {
                    self.pool
                        .query(move |mut conn| {
                            diesel::update(downlogs::table.find(id))
                                .set(downlogs::status.eq(status))
                                .execute(&mut conn)
                        })
                        .await
                }
                pub async fn find_by_id(&self, id: i32) -> AppResult<Downlog> {
                    self.pool
                        .query(move |mut conn| {
                            downlogs::table.find(id).first::<Downlog>(&mut conn)
                        })
                        .await
                }
                pub async fn find_by_user_id(
                    &self,
                    user_id: i32,
                ) -> AppResult<Vec<Downlog>> {
                    self.pool
                        .query(move |mut conn| {
                            downlogs::table
                                .filter(downlogs::user_id.eq(user_id))
                                .load::<Downlog>(&mut conn)
                        })
                        .await
                }
                pub async fn remove_by_id(&self, id: i32) -> AppResult<usize> {
                    self.pool
                        .query(move |mut conn| {
                            diesel::delete(downlogs::table.find(id)).execute(&mut conn)
                        })
                        .await
                }
            }
        }
        pub mod resources {
            use crate::models::schema::resources;
            use chrono::NaiveDateTime;
            use nidrs::{injectable, openapi::schema, AppResult, Inject};
            use nidrs_diesel::{PoolManager, PostgresPoolManager};
            use serde::{Deserialize, Serialize};
            use diesel::{connection::LoadConnection, prelude::*};
            #[diesel(table_name = resources)]
            #[diesel(check_for_backend(diesel::pg::Pg))]
            pub struct Resource {
                pub id: i32,
                pub room_id: i32,
                pub name: String,
                pub size: i32,
                pub key: String,
                pub length: i32,
                pub creator_id: i32,
                pub down_count: i32,
                pub blank: bool,
                pub created_at: NaiveDateTime,
                pub updated_at: NaiveDateTime,
                pub deleted_at: Option<NaiveDateTime>,
            }
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::expression::Selectable;
                impl<__DB: diesel::backend::Backend> Selectable<__DB> for Resource {
                    type SelectExpression = (
                        resources::id,
                        resources::room_id,
                        resources::name,
                        resources::size,
                        resources::key,
                        resources::length,
                        resources::creator_id,
                        resources::down_count,
                        resources::blank,
                        resources::created_at,
                        resources::updated_at,
                        resources::deleted_at,
                    );
                    fn construct_selection() -> Self::SelectExpression {
                        (
                            resources::id,
                            resources::room_id,
                            resources::name,
                            resources::size,
                            resources::key,
                            resources::length,
                            resources::creator_id,
                            resources::down_count,
                            resources::blank,
                            resources::created_at,
                            resources::updated_at,
                            resources::deleted_at,
                        )
                    }
                }
                fn _check_field_compatibility<__DB: diesel::backend::Backend>()
                where
                    i32: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<resources::id>,
                        diesel::pg::Pg,
                    >,
                    i32: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<resources::room_id>,
                        diesel::pg::Pg,
                    >,
                    String: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<resources::name>,
                        diesel::pg::Pg,
                    >,
                    i32: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<resources::size>,
                        diesel::pg::Pg,
                    >,
                    String: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<resources::key>,
                        diesel::pg::Pg,
                    >,
                    i32: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<resources::length>,
                        diesel::pg::Pg,
                    >,
                    i32: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<resources::creator_id>,
                        diesel::pg::Pg,
                    >,
                    i32: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<resources::down_count>,
                        diesel::pg::Pg,
                    >,
                    bool: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<resources::blank>,
                        diesel::pg::Pg,
                    >,
                    NaiveDateTime: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<resources::created_at>,
                        diesel::pg::Pg,
                    >,
                    NaiveDateTime: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<resources::updated_at>,
                        diesel::pg::Pg,
                    >,
                    Option<
                        NaiveDateTime,
                    >: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<resources::deleted_at>,
                        diesel::pg::Pg,
                    >,
                {}
            };
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::deserialize::{self, FromStaticSqlRow, Queryable};
                use diesel::row::{Row as _, Field as _};
                use std::convert::TryInto;
                impl<
                    __DB: diesel::backend::Backend,
                    __ST0,
                    __ST1,
                    __ST2,
                    __ST3,
                    __ST4,
                    __ST5,
                    __ST6,
                    __ST7,
                    __ST8,
                    __ST9,
                    __ST10,
                    __ST11,
                > Queryable<
                    (
                        __ST0,
                        __ST1,
                        __ST2,
                        __ST3,
                        __ST4,
                        __ST5,
                        __ST6,
                        __ST7,
                        __ST8,
                        __ST9,
                        __ST10,
                        __ST11,
                    ),
                    __DB,
                > for Resource
                where
                    (
                        i32,
                        i32,
                        String,
                        i32,
                        String,
                        i32,
                        i32,
                        i32,
                        bool,
                        NaiveDateTime,
                        NaiveDateTime,
                        Option<NaiveDateTime>,
                    ): FromStaticSqlRow<
                        (
                            __ST0,
                            __ST1,
                            __ST2,
                            __ST3,
                            __ST4,
                            __ST5,
                            __ST6,
                            __ST7,
                            __ST8,
                            __ST9,
                            __ST10,
                            __ST11,
                        ),
                        __DB,
                    >,
                {
                    type Row = (
                        i32,
                        i32,
                        String,
                        i32,
                        String,
                        i32,
                        i32,
                        i32,
                        bool,
                        NaiveDateTime,
                        NaiveDateTime,
                        Option<NaiveDateTime>,
                    );
                    fn build(row: Self::Row) -> deserialize::Result<Self> {
                        Ok(Self {
                            id: row.0.try_into()?,
                            room_id: row.1.try_into()?,
                            name: row.2.try_into()?,
                            size: row.3.try_into()?,
                            key: row.4.try_into()?,
                            length: row.5.try_into()?,
                            creator_id: row.6.try_into()?,
                            down_count: row.7.try_into()?,
                            blank: row.8.try_into()?,
                            created_at: row.9.try_into()?,
                            updated_at: row.10.try_into()?,
                            deleted_at: row.11.try_into()?,
                        })
                    }
                }
            };
            #[automatically_derived]
            impl ::core::fmt::Debug for Resource {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let names: &'static _ = &[
                        "id",
                        "room_id",
                        "name",
                        "size",
                        "key",
                        "length",
                        "creator_id",
                        "down_count",
                        "blank",
                        "created_at",
                        "updated_at",
                        "deleted_at",
                    ];
                    let values: &[&dyn ::core::fmt::Debug] = &[
                        &self.id,
                        &self.room_id,
                        &self.name,
                        &self.size,
                        &self.key,
                        &self.length,
                        &self.creator_id,
                        &self.down_count,
                        &self.blank,
                        &self.created_at,
                        &self.updated_at,
                        &&self.deleted_at,
                    ];
                    ::core::fmt::Formatter::debug_struct_fields_finish(
                        f,
                        "Resource",
                        names,
                        values,
                    )
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for Resource {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Resource",
                            false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                                + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "id",
                            &self.id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "room_id",
                            &self.room_id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "name",
                            &self.name,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "size",
                            &self.size,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "key",
                            &self.key,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "length",
                            &self.length,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "creator_id",
                            &self.creator_id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "down_count",
                            &self.down_count,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "blank",
                            &self.blank,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "created_at",
                            &self.created_at,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "updated_at",
                            &self.updated_at,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "deleted_at",
                            &self.deleted_at,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            impl utoipa::IntoParams for Resource {
                fn into_params(
                    parameter_in_provider: impl Fn(
                    ) -> Option<utoipa::openapi::path::ParameterIn>,
                ) -> Vec<utoipa::openapi::path::Parameter> {
                    [
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("id")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::Integer)
                                        .format(
                                            Some(
                                                utoipa::openapi::SchemaFormat::KnownFormat(
                                                    utoipa::openapi::KnownFormat::Int32,
                                                ),
                                            ),
                                        ),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("room_id")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::Integer)
                                        .format(
                                            Some(
                                                utoipa::openapi::SchemaFormat::KnownFormat(
                                                    utoipa::openapi::KnownFormat::Int32,
                                                ),
                                            ),
                                        ),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("name")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::String),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("size")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::Integer)
                                        .format(
                                            Some(
                                                utoipa::openapi::SchemaFormat::KnownFormat(
                                                    utoipa::openapi::KnownFormat::Int32,
                                                ),
                                            ),
                                        ),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("key")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::String),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("length")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::Integer)
                                        .format(
                                            Some(
                                                utoipa::openapi::SchemaFormat::KnownFormat(
                                                    utoipa::openapi::KnownFormat::Int32,
                                                ),
                                            ),
                                        ),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("creator_id")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::Integer)
                                        .format(
                                            Some(
                                                utoipa::openapi::SchemaFormat::KnownFormat(
                                                    utoipa::openapi::KnownFormat::Int32,
                                                ),
                                            ),
                                        ),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("down_count")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::Integer)
                                        .format(
                                            Some(
                                                utoipa::openapi::SchemaFormat::KnownFormat(
                                                    utoipa::openapi::KnownFormat::Int32,
                                                ),
                                            ),
                                        ),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("blank")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::Boolean),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("created_at")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::Ref::from_schema_name("NaiveDateTime"),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("updated_at")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::Ref::from_schema_name("NaiveDateTime"),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("deleted_at")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::False)
                            .schema(
                                Some(
                                    utoipa::openapi::schema::AllOfBuilder::new()
                                        .nullable(true)
                                        .item(
                                            utoipa::openapi::Ref::from_schema_name("NaiveDateTime"),
                                        ),
                                ),
                            )
                            .build(),
                    ]
                        .to_vec()
                }
            }
            impl<'__s> utoipa::ToSchema<'__s> for Resource {
                fn schema() -> (
                    &'__s str,
                    utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
                ) {
                    (
                        "Resource",
                        utoipa::openapi::ObjectBuilder::new()
                            .property(
                                "id",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Integer)
                                    .format(
                                        Some(
                                            utoipa::openapi::SchemaFormat::KnownFormat(
                                                utoipa::openapi::KnownFormat::Int32,
                                            ),
                                        ),
                                    ),
                            )
                            .required("id")
                            .property(
                                "room_id",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Integer)
                                    .format(
                                        Some(
                                            utoipa::openapi::SchemaFormat::KnownFormat(
                                                utoipa::openapi::KnownFormat::Int32,
                                            ),
                                        ),
                                    ),
                            )
                            .required("room_id")
                            .property(
                                "name",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::String),
                            )
                            .required("name")
                            .property(
                                "size",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Integer)
                                    .format(
                                        Some(
                                            utoipa::openapi::SchemaFormat::KnownFormat(
                                                utoipa::openapi::KnownFormat::Int32,
                                            ),
                                        ),
                                    ),
                            )
                            .required("size")
                            .property(
                                "key",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::String),
                            )
                            .required("key")
                            .property(
                                "length",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Integer)
                                    .format(
                                        Some(
                                            utoipa::openapi::SchemaFormat::KnownFormat(
                                                utoipa::openapi::KnownFormat::Int32,
                                            ),
                                        ),
                                    ),
                            )
                            .required("length")
                            .property(
                                "creator_id",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Integer)
                                    .format(
                                        Some(
                                            utoipa::openapi::SchemaFormat::KnownFormat(
                                                utoipa::openapi::KnownFormat::Int32,
                                            ),
                                        ),
                                    ),
                            )
                            .required("creator_id")
                            .property(
                                "down_count",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Integer)
                                    .format(
                                        Some(
                                            utoipa::openapi::SchemaFormat::KnownFormat(
                                                utoipa::openapi::KnownFormat::Int32,
                                            ),
                                        ),
                                    ),
                            )
                            .required("down_count")
                            .property(
                                "blank",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Boolean),
                            )
                            .required("blank")
                            .property(
                                "created_at",
                                utoipa::openapi::Ref::from_schema_name("NaiveDateTime"),
                            )
                            .required("created_at")
                            .property(
                                "updated_at",
                                utoipa::openapi::Ref::from_schema_name("NaiveDateTime"),
                            )
                            .required("updated_at")
                            .property(
                                "deleted_at",
                                utoipa::openapi::schema::AllOfBuilder::new()
                                    .nullable(true)
                                    .item(
                                        utoipa::openapi::Ref::from_schema_name("NaiveDateTime"),
                                    ),
                            )
                            .into(),
                    )
                }
            }
            impl nidrs::openapi::ToParamDto for Resource {
                fn to_param_dto(
                    dto_type: nidrs::openapi::ParamDtoIn,
                ) -> nidrs::openapi::ParamDto {
                    use nidrs::openapi::utoipa::IntoParams;
                    use nidrs::openapi::utoipa::ToSchema;
                    match dto_type {
                        nidrs::openapi::ParamDtoIn::Param(p) => {
                            nidrs::openapi::ParamDto::ParamList(
                                Self::into_params(|| Some(p.clone())),
                            )
                        }
                        nidrs::openapi::ParamDtoIn::Body => {
                            nidrs::openapi::ParamDto::BodySchema(Self::schema())
                        }
                    }
                }
            }
            #[diesel(table_name = resources)]
            pub struct CreateResource {
                pub room_id: i32,
                pub name: String,
                pub size: i32,
                pub key: String,
                pub length: i32,
                pub creator_id: i32,
                pub blank: bool,
            }
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::insertable::Insertable;
                use diesel::internal::derives::insertable::UndecoratedInsertRecord;
                use diesel::prelude::*;
                #[allow(unused_qualifications)]
                impl Insertable<resources::table> for CreateResource {
                    type Values = <(
                        std::option::Option<diesel::dsl::Eq<resources::room_id, i32>>,
                        std::option::Option<diesel::dsl::Eq<resources::name, String>>,
                        std::option::Option<diesel::dsl::Eq<resources::size, i32>>,
                        std::option::Option<diesel::dsl::Eq<resources::key, String>>,
                        std::option::Option<diesel::dsl::Eq<resources::length, i32>>,
                        std::option::Option<diesel::dsl::Eq<resources::creator_id, i32>>,
                        std::option::Option<diesel::dsl::Eq<resources::blank, bool>>,
                    ) as Insertable<resources::table>>::Values;
                    fn values(
                        self,
                    ) -> <(
                        std::option::Option<diesel::dsl::Eq<resources::room_id, i32>>,
                        std::option::Option<diesel::dsl::Eq<resources::name, String>>,
                        std::option::Option<diesel::dsl::Eq<resources::size, i32>>,
                        std::option::Option<diesel::dsl::Eq<resources::key, String>>,
                        std::option::Option<diesel::dsl::Eq<resources::length, i32>>,
                        std::option::Option<diesel::dsl::Eq<resources::creator_id, i32>>,
                        std::option::Option<diesel::dsl::Eq<resources::blank, bool>>,
                    ) as Insertable<resources::table>>::Values {
                        (
                            std::option::Option::Some(
                                resources::room_id.eq(self.room_id),
                            ),
                            std::option::Option::Some(resources::name.eq(self.name)),
                            std::option::Option::Some(resources::size.eq(self.size)),
                            std::option::Option::Some(resources::key.eq(self.key)),
                            std::option::Option::Some(resources::length.eq(self.length)),
                            std::option::Option::Some(
                                resources::creator_id.eq(self.creator_id),
                            ),
                            std::option::Option::Some(resources::blank.eq(self.blank)),
                        )
                            .values()
                    }
                }
                #[allow(unused_qualifications)]
                impl<'insert> Insertable<resources::table> for &'insert CreateResource {
                    type Values = <(
                        std::option::Option<
                            diesel::dsl::Eq<resources::room_id, &'insert i32>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<resources::name, &'insert String>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<resources::size, &'insert i32>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<resources::key, &'insert String>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<resources::length, &'insert i32>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<resources::creator_id, &'insert i32>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<resources::blank, &'insert bool>,
                        >,
                    ) as Insertable<resources::table>>::Values;
                    fn values(
                        self,
                    ) -> <(
                        std::option::Option<
                            diesel::dsl::Eq<resources::room_id, &'insert i32>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<resources::name, &'insert String>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<resources::size, &'insert i32>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<resources::key, &'insert String>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<resources::length, &'insert i32>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<resources::creator_id, &'insert i32>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<resources::blank, &'insert bool>,
                        >,
                    ) as Insertable<resources::table>>::Values {
                        (
                            std::option::Option::Some(
                                resources::room_id.eq(&self.room_id),
                            ),
                            std::option::Option::Some(resources::name.eq(&self.name)),
                            std::option::Option::Some(resources::size.eq(&self.size)),
                            std::option::Option::Some(resources::key.eq(&self.key)),
                            std::option::Option::Some(
                                resources::length.eq(&self.length),
                            ),
                            std::option::Option::Some(
                                resources::creator_id.eq(&self.creator_id),
                            ),
                            std::option::Option::Some(resources::blank.eq(&self.blank)),
                        )
                            .values()
                    }
                }
                impl UndecoratedInsertRecord<resources::table> for CreateResource {}
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for CreateResource {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "CreateResource",
                            false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "room_id",
                            &self.room_id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "name",
                            &self.name,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "size",
                            &self.size,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "key",
                            &self.key,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "length",
                            &self.length,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "creator_id",
                            &self.creator_id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "blank",
                            &self.blank,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for CreateResource {
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
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
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
                                    "room_id" => _serde::__private::Ok(__Field::__field0),
                                    "name" => _serde::__private::Ok(__Field::__field1),
                                    "size" => _serde::__private::Ok(__Field::__field2),
                                    "key" => _serde::__private::Ok(__Field::__field3),
                                    "length" => _serde::__private::Ok(__Field::__field4),
                                    "creator_id" => _serde::__private::Ok(__Field::__field5),
                                    "blank" => _serde::__private::Ok(__Field::__field6),
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
                                    b"room_id" => _serde::__private::Ok(__Field::__field0),
                                    b"name" => _serde::__private::Ok(__Field::__field1),
                                    b"size" => _serde::__private::Ok(__Field::__field2),
                                    b"key" => _serde::__private::Ok(__Field::__field3),
                                    b"length" => _serde::__private::Ok(__Field::__field4),
                                    b"creator_id" => _serde::__private::Ok(__Field::__field5),
                                    b"blank" => _serde::__private::Ok(__Field::__field6),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
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
                            marker: _serde::__private::PhantomData<CreateResource>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = CreateResource;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct CreateResource",
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
                                    i32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct CreateResource with 7 elements",
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
                                                &"struct CreateResource with 7 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    i32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct CreateResource with 7 elements",
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
                                                &"struct CreateResource with 7 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field4 = match _serde::de::SeqAccess::next_element::<
                                    i32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                4usize,
                                                &"struct CreateResource with 7 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field5 = match _serde::de::SeqAccess::next_element::<
                                    i32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                5usize,
                                                &"struct CreateResource with 7 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field6 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                6usize,
                                                &"struct CreateResource with 7 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(CreateResource {
                                    room_id: __field0,
                                    name: __field1,
                                    size: __field2,
                                    key: __field3,
                                    length: __field4,
                                    creator_id: __field5,
                                    blank: __field6,
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
                                let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field2: _serde::__private::Option<i32> = _serde::__private::None;
                                let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field4: _serde::__private::Option<i32> = _serde::__private::None;
                                let mut __field5: _serde::__private::Option<i32> = _serde::__private::None;
                                let mut __field6: _serde::__private::Option<bool> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "room_id",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("size"),
                                                );
                                            }
                                            __field2 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field3 => {
                                            if _serde::__private::Option::is_some(&__field3) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("key"),
                                                );
                                            }
                                            __field3 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field4 => {
                                            if _serde::__private::Option::is_some(&__field4) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("length"),
                                                );
                                            }
                                            __field4 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field5 => {
                                            if _serde::__private::Option::is_some(&__field5) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "creator_id",
                                                    ),
                                                );
                                            }
                                            __field5 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field6 => {
                                            if _serde::__private::Option::is_some(&__field6) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("blank"),
                                                );
                                            }
                                            __field6 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
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
                                        _serde::__private::de::missing_field("room_id")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("name")?
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("size")?
                                    }
                                };
                                let __field3 = match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("key")?
                                    }
                                };
                                let __field4 = match __field4 {
                                    _serde::__private::Some(__field4) => __field4,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("length")?
                                    }
                                };
                                let __field5 = match __field5 {
                                    _serde::__private::Some(__field5) => __field5,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("creator_id")?
                                    }
                                };
                                let __field6 = match __field6 {
                                    _serde::__private::Some(__field6) => __field6,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("blank")?
                                    }
                                };
                                _serde::__private::Ok(CreateResource {
                                    room_id: __field0,
                                    name: __field1,
                                    size: __field2,
                                    key: __field3,
                                    length: __field4,
                                    creator_id: __field5,
                                    blank: __field6,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "room_id",
                            "name",
                            "size",
                            "key",
                            "length",
                            "creator_id",
                            "blank",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "CreateResource",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<CreateResource>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            pub struct ResourceEntity {
                pool: Inject<PostgresPoolManager>,
            }
            #[automatically_derived]
            impl ::core::default::Default for ResourceEntity {
                #[inline]
                fn default() -> ResourceEntity {
                    ResourceEntity {
                        pool: ::core::default::Default::default(),
                    }
                }
            }
            impl nidrs::Service for ResourceEntity {
                fn inject(
                    &self,
                    ctx: nidrs::ModuleCtx,
                    module_name: &str,
                ) -> nidrs::ModuleCtx {
                    let service = ctx
                        .get_service::<
                            PostgresPoolManager,
                        >(&module_name, "PostgresPoolManager");
                    self.pool.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for ResourceEntity {
                fn __meta() -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set("service", "ResourceEntity");
                    meta.set_data(nidrs::datasets::ServiceName::from("ResourceEntity"));
                    meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                    meta.set("module", "AppModule");
                    meta.set("global", "app");
                    meta
                }
            }
            impl ResourceEntity {
                pub async fn all(&self) -> AppResult<Vec<Resource>> {
                    self.pool
                        .query(|mut conn| resources::table.load::<Resource>(&mut conn))
                        .await
                }
                pub async fn create(
                    &self,
                    new_resource: CreateResource,
                ) -> AppResult<usize> {
                    self.pool
                        .query(move |mut conn| {
                            diesel::insert_into(resources::table)
                                .values(&new_resource)
                                .execute(&mut conn)
                        })
                        .await
                }
                pub async fn update(&self, id: i32, name: String) -> AppResult<usize> {
                    self.pool
                        .query(move |mut conn| {
                            diesel::update(resources::table.find(id))
                                .set(resources::name.eq(name))
                                .execute(&mut conn)
                        })
                        .await
                }
                pub async fn find_by_id(&self, id: i32) -> AppResult<Resource> {
                    self.pool
                        .query(move |mut conn| {
                            resources::table.find(id).first::<Resource>(&mut conn)
                        })
                        .await
                }
                pub async fn remove_by_id(&self, id: i32) -> AppResult<usize> {
                    self.pool
                        .query(move |mut conn| {
                            diesel::delete(resources::table.find(id)).execute(&mut conn)
                        })
                        .await
                }
            }
        }
        pub mod rooms {
            use crate::models::schema::rooms;
            use chrono::NaiveDateTime;
            use diesel::{connection::LoadConnection, prelude::*};
            use nidrs::{injectable, openapi::schema, AppResult, Inject};
            use nidrs_diesel::{PoolManager, PostgresPoolManager};
            use serde::{Deserialize, Serialize};
            #[diesel(table_name = rooms)]
            #[diesel(check_for_backend(diesel::pg::Pg))]
            pub struct Room {
                pub id: i32,
                pub name: String,
                pub blank: bool,
                pub creator_id: i32,
                pub created_at: NaiveDateTime,
                pub updated_at: NaiveDateTime,
                pub deleted_at: Option<NaiveDateTime>,
            }
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::expression::Selectable;
                impl<__DB: diesel::backend::Backend> Selectable<__DB> for Room {
                    type SelectExpression = (
                        rooms::id,
                        rooms::name,
                        rooms::blank,
                        rooms::creator_id,
                        rooms::created_at,
                        rooms::updated_at,
                        rooms::deleted_at,
                    );
                    fn construct_selection() -> Self::SelectExpression {
                        (
                            rooms::id,
                            rooms::name,
                            rooms::blank,
                            rooms::creator_id,
                            rooms::created_at,
                            rooms::updated_at,
                            rooms::deleted_at,
                        )
                    }
                }
                fn _check_field_compatibility<__DB: diesel::backend::Backend>()
                where
                    i32: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<rooms::id>,
                        diesel::pg::Pg,
                    >,
                    String: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<rooms::name>,
                        diesel::pg::Pg,
                    >,
                    bool: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<rooms::blank>,
                        diesel::pg::Pg,
                    >,
                    i32: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<rooms::creator_id>,
                        diesel::pg::Pg,
                    >,
                    NaiveDateTime: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<rooms::created_at>,
                        diesel::pg::Pg,
                    >,
                    NaiveDateTime: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<rooms::updated_at>,
                        diesel::pg::Pg,
                    >,
                    Option<
                        NaiveDateTime,
                    >: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<rooms::deleted_at>,
                        diesel::pg::Pg,
                    >,
                {}
            };
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::deserialize::{self, FromStaticSqlRow, Queryable};
                use diesel::row::{Row as _, Field as _};
                use std::convert::TryInto;
                impl<
                    __DB: diesel::backend::Backend,
                    __ST0,
                    __ST1,
                    __ST2,
                    __ST3,
                    __ST4,
                    __ST5,
                    __ST6,
                > Queryable<(__ST0, __ST1, __ST2, __ST3, __ST4, __ST5, __ST6), __DB>
                for Room
                where
                    (
                        i32,
                        String,
                        bool,
                        i32,
                        NaiveDateTime,
                        NaiveDateTime,
                        Option<NaiveDateTime>,
                    ): FromStaticSqlRow<
                        (__ST0, __ST1, __ST2, __ST3, __ST4, __ST5, __ST6),
                        __DB,
                    >,
                {
                    type Row = (
                        i32,
                        String,
                        bool,
                        i32,
                        NaiveDateTime,
                        NaiveDateTime,
                        Option<NaiveDateTime>,
                    );
                    fn build(row: Self::Row) -> deserialize::Result<Self> {
                        Ok(Self {
                            id: row.0.try_into()?,
                            name: row.1.try_into()?,
                            blank: row.2.try_into()?,
                            creator_id: row.3.try_into()?,
                            created_at: row.4.try_into()?,
                            updated_at: row.5.try_into()?,
                            deleted_at: row.6.try_into()?,
                        })
                    }
                }
            };
            #[automatically_derived]
            impl ::core::fmt::Debug for Room {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let names: &'static _ = &[
                        "id",
                        "name",
                        "blank",
                        "creator_id",
                        "created_at",
                        "updated_at",
                        "deleted_at",
                    ];
                    let values: &[&dyn ::core::fmt::Debug] = &[
                        &self.id,
                        &self.name,
                        &self.blank,
                        &self.creator_id,
                        &self.created_at,
                        &self.updated_at,
                        &&self.deleted_at,
                    ];
                    ::core::fmt::Formatter::debug_struct_fields_finish(
                        f,
                        "Room",
                        names,
                        values,
                    )
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for Room {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Room",
                            false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "id",
                            &self.id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "name",
                            &self.name,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "blank",
                            &self.blank,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "creator_id",
                            &self.creator_id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "created_at",
                            &self.created_at,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "updated_at",
                            &self.updated_at,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "deleted_at",
                            &self.deleted_at,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            impl utoipa::IntoParams for Room {
                fn into_params(
                    parameter_in_provider: impl Fn(
                    ) -> Option<utoipa::openapi::path::ParameterIn>,
                ) -> Vec<utoipa::openapi::path::Parameter> {
                    [
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("id")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::Integer)
                                        .format(
                                            Some(
                                                utoipa::openapi::SchemaFormat::KnownFormat(
                                                    utoipa::openapi::KnownFormat::Int32,
                                                ),
                                            ),
                                        ),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("name")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::String),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("blank")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::Boolean),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("creator_id")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::Integer)
                                        .format(
                                            Some(
                                                utoipa::openapi::SchemaFormat::KnownFormat(
                                                    utoipa::openapi::KnownFormat::Int32,
                                                ),
                                            ),
                                        ),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("created_at")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::Ref::from_schema_name("NaiveDateTime"),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("updated_at")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::Ref::from_schema_name("NaiveDateTime"),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("deleted_at")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::False)
                            .schema(
                                Some(
                                    utoipa::openapi::schema::AllOfBuilder::new()
                                        .nullable(true)
                                        .item(
                                            utoipa::openapi::Ref::from_schema_name("NaiveDateTime"),
                                        ),
                                ),
                            )
                            .build(),
                    ]
                        .to_vec()
                }
            }
            impl<'__s> utoipa::ToSchema<'__s> for Room {
                fn schema() -> (
                    &'__s str,
                    utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
                ) {
                    (
                        "Room",
                        utoipa::openapi::ObjectBuilder::new()
                            .property(
                                "id",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Integer)
                                    .format(
                                        Some(
                                            utoipa::openapi::SchemaFormat::KnownFormat(
                                                utoipa::openapi::KnownFormat::Int32,
                                            ),
                                        ),
                                    ),
                            )
                            .required("id")
                            .property(
                                "name",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::String),
                            )
                            .required("name")
                            .property(
                                "blank",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Boolean),
                            )
                            .required("blank")
                            .property(
                                "creator_id",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Integer)
                                    .format(
                                        Some(
                                            utoipa::openapi::SchemaFormat::KnownFormat(
                                                utoipa::openapi::KnownFormat::Int32,
                                            ),
                                        ),
                                    ),
                            )
                            .required("creator_id")
                            .property(
                                "created_at",
                                utoipa::openapi::Ref::from_schema_name("NaiveDateTime"),
                            )
                            .required("created_at")
                            .property(
                                "updated_at",
                                utoipa::openapi::Ref::from_schema_name("NaiveDateTime"),
                            )
                            .required("updated_at")
                            .property(
                                "deleted_at",
                                utoipa::openapi::schema::AllOfBuilder::new()
                                    .nullable(true)
                                    .item(
                                        utoipa::openapi::Ref::from_schema_name("NaiveDateTime"),
                                    ),
                            )
                            .into(),
                    )
                }
            }
            impl nidrs::openapi::ToParamDto for Room {
                fn to_param_dto(
                    dto_type: nidrs::openapi::ParamDtoIn,
                ) -> nidrs::openapi::ParamDto {
                    use nidrs::openapi::utoipa::IntoParams;
                    use nidrs::openapi::utoipa::ToSchema;
                    match dto_type {
                        nidrs::openapi::ParamDtoIn::Param(p) => {
                            nidrs::openapi::ParamDto::ParamList(
                                Self::into_params(|| Some(p.clone())),
                            )
                        }
                        nidrs::openapi::ParamDtoIn::Body => {
                            nidrs::openapi::ParamDto::BodySchema(Self::schema())
                        }
                    }
                }
            }
            #[diesel(table_name = rooms)]
            pub struct CreateRoom {
                pub name: String,
                pub blank: bool,
                pub creator_id: i32,
            }
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::insertable::Insertable;
                use diesel::internal::derives::insertable::UndecoratedInsertRecord;
                use diesel::prelude::*;
                #[allow(unused_qualifications)]
                impl Insertable<rooms::table> for CreateRoom {
                    type Values = <(
                        std::option::Option<diesel::dsl::Eq<rooms::name, String>>,
                        std::option::Option<diesel::dsl::Eq<rooms::blank, bool>>,
                        std::option::Option<diesel::dsl::Eq<rooms::creator_id, i32>>,
                    ) as Insertable<rooms::table>>::Values;
                    fn values(
                        self,
                    ) -> <(
                        std::option::Option<diesel::dsl::Eq<rooms::name, String>>,
                        std::option::Option<diesel::dsl::Eq<rooms::blank, bool>>,
                        std::option::Option<diesel::dsl::Eq<rooms::creator_id, i32>>,
                    ) as Insertable<rooms::table>>::Values {
                        (
                            std::option::Option::Some(rooms::name.eq(self.name)),
                            std::option::Option::Some(rooms::blank.eq(self.blank)),
                            std::option::Option::Some(
                                rooms::creator_id.eq(self.creator_id),
                            ),
                        )
                            .values()
                    }
                }
                #[allow(unused_qualifications)]
                impl<'insert> Insertable<rooms::table> for &'insert CreateRoom {
                    type Values = <(
                        std::option::Option<
                            diesel::dsl::Eq<rooms::name, &'insert String>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<rooms::blank, &'insert bool>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<rooms::creator_id, &'insert i32>,
                        >,
                    ) as Insertable<rooms::table>>::Values;
                    fn values(
                        self,
                    ) -> <(
                        std::option::Option<
                            diesel::dsl::Eq<rooms::name, &'insert String>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<rooms::blank, &'insert bool>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<rooms::creator_id, &'insert i32>,
                        >,
                    ) as Insertable<rooms::table>>::Values {
                        (
                            std::option::Option::Some(rooms::name.eq(&self.name)),
                            std::option::Option::Some(rooms::blank.eq(&self.blank)),
                            std::option::Option::Some(
                                rooms::creator_id.eq(&self.creator_id),
                            ),
                        )
                            .values()
                    }
                }
                impl UndecoratedInsertRecord<rooms::table> for CreateRoom {}
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for CreateRoom {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "CreateRoom",
                            false as usize + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "name",
                            &self.name,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "blank",
                            &self.blank,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "creator_id",
                            &self.creator_id,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for CreateRoom {
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
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
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
                                    "name" => _serde::__private::Ok(__Field::__field0),
                                    "blank" => _serde::__private::Ok(__Field::__field1),
                                    "creator_id" => _serde::__private::Ok(__Field::__field2),
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
                                    b"name" => _serde::__private::Ok(__Field::__field0),
                                    b"blank" => _serde::__private::Ok(__Field::__field1),
                                    b"creator_id" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
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
                            marker: _serde::__private::PhantomData<CreateRoom>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = CreateRoom;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct CreateRoom",
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
                                                &"struct CreateRoom with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct CreateRoom with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    i32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct CreateRoom with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(CreateRoom {
                                    name: __field0,
                                    blank: __field1,
                                    creator_id: __field2,
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
                                let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                                let mut __field2: _serde::__private::Option<i32> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("blank"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "creator_id",
                                                    ),
                                                );
                                            }
                                            __field2 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
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
                                        _serde::__private::de::missing_field("name")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("blank")?
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("creator_id")?
                                    }
                                };
                                _serde::__private::Ok(CreateRoom {
                                    name: __field0,
                                    blank: __field1,
                                    creator_id: __field2,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "name",
                            "blank",
                            "creator_id",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "CreateRoom",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<CreateRoom>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            pub struct RoomEntity {
                pool: Inject<PostgresPoolManager>,
            }
            #[automatically_derived]
            impl ::core::default::Default for RoomEntity {
                #[inline]
                fn default() -> RoomEntity {
                    RoomEntity {
                        pool: ::core::default::Default::default(),
                    }
                }
            }
            impl nidrs::Service for RoomEntity {
                fn inject(
                    &self,
                    ctx: nidrs::ModuleCtx,
                    module_name: &str,
                ) -> nidrs::ModuleCtx {
                    let service = ctx
                        .get_service::<
                            PostgresPoolManager,
                        >(&module_name, "PostgresPoolManager");
                    self.pool.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for RoomEntity {
                fn __meta() -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set("service", "RoomEntity");
                    meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                    meta.set_data(nidrs::datasets::ServiceName::from("RoomEntity"));
                    meta.set("module", "AppModule");
                    meta.set("global", "app");
                    meta
                }
            }
            impl RoomEntity {
                pub async fn all(&self) -> AppResult<Vec<Room>> {
                    self.pool
                        .query(|mut conn| rooms::table.load::<Room>(&mut conn))
                        .await
                }
                pub async fn create(&self, new_room: CreateRoom) -> AppResult<usize> {
                    self.pool
                        .query(move |mut conn| {
                            diesel::insert_into(rooms::table)
                                .values(&new_room)
                                .execute(&mut conn)
                        })
                        .await
                }
                pub async fn update(&self, id: i32, name: String) -> AppResult<usize> {
                    self.pool
                        .query(move |mut conn| {
                            diesel::update(rooms::table.find(id))
                                .set(rooms::name.eq(name))
                                .execute(&mut conn)
                        })
                        .await
                }
                pub async fn find_by_id(&self, id: i32) -> AppResult<Room> {
                    self.pool
                        .query(move |mut conn| {
                            rooms::table.find(id).first::<Room>(&mut conn)
                        })
                        .await
                }
                pub async fn remove_by_id(&self, id: i32) -> AppResult<usize> {
                    self.pool
                        .query(move |mut conn| {
                            diesel::delete(rooms::table.find(id)).execute(&mut conn)
                        })
                        .await
                }
            }
        }
        pub mod users {
            use crate::models::schema::users;
            use chrono::NaiveDateTime;
            use diesel::{connection::LoadConnection, prelude::*};
            use nidrs::{injectable, openapi::schema, AppResult, Inject};
            use nidrs_diesel::{PoolManager, PostgresPoolManager};
            use serde::{Deserialize, Serialize};
            #[diesel(table_name = users)]
            #[diesel(check_for_backend(diesel::pg::Pg))]
            pub struct User {
                pub id: i32,
                pub name: String,
                pub unionid: String,
                pub openid: String,
                pub derive: String,
                pub blank: bool,
            }
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::expression::Selectable;
                impl<__DB: diesel::backend::Backend> Selectable<__DB> for User {
                    type SelectExpression = (
                        users::id,
                        users::name,
                        users::unionid,
                        users::openid,
                        users::derive,
                        users::blank,
                    );
                    fn construct_selection() -> Self::SelectExpression {
                        (
                            users::id,
                            users::name,
                            users::unionid,
                            users::openid,
                            users::derive,
                            users::blank,
                        )
                    }
                }
                fn _check_field_compatibility<__DB: diesel::backend::Backend>()
                where
                    i32: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<users::id>,
                        diesel::pg::Pg,
                    >,
                    String: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<users::name>,
                        diesel::pg::Pg,
                    >,
                    String: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<users::unionid>,
                        diesel::pg::Pg,
                    >,
                    String: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<users::openid>,
                        diesel::pg::Pg,
                    >,
                    String: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<users::derive>,
                        diesel::pg::Pg,
                    >,
                    bool: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<users::blank>,
                        diesel::pg::Pg,
                    >,
                {}
            };
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::deserialize::{self, FromStaticSqlRow, Queryable};
                use diesel::row::{Row as _, Field as _};
                use std::convert::TryInto;
                impl<
                    __DB: diesel::backend::Backend,
                    __ST0,
                    __ST1,
                    __ST2,
                    __ST3,
                    __ST4,
                    __ST5,
                > Queryable<(__ST0, __ST1, __ST2, __ST3, __ST4, __ST5), __DB> for User
                where
                    (
                        i32,
                        String,
                        String,
                        String,
                        String,
                        bool,
                    ): FromStaticSqlRow<
                        (__ST0, __ST1, __ST2, __ST3, __ST4, __ST5),
                        __DB,
                    >,
                {
                    type Row = (i32, String, String, String, String, bool);
                    fn build(row: Self::Row) -> deserialize::Result<Self> {
                        Ok(Self {
                            id: row.0.try_into()?,
                            name: row.1.try_into()?,
                            unionid: row.2.try_into()?,
                            openid: row.3.try_into()?,
                            derive: row.4.try_into()?,
                            blank: row.5.try_into()?,
                        })
                    }
                }
            };
            #[automatically_derived]
            impl ::core::fmt::Debug for User {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let names: &'static _ = &[
                        "id",
                        "name",
                        "unionid",
                        "openid",
                        "derive",
                        "blank",
                    ];
                    let values: &[&dyn ::core::fmt::Debug] = &[
                        &self.id,
                        &self.name,
                        &self.unionid,
                        &self.openid,
                        &self.derive,
                        &&self.blank,
                    ];
                    ::core::fmt::Formatter::debug_struct_fields_finish(
                        f,
                        "User",
                        names,
                        values,
                    )
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for User {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "User",
                            false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "id",
                            &self.id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "name",
                            &self.name,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "unionid",
                            &self.unionid,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "openid",
                            &self.openid,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "derive",
                            &self.derive,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "blank",
                            &self.blank,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            impl utoipa::IntoParams for User {
                fn into_params(
                    parameter_in_provider: impl Fn(
                    ) -> Option<utoipa::openapi::path::ParameterIn>,
                ) -> Vec<utoipa::openapi::path::Parameter> {
                    [
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("id")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::Integer)
                                        .format(
                                            Some(
                                                utoipa::openapi::SchemaFormat::KnownFormat(
                                                    utoipa::openapi::KnownFormat::Int32,
                                                ),
                                            ),
                                        ),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("name")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::String),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("unionid")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::String),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("openid")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::String),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("derive")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::String),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("blank")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::Boolean),
                                ),
                            )
                            .build(),
                    ]
                        .to_vec()
                }
            }
            impl<'__s> utoipa::ToSchema<'__s> for User {
                fn schema() -> (
                    &'__s str,
                    utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
                ) {
                    (
                        "User",
                        utoipa::openapi::ObjectBuilder::new()
                            .property(
                                "id",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Integer)
                                    .format(
                                        Some(
                                            utoipa::openapi::SchemaFormat::KnownFormat(
                                                utoipa::openapi::KnownFormat::Int32,
                                            ),
                                        ),
                                    ),
                            )
                            .required("id")
                            .property(
                                "name",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::String),
                            )
                            .required("name")
                            .property(
                                "unionid",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::String),
                            )
                            .required("unionid")
                            .property(
                                "openid",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::String),
                            )
                            .required("openid")
                            .property(
                                "derive",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::String),
                            )
                            .required("derive")
                            .property(
                                "blank",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Boolean),
                            )
                            .required("blank")
                            .into(),
                    )
                }
            }
            impl nidrs::openapi::ToParamDto for User {
                fn to_param_dto(
                    dto_type: nidrs::openapi::ParamDtoIn,
                ) -> nidrs::openapi::ParamDto {
                    use nidrs::openapi::utoipa::IntoParams;
                    use nidrs::openapi::utoipa::ToSchema;
                    match dto_type {
                        nidrs::openapi::ParamDtoIn::Param(p) => {
                            nidrs::openapi::ParamDto::ParamList(
                                Self::into_params(|| Some(p.clone())),
                            )
                        }
                        nidrs::openapi::ParamDtoIn::Body => {
                            nidrs::openapi::ParamDto::BodySchema(Self::schema())
                        }
                    }
                }
            }
            #[diesel(table_name = users)]
            pub struct CreateUser {
                pub name: String,
                pub unionid: String,
                pub openid: String,
                pub derive: String,
            }
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::insertable::Insertable;
                use diesel::internal::derives::insertable::UndecoratedInsertRecord;
                use diesel::prelude::*;
                #[allow(unused_qualifications)]
                impl Insertable<users::table> for CreateUser {
                    type Values = <(
                        std::option::Option<diesel::dsl::Eq<users::name, String>>,
                        std::option::Option<diesel::dsl::Eq<users::unionid, String>>,
                        std::option::Option<diesel::dsl::Eq<users::openid, String>>,
                        std::option::Option<diesel::dsl::Eq<users::derive, String>>,
                    ) as Insertable<users::table>>::Values;
                    fn values(
                        self,
                    ) -> <(
                        std::option::Option<diesel::dsl::Eq<users::name, String>>,
                        std::option::Option<diesel::dsl::Eq<users::unionid, String>>,
                        std::option::Option<diesel::dsl::Eq<users::openid, String>>,
                        std::option::Option<diesel::dsl::Eq<users::derive, String>>,
                    ) as Insertable<users::table>>::Values {
                        (
                            std::option::Option::Some(users::name.eq(self.name)),
                            std::option::Option::Some(users::unionid.eq(self.unionid)),
                            std::option::Option::Some(users::openid.eq(self.openid)),
                            std::option::Option::Some(users::derive.eq(self.derive)),
                        )
                            .values()
                    }
                }
                #[allow(unused_qualifications)]
                impl<'insert> Insertable<users::table> for &'insert CreateUser {
                    type Values = <(
                        std::option::Option<
                            diesel::dsl::Eq<users::name, &'insert String>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<users::unionid, &'insert String>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<users::openid, &'insert String>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<users::derive, &'insert String>,
                        >,
                    ) as Insertable<users::table>>::Values;
                    fn values(
                        self,
                    ) -> <(
                        std::option::Option<
                            diesel::dsl::Eq<users::name, &'insert String>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<users::unionid, &'insert String>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<users::openid, &'insert String>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<users::derive, &'insert String>,
                        >,
                    ) as Insertable<users::table>>::Values {
                        (
                            std::option::Option::Some(users::name.eq(&self.name)),
                            std::option::Option::Some(users::unionid.eq(&self.unionid)),
                            std::option::Option::Some(users::openid.eq(&self.openid)),
                            std::option::Option::Some(users::derive.eq(&self.derive)),
                        )
                            .values()
                    }
                }
                impl UndecoratedInsertRecord<users::table> for CreateUser {}
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for CreateUser {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "CreateUser",
                            false as usize + 1 + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "name",
                            &self.name,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "unionid",
                            &self.unionid,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "openid",
                            &self.openid,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "derive",
                            &self.derive,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for CreateUser {
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
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
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
                                    "name" => _serde::__private::Ok(__Field::__field0),
                                    "unionid" => _serde::__private::Ok(__Field::__field1),
                                    "openid" => _serde::__private::Ok(__Field::__field2),
                                    "derive" => _serde::__private::Ok(__Field::__field3),
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
                                    b"name" => _serde::__private::Ok(__Field::__field0),
                                    b"unionid" => _serde::__private::Ok(__Field::__field1),
                                    b"openid" => _serde::__private::Ok(__Field::__field2),
                                    b"derive" => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
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
                            marker: _serde::__private::PhantomData<CreateUser>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = CreateUser;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct CreateUser",
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
                                                &"struct CreateUser with 4 elements",
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
                                                &"struct CreateUser with 4 elements",
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
                                                &"struct CreateUser with 4 elements",
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
                                                &"struct CreateUser with 4 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(CreateUser {
                                    name: __field0,
                                    unionid: __field1,
                                    openid: __field2,
                                    derive: __field3,
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
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("name"),
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
                                                        "unionid",
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
                                                    <__A::Error as _serde::de::Error>::duplicate_field("openid"),
                                                );
                                            }
                                            __field2 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field3 => {
                                            if _serde::__private::Option::is_some(&__field3) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("derive"),
                                                );
                                            }
                                            __field3 = _serde::__private::Some(
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
                                        _serde::__private::de::missing_field("name")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("unionid")?
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("openid")?
                                    }
                                };
                                let __field3 = match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("derive")?
                                    }
                                };
                                _serde::__private::Ok(CreateUser {
                                    name: __field0,
                                    unionid: __field1,
                                    openid: __field2,
                                    derive: __field3,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "name",
                            "unionid",
                            "openid",
                            "derive",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "CreateUser",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<CreateUser>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            pub struct UserEntity {
                pool: Inject<PostgresPoolManager>,
            }
            #[automatically_derived]
            impl ::core::default::Default for UserEntity {
                #[inline]
                fn default() -> UserEntity {
                    UserEntity {
                        pool: ::core::default::Default::default(),
                    }
                }
            }
            impl nidrs::Service for UserEntity {
                fn inject(
                    &self,
                    ctx: nidrs::ModuleCtx,
                    module_name: &str,
                ) -> nidrs::ModuleCtx {
                    let service = ctx
                        .get_service::<
                            PostgresPoolManager,
                        >(&module_name, "PostgresPoolManager");
                    self.pool.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for UserEntity {
                fn __meta() -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set_data(nidrs::datasets::ServiceName::from("UserEntity"));
                    meta.set("service", "UserEntity");
                    meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                    meta.set("module", "AppModule");
                    meta.set("global", "app");
                    meta
                }
            }
            impl UserEntity {
                pub async fn all(&self) -> AppResult<Vec<User>> {
                    self.pool
                        .query(|mut conn| {
                            users::table
                                .select(User::as_select())
                                .load::<User>(&mut conn)
                        })
                        .await
                }
                pub async fn create(&self, openid: String) -> AppResult<usize> {
                    self.pool
                        .query(|mut conn| {
                            let new_user = CreateUser {
                                name: "".to_string(),
                                unionid: "default".to_string(),
                                openid,
                                derive: "".to_string(),
                            };
                            diesel::insert_into(users::table)
                                .values(&new_user)
                                .execute(&mut conn)
                        })
                        .await
                }
                pub async fn update(&self, id: i32, name: String) -> AppResult<usize> {
                    self.pool
                        .query(move |mut conn| {
                            diesel::update(users::table.find(id))
                                .set(users::name.eq(name))
                                .execute(&mut conn)
                        })
                        .await
                }
                pub async fn find_by_id(&self, id: i32) -> AppResult<User> {
                    self.pool
                        .query(move |mut conn| {
                            users::table
                                .find(id)
                                .select(User::as_select())
                                .first::<User>(&mut conn)
                        })
                        .await
                }
                pub async fn find_by_openid(&self, openid: String) -> AppResult<User> {
                    self.pool
                        .query(move |mut conn| {
                            users::table
                                .filter(users::openid.eq(openid))
                                .select(User::as_select())
                                .first::<User>(&mut conn)
                        })
                        .await
                }
                pub async fn remove_by_id(&self, id: i32) -> AppResult<usize> {
                    self.pool
                        .query(move |mut conn| {
                            diesel::delete(users::table.find(id)).execute(&mut conn)
                        })
                        .await
                }
            }
        }
        pub mod users_extra {
            use crate::models::schema::users_extra;
            use chrono::NaiveDateTime;
            use nidrs::{injectable, openapi::schema, AppResult, Inject};
            use nidrs_diesel::{PoolManager, PostgresPoolManager};
            use serde::{Deserialize, Serialize};
            use diesel::{connection::LoadConnection, prelude::*};
            #[diesel(table_name = users_extra)]
            #[diesel(check_for_backend(diesel::pg::Pg))]
            pub struct UserExtra {
                pub id: i32,
                pub user_id: i32,
                pub first_launch_path: String,
                pub first_launch_scene: String,
            }
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::expression::Selectable;
                impl<__DB: diesel::backend::Backend> Selectable<__DB> for UserExtra {
                    type SelectExpression = (
                        users_extra::id,
                        users_extra::user_id,
                        users_extra::first_launch_path,
                        users_extra::first_launch_scene,
                    );
                    fn construct_selection() -> Self::SelectExpression {
                        (
                            users_extra::id,
                            users_extra::user_id,
                            users_extra::first_launch_path,
                            users_extra::first_launch_scene,
                        )
                    }
                }
                fn _check_field_compatibility<__DB: diesel::backend::Backend>()
                where
                    i32: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<users_extra::id>,
                        diesel::pg::Pg,
                    >,
                    i32: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<users_extra::user_id>,
                        diesel::pg::Pg,
                    >,
                    String: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<users_extra::first_launch_path>,
                        diesel::pg::Pg,
                    >,
                    String: diesel::deserialize::FromSqlRow<
                        diesel::dsl::SqlTypeOf<users_extra::first_launch_scene>,
                        diesel::pg::Pg,
                    >,
                {}
            };
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::deserialize::{self, FromStaticSqlRow, Queryable};
                use diesel::row::{Row as _, Field as _};
                use std::convert::TryInto;
                impl<
                    __DB: diesel::backend::Backend,
                    __ST0,
                    __ST1,
                    __ST2,
                    __ST3,
                > Queryable<(__ST0, __ST1, __ST2, __ST3), __DB> for UserExtra
                where
                    (
                        i32,
                        i32,
                        String,
                        String,
                    ): FromStaticSqlRow<(__ST0, __ST1, __ST2, __ST3), __DB>,
                {
                    type Row = (i32, i32, String, String);
                    fn build(row: Self::Row) -> deserialize::Result<Self> {
                        Ok(Self {
                            id: row.0.try_into()?,
                            user_id: row.1.try_into()?,
                            first_launch_path: row.2.try_into()?,
                            first_launch_scene: row.3.try_into()?,
                        })
                    }
                }
            };
            #[automatically_derived]
            impl ::core::fmt::Debug for UserExtra {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "UserExtra",
                        "id",
                        &self.id,
                        "user_id",
                        &self.user_id,
                        "first_launch_path",
                        &self.first_launch_path,
                        "first_launch_scene",
                        &&self.first_launch_scene,
                    )
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for UserExtra {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "UserExtra",
                            false as usize + 1 + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "id",
                            &self.id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "user_id",
                            &self.user_id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "first_launch_path",
                            &self.first_launch_path,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "first_launch_scene",
                            &self.first_launch_scene,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            impl utoipa::IntoParams for UserExtra {
                fn into_params(
                    parameter_in_provider: impl Fn(
                    ) -> Option<utoipa::openapi::path::ParameterIn>,
                ) -> Vec<utoipa::openapi::path::Parameter> {
                    [
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("id")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::Integer)
                                        .format(
                                            Some(
                                                utoipa::openapi::SchemaFormat::KnownFormat(
                                                    utoipa::openapi::KnownFormat::Int32,
                                                ),
                                            ),
                                        ),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("user_id")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::Integer)
                                        .format(
                                            Some(
                                                utoipa::openapi::SchemaFormat::KnownFormat(
                                                    utoipa::openapi::KnownFormat::Int32,
                                                ),
                                            ),
                                        ),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("first_launch_path")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::String),
                                ),
                            )
                            .build(),
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("first_launch_scene")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::String),
                                ),
                            )
                            .build(),
                    ]
                        .to_vec()
                }
            }
            impl<'__s> utoipa::ToSchema<'__s> for UserExtra {
                fn schema() -> (
                    &'__s str,
                    utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
                ) {
                    (
                        "UserExtra",
                        utoipa::openapi::ObjectBuilder::new()
                            .property(
                                "id",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Integer)
                                    .format(
                                        Some(
                                            utoipa::openapi::SchemaFormat::KnownFormat(
                                                utoipa::openapi::KnownFormat::Int32,
                                            ),
                                        ),
                                    ),
                            )
                            .required("id")
                            .property(
                                "user_id",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::Integer)
                                    .format(
                                        Some(
                                            utoipa::openapi::SchemaFormat::KnownFormat(
                                                utoipa::openapi::KnownFormat::Int32,
                                            ),
                                        ),
                                    ),
                            )
                            .required("user_id")
                            .property(
                                "first_launch_path",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::String),
                            )
                            .required("first_launch_path")
                            .property(
                                "first_launch_scene",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::String),
                            )
                            .required("first_launch_scene")
                            .into(),
                    )
                }
            }
            impl nidrs::openapi::ToParamDto for UserExtra {
                fn to_param_dto(
                    dto_type: nidrs::openapi::ParamDtoIn,
                ) -> nidrs::openapi::ParamDto {
                    use nidrs::openapi::utoipa::IntoParams;
                    use nidrs::openapi::utoipa::ToSchema;
                    match dto_type {
                        nidrs::openapi::ParamDtoIn::Param(p) => {
                            nidrs::openapi::ParamDto::ParamList(
                                Self::into_params(|| Some(p.clone())),
                            )
                        }
                        nidrs::openapi::ParamDtoIn::Body => {
                            nidrs::openapi::ParamDto::BodySchema(Self::schema())
                        }
                    }
                }
            }
            #[diesel(table_name = users_extra)]
            pub struct CreateUserExtra {
                pub user_id: i32,
                pub first_launch_path: String,
                pub first_launch_scene: String,
            }
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::insertable::Insertable;
                use diesel::internal::derives::insertable::UndecoratedInsertRecord;
                use diesel::prelude::*;
                #[allow(unused_qualifications)]
                impl Insertable<users_extra::table> for CreateUserExtra {
                    type Values = <(
                        std::option::Option<diesel::dsl::Eq<users_extra::user_id, i32>>,
                        std::option::Option<
                            diesel::dsl::Eq<users_extra::first_launch_path, String>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<users_extra::first_launch_scene, String>,
                        >,
                    ) as Insertable<users_extra::table>>::Values;
                    fn values(
                        self,
                    ) -> <(
                        std::option::Option<diesel::dsl::Eq<users_extra::user_id, i32>>,
                        std::option::Option<
                            diesel::dsl::Eq<users_extra::first_launch_path, String>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<users_extra::first_launch_scene, String>,
                        >,
                    ) as Insertable<users_extra::table>>::Values {
                        (
                            std::option::Option::Some(
                                users_extra::user_id.eq(self.user_id),
                            ),
                            std::option::Option::Some(
                                users_extra::first_launch_path.eq(self.first_launch_path),
                            ),
                            std::option::Option::Some(
                                users_extra::first_launch_scene.eq(self.first_launch_scene),
                            ),
                        )
                            .values()
                    }
                }
                #[allow(unused_qualifications)]
                impl<'insert> Insertable<users_extra::table>
                for &'insert CreateUserExtra {
                    type Values = <(
                        std::option::Option<
                            diesel::dsl::Eq<users_extra::user_id, &'insert i32>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<
                                users_extra::first_launch_path,
                                &'insert String,
                            >,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<
                                users_extra::first_launch_scene,
                                &'insert String,
                            >,
                        >,
                    ) as Insertable<users_extra::table>>::Values;
                    fn values(
                        self,
                    ) -> <(
                        std::option::Option<
                            diesel::dsl::Eq<users_extra::user_id, &'insert i32>,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<
                                users_extra::first_launch_path,
                                &'insert String,
                            >,
                        >,
                        std::option::Option<
                            diesel::dsl::Eq<
                                users_extra::first_launch_scene,
                                &'insert String,
                            >,
                        >,
                    ) as Insertable<users_extra::table>>::Values {
                        (
                            std::option::Option::Some(
                                users_extra::user_id.eq(&self.user_id),
                            ),
                            std::option::Option::Some(
                                users_extra::first_launch_path.eq(&self.first_launch_path),
                            ),
                            std::option::Option::Some(
                                users_extra::first_launch_scene.eq(&self.first_launch_scene),
                            ),
                        )
                            .values()
                    }
                }
                impl UndecoratedInsertRecord<users_extra::table> for CreateUserExtra {}
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for CreateUserExtra {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "CreateUserExtra",
                            false as usize + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "user_id",
                            &self.user_id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "first_launch_path",
                            &self.first_launch_path,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "first_launch_scene",
                            &self.first_launch_scene,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for CreateUserExtra {
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
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
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
                                    "user_id" => _serde::__private::Ok(__Field::__field0),
                                    "first_launch_path" => {
                                        _serde::__private::Ok(__Field::__field1)
                                    }
                                    "first_launch_scene" => {
                                        _serde::__private::Ok(__Field::__field2)
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
                                    b"user_id" => _serde::__private::Ok(__Field::__field0),
                                    b"first_launch_path" => {
                                        _serde::__private::Ok(__Field::__field1)
                                    }
                                    b"first_launch_scene" => {
                                        _serde::__private::Ok(__Field::__field2)
                                    }
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
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
                            marker: _serde::__private::PhantomData<CreateUserExtra>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = CreateUserExtra;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct CreateUserExtra",
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
                                    i32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct CreateUserExtra with 3 elements",
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
                                                &"struct CreateUserExtra with 3 elements",
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
                                                &"struct CreateUserExtra with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(CreateUserExtra {
                                    user_id: __field0,
                                    first_launch_path: __field1,
                                    first_launch_scene: __field2,
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
                                let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "user_id",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "first_launch_path",
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
                                                        "first_launch_scene",
                                                    ),
                                                );
                                            }
                                            __field2 = _serde::__private::Some(
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
                                        _serde::__private::de::missing_field("user_id")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("first_launch_path")?
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("first_launch_scene")?
                                    }
                                };
                                _serde::__private::Ok(CreateUserExtra {
                                    user_id: __field0,
                                    first_launch_path: __field1,
                                    first_launch_scene: __field2,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "user_id",
                            "first_launch_path",
                            "first_launch_scene",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "CreateUserExtra",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<CreateUserExtra>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            pub struct UserExtraEntity {
                pool: Inject<PostgresPoolManager>,
            }
            #[automatically_derived]
            impl ::core::default::Default for UserExtraEntity {
                #[inline]
                fn default() -> UserExtraEntity {
                    UserExtraEntity {
                        pool: ::core::default::Default::default(),
                    }
                }
            }
            impl nidrs::Service for UserExtraEntity {
                fn inject(
                    &self,
                    ctx: nidrs::ModuleCtx,
                    module_name: &str,
                ) -> nidrs::ModuleCtx {
                    let service = ctx
                        .get_service::<
                            PostgresPoolManager,
                        >(&module_name, "PostgresPoolManager");
                    self.pool.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for UserExtraEntity {
                fn __meta() -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set("service", "UserExtraEntity");
                    meta.set_data(nidrs::datasets::ServiceName::from("UserExtraEntity"));
                    meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                    meta.set("module", "AppModule");
                    meta.set("global", "app");
                    meta
                }
            }
            impl UserExtraEntity {
                pub async fn all(&self) -> AppResult<Vec<UserExtra>> {
                    self.pool
                        .query(|mut conn| {
                            users_extra::table.load::<UserExtra>(&mut conn)
                        })
                        .await
                }
                pub async fn create(
                    &self,
                    user_id: i32,
                    first_launch_path: String,
                    first_launch_scene: String,
                ) -> AppResult<usize> {
                    self.pool
                        .query(move |mut conn| {
                            let new_user_extra = CreateUserExtra {
                                user_id,
                                first_launch_path,
                                first_launch_scene,
                            };
                            diesel::insert_into(users_extra::table)
                                .values(&new_user_extra)
                                .execute(&mut conn)
                        })
                        .await
                }
                pub async fn update(
                    &self,
                    id: i32,
                    first_launch_path: String,
                    first_launch_scene: String,
                ) -> AppResult<usize> {
                    self.pool
                        .query(move |mut conn| {
                            diesel::update(users_extra::table.find(id))
                                .set((
                                    users_extra::first_launch_path.eq(first_launch_path),
                                    users_extra::first_launch_scene.eq(first_launch_scene),
                                ))
                                .execute(&mut conn)
                        })
                        .await
                }
                pub async fn find_by_id(&self, id: i32) -> AppResult<UserExtra> {
                    self.pool
                        .query(move |mut conn| {
                            users_extra::table.find(id).first::<UserExtra>(&mut conn)
                        })
                        .await
                }
                pub async fn remove_by_id(&self, id: i32) -> AppResult<usize> {
                    self.pool
                        .query(move |mut conn| {
                            diesel::delete(users_extra::table.find(id))
                                .execute(&mut conn)
                        })
                        .await
                }
            }
        }
    }
    pub mod schema {
        #[allow(unused_imports, dead_code, unreachable_pub, unused_qualifications)]
        pub mod downlogs {
            use ::diesel;
            pub use self::columns::*;
            use diesel::sql_types::*;
            /// Re-exports all of the columns of this table, as well as the
            /// table struct renamed to the module name. This is meant to be
            /// glob imported for functions which only deal with one table.
            pub mod dsl {
                pub use super::columns::id;
                pub use super::columns::resource_id;
                pub use super::columns::user_id;
                pub use super::columns::status;
                pub use super::columns::created_at;
                pub use super::table as downlogs;
            }
            #[allow(non_upper_case_globals, dead_code)]
            /// A tuple of all of the columns on this table
            pub const all_columns: (id, resource_id, user_id, status, created_at) = (
                id,
                resource_id,
                user_id,
                status,
                created_at,
            );
            #[allow(non_camel_case_types)]
            /// The actual table struct
            ///
            /// This is the type which provides the base methods of the query
            /// builder, such as `.select` and `.filter`.
            pub struct table;
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for table {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "table")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::clone::Clone for table {
                #[inline]
                fn clone(&self) -> table {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::marker::Copy for table {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for table {
                    type QueryId = table;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::default::Default for table {
                #[inline]
                fn default() -> table {
                    table {}
                }
            }
            impl table {
                #[allow(dead_code)]
                /// Represents `table_name.*`, which is sometimes necessary
                /// for efficient count queries. It cannot be used in place of
                /// `all_columns`
                pub fn star(&self) -> star {
                    star
                }
            }
            /// The SQL type of all of the columns on this table
            pub type SqlType = (Int4, Int4, Int4, Int4, Timestamp);
            /// Helper type for representing a boxed query from this table
            pub type BoxedQuery<'a, DB, ST = SqlType> = diesel::internal::table_macro::BoxedSelectStatement<
                'a,
                ST,
                diesel::internal::table_macro::FromClause<table>,
                DB,
            >;
            impl diesel::QuerySource for table {
                type FromClause = diesel::internal::table_macro::StaticQueryFragmentInstance<
                    table,
                >;
                type DefaultSelection = <Self as diesel::Table>::AllColumns;
                fn from_clause(&self) -> Self::FromClause {
                    diesel::internal::table_macro::StaticQueryFragmentInstance::new()
                }
                fn default_selection(&self) -> Self::DefaultSelection {
                    use diesel::Table;
                    Self::all_columns()
                }
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for table
            where
                DB: diesel::backend::Backend,
                <table as diesel::internal::table_macro::StaticQueryFragment>::Component: diesel::query_builder::QueryFragment<
                    DB,
                >,
            {
                fn walk_ast<'b>(
                    &'b self,
                    __diesel_internal_pass: diesel::query_builder::AstPass<'_, 'b, DB>,
                ) -> diesel::result::QueryResult<()> {
                    <table as diesel::internal::table_macro::StaticQueryFragment>::STATIC_COMPONENT
                        .walk_ast(__diesel_internal_pass)
                }
            }
            impl diesel::internal::table_macro::StaticQueryFragment for table {
                type Component = diesel::internal::table_macro::Identifier<'static>;
                const STATIC_COMPONENT: &'static Self::Component = &diesel::internal::table_macro::Identifier(
                    "downlogs",
                );
            }
            impl diesel::query_builder::AsQuery for table {
                type SqlType = SqlType;
                type Query = diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<Self>,
                >;
                fn as_query(self) -> Self::Query {
                    diesel::internal::table_macro::SelectStatement::simple(self)
                }
            }
            impl diesel::Table for table {
                type PrimaryKey = id;
                type AllColumns = (id, resource_id, user_id, status, created_at);
                fn primary_key(&self) -> Self::PrimaryKey {
                    id
                }
                fn all_columns() -> Self::AllColumns {
                    (id, resource_id, user_id, status, created_at)
                }
            }
            impl diesel::associations::HasTable for table {
                type Table = Self;
                fn table() -> Self::Table {
                    table
                }
            }
            impl diesel::query_builder::IntoUpdateTarget for table {
                type WhereClause = <<Self as diesel::query_builder::AsQuery>::Query as diesel::query_builder::IntoUpdateTarget>::WhereClause;
                fn into_update_target(
                    self,
                ) -> diesel::query_builder::UpdateTarget<
                    Self::Table,
                    Self::WhereClause,
                > {
                    use diesel::query_builder::AsQuery;
                    let q: diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<table>,
                    > = self.as_query();
                    q.into_update_target()
                }
            }
            impl diesel::query_source::AppearsInFromClause<table> for table {
                type Count = diesel::query_source::Once;
            }
            impl<S> diesel::internal::table_macro::AliasAppearsInFromClause<S, table>
            for table
            where
                S: diesel::query_source::AliasSource<Target = table>,
            {
                type Count = diesel::query_source::Never;
            }
            impl<
                S1,
                S2,
            > diesel::internal::table_macro::AliasAliasAppearsInFromClause<table, S2, S1>
            for table
            where
                S1: diesel::query_source::AliasSource<Target = table>,
                S2: diesel::query_source::AliasSource<Target = table>,
                S1: diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<
                    S2,
                    table,
                >,
            {
                type Count = <S1 as diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<
                    S2,
                    table,
                >>::Count;
            }
            impl<
                S,
            > diesel::query_source::AppearsInFromClause<diesel::query_source::Alias<S>>
            for table
            where
                S: diesel::query_source::AliasSource,
            {
                type Count = diesel::query_source::Never;
            }
            impl<
                S,
                C,
            > diesel::internal::table_macro::FieldAliasMapperAssociatedTypesDisjointnessTrick<
                table,
                S,
                C,
            > for table
            where
                S: diesel::query_source::AliasSource<Target = table>
                    + ::std::clone::Clone,
                C: diesel::query_source::Column<Table = table>,
            {
                type Out = diesel::query_source::AliasedField<S, C>;
                fn map(
                    __diesel_internal_column: C,
                    __diesel_internal_alias: &diesel::query_source::Alias<S>,
                ) -> Self::Out {
                    __diesel_internal_alias.field(__diesel_internal_column)
                }
            }
            impl diesel::query_source::AppearsInFromClause<table>
            for diesel::internal::table_macro::NoFromClause {
                type Count = diesel::query_source::Never;
            }
            impl<
                Left,
                Right,
                Kind,
            > diesel::JoinTo<diesel::internal::table_macro::Join<Left, Right, Kind>>
            for table
            where
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    Kind,
                >: diesel::JoinTo<table>,
                Left: diesel::query_source::QuerySource,
                Right: diesel::query_source::QuerySource,
            {
                type FromClause = diesel::internal::table_macro::Join<Left, Right, Kind>;
                type OnClause = <diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    Kind,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        Kind,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::Join::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                Join,
                On,
            > diesel::JoinTo<diesel::internal::table_macro::JoinOn<Join, On>> for table
            where
                diesel::internal::table_macro::JoinOn<Join, On>: diesel::JoinTo<table>,
            {
                type FromClause = diesel::internal::table_macro::JoinOn<Join, On>;
                type OnClause = <diesel::internal::table_macro::JoinOn<
                    Join,
                    On,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::JoinOn<
                        Join,
                        On,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::JoinOn::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                F,
                S,
                D,
                W,
                O,
                L,
                Of,
                G,
            > diesel::JoinTo<
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >,
            > for table
            where
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >: diesel::JoinTo<table>,
                F: diesel::query_source::QuerySource,
            {
                type FromClause = diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >;
                type OnClause = <diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<F>,
                        S,
                        D,
                        W,
                        O,
                        L,
                        Of,
                        G,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::SelectStatement::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                'a,
                QS,
                ST,
                DB,
            > diesel::JoinTo<
                diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >,
            > for table
            where
                diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >: diesel::JoinTo<table>,
                QS: diesel::query_source::QuerySource,
            {
                type FromClause = diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >;
                type OnClause = <diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::BoxedSelectStatement<
                        'a,
                        diesel::internal::table_macro::FromClause<QS>,
                        ST,
                        DB,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::BoxedSelectStatement::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<S> diesel::JoinTo<diesel::query_source::Alias<S>> for table
            where
                diesel::query_source::Alias<S>: diesel::JoinTo<table>,
            {
                type FromClause = diesel::query_source::Alias<S>;
                type OnClause = <diesel::query_source::Alias<
                    S,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::query_source::Alias<S>,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::query_source::Alias::<
                        S,
                    >::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<T> diesel::insertable::Insertable<T> for table
            where
                <table as diesel::query_builder::AsQuery>::Query: diesel::insertable::Insertable<
                    T,
                >,
            {
                type Values = <<table as diesel::query_builder::AsQuery>::Query as diesel::insertable::Insertable<
                    T,
                >>::Values;
                fn values(self) -> Self::Values {
                    use diesel::query_builder::AsQuery;
                    self.as_query().values()
                }
            }
            impl<'a, T> diesel::insertable::Insertable<T> for &'a table
            where
                table: diesel::insertable::Insertable<T>,
            {
                type Values = <table as diesel::insertable::Insertable<T>>::Values;
                fn values(self) -> Self::Values {
                    (*self).values()
                }
            }
            impl<S> diesel::JoinTo<diesel::query_builder::Only<S>> for table
            where
                diesel::query_builder::Only<S>: diesel::JoinTo<table>,
            {
                type FromClause = diesel::query_builder::Only<S>;
                type OnClause = <diesel::query_builder::Only<
                    S,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::query_builder::Only<S>,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::query_builder::Only::<
                        S,
                    >::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Only<table>,
            > for table {
                type Count = diesel::query_source::Once;
            }
            impl diesel::query_source::AppearsInFromClause<table>
            for diesel::query_builder::Only<table> {
                type Count = diesel::query_source::Once;
            }
            impl<S, TSM> diesel::JoinTo<diesel::query_builder::Tablesample<S, TSM>>
            for table
            where
                diesel::query_builder::Tablesample<S, TSM>: diesel::JoinTo<table>,
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type FromClause = diesel::query_builder::Tablesample<S, TSM>;
                type OnClause = <diesel::query_builder::Tablesample<
                    S,
                    TSM,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::query_builder::Tablesample<S, TSM>,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::query_builder::Tablesample::<
                        S,
                        TSM,
                    >::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                TSM,
            > diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<table, TSM>,
            > for table
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<TSM> diesel::query_source::AppearsInFromClause<table>
            for diesel::query_builder::Tablesample<table, TSM>
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            /// Contains all of the columns of this table
            pub mod columns {
                use ::diesel;
                use super::table;
                use diesel::sql_types::*;
                #[allow(non_camel_case_types, dead_code)]
                /// Represents `table_name.*`, which is sometimes needed for
                /// efficient count queries. It cannot be used in place of
                /// `all_columns`, and has a `SqlType` of `()` to prevent it
                /// being used that way
                pub struct star;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for star {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "star")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for star {
                    #[inline]
                    fn clone(&self) -> star {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for star {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for star {
                        type QueryId = star;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                impl<__GB> diesel::expression::ValidGrouping<__GB> for star
                where
                    (
                        id,
                        resource_id,
                        user_id,
                        status,
                        created_at,
                    ): diesel::expression::ValidGrouping<__GB>,
                {
                    type IsAggregate = <(
                        id,
                        resource_id,
                        user_id,
                        status,
                        created_at,
                    ) as diesel::expression::ValidGrouping<__GB>>::IsAggregate;
                }
                impl diesel::Expression for star {
                    type SqlType = diesel::expression::expression_types::NotSelectable;
                }
                impl<
                    DB: diesel::backend::Backend,
                > diesel::query_builder::QueryFragment<DB> for star
                where
                    <table as diesel::QuerySource>::FromClause: diesel::query_builder::QueryFragment<
                        DB,
                    >,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        use diesel::QuerySource;
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_sql("*");
                        Ok(())
                    }
                }
                impl diesel::SelectableExpression<table> for star {}
                impl diesel::AppearsOnTable<table> for star {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct id;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for id {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "id")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for id {
                    #[inline]
                    fn clone(&self) -> id {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for id {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for id {
                        type QueryId = id;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for id {
                    #[inline]
                    fn default() -> id {
                        id {}
                    }
                }
                impl diesel::expression::Expression for id {
                    type SqlType = Int4;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for id
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("id")
                    }
                }
                impl diesel::SelectableExpression<super::table> for id {}
                impl<QS> diesel::AppearsOnTable<QS> for id
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for id
                where
                    id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for id
                where
                    id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for id
                where
                    id: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for id
                where
                    From: diesel::query_source::QuerySource,
                    id: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for id
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        id,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for id {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for id {
                    type Table = super::table;
                    const NAME: &'static str = "id";
                }
                impl<T> diesel::EqAll<T> for id
                where
                    T: diesel::expression::AsExpression<Int4>,
                    diesel::dsl::Eq<
                        id,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Div<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Div<
                        Self,
                        Rhs::Expression,
                    >;
                    fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Div::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Mul<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Mul<
                        Self,
                        Rhs::Expression,
                    >;
                    fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Mul::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for id {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for id {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct resource_id;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for resource_id {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "resource_id")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for resource_id {
                    #[inline]
                    fn clone(&self) -> resource_id {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for resource_id {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for resource_id {
                        type QueryId = resource_id;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for resource_id {
                    #[inline]
                    fn default() -> resource_id {
                        resource_id {}
                    }
                }
                impl diesel::expression::Expression for resource_id {
                    type SqlType = Int4;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for resource_id
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("resource_id")
                    }
                }
                impl diesel::SelectableExpression<super::table> for resource_id {}
                impl<QS> diesel::AppearsOnTable<QS> for resource_id
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for resource_id
                where
                    resource_id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for resource_id
                where
                    resource_id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for resource_id
                where
                    resource_id: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for resource_id
                where
                    From: diesel::query_source::QuerySource,
                    resource_id: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for resource_id
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        resource_id,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for resource_id {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<resource_id>
                for resource_id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for resource_id {
                    type Table = super::table;
                    const NAME: &'static str = "resource_id";
                }
                impl<T> diesel::EqAll<T> for resource_id
                where
                    T: diesel::expression::AsExpression<Int4>,
                    diesel::dsl::Eq<
                        resource_id,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for resource_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<resource_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for resource_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<resource_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Div<Rhs> for resource_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<resource_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Div<
                        Self,
                        Rhs::Expression,
                    >;
                    fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Div::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Mul<Rhs> for resource_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<resource_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Mul<
                        Self,
                        Rhs::Expression,
                    >;
                    fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Mul::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for resource_id {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for resource_id {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for resource_id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for resource_id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct user_id;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for user_id {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "user_id")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for user_id {
                    #[inline]
                    fn clone(&self) -> user_id {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for user_id {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for user_id {
                        type QueryId = user_id;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for user_id {
                    #[inline]
                    fn default() -> user_id {
                        user_id {}
                    }
                }
                impl diesel::expression::Expression for user_id {
                    type SqlType = Int4;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for user_id
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("user_id")
                    }
                }
                impl diesel::SelectableExpression<super::table> for user_id {}
                impl<QS> diesel::AppearsOnTable<QS> for user_id
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for user_id
                where
                    user_id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for user_id
                where
                    user_id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for user_id
                where
                    user_id: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for user_id
                where
                    From: diesel::query_source::QuerySource,
                    user_id: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for user_id
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        user_id,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for user_id {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<user_id> for user_id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for user_id {
                    type Table = super::table;
                    const NAME: &'static str = "user_id";
                }
                impl<T> diesel::EqAll<T> for user_id
                where
                    T: diesel::expression::AsExpression<Int4>,
                    diesel::dsl::Eq<
                        user_id,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for user_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<user_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for user_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<user_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Div<Rhs> for user_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<user_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Div<
                        Self,
                        Rhs::Expression,
                    >;
                    fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Div::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Mul<Rhs> for user_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<user_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Mul<
                        Self,
                        Rhs::Expression,
                    >;
                    fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Mul::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for user_id {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for user_id {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for user_id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for user_id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct status;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for status {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "status")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for status {
                    #[inline]
                    fn clone(&self) -> status {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for status {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for status {
                        type QueryId = status;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for status {
                    #[inline]
                    fn default() -> status {
                        status {}
                    }
                }
                impl diesel::expression::Expression for status {
                    type SqlType = Int4;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for status
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("status")
                    }
                }
                impl diesel::SelectableExpression<super::table> for status {}
                impl<QS> diesel::AppearsOnTable<QS> for status
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for status
                where
                    status: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for status
                where
                    status: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for status
                where
                    status: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for status
                where
                    From: diesel::query_source::QuerySource,
                    status: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for status
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        status,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for status {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<status> for status {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for status {
                    type Table = super::table;
                    const NAME: &'static str = "status";
                }
                impl<T> diesel::EqAll<T> for status
                where
                    T: diesel::expression::AsExpression<Int4>,
                    diesel::dsl::Eq<
                        status,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for status
                where
                    Rhs: diesel::expression::AsExpression<
                        <<status as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for status
                where
                    Rhs: diesel::expression::AsExpression<
                        <<status as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Div<Rhs> for status
                where
                    Rhs: diesel::expression::AsExpression<
                        <<status as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Div<
                        Self,
                        Rhs::Expression,
                    >;
                    fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Div::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Mul<Rhs> for status
                where
                    Rhs: diesel::expression::AsExpression<
                        <<status as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Mul<
                        Self,
                        Rhs::Expression,
                    >;
                    fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Mul::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for status {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for status {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for status
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for status
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct created_at;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for created_at {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "created_at")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for created_at {
                    #[inline]
                    fn clone(&self) -> created_at {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for created_at {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for created_at {
                        type QueryId = created_at;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for created_at {
                    #[inline]
                    fn default() -> created_at {
                        created_at {}
                    }
                }
                impl diesel::expression::Expression for created_at {
                    type SqlType = Timestamp;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for created_at
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("created_at")
                    }
                }
                impl diesel::SelectableExpression<super::table> for created_at {}
                impl<QS> diesel::AppearsOnTable<QS> for created_at
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for created_at
                where
                    created_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for created_at
                where
                    created_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for created_at
                where
                    created_at: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for created_at
                where
                    From: diesel::query_source::QuerySource,
                    created_at: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for created_at
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        created_at,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for created_at {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at>
                for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for created_at {
                    type Table = super::table;
                    const NAME: &'static str = "created_at";
                }
                impl<T> diesel::EqAll<T> for created_at
                where
                    T: diesel::expression::AsExpression<Timestamp>,
                    diesel::dsl::Eq<
                        created_at,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for created_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<created_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for created_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<created_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for created_at {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for created_at {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for created_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for created_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                impl diesel::expression::IsContainedInGroupBy<id> for resource_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<resource_id> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for user_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<user_id> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for status {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<status> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<resource_id> for user_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<user_id> for resource_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<resource_id> for status {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<status> for resource_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<resource_id>
                for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at>
                for resource_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<user_id> for status {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<status> for user_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<user_id> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for user_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<status> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for status {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
            }
        }
        #[allow(unused_imports, dead_code, unreachable_pub, unused_qualifications)]
        pub mod resources {
            use ::diesel;
            pub use self::columns::*;
            use diesel::sql_types::*;
            /// Re-exports all of the columns of this table, as well as the
            /// table struct renamed to the module name. This is meant to be
            /// glob imported for functions which only deal with one table.
            pub mod dsl {
                pub use super::columns::id;
                pub use super::columns::room_id;
                pub use super::columns::name;
                pub use super::columns::size;
                pub use super::columns::key;
                pub use super::columns::length;
                pub use super::columns::creator_id;
                pub use super::columns::down_count;
                pub use super::columns::blank;
                pub use super::columns::created_at;
                pub use super::columns::updated_at;
                pub use super::columns::deleted_at;
                pub use super::table as resources;
            }
            #[allow(non_upper_case_globals, dead_code)]
            /// A tuple of all of the columns on this table
            pub const all_columns: (
                id,
                room_id,
                name,
                size,
                key,
                length,
                creator_id,
                down_count,
                blank,
                created_at,
                updated_at,
                deleted_at,
            ) = (
                id,
                room_id,
                name,
                size,
                key,
                length,
                creator_id,
                down_count,
                blank,
                created_at,
                updated_at,
                deleted_at,
            );
            #[allow(non_camel_case_types)]
            /// The actual table struct
            ///
            /// This is the type which provides the base methods of the query
            /// builder, such as `.select` and `.filter`.
            pub struct table;
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for table {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "table")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::clone::Clone for table {
                #[inline]
                fn clone(&self) -> table {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::marker::Copy for table {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for table {
                    type QueryId = table;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::default::Default for table {
                #[inline]
                fn default() -> table {
                    table {}
                }
            }
            impl table {
                #[allow(dead_code)]
                /// Represents `table_name.*`, which is sometimes necessary
                /// for efficient count queries. It cannot be used in place of
                /// `all_columns`
                pub fn star(&self) -> star {
                    star
                }
            }
            /// The SQL type of all of the columns on this table
            pub type SqlType = (
                Int4,
                Int4,
                Varchar,
                Int4,
                Varchar,
                Int4,
                Int4,
                Int4,
                Bool,
                Timestamp,
                Timestamp,
                Nullable<Timestamp>,
            );
            /// Helper type for representing a boxed query from this table
            pub type BoxedQuery<'a, DB, ST = SqlType> = diesel::internal::table_macro::BoxedSelectStatement<
                'a,
                ST,
                diesel::internal::table_macro::FromClause<table>,
                DB,
            >;
            impl diesel::QuerySource for table {
                type FromClause = diesel::internal::table_macro::StaticQueryFragmentInstance<
                    table,
                >;
                type DefaultSelection = <Self as diesel::Table>::AllColumns;
                fn from_clause(&self) -> Self::FromClause {
                    diesel::internal::table_macro::StaticQueryFragmentInstance::new()
                }
                fn default_selection(&self) -> Self::DefaultSelection {
                    use diesel::Table;
                    Self::all_columns()
                }
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for table
            where
                DB: diesel::backend::Backend,
                <table as diesel::internal::table_macro::StaticQueryFragment>::Component: diesel::query_builder::QueryFragment<
                    DB,
                >,
            {
                fn walk_ast<'b>(
                    &'b self,
                    __diesel_internal_pass: diesel::query_builder::AstPass<'_, 'b, DB>,
                ) -> diesel::result::QueryResult<()> {
                    <table as diesel::internal::table_macro::StaticQueryFragment>::STATIC_COMPONENT
                        .walk_ast(__diesel_internal_pass)
                }
            }
            impl diesel::internal::table_macro::StaticQueryFragment for table {
                type Component = diesel::internal::table_macro::Identifier<'static>;
                const STATIC_COMPONENT: &'static Self::Component = &diesel::internal::table_macro::Identifier(
                    "resources",
                );
            }
            impl diesel::query_builder::AsQuery for table {
                type SqlType = SqlType;
                type Query = diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<Self>,
                >;
                fn as_query(self) -> Self::Query {
                    diesel::internal::table_macro::SelectStatement::simple(self)
                }
            }
            impl diesel::Table for table {
                type PrimaryKey = id;
                type AllColumns = (
                    id,
                    room_id,
                    name,
                    size,
                    key,
                    length,
                    creator_id,
                    down_count,
                    blank,
                    created_at,
                    updated_at,
                    deleted_at,
                );
                fn primary_key(&self) -> Self::PrimaryKey {
                    id
                }
                fn all_columns() -> Self::AllColumns {
                    (
                        id,
                        room_id,
                        name,
                        size,
                        key,
                        length,
                        creator_id,
                        down_count,
                        blank,
                        created_at,
                        updated_at,
                        deleted_at,
                    )
                }
            }
            impl diesel::associations::HasTable for table {
                type Table = Self;
                fn table() -> Self::Table {
                    table
                }
            }
            impl diesel::query_builder::IntoUpdateTarget for table {
                type WhereClause = <<Self as diesel::query_builder::AsQuery>::Query as diesel::query_builder::IntoUpdateTarget>::WhereClause;
                fn into_update_target(
                    self,
                ) -> diesel::query_builder::UpdateTarget<
                    Self::Table,
                    Self::WhereClause,
                > {
                    use diesel::query_builder::AsQuery;
                    let q: diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<table>,
                    > = self.as_query();
                    q.into_update_target()
                }
            }
            impl diesel::query_source::AppearsInFromClause<table> for table {
                type Count = diesel::query_source::Once;
            }
            impl<S> diesel::internal::table_macro::AliasAppearsInFromClause<S, table>
            for table
            where
                S: diesel::query_source::AliasSource<Target = table>,
            {
                type Count = diesel::query_source::Never;
            }
            impl<
                S1,
                S2,
            > diesel::internal::table_macro::AliasAliasAppearsInFromClause<table, S2, S1>
            for table
            where
                S1: diesel::query_source::AliasSource<Target = table>,
                S2: diesel::query_source::AliasSource<Target = table>,
                S1: diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<
                    S2,
                    table,
                >,
            {
                type Count = <S1 as diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<
                    S2,
                    table,
                >>::Count;
            }
            impl<
                S,
            > diesel::query_source::AppearsInFromClause<diesel::query_source::Alias<S>>
            for table
            where
                S: diesel::query_source::AliasSource,
            {
                type Count = diesel::query_source::Never;
            }
            impl<
                S,
                C,
            > diesel::internal::table_macro::FieldAliasMapperAssociatedTypesDisjointnessTrick<
                table,
                S,
                C,
            > for table
            where
                S: diesel::query_source::AliasSource<Target = table>
                    + ::std::clone::Clone,
                C: diesel::query_source::Column<Table = table>,
            {
                type Out = diesel::query_source::AliasedField<S, C>;
                fn map(
                    __diesel_internal_column: C,
                    __diesel_internal_alias: &diesel::query_source::Alias<S>,
                ) -> Self::Out {
                    __diesel_internal_alias.field(__diesel_internal_column)
                }
            }
            impl diesel::query_source::AppearsInFromClause<table>
            for diesel::internal::table_macro::NoFromClause {
                type Count = diesel::query_source::Never;
            }
            impl<
                Left,
                Right,
                Kind,
            > diesel::JoinTo<diesel::internal::table_macro::Join<Left, Right, Kind>>
            for table
            where
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    Kind,
                >: diesel::JoinTo<table>,
                Left: diesel::query_source::QuerySource,
                Right: diesel::query_source::QuerySource,
            {
                type FromClause = diesel::internal::table_macro::Join<Left, Right, Kind>;
                type OnClause = <diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    Kind,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        Kind,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::Join::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                Join,
                On,
            > diesel::JoinTo<diesel::internal::table_macro::JoinOn<Join, On>> for table
            where
                diesel::internal::table_macro::JoinOn<Join, On>: diesel::JoinTo<table>,
            {
                type FromClause = diesel::internal::table_macro::JoinOn<Join, On>;
                type OnClause = <diesel::internal::table_macro::JoinOn<
                    Join,
                    On,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::JoinOn<
                        Join,
                        On,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::JoinOn::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                F,
                S,
                D,
                W,
                O,
                L,
                Of,
                G,
            > diesel::JoinTo<
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >,
            > for table
            where
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >: diesel::JoinTo<table>,
                F: diesel::query_source::QuerySource,
            {
                type FromClause = diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >;
                type OnClause = <diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<F>,
                        S,
                        D,
                        W,
                        O,
                        L,
                        Of,
                        G,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::SelectStatement::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                'a,
                QS,
                ST,
                DB,
            > diesel::JoinTo<
                diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >,
            > for table
            where
                diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >: diesel::JoinTo<table>,
                QS: diesel::query_source::QuerySource,
            {
                type FromClause = diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >;
                type OnClause = <diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::BoxedSelectStatement<
                        'a,
                        diesel::internal::table_macro::FromClause<QS>,
                        ST,
                        DB,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::BoxedSelectStatement::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<S> diesel::JoinTo<diesel::query_source::Alias<S>> for table
            where
                diesel::query_source::Alias<S>: diesel::JoinTo<table>,
            {
                type FromClause = diesel::query_source::Alias<S>;
                type OnClause = <diesel::query_source::Alias<
                    S,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::query_source::Alias<S>,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::query_source::Alias::<
                        S,
                    >::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<T> diesel::insertable::Insertable<T> for table
            where
                <table as diesel::query_builder::AsQuery>::Query: diesel::insertable::Insertable<
                    T,
                >,
            {
                type Values = <<table as diesel::query_builder::AsQuery>::Query as diesel::insertable::Insertable<
                    T,
                >>::Values;
                fn values(self) -> Self::Values {
                    use diesel::query_builder::AsQuery;
                    self.as_query().values()
                }
            }
            impl<'a, T> diesel::insertable::Insertable<T> for &'a table
            where
                table: diesel::insertable::Insertable<T>,
            {
                type Values = <table as diesel::insertable::Insertable<T>>::Values;
                fn values(self) -> Self::Values {
                    (*self).values()
                }
            }
            impl<S> diesel::JoinTo<diesel::query_builder::Only<S>> for table
            where
                diesel::query_builder::Only<S>: diesel::JoinTo<table>,
            {
                type FromClause = diesel::query_builder::Only<S>;
                type OnClause = <diesel::query_builder::Only<
                    S,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::query_builder::Only<S>,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::query_builder::Only::<
                        S,
                    >::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Only<table>,
            > for table {
                type Count = diesel::query_source::Once;
            }
            impl diesel::query_source::AppearsInFromClause<table>
            for diesel::query_builder::Only<table> {
                type Count = diesel::query_source::Once;
            }
            impl<S, TSM> diesel::JoinTo<diesel::query_builder::Tablesample<S, TSM>>
            for table
            where
                diesel::query_builder::Tablesample<S, TSM>: diesel::JoinTo<table>,
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type FromClause = diesel::query_builder::Tablesample<S, TSM>;
                type OnClause = <diesel::query_builder::Tablesample<
                    S,
                    TSM,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::query_builder::Tablesample<S, TSM>,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::query_builder::Tablesample::<
                        S,
                        TSM,
                    >::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                TSM,
            > diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<table, TSM>,
            > for table
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<TSM> diesel::query_source::AppearsInFromClause<table>
            for diesel::query_builder::Tablesample<table, TSM>
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            /// Contains all of the columns of this table
            pub mod columns {
                use ::diesel;
                use super::table;
                use diesel::sql_types::*;
                #[allow(non_camel_case_types, dead_code)]
                /// Represents `table_name.*`, which is sometimes needed for
                /// efficient count queries. It cannot be used in place of
                /// `all_columns`, and has a `SqlType` of `()` to prevent it
                /// being used that way
                pub struct star;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for star {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "star")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for star {
                    #[inline]
                    fn clone(&self) -> star {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for star {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for star {
                        type QueryId = star;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                impl<__GB> diesel::expression::ValidGrouping<__GB> for star
                where
                    (
                        id,
                        room_id,
                        name,
                        size,
                        key,
                        length,
                        creator_id,
                        down_count,
                        blank,
                        created_at,
                        updated_at,
                        deleted_at,
                    ): diesel::expression::ValidGrouping<__GB>,
                {
                    type IsAggregate = <(
                        id,
                        room_id,
                        name,
                        size,
                        key,
                        length,
                        creator_id,
                        down_count,
                        blank,
                        created_at,
                        updated_at,
                        deleted_at,
                    ) as diesel::expression::ValidGrouping<__GB>>::IsAggregate;
                }
                impl diesel::Expression for star {
                    type SqlType = diesel::expression::expression_types::NotSelectable;
                }
                impl<
                    DB: diesel::backend::Backend,
                > diesel::query_builder::QueryFragment<DB> for star
                where
                    <table as diesel::QuerySource>::FromClause: diesel::query_builder::QueryFragment<
                        DB,
                    >,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        use diesel::QuerySource;
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_sql("*");
                        Ok(())
                    }
                }
                impl diesel::SelectableExpression<table> for star {}
                impl diesel::AppearsOnTable<table> for star {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct id;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for id {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "id")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for id {
                    #[inline]
                    fn clone(&self) -> id {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for id {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for id {
                        type QueryId = id;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for id {
                    #[inline]
                    fn default() -> id {
                        id {}
                    }
                }
                impl diesel::expression::Expression for id {
                    type SqlType = Int4;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for id
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("id")
                    }
                }
                impl diesel::SelectableExpression<super::table> for id {}
                impl<QS> diesel::AppearsOnTable<QS> for id
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for id
                where
                    id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for id
                where
                    id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for id
                where
                    id: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for id
                where
                    From: diesel::query_source::QuerySource,
                    id: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for id
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        id,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for id {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for id {
                    type Table = super::table;
                    const NAME: &'static str = "id";
                }
                impl<T> diesel::EqAll<T> for id
                where
                    T: diesel::expression::AsExpression<Int4>,
                    diesel::dsl::Eq<
                        id,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Div<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Div<
                        Self,
                        Rhs::Expression,
                    >;
                    fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Div::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Mul<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Mul<
                        Self,
                        Rhs::Expression,
                    >;
                    fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Mul::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for id {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for id {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct room_id;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for room_id {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "room_id")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for room_id {
                    #[inline]
                    fn clone(&self) -> room_id {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for room_id {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for room_id {
                        type QueryId = room_id;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for room_id {
                    #[inline]
                    fn default() -> room_id {
                        room_id {}
                    }
                }
                impl diesel::expression::Expression for room_id {
                    type SqlType = Int4;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for room_id
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("room_id")
                    }
                }
                impl diesel::SelectableExpression<super::table> for room_id {}
                impl<QS> diesel::AppearsOnTable<QS> for room_id
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for room_id
                where
                    room_id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for room_id
                where
                    room_id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for room_id
                where
                    room_id: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for room_id
                where
                    From: diesel::query_source::QuerySource,
                    room_id: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for room_id
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        room_id,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for room_id {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<room_id> for room_id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for room_id {
                    type Table = super::table;
                    const NAME: &'static str = "room_id";
                }
                impl<T> diesel::EqAll<T> for room_id
                where
                    T: diesel::expression::AsExpression<Int4>,
                    diesel::dsl::Eq<
                        room_id,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for room_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<room_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for room_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<room_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Div<Rhs> for room_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<room_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Div<
                        Self,
                        Rhs::Expression,
                    >;
                    fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Div::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Mul<Rhs> for room_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<room_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Mul<
                        Self,
                        Rhs::Expression,
                    >;
                    fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Mul::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for room_id {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for room_id {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for room_id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for room_id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct name;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for name {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "name")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for name {
                    #[inline]
                    fn clone(&self) -> name {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for name {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for name {
                        type QueryId = name;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for name {
                    #[inline]
                    fn default() -> name {
                        name {}
                    }
                }
                impl diesel::expression::Expression for name {
                    type SqlType = Varchar;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for name
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("name")
                    }
                }
                impl diesel::SelectableExpression<super::table> for name {}
                impl<QS> diesel::AppearsOnTable<QS> for name
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for name
                where
                    name: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for name
                where
                    name: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for name
                where
                    name: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for name
                where
                    From: diesel::query_source::QuerySource,
                    name: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for name
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        name,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for name {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for name {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for name {
                    type Table = super::table;
                    const NAME: &'static str = "name";
                }
                impl<T> diesel::EqAll<T> for name
                where
                    T: diesel::expression::AsExpression<Varchar>,
                    diesel::dsl::Eq<
                        name,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for name {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for name {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for name
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for name
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct size;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for size {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "size")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for size {
                    #[inline]
                    fn clone(&self) -> size {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for size {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for size {
                        type QueryId = size;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for size {
                    #[inline]
                    fn default() -> size {
                        size {}
                    }
                }
                impl diesel::expression::Expression for size {
                    type SqlType = Int4;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for size
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("size")
                    }
                }
                impl diesel::SelectableExpression<super::table> for size {}
                impl<QS> diesel::AppearsOnTable<QS> for size
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for size
                where
                    size: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for size
                where
                    size: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for size
                where
                    size: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for size
                where
                    From: diesel::query_source::QuerySource,
                    size: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for size
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        size,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for size {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<size> for size {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for size {
                    type Table = super::table;
                    const NAME: &'static str = "size";
                }
                impl<T> diesel::EqAll<T> for size
                where
                    T: diesel::expression::AsExpression<Int4>,
                    diesel::dsl::Eq<
                        size,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for size
                where
                    Rhs: diesel::expression::AsExpression<
                        <<size as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for size
                where
                    Rhs: diesel::expression::AsExpression<
                        <<size as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Div<Rhs> for size
                where
                    Rhs: diesel::expression::AsExpression<
                        <<size as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Div<
                        Self,
                        Rhs::Expression,
                    >;
                    fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Div::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Mul<Rhs> for size
                where
                    Rhs: diesel::expression::AsExpression<
                        <<size as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Mul<
                        Self,
                        Rhs::Expression,
                    >;
                    fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Mul::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for size {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for size {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for size
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for size
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct key;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for key {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "key")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for key {
                    #[inline]
                    fn clone(&self) -> key {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for key {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for key {
                        type QueryId = key;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for key {
                    #[inline]
                    fn default() -> key {
                        key {}
                    }
                }
                impl diesel::expression::Expression for key {
                    type SqlType = Varchar;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for key
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("key")
                    }
                }
                impl diesel::SelectableExpression<super::table> for key {}
                impl<QS> diesel::AppearsOnTable<QS> for key
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for key
                where
                    key: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for key
                where
                    key: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for key
                where
                    key: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for key
                where
                    From: diesel::query_source::QuerySource,
                    key: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for key
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        key,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for key {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<key> for key {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for key {
                    type Table = super::table;
                    const NAME: &'static str = "key";
                }
                impl<T> diesel::EqAll<T> for key
                where
                    T: diesel::expression::AsExpression<Varchar>,
                    diesel::dsl::Eq<
                        key,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for key {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for key {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for key
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for key
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct length;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for length {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "length")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for length {
                    #[inline]
                    fn clone(&self) -> length {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for length {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for length {
                        type QueryId = length;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for length {
                    #[inline]
                    fn default() -> length {
                        length {}
                    }
                }
                impl diesel::expression::Expression for length {
                    type SqlType = Int4;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for length
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("length")
                    }
                }
                impl diesel::SelectableExpression<super::table> for length {}
                impl<QS> diesel::AppearsOnTable<QS> for length
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for length
                where
                    length: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for length
                where
                    length: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for length
                where
                    length: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for length
                where
                    From: diesel::query_source::QuerySource,
                    length: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for length
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        length,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for length {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<length> for length {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for length {
                    type Table = super::table;
                    const NAME: &'static str = "length";
                }
                impl<T> diesel::EqAll<T> for length
                where
                    T: diesel::expression::AsExpression<Int4>,
                    diesel::dsl::Eq<
                        length,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for length
                where
                    Rhs: diesel::expression::AsExpression<
                        <<length as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for length
                where
                    Rhs: diesel::expression::AsExpression<
                        <<length as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Div<Rhs> for length
                where
                    Rhs: diesel::expression::AsExpression<
                        <<length as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Div<
                        Self,
                        Rhs::Expression,
                    >;
                    fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Div::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Mul<Rhs> for length
                where
                    Rhs: diesel::expression::AsExpression<
                        <<length as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Mul<
                        Self,
                        Rhs::Expression,
                    >;
                    fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Mul::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for length {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for length {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for length
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for length
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct creator_id;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for creator_id {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "creator_id")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for creator_id {
                    #[inline]
                    fn clone(&self) -> creator_id {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for creator_id {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for creator_id {
                        type QueryId = creator_id;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for creator_id {
                    #[inline]
                    fn default() -> creator_id {
                        creator_id {}
                    }
                }
                impl diesel::expression::Expression for creator_id {
                    type SqlType = Int4;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for creator_id
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("creator_id")
                    }
                }
                impl diesel::SelectableExpression<super::table> for creator_id {}
                impl<QS> diesel::AppearsOnTable<QS> for creator_id
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for creator_id
                where
                    creator_id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for creator_id
                where
                    creator_id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for creator_id
                where
                    creator_id: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for creator_id
                where
                    From: diesel::query_source::QuerySource,
                    creator_id: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for creator_id
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        creator_id,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for creator_id {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id>
                for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for creator_id {
                    type Table = super::table;
                    const NAME: &'static str = "creator_id";
                }
                impl<T> diesel::EqAll<T> for creator_id
                where
                    T: diesel::expression::AsExpression<Int4>,
                    diesel::dsl::Eq<
                        creator_id,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for creator_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<creator_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for creator_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<creator_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Div<Rhs> for creator_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<creator_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Div<
                        Self,
                        Rhs::Expression,
                    >;
                    fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Div::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Mul<Rhs> for creator_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<creator_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Mul<
                        Self,
                        Rhs::Expression,
                    >;
                    fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Mul::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for creator_id {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for creator_id {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for creator_id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for creator_id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct down_count;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for down_count {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "down_count")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for down_count {
                    #[inline]
                    fn clone(&self) -> down_count {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for down_count {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for down_count {
                        type QueryId = down_count;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for down_count {
                    #[inline]
                    fn default() -> down_count {
                        down_count {}
                    }
                }
                impl diesel::expression::Expression for down_count {
                    type SqlType = Int4;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for down_count
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("down_count")
                    }
                }
                impl diesel::SelectableExpression<super::table> for down_count {}
                impl<QS> diesel::AppearsOnTable<QS> for down_count
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for down_count
                where
                    down_count: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for down_count
                where
                    down_count: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for down_count
                where
                    down_count: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for down_count
                where
                    From: diesel::query_source::QuerySource,
                    down_count: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for down_count
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        down_count,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for down_count {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<down_count>
                for down_count {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for down_count {
                    type Table = super::table;
                    const NAME: &'static str = "down_count";
                }
                impl<T> diesel::EqAll<T> for down_count
                where
                    T: diesel::expression::AsExpression<Int4>,
                    diesel::dsl::Eq<
                        down_count,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for down_count
                where
                    Rhs: diesel::expression::AsExpression<
                        <<down_count as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for down_count
                where
                    Rhs: diesel::expression::AsExpression<
                        <<down_count as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Div<Rhs> for down_count
                where
                    Rhs: diesel::expression::AsExpression<
                        <<down_count as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Div<
                        Self,
                        Rhs::Expression,
                    >;
                    fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Div::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Mul<Rhs> for down_count
                where
                    Rhs: diesel::expression::AsExpression<
                        <<down_count as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Mul<
                        Self,
                        Rhs::Expression,
                    >;
                    fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Mul::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for down_count {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for down_count {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for down_count
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for down_count
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct blank;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for blank {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "blank")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for blank {
                    #[inline]
                    fn clone(&self) -> blank {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for blank {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for blank {
                        type QueryId = blank;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for blank {
                    #[inline]
                    fn default() -> blank {
                        blank {}
                    }
                }
                impl diesel::expression::Expression for blank {
                    type SqlType = Bool;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for blank
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("blank")
                    }
                }
                impl diesel::SelectableExpression<super::table> for blank {}
                impl<QS> diesel::AppearsOnTable<QS> for blank
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for blank
                where
                    blank: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for blank
                where
                    blank: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for blank
                where
                    blank: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for blank
                where
                    From: diesel::query_source::QuerySource,
                    blank: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for blank
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        blank,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for blank {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for blank {
                    type Table = super::table;
                    const NAME: &'static str = "blank";
                }
                impl<T> diesel::EqAll<T> for blank
                where
                    T: diesel::expression::AsExpression<Bool>,
                    diesel::dsl::Eq<
                        blank,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for blank {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for blank {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for blank
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for blank
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct created_at;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for created_at {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "created_at")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for created_at {
                    #[inline]
                    fn clone(&self) -> created_at {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for created_at {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for created_at {
                        type QueryId = created_at;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for created_at {
                    #[inline]
                    fn default() -> created_at {
                        created_at {}
                    }
                }
                impl diesel::expression::Expression for created_at {
                    type SqlType = Timestamp;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for created_at
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("created_at")
                    }
                }
                impl diesel::SelectableExpression<super::table> for created_at {}
                impl<QS> diesel::AppearsOnTable<QS> for created_at
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for created_at
                where
                    created_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for created_at
                where
                    created_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for created_at
                where
                    created_at: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for created_at
                where
                    From: diesel::query_source::QuerySource,
                    created_at: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for created_at
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        created_at,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for created_at {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at>
                for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for created_at {
                    type Table = super::table;
                    const NAME: &'static str = "created_at";
                }
                impl<T> diesel::EqAll<T> for created_at
                where
                    T: diesel::expression::AsExpression<Timestamp>,
                    diesel::dsl::Eq<
                        created_at,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for created_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<created_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for created_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<created_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for created_at {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for created_at {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for created_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for created_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct updated_at;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for updated_at {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "updated_at")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for updated_at {
                    #[inline]
                    fn clone(&self) -> updated_at {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for updated_at {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for updated_at {
                        type QueryId = updated_at;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for updated_at {
                    #[inline]
                    fn default() -> updated_at {
                        updated_at {}
                    }
                }
                impl diesel::expression::Expression for updated_at {
                    type SqlType = Timestamp;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for updated_at
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("updated_at")
                    }
                }
                impl diesel::SelectableExpression<super::table> for updated_at {}
                impl<QS> diesel::AppearsOnTable<QS> for updated_at
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for updated_at
                where
                    updated_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for updated_at
                where
                    updated_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for updated_at
                where
                    updated_at: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for updated_at
                where
                    From: diesel::query_source::QuerySource,
                    updated_at: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for updated_at
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        updated_at,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for updated_at {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at>
                for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for updated_at {
                    type Table = super::table;
                    const NAME: &'static str = "updated_at";
                }
                impl<T> diesel::EqAll<T> for updated_at
                where
                    T: diesel::expression::AsExpression<Timestamp>,
                    diesel::dsl::Eq<
                        updated_at,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for updated_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<updated_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for updated_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<updated_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for updated_at {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for updated_at {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for updated_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for updated_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct deleted_at;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for deleted_at {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "deleted_at")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for deleted_at {
                    #[inline]
                    fn clone(&self) -> deleted_at {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for deleted_at {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for deleted_at {
                        type QueryId = deleted_at;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for deleted_at {
                    #[inline]
                    fn default() -> deleted_at {
                        deleted_at {}
                    }
                }
                impl diesel::expression::Expression for deleted_at {
                    type SqlType = Nullable<Timestamp>;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for deleted_at
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("deleted_at")
                    }
                }
                impl diesel::SelectableExpression<super::table> for deleted_at {}
                impl<QS> diesel::AppearsOnTable<QS> for deleted_at
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for deleted_at
                where
                    deleted_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for deleted_at
                where
                    deleted_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for deleted_at
                where
                    deleted_at: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for deleted_at
                where
                    From: diesel::query_source::QuerySource,
                    deleted_at: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for deleted_at
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        deleted_at,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for deleted_at {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at>
                for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for deleted_at {
                    type Table = super::table;
                    const NAME: &'static str = "deleted_at";
                }
                impl<T> diesel::EqAll<T> for deleted_at
                where
                    T: diesel::expression::AsExpression<Nullable<Timestamp>>,
                    diesel::dsl::Eq<
                        deleted_at,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for deleted_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<deleted_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for deleted_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<deleted_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for deleted_at {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for deleted_at {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for deleted_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for deleted_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                impl diesel::expression::IsContainedInGroupBy<id> for room_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<room_id> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for size {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<size> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for key {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<key> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for length {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<length> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for down_count {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<down_count> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<room_id> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for room_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<room_id> for size {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<size> for room_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<room_id> for key {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<key> for room_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<room_id> for length {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<length> for room_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<room_id> for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id> for room_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<room_id> for down_count {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<down_count> for room_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<room_id> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for room_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<room_id> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for room_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<room_id> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for room_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<room_id> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for room_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for size {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<size> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for key {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<key> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for length {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<length> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for down_count {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<down_count> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<size> for key {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<key> for size {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<size> for length {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<length> for size {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<size> for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id> for size {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<size> for down_count {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<down_count> for size {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<size> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for size {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<size> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for size {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<size> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for size {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<size> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for size {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<key> for length {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<length> for key {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<key> for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id> for key {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<key> for down_count {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<down_count> for key {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<key> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for key {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<key> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for key {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<key> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for key {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<key> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for key {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<length> for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id> for length {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<length> for down_count {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<down_count> for length {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<length> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for length {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<length> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for length {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<length> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for length {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<length> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for length {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id>
                for down_count {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<down_count>
                for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id>
                for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at>
                for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id>
                for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at>
                for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id>
                for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at>
                for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<down_count> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for down_count {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<down_count>
                for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at>
                for down_count {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<down_count>
                for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at>
                for down_count {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<down_count>
                for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at>
                for down_count {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at>
                for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at>
                for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at>
                for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at>
                for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at>
                for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at>
                for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
            }
        }
        #[allow(unused_imports, dead_code, unreachable_pub, unused_qualifications)]
        pub mod rooms {
            use ::diesel;
            pub use self::columns::*;
            use diesel::sql_types::*;
            /// Re-exports all of the columns of this table, as well as the
            /// table struct renamed to the module name. This is meant to be
            /// glob imported for functions which only deal with one table.
            pub mod dsl {
                pub use super::columns::id;
                pub use super::columns::name;
                pub use super::columns::blank;
                pub use super::columns::creator_id;
                pub use super::columns::created_at;
                pub use super::columns::updated_at;
                pub use super::columns::deleted_at;
                pub use super::table as rooms;
            }
            #[allow(non_upper_case_globals, dead_code)]
            /// A tuple of all of the columns on this table
            pub const all_columns: (
                id,
                name,
                blank,
                creator_id,
                created_at,
                updated_at,
                deleted_at,
            ) = (id, name, blank, creator_id, created_at, updated_at, deleted_at);
            #[allow(non_camel_case_types)]
            /// The actual table struct
            ///
            /// This is the type which provides the base methods of the query
            /// builder, such as `.select` and `.filter`.
            pub struct table;
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for table {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "table")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::clone::Clone for table {
                #[inline]
                fn clone(&self) -> table {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::marker::Copy for table {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for table {
                    type QueryId = table;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::default::Default for table {
                #[inline]
                fn default() -> table {
                    table {}
                }
            }
            impl table {
                #[allow(dead_code)]
                /// Represents `table_name.*`, which is sometimes necessary
                /// for efficient count queries. It cannot be used in place of
                /// `all_columns`
                pub fn star(&self) -> star {
                    star
                }
            }
            /// The SQL type of all of the columns on this table
            pub type SqlType = (
                Int4,
                Varchar,
                Bool,
                Int4,
                Timestamp,
                Timestamp,
                Nullable<Timestamp>,
            );
            /// Helper type for representing a boxed query from this table
            pub type BoxedQuery<'a, DB, ST = SqlType> = diesel::internal::table_macro::BoxedSelectStatement<
                'a,
                ST,
                diesel::internal::table_macro::FromClause<table>,
                DB,
            >;
            impl diesel::QuerySource for table {
                type FromClause = diesel::internal::table_macro::StaticQueryFragmentInstance<
                    table,
                >;
                type DefaultSelection = <Self as diesel::Table>::AllColumns;
                fn from_clause(&self) -> Self::FromClause {
                    diesel::internal::table_macro::StaticQueryFragmentInstance::new()
                }
                fn default_selection(&self) -> Self::DefaultSelection {
                    use diesel::Table;
                    Self::all_columns()
                }
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for table
            where
                DB: diesel::backend::Backend,
                <table as diesel::internal::table_macro::StaticQueryFragment>::Component: diesel::query_builder::QueryFragment<
                    DB,
                >,
            {
                fn walk_ast<'b>(
                    &'b self,
                    __diesel_internal_pass: diesel::query_builder::AstPass<'_, 'b, DB>,
                ) -> diesel::result::QueryResult<()> {
                    <table as diesel::internal::table_macro::StaticQueryFragment>::STATIC_COMPONENT
                        .walk_ast(__diesel_internal_pass)
                }
            }
            impl diesel::internal::table_macro::StaticQueryFragment for table {
                type Component = diesel::internal::table_macro::Identifier<'static>;
                const STATIC_COMPONENT: &'static Self::Component = &diesel::internal::table_macro::Identifier(
                    "rooms",
                );
            }
            impl diesel::query_builder::AsQuery for table {
                type SqlType = SqlType;
                type Query = diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<Self>,
                >;
                fn as_query(self) -> Self::Query {
                    diesel::internal::table_macro::SelectStatement::simple(self)
                }
            }
            impl diesel::Table for table {
                type PrimaryKey = id;
                type AllColumns = (
                    id,
                    name,
                    blank,
                    creator_id,
                    created_at,
                    updated_at,
                    deleted_at,
                );
                fn primary_key(&self) -> Self::PrimaryKey {
                    id
                }
                fn all_columns() -> Self::AllColumns {
                    (id, name, blank, creator_id, created_at, updated_at, deleted_at)
                }
            }
            impl diesel::associations::HasTable for table {
                type Table = Self;
                fn table() -> Self::Table {
                    table
                }
            }
            impl diesel::query_builder::IntoUpdateTarget for table {
                type WhereClause = <<Self as diesel::query_builder::AsQuery>::Query as diesel::query_builder::IntoUpdateTarget>::WhereClause;
                fn into_update_target(
                    self,
                ) -> diesel::query_builder::UpdateTarget<
                    Self::Table,
                    Self::WhereClause,
                > {
                    use diesel::query_builder::AsQuery;
                    let q: diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<table>,
                    > = self.as_query();
                    q.into_update_target()
                }
            }
            impl diesel::query_source::AppearsInFromClause<table> for table {
                type Count = diesel::query_source::Once;
            }
            impl<S> diesel::internal::table_macro::AliasAppearsInFromClause<S, table>
            for table
            where
                S: diesel::query_source::AliasSource<Target = table>,
            {
                type Count = diesel::query_source::Never;
            }
            impl<
                S1,
                S2,
            > diesel::internal::table_macro::AliasAliasAppearsInFromClause<table, S2, S1>
            for table
            where
                S1: diesel::query_source::AliasSource<Target = table>,
                S2: diesel::query_source::AliasSource<Target = table>,
                S1: diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<
                    S2,
                    table,
                >,
            {
                type Count = <S1 as diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<
                    S2,
                    table,
                >>::Count;
            }
            impl<
                S,
            > diesel::query_source::AppearsInFromClause<diesel::query_source::Alias<S>>
            for table
            where
                S: diesel::query_source::AliasSource,
            {
                type Count = diesel::query_source::Never;
            }
            impl<
                S,
                C,
            > diesel::internal::table_macro::FieldAliasMapperAssociatedTypesDisjointnessTrick<
                table,
                S,
                C,
            > for table
            where
                S: diesel::query_source::AliasSource<Target = table>
                    + ::std::clone::Clone,
                C: diesel::query_source::Column<Table = table>,
            {
                type Out = diesel::query_source::AliasedField<S, C>;
                fn map(
                    __diesel_internal_column: C,
                    __diesel_internal_alias: &diesel::query_source::Alias<S>,
                ) -> Self::Out {
                    __diesel_internal_alias.field(__diesel_internal_column)
                }
            }
            impl diesel::query_source::AppearsInFromClause<table>
            for diesel::internal::table_macro::NoFromClause {
                type Count = diesel::query_source::Never;
            }
            impl<
                Left,
                Right,
                Kind,
            > diesel::JoinTo<diesel::internal::table_macro::Join<Left, Right, Kind>>
            for table
            where
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    Kind,
                >: diesel::JoinTo<table>,
                Left: diesel::query_source::QuerySource,
                Right: diesel::query_source::QuerySource,
            {
                type FromClause = diesel::internal::table_macro::Join<Left, Right, Kind>;
                type OnClause = <diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    Kind,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        Kind,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::Join::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                Join,
                On,
            > diesel::JoinTo<diesel::internal::table_macro::JoinOn<Join, On>> for table
            where
                diesel::internal::table_macro::JoinOn<Join, On>: diesel::JoinTo<table>,
            {
                type FromClause = diesel::internal::table_macro::JoinOn<Join, On>;
                type OnClause = <diesel::internal::table_macro::JoinOn<
                    Join,
                    On,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::JoinOn<
                        Join,
                        On,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::JoinOn::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                F,
                S,
                D,
                W,
                O,
                L,
                Of,
                G,
            > diesel::JoinTo<
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >,
            > for table
            where
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >: diesel::JoinTo<table>,
                F: diesel::query_source::QuerySource,
            {
                type FromClause = diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >;
                type OnClause = <diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<F>,
                        S,
                        D,
                        W,
                        O,
                        L,
                        Of,
                        G,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::SelectStatement::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                'a,
                QS,
                ST,
                DB,
            > diesel::JoinTo<
                diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >,
            > for table
            where
                diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >: diesel::JoinTo<table>,
                QS: diesel::query_source::QuerySource,
            {
                type FromClause = diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >;
                type OnClause = <diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::BoxedSelectStatement<
                        'a,
                        diesel::internal::table_macro::FromClause<QS>,
                        ST,
                        DB,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::BoxedSelectStatement::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<S> diesel::JoinTo<diesel::query_source::Alias<S>> for table
            where
                diesel::query_source::Alias<S>: diesel::JoinTo<table>,
            {
                type FromClause = diesel::query_source::Alias<S>;
                type OnClause = <diesel::query_source::Alias<
                    S,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::query_source::Alias<S>,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::query_source::Alias::<
                        S,
                    >::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<T> diesel::insertable::Insertable<T> for table
            where
                <table as diesel::query_builder::AsQuery>::Query: diesel::insertable::Insertable<
                    T,
                >,
            {
                type Values = <<table as diesel::query_builder::AsQuery>::Query as diesel::insertable::Insertable<
                    T,
                >>::Values;
                fn values(self) -> Self::Values {
                    use diesel::query_builder::AsQuery;
                    self.as_query().values()
                }
            }
            impl<'a, T> diesel::insertable::Insertable<T> for &'a table
            where
                table: diesel::insertable::Insertable<T>,
            {
                type Values = <table as diesel::insertable::Insertable<T>>::Values;
                fn values(self) -> Self::Values {
                    (*self).values()
                }
            }
            impl<S> diesel::JoinTo<diesel::query_builder::Only<S>> for table
            where
                diesel::query_builder::Only<S>: diesel::JoinTo<table>,
            {
                type FromClause = diesel::query_builder::Only<S>;
                type OnClause = <diesel::query_builder::Only<
                    S,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::query_builder::Only<S>,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::query_builder::Only::<
                        S,
                    >::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Only<table>,
            > for table {
                type Count = diesel::query_source::Once;
            }
            impl diesel::query_source::AppearsInFromClause<table>
            for diesel::query_builder::Only<table> {
                type Count = diesel::query_source::Once;
            }
            impl<S, TSM> diesel::JoinTo<diesel::query_builder::Tablesample<S, TSM>>
            for table
            where
                diesel::query_builder::Tablesample<S, TSM>: diesel::JoinTo<table>,
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type FromClause = diesel::query_builder::Tablesample<S, TSM>;
                type OnClause = <diesel::query_builder::Tablesample<
                    S,
                    TSM,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::query_builder::Tablesample<S, TSM>,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::query_builder::Tablesample::<
                        S,
                        TSM,
                    >::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                TSM,
            > diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<table, TSM>,
            > for table
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<TSM> diesel::query_source::AppearsInFromClause<table>
            for diesel::query_builder::Tablesample<table, TSM>
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            /// Contains all of the columns of this table
            pub mod columns {
                use ::diesel;
                use super::table;
                use diesel::sql_types::*;
                #[allow(non_camel_case_types, dead_code)]
                /// Represents `table_name.*`, which is sometimes needed for
                /// efficient count queries. It cannot be used in place of
                /// `all_columns`, and has a `SqlType` of `()` to prevent it
                /// being used that way
                pub struct star;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for star {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "star")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for star {
                    #[inline]
                    fn clone(&self) -> star {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for star {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for star {
                        type QueryId = star;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                impl<__GB> diesel::expression::ValidGrouping<__GB> for star
                where
                    (
                        id,
                        name,
                        blank,
                        creator_id,
                        created_at,
                        updated_at,
                        deleted_at,
                    ): diesel::expression::ValidGrouping<__GB>,
                {
                    type IsAggregate = <(
                        id,
                        name,
                        blank,
                        creator_id,
                        created_at,
                        updated_at,
                        deleted_at,
                    ) as diesel::expression::ValidGrouping<__GB>>::IsAggregate;
                }
                impl diesel::Expression for star {
                    type SqlType = diesel::expression::expression_types::NotSelectable;
                }
                impl<
                    DB: diesel::backend::Backend,
                > diesel::query_builder::QueryFragment<DB> for star
                where
                    <table as diesel::QuerySource>::FromClause: diesel::query_builder::QueryFragment<
                        DB,
                    >,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        use diesel::QuerySource;
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_sql("*");
                        Ok(())
                    }
                }
                impl diesel::SelectableExpression<table> for star {}
                impl diesel::AppearsOnTable<table> for star {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct id;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for id {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "id")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for id {
                    #[inline]
                    fn clone(&self) -> id {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for id {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for id {
                        type QueryId = id;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for id {
                    #[inline]
                    fn default() -> id {
                        id {}
                    }
                }
                impl diesel::expression::Expression for id {
                    type SqlType = Int4;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for id
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("id")
                    }
                }
                impl diesel::SelectableExpression<super::table> for id {}
                impl<QS> diesel::AppearsOnTable<QS> for id
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for id
                where
                    id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for id
                where
                    id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for id
                where
                    id: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for id
                where
                    From: diesel::query_source::QuerySource,
                    id: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for id
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        id,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for id {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for id {
                    type Table = super::table;
                    const NAME: &'static str = "id";
                }
                impl<T> diesel::EqAll<T> for id
                where
                    T: diesel::expression::AsExpression<Int4>,
                    diesel::dsl::Eq<
                        id,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Div<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Div<
                        Self,
                        Rhs::Expression,
                    >;
                    fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Div::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Mul<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Mul<
                        Self,
                        Rhs::Expression,
                    >;
                    fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Mul::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for id {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for id {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct name;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for name {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "name")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for name {
                    #[inline]
                    fn clone(&self) -> name {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for name {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for name {
                        type QueryId = name;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for name {
                    #[inline]
                    fn default() -> name {
                        name {}
                    }
                }
                impl diesel::expression::Expression for name {
                    type SqlType = Varchar;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for name
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("name")
                    }
                }
                impl diesel::SelectableExpression<super::table> for name {}
                impl<QS> diesel::AppearsOnTable<QS> for name
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for name
                where
                    name: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for name
                where
                    name: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for name
                where
                    name: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for name
                where
                    From: diesel::query_source::QuerySource,
                    name: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for name
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        name,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for name {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for name {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for name {
                    type Table = super::table;
                    const NAME: &'static str = "name";
                }
                impl<T> diesel::EqAll<T> for name
                where
                    T: diesel::expression::AsExpression<Varchar>,
                    diesel::dsl::Eq<
                        name,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for name {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for name {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for name
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for name
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct blank;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for blank {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "blank")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for blank {
                    #[inline]
                    fn clone(&self) -> blank {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for blank {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for blank {
                        type QueryId = blank;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for blank {
                    #[inline]
                    fn default() -> blank {
                        blank {}
                    }
                }
                impl diesel::expression::Expression for blank {
                    type SqlType = Bool;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for blank
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("blank")
                    }
                }
                impl diesel::SelectableExpression<super::table> for blank {}
                impl<QS> diesel::AppearsOnTable<QS> for blank
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for blank
                where
                    blank: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for blank
                where
                    blank: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for blank
                where
                    blank: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for blank
                where
                    From: diesel::query_source::QuerySource,
                    blank: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for blank
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        blank,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for blank {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for blank {
                    type Table = super::table;
                    const NAME: &'static str = "blank";
                }
                impl<T> diesel::EqAll<T> for blank
                where
                    T: diesel::expression::AsExpression<Bool>,
                    diesel::dsl::Eq<
                        blank,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for blank {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for blank {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for blank
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for blank
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct creator_id;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for creator_id {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "creator_id")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for creator_id {
                    #[inline]
                    fn clone(&self) -> creator_id {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for creator_id {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for creator_id {
                        type QueryId = creator_id;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for creator_id {
                    #[inline]
                    fn default() -> creator_id {
                        creator_id {}
                    }
                }
                impl diesel::expression::Expression for creator_id {
                    type SqlType = Int4;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for creator_id
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("creator_id")
                    }
                }
                impl diesel::SelectableExpression<super::table> for creator_id {}
                impl<QS> diesel::AppearsOnTable<QS> for creator_id
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for creator_id
                where
                    creator_id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for creator_id
                where
                    creator_id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for creator_id
                where
                    creator_id: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for creator_id
                where
                    From: diesel::query_source::QuerySource,
                    creator_id: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for creator_id
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        creator_id,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for creator_id {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id>
                for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for creator_id {
                    type Table = super::table;
                    const NAME: &'static str = "creator_id";
                }
                impl<T> diesel::EqAll<T> for creator_id
                where
                    T: diesel::expression::AsExpression<Int4>,
                    diesel::dsl::Eq<
                        creator_id,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for creator_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<creator_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for creator_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<creator_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Div<Rhs> for creator_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<creator_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Div<
                        Self,
                        Rhs::Expression,
                    >;
                    fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Div::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Mul<Rhs> for creator_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<creator_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Mul<
                        Self,
                        Rhs::Expression,
                    >;
                    fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Mul::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for creator_id {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for creator_id {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for creator_id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for creator_id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct created_at;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for created_at {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "created_at")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for created_at {
                    #[inline]
                    fn clone(&self) -> created_at {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for created_at {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for created_at {
                        type QueryId = created_at;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for created_at {
                    #[inline]
                    fn default() -> created_at {
                        created_at {}
                    }
                }
                impl diesel::expression::Expression for created_at {
                    type SqlType = Timestamp;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for created_at
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("created_at")
                    }
                }
                impl diesel::SelectableExpression<super::table> for created_at {}
                impl<QS> diesel::AppearsOnTable<QS> for created_at
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for created_at
                where
                    created_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for created_at
                where
                    created_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for created_at
                where
                    created_at: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for created_at
                where
                    From: diesel::query_source::QuerySource,
                    created_at: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for created_at
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        created_at,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for created_at {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at>
                for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for created_at {
                    type Table = super::table;
                    const NAME: &'static str = "created_at";
                }
                impl<T> diesel::EqAll<T> for created_at
                where
                    T: diesel::expression::AsExpression<Timestamp>,
                    diesel::dsl::Eq<
                        created_at,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for created_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<created_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for created_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<created_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for created_at {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for created_at {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for created_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for created_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct updated_at;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for updated_at {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "updated_at")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for updated_at {
                    #[inline]
                    fn clone(&self) -> updated_at {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for updated_at {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for updated_at {
                        type QueryId = updated_at;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for updated_at {
                    #[inline]
                    fn default() -> updated_at {
                        updated_at {}
                    }
                }
                impl diesel::expression::Expression for updated_at {
                    type SqlType = Timestamp;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for updated_at
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("updated_at")
                    }
                }
                impl diesel::SelectableExpression<super::table> for updated_at {}
                impl<QS> diesel::AppearsOnTable<QS> for updated_at
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for updated_at
                where
                    updated_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for updated_at
                where
                    updated_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for updated_at
                where
                    updated_at: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for updated_at
                where
                    From: diesel::query_source::QuerySource,
                    updated_at: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for updated_at
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        updated_at,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for updated_at {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at>
                for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for updated_at {
                    type Table = super::table;
                    const NAME: &'static str = "updated_at";
                }
                impl<T> diesel::EqAll<T> for updated_at
                where
                    T: diesel::expression::AsExpression<Timestamp>,
                    diesel::dsl::Eq<
                        updated_at,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for updated_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<updated_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for updated_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<updated_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for updated_at {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for updated_at {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for updated_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for updated_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct deleted_at;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for deleted_at {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "deleted_at")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for deleted_at {
                    #[inline]
                    fn clone(&self) -> deleted_at {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for deleted_at {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for deleted_at {
                        type QueryId = deleted_at;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for deleted_at {
                    #[inline]
                    fn default() -> deleted_at {
                        deleted_at {}
                    }
                }
                impl diesel::expression::Expression for deleted_at {
                    type SqlType = Nullable<Timestamp>;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for deleted_at
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("deleted_at")
                    }
                }
                impl diesel::SelectableExpression<super::table> for deleted_at {}
                impl<QS> diesel::AppearsOnTable<QS> for deleted_at
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for deleted_at
                where
                    deleted_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for deleted_at
                where
                    deleted_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for deleted_at
                where
                    deleted_at: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for deleted_at
                where
                    From: diesel::query_source::QuerySource,
                    deleted_at: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for deleted_at
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        deleted_at,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for deleted_at {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at>
                for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for deleted_at {
                    type Table = super::table;
                    const NAME: &'static str = "deleted_at";
                }
                impl<T> diesel::EqAll<T> for deleted_at
                where
                    T: diesel::expression::AsExpression<Nullable<Timestamp>>,
                    diesel::dsl::Eq<
                        deleted_at,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for deleted_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<deleted_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for deleted_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<deleted_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for deleted_at {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for deleted_at {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for deleted_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for deleted_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                impl diesel::expression::IsContainedInGroupBy<id> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id>
                for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at>
                for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id>
                for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at>
                for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<creator_id>
                for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at>
                for creator_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at>
                for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at>
                for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at>
                for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at>
                for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at>
                for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at>
                for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
            }
        }
        #[allow(unused_imports, dead_code, unreachable_pub, unused_qualifications)]
        pub mod users {
            use ::diesel;
            pub use self::columns::*;
            use diesel::sql_types::*;
            /// Re-exports all of the columns of this table, as well as the
            /// table struct renamed to the module name. This is meant to be
            /// glob imported for functions which only deal with one table.
            pub mod dsl {
                pub use super::columns::id;
                pub use super::columns::unionid;
                pub use super::columns::platform;
                pub use super::columns::openid;
                pub use super::columns::name;
                pub use super::columns::derive;
                pub use super::columns::out_ip;
                pub use super::columns::in_ip;
                pub use super::columns::blank;
                pub use super::columns::created_at;
                pub use super::columns::updated_at;
                pub use super::columns::deleted_at;
                pub use super::table as users;
            }
            #[allow(non_upper_case_globals, dead_code)]
            /// A tuple of all of the columns on this table
            pub const all_columns: (
                id,
                unionid,
                platform,
                openid,
                name,
                derive,
                out_ip,
                in_ip,
                blank,
                created_at,
                updated_at,
                deleted_at,
            ) = (
                id,
                unionid,
                platform,
                openid,
                name,
                derive,
                out_ip,
                in_ip,
                blank,
                created_at,
                updated_at,
                deleted_at,
            );
            #[allow(non_camel_case_types)]
            /// The actual table struct
            ///
            /// This is the type which provides the base methods of the query
            /// builder, such as `.select` and `.filter`.
            pub struct table;
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for table {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "table")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::clone::Clone for table {
                #[inline]
                fn clone(&self) -> table {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::marker::Copy for table {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for table {
                    type QueryId = table;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::default::Default for table {
                #[inline]
                fn default() -> table {
                    table {}
                }
            }
            impl table {
                #[allow(dead_code)]
                /// Represents `table_name.*`, which is sometimes necessary
                /// for efficient count queries. It cannot be used in place of
                /// `all_columns`
                pub fn star(&self) -> star {
                    star
                }
            }
            /// The SQL type of all of the columns on this table
            pub type SqlType = (
                Int4,
                Varchar,
                Varchar,
                Varchar,
                Varchar,
                Varchar,
                Varchar,
                Varchar,
                Bool,
                Timestamp,
                Timestamp,
                Nullable<Timestamp>,
            );
            /// Helper type for representing a boxed query from this table
            pub type BoxedQuery<'a, DB, ST = SqlType> = diesel::internal::table_macro::BoxedSelectStatement<
                'a,
                ST,
                diesel::internal::table_macro::FromClause<table>,
                DB,
            >;
            impl diesel::QuerySource for table {
                type FromClause = diesel::internal::table_macro::StaticQueryFragmentInstance<
                    table,
                >;
                type DefaultSelection = <Self as diesel::Table>::AllColumns;
                fn from_clause(&self) -> Self::FromClause {
                    diesel::internal::table_macro::StaticQueryFragmentInstance::new()
                }
                fn default_selection(&self) -> Self::DefaultSelection {
                    use diesel::Table;
                    Self::all_columns()
                }
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for table
            where
                DB: diesel::backend::Backend,
                <table as diesel::internal::table_macro::StaticQueryFragment>::Component: diesel::query_builder::QueryFragment<
                    DB,
                >,
            {
                fn walk_ast<'b>(
                    &'b self,
                    __diesel_internal_pass: diesel::query_builder::AstPass<'_, 'b, DB>,
                ) -> diesel::result::QueryResult<()> {
                    <table as diesel::internal::table_macro::StaticQueryFragment>::STATIC_COMPONENT
                        .walk_ast(__diesel_internal_pass)
                }
            }
            impl diesel::internal::table_macro::StaticQueryFragment for table {
                type Component = diesel::internal::table_macro::Identifier<'static>;
                const STATIC_COMPONENT: &'static Self::Component = &diesel::internal::table_macro::Identifier(
                    "users",
                );
            }
            impl diesel::query_builder::AsQuery for table {
                type SqlType = SqlType;
                type Query = diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<Self>,
                >;
                fn as_query(self) -> Self::Query {
                    diesel::internal::table_macro::SelectStatement::simple(self)
                }
            }
            impl diesel::Table for table {
                type PrimaryKey = id;
                type AllColumns = (
                    id,
                    unionid,
                    platform,
                    openid,
                    name,
                    derive,
                    out_ip,
                    in_ip,
                    blank,
                    created_at,
                    updated_at,
                    deleted_at,
                );
                fn primary_key(&self) -> Self::PrimaryKey {
                    id
                }
                fn all_columns() -> Self::AllColumns {
                    (
                        id,
                        unionid,
                        platform,
                        openid,
                        name,
                        derive,
                        out_ip,
                        in_ip,
                        blank,
                        created_at,
                        updated_at,
                        deleted_at,
                    )
                }
            }
            impl diesel::associations::HasTable for table {
                type Table = Self;
                fn table() -> Self::Table {
                    table
                }
            }
            impl diesel::query_builder::IntoUpdateTarget for table {
                type WhereClause = <<Self as diesel::query_builder::AsQuery>::Query as diesel::query_builder::IntoUpdateTarget>::WhereClause;
                fn into_update_target(
                    self,
                ) -> diesel::query_builder::UpdateTarget<
                    Self::Table,
                    Self::WhereClause,
                > {
                    use diesel::query_builder::AsQuery;
                    let q: diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<table>,
                    > = self.as_query();
                    q.into_update_target()
                }
            }
            impl diesel::query_source::AppearsInFromClause<table> for table {
                type Count = diesel::query_source::Once;
            }
            impl<S> diesel::internal::table_macro::AliasAppearsInFromClause<S, table>
            for table
            where
                S: diesel::query_source::AliasSource<Target = table>,
            {
                type Count = diesel::query_source::Never;
            }
            impl<
                S1,
                S2,
            > diesel::internal::table_macro::AliasAliasAppearsInFromClause<table, S2, S1>
            for table
            where
                S1: diesel::query_source::AliasSource<Target = table>,
                S2: diesel::query_source::AliasSource<Target = table>,
                S1: diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<
                    S2,
                    table,
                >,
            {
                type Count = <S1 as diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<
                    S2,
                    table,
                >>::Count;
            }
            impl<
                S,
            > diesel::query_source::AppearsInFromClause<diesel::query_source::Alias<S>>
            for table
            where
                S: diesel::query_source::AliasSource,
            {
                type Count = diesel::query_source::Never;
            }
            impl<
                S,
                C,
            > diesel::internal::table_macro::FieldAliasMapperAssociatedTypesDisjointnessTrick<
                table,
                S,
                C,
            > for table
            where
                S: diesel::query_source::AliasSource<Target = table>
                    + ::std::clone::Clone,
                C: diesel::query_source::Column<Table = table>,
            {
                type Out = diesel::query_source::AliasedField<S, C>;
                fn map(
                    __diesel_internal_column: C,
                    __diesel_internal_alias: &diesel::query_source::Alias<S>,
                ) -> Self::Out {
                    __diesel_internal_alias.field(__diesel_internal_column)
                }
            }
            impl diesel::query_source::AppearsInFromClause<table>
            for diesel::internal::table_macro::NoFromClause {
                type Count = diesel::query_source::Never;
            }
            impl<
                Left,
                Right,
                Kind,
            > diesel::JoinTo<diesel::internal::table_macro::Join<Left, Right, Kind>>
            for table
            where
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    Kind,
                >: diesel::JoinTo<table>,
                Left: diesel::query_source::QuerySource,
                Right: diesel::query_source::QuerySource,
            {
                type FromClause = diesel::internal::table_macro::Join<Left, Right, Kind>;
                type OnClause = <diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    Kind,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        Kind,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::Join::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                Join,
                On,
            > diesel::JoinTo<diesel::internal::table_macro::JoinOn<Join, On>> for table
            where
                diesel::internal::table_macro::JoinOn<Join, On>: diesel::JoinTo<table>,
            {
                type FromClause = diesel::internal::table_macro::JoinOn<Join, On>;
                type OnClause = <diesel::internal::table_macro::JoinOn<
                    Join,
                    On,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::JoinOn<
                        Join,
                        On,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::JoinOn::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                F,
                S,
                D,
                W,
                O,
                L,
                Of,
                G,
            > diesel::JoinTo<
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >,
            > for table
            where
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >: diesel::JoinTo<table>,
                F: diesel::query_source::QuerySource,
            {
                type FromClause = diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >;
                type OnClause = <diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<F>,
                        S,
                        D,
                        W,
                        O,
                        L,
                        Of,
                        G,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::SelectStatement::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                'a,
                QS,
                ST,
                DB,
            > diesel::JoinTo<
                diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >,
            > for table
            where
                diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >: diesel::JoinTo<table>,
                QS: diesel::query_source::QuerySource,
            {
                type FromClause = diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >;
                type OnClause = <diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::BoxedSelectStatement<
                        'a,
                        diesel::internal::table_macro::FromClause<QS>,
                        ST,
                        DB,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::BoxedSelectStatement::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<S> diesel::JoinTo<diesel::query_source::Alias<S>> for table
            where
                diesel::query_source::Alias<S>: diesel::JoinTo<table>,
            {
                type FromClause = diesel::query_source::Alias<S>;
                type OnClause = <diesel::query_source::Alias<
                    S,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::query_source::Alias<S>,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::query_source::Alias::<
                        S,
                    >::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<T> diesel::insertable::Insertable<T> for table
            where
                <table as diesel::query_builder::AsQuery>::Query: diesel::insertable::Insertable<
                    T,
                >,
            {
                type Values = <<table as diesel::query_builder::AsQuery>::Query as diesel::insertable::Insertable<
                    T,
                >>::Values;
                fn values(self) -> Self::Values {
                    use diesel::query_builder::AsQuery;
                    self.as_query().values()
                }
            }
            impl<'a, T> diesel::insertable::Insertable<T> for &'a table
            where
                table: diesel::insertable::Insertable<T>,
            {
                type Values = <table as diesel::insertable::Insertable<T>>::Values;
                fn values(self) -> Self::Values {
                    (*self).values()
                }
            }
            impl<S> diesel::JoinTo<diesel::query_builder::Only<S>> for table
            where
                diesel::query_builder::Only<S>: diesel::JoinTo<table>,
            {
                type FromClause = diesel::query_builder::Only<S>;
                type OnClause = <diesel::query_builder::Only<
                    S,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::query_builder::Only<S>,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::query_builder::Only::<
                        S,
                    >::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Only<table>,
            > for table {
                type Count = diesel::query_source::Once;
            }
            impl diesel::query_source::AppearsInFromClause<table>
            for diesel::query_builder::Only<table> {
                type Count = diesel::query_source::Once;
            }
            impl<S, TSM> diesel::JoinTo<diesel::query_builder::Tablesample<S, TSM>>
            for table
            where
                diesel::query_builder::Tablesample<S, TSM>: diesel::JoinTo<table>,
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type FromClause = diesel::query_builder::Tablesample<S, TSM>;
                type OnClause = <diesel::query_builder::Tablesample<
                    S,
                    TSM,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::query_builder::Tablesample<S, TSM>,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::query_builder::Tablesample::<
                        S,
                        TSM,
                    >::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                TSM,
            > diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<table, TSM>,
            > for table
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<TSM> diesel::query_source::AppearsInFromClause<table>
            for diesel::query_builder::Tablesample<table, TSM>
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            /// Contains all of the columns of this table
            pub mod columns {
                use ::diesel;
                use super::table;
                use diesel::sql_types::*;
                #[allow(non_camel_case_types, dead_code)]
                /// Represents `table_name.*`, which is sometimes needed for
                /// efficient count queries. It cannot be used in place of
                /// `all_columns`, and has a `SqlType` of `()` to prevent it
                /// being used that way
                pub struct star;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for star {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "star")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for star {
                    #[inline]
                    fn clone(&self) -> star {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for star {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for star {
                        type QueryId = star;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                impl<__GB> diesel::expression::ValidGrouping<__GB> for star
                where
                    (
                        id,
                        unionid,
                        platform,
                        openid,
                        name,
                        derive,
                        out_ip,
                        in_ip,
                        blank,
                        created_at,
                        updated_at,
                        deleted_at,
                    ): diesel::expression::ValidGrouping<__GB>,
                {
                    type IsAggregate = <(
                        id,
                        unionid,
                        platform,
                        openid,
                        name,
                        derive,
                        out_ip,
                        in_ip,
                        blank,
                        created_at,
                        updated_at,
                        deleted_at,
                    ) as diesel::expression::ValidGrouping<__GB>>::IsAggregate;
                }
                impl diesel::Expression for star {
                    type SqlType = diesel::expression::expression_types::NotSelectable;
                }
                impl<
                    DB: diesel::backend::Backend,
                > diesel::query_builder::QueryFragment<DB> for star
                where
                    <table as diesel::QuerySource>::FromClause: diesel::query_builder::QueryFragment<
                        DB,
                    >,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        use diesel::QuerySource;
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_sql("*");
                        Ok(())
                    }
                }
                impl diesel::SelectableExpression<table> for star {}
                impl diesel::AppearsOnTable<table> for star {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct id;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for id {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "id")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for id {
                    #[inline]
                    fn clone(&self) -> id {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for id {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for id {
                        type QueryId = id;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for id {
                    #[inline]
                    fn default() -> id {
                        id {}
                    }
                }
                impl diesel::expression::Expression for id {
                    type SqlType = Int4;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for id
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("id")
                    }
                }
                impl diesel::SelectableExpression<super::table> for id {}
                impl<QS> diesel::AppearsOnTable<QS> for id
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for id
                where
                    id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for id
                where
                    id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for id
                where
                    id: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for id
                where
                    From: diesel::query_source::QuerySource,
                    id: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for id
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        id,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for id {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for id {
                    type Table = super::table;
                    const NAME: &'static str = "id";
                }
                impl<T> diesel::EqAll<T> for id
                where
                    T: diesel::expression::AsExpression<Int4>,
                    diesel::dsl::Eq<
                        id,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Div<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Div<
                        Self,
                        Rhs::Expression,
                    >;
                    fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Div::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Mul<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Mul<
                        Self,
                        Rhs::Expression,
                    >;
                    fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Mul::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for id {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for id {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct unionid;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for unionid {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "unionid")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for unionid {
                    #[inline]
                    fn clone(&self) -> unionid {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for unionid {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for unionid {
                        type QueryId = unionid;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for unionid {
                    #[inline]
                    fn default() -> unionid {
                        unionid {}
                    }
                }
                impl diesel::expression::Expression for unionid {
                    type SqlType = Varchar;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for unionid
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("unionid")
                    }
                }
                impl diesel::SelectableExpression<super::table> for unionid {}
                impl<QS> diesel::AppearsOnTable<QS> for unionid
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for unionid
                where
                    unionid: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for unionid
                where
                    unionid: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for unionid
                where
                    unionid: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for unionid
                where
                    From: diesel::query_source::QuerySource,
                    unionid: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for unionid
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        unionid,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for unionid {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<unionid> for unionid {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for unionid {
                    type Table = super::table;
                    const NAME: &'static str = "unionid";
                }
                impl<T> diesel::EqAll<T> for unionid
                where
                    T: diesel::expression::AsExpression<Varchar>,
                    diesel::dsl::Eq<
                        unionid,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for unionid {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for unionid {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for unionid
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for unionid
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct platform;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for platform {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "platform")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for platform {
                    #[inline]
                    fn clone(&self) -> platform {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for platform {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for platform {
                        type QueryId = platform;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for platform {
                    #[inline]
                    fn default() -> platform {
                        platform {}
                    }
                }
                impl diesel::expression::Expression for platform {
                    type SqlType = Varchar;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for platform
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("platform")
                    }
                }
                impl diesel::SelectableExpression<super::table> for platform {}
                impl<QS> diesel::AppearsOnTable<QS> for platform
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for platform
                where
                    platform: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for platform
                where
                    platform: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for platform
                where
                    platform: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for platform
                where
                    From: diesel::query_source::QuerySource,
                    platform: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for platform
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        platform,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for platform {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<platform> for platform {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for platform {
                    type Table = super::table;
                    const NAME: &'static str = "platform";
                }
                impl<T> diesel::EqAll<T> for platform
                where
                    T: diesel::expression::AsExpression<Varchar>,
                    diesel::dsl::Eq<
                        platform,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for platform {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for platform {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for platform
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for platform
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct openid;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for openid {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "openid")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for openid {
                    #[inline]
                    fn clone(&self) -> openid {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for openid {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for openid {
                        type QueryId = openid;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for openid {
                    #[inline]
                    fn default() -> openid {
                        openid {}
                    }
                }
                impl diesel::expression::Expression for openid {
                    type SqlType = Varchar;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for openid
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("openid")
                    }
                }
                impl diesel::SelectableExpression<super::table> for openid {}
                impl<QS> diesel::AppearsOnTable<QS> for openid
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for openid
                where
                    openid: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for openid
                where
                    openid: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for openid
                where
                    openid: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for openid
                where
                    From: diesel::query_source::QuerySource,
                    openid: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for openid
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        openid,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for openid {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<openid> for openid {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for openid {
                    type Table = super::table;
                    const NAME: &'static str = "openid";
                }
                impl<T> diesel::EqAll<T> for openid
                where
                    T: diesel::expression::AsExpression<Varchar>,
                    diesel::dsl::Eq<
                        openid,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for openid {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for openid {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for openid
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for openid
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct name;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for name {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "name")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for name {
                    #[inline]
                    fn clone(&self) -> name {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for name {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for name {
                        type QueryId = name;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for name {
                    #[inline]
                    fn default() -> name {
                        name {}
                    }
                }
                impl diesel::expression::Expression for name {
                    type SqlType = Varchar;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for name
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("name")
                    }
                }
                impl diesel::SelectableExpression<super::table> for name {}
                impl<QS> diesel::AppearsOnTable<QS> for name
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for name
                where
                    name: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for name
                where
                    name: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for name
                where
                    name: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for name
                where
                    From: diesel::query_source::QuerySource,
                    name: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for name
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        name,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for name {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for name {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for name {
                    type Table = super::table;
                    const NAME: &'static str = "name";
                }
                impl<T> diesel::EqAll<T> for name
                where
                    T: diesel::expression::AsExpression<Varchar>,
                    diesel::dsl::Eq<
                        name,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for name {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for name {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for name
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for name
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct derive;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for derive {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "derive")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for derive {
                    #[inline]
                    fn clone(&self) -> derive {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for derive {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for derive {
                        type QueryId = derive;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for derive {
                    #[inline]
                    fn default() -> derive {
                        derive {}
                    }
                }
                impl diesel::expression::Expression for derive {
                    type SqlType = Varchar;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for derive
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("derive")
                    }
                }
                impl diesel::SelectableExpression<super::table> for derive {}
                impl<QS> diesel::AppearsOnTable<QS> for derive
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for derive
                where
                    derive: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for derive
                where
                    derive: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for derive
                where
                    derive: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for derive
                where
                    From: diesel::query_source::QuerySource,
                    derive: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for derive
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        derive,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for derive {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<derive> for derive {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for derive {
                    type Table = super::table;
                    const NAME: &'static str = "derive";
                }
                impl<T> diesel::EqAll<T> for derive
                where
                    T: diesel::expression::AsExpression<Varchar>,
                    diesel::dsl::Eq<
                        derive,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for derive {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for derive {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for derive
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for derive
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct out_ip;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for out_ip {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "out_ip")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for out_ip {
                    #[inline]
                    fn clone(&self) -> out_ip {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for out_ip {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for out_ip {
                        type QueryId = out_ip;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for out_ip {
                    #[inline]
                    fn default() -> out_ip {
                        out_ip {}
                    }
                }
                impl diesel::expression::Expression for out_ip {
                    type SqlType = Varchar;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for out_ip
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("out_ip")
                    }
                }
                impl diesel::SelectableExpression<super::table> for out_ip {}
                impl<QS> diesel::AppearsOnTable<QS> for out_ip
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for out_ip
                where
                    out_ip: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for out_ip
                where
                    out_ip: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for out_ip
                where
                    out_ip: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for out_ip
                where
                    From: diesel::query_source::QuerySource,
                    out_ip: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for out_ip
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        out_ip,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for out_ip {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<out_ip> for out_ip {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for out_ip {
                    type Table = super::table;
                    const NAME: &'static str = "out_ip";
                }
                impl<T> diesel::EqAll<T> for out_ip
                where
                    T: diesel::expression::AsExpression<Varchar>,
                    diesel::dsl::Eq<
                        out_ip,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for out_ip {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for out_ip {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for out_ip
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for out_ip
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct in_ip;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for in_ip {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "in_ip")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for in_ip {
                    #[inline]
                    fn clone(&self) -> in_ip {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for in_ip {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for in_ip {
                        type QueryId = in_ip;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for in_ip {
                    #[inline]
                    fn default() -> in_ip {
                        in_ip {}
                    }
                }
                impl diesel::expression::Expression for in_ip {
                    type SqlType = Varchar;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for in_ip
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("in_ip")
                    }
                }
                impl diesel::SelectableExpression<super::table> for in_ip {}
                impl<QS> diesel::AppearsOnTable<QS> for in_ip
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for in_ip
                where
                    in_ip: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for in_ip
                where
                    in_ip: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for in_ip
                where
                    in_ip: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for in_ip
                where
                    From: diesel::query_source::QuerySource,
                    in_ip: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for in_ip
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        in_ip,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for in_ip {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<in_ip> for in_ip {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for in_ip {
                    type Table = super::table;
                    const NAME: &'static str = "in_ip";
                }
                impl<T> diesel::EqAll<T> for in_ip
                where
                    T: diesel::expression::AsExpression<Varchar>,
                    diesel::dsl::Eq<
                        in_ip,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for in_ip {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for in_ip {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for in_ip
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for in_ip
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct blank;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for blank {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "blank")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for blank {
                    #[inline]
                    fn clone(&self) -> blank {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for blank {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for blank {
                        type QueryId = blank;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for blank {
                    #[inline]
                    fn default() -> blank {
                        blank {}
                    }
                }
                impl diesel::expression::Expression for blank {
                    type SqlType = Bool;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for blank
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("blank")
                    }
                }
                impl diesel::SelectableExpression<super::table> for blank {}
                impl<QS> diesel::AppearsOnTable<QS> for blank
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for blank
                where
                    blank: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for blank
                where
                    blank: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for blank
                where
                    blank: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for blank
                where
                    From: diesel::query_source::QuerySource,
                    blank: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for blank
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        blank,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for blank {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for blank {
                    type Table = super::table;
                    const NAME: &'static str = "blank";
                }
                impl<T> diesel::EqAll<T> for blank
                where
                    T: diesel::expression::AsExpression<Bool>,
                    diesel::dsl::Eq<
                        blank,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for blank {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for blank {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for blank
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for blank
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct created_at;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for created_at {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "created_at")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for created_at {
                    #[inline]
                    fn clone(&self) -> created_at {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for created_at {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for created_at {
                        type QueryId = created_at;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for created_at {
                    #[inline]
                    fn default() -> created_at {
                        created_at {}
                    }
                }
                impl diesel::expression::Expression for created_at {
                    type SqlType = Timestamp;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for created_at
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("created_at")
                    }
                }
                impl diesel::SelectableExpression<super::table> for created_at {}
                impl<QS> diesel::AppearsOnTable<QS> for created_at
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for created_at
                where
                    created_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for created_at
                where
                    created_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for created_at
                where
                    created_at: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for created_at
                where
                    From: diesel::query_source::QuerySource,
                    created_at: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for created_at
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        created_at,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for created_at {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at>
                for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for created_at {
                    type Table = super::table;
                    const NAME: &'static str = "created_at";
                }
                impl<T> diesel::EqAll<T> for created_at
                where
                    T: diesel::expression::AsExpression<Timestamp>,
                    diesel::dsl::Eq<
                        created_at,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for created_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<created_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for created_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<created_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for created_at {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for created_at {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for created_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for created_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct updated_at;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for updated_at {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "updated_at")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for updated_at {
                    #[inline]
                    fn clone(&self) -> updated_at {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for updated_at {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for updated_at {
                        type QueryId = updated_at;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for updated_at {
                    #[inline]
                    fn default() -> updated_at {
                        updated_at {}
                    }
                }
                impl diesel::expression::Expression for updated_at {
                    type SqlType = Timestamp;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for updated_at
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("updated_at")
                    }
                }
                impl diesel::SelectableExpression<super::table> for updated_at {}
                impl<QS> diesel::AppearsOnTable<QS> for updated_at
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for updated_at
                where
                    updated_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for updated_at
                where
                    updated_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for updated_at
                where
                    updated_at: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for updated_at
                where
                    From: diesel::query_source::QuerySource,
                    updated_at: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for updated_at
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        updated_at,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for updated_at {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at>
                for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for updated_at {
                    type Table = super::table;
                    const NAME: &'static str = "updated_at";
                }
                impl<T> diesel::EqAll<T> for updated_at
                where
                    T: diesel::expression::AsExpression<Timestamp>,
                    diesel::dsl::Eq<
                        updated_at,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for updated_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<updated_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for updated_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<updated_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for updated_at {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for updated_at {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for updated_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for updated_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct deleted_at;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for deleted_at {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "deleted_at")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for deleted_at {
                    #[inline]
                    fn clone(&self) -> deleted_at {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for deleted_at {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for deleted_at {
                        type QueryId = deleted_at;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for deleted_at {
                    #[inline]
                    fn default() -> deleted_at {
                        deleted_at {}
                    }
                }
                impl diesel::expression::Expression for deleted_at {
                    type SqlType = Nullable<Timestamp>;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for deleted_at
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("deleted_at")
                    }
                }
                impl diesel::SelectableExpression<super::table> for deleted_at {}
                impl<QS> diesel::AppearsOnTable<QS> for deleted_at
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for deleted_at
                where
                    deleted_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for deleted_at
                where
                    deleted_at: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for deleted_at
                where
                    deleted_at: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for deleted_at
                where
                    From: diesel::query_source::QuerySource,
                    deleted_at: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for deleted_at
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        deleted_at,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for deleted_at {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at>
                for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for deleted_at {
                    type Table = super::table;
                    const NAME: &'static str = "deleted_at";
                }
                impl<T> diesel::EqAll<T> for deleted_at
                where
                    T: diesel::expression::AsExpression<Nullable<Timestamp>>,
                    diesel::dsl::Eq<
                        deleted_at,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for deleted_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<deleted_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for deleted_at
                where
                    Rhs: diesel::expression::AsExpression<
                        <<deleted_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for deleted_at {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for deleted_at {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for deleted_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for deleted_at
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                impl diesel::expression::IsContainedInGroupBy<id> for unionid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<unionid> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for platform {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<platform> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for openid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<openid> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for derive {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<derive> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for out_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<out_ip> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for in_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<in_ip> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<unionid> for platform {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<platform> for unionid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<unionid> for openid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<openid> for unionid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<unionid> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for unionid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<unionid> for derive {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<derive> for unionid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<unionid> for out_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<out_ip> for unionid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<unionid> for in_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<in_ip> for unionid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<unionid> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for unionid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<unionid> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for unionid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<unionid> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for unionid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<unionid> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for unionid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<platform> for openid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<openid> for platform {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<platform> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for platform {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<platform> for derive {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<derive> for platform {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<platform> for out_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<out_ip> for platform {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<platform> for in_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<in_ip> for platform {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<platform> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for platform {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<platform> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for platform {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<platform> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for platform {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<platform> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for platform {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<openid> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for openid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<openid> for derive {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<derive> for openid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<openid> for out_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<out_ip> for openid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<openid> for in_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<in_ip> for openid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<openid> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for openid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<openid> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for openid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<openid> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for openid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<openid> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for openid {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for derive {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<derive> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for out_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<out_ip> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for in_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<in_ip> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<derive> for out_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<out_ip> for derive {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<derive> for in_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<in_ip> for derive {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<derive> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for derive {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<derive> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for derive {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<derive> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for derive {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<derive> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for derive {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<out_ip> for in_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<in_ip> for out_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<out_ip> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for out_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<out_ip> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for out_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<out_ip> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for out_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<out_ip> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for out_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<in_ip> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for in_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<in_ip> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for in_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<in_ip> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for in_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<in_ip> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for in_ip {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<blank> for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at> for blank {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at>
                for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at>
                for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at>
                for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at>
                for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at>
                for deleted_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<deleted_at>
                for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
            }
        }
        #[allow(unused_imports, dead_code, unreachable_pub, unused_qualifications)]
        pub mod users_extra {
            use ::diesel;
            pub use self::columns::*;
            use diesel::sql_types::*;
            /// Re-exports all of the columns of this table, as well as the
            /// table struct renamed to the module name. This is meant to be
            /// glob imported for functions which only deal with one table.
            pub mod dsl {
                pub use super::columns::id;
                pub use super::columns::user_id;
                pub use super::columns::first_launch_path;
                pub use super::columns::first_launch_scene;
                pub use super::table as users_extra;
            }
            #[allow(non_upper_case_globals, dead_code)]
            /// A tuple of all of the columns on this table
            pub const all_columns: (
                id,
                user_id,
                first_launch_path,
                first_launch_scene,
            ) = (id, user_id, first_launch_path, first_launch_scene);
            #[allow(non_camel_case_types)]
            /// The actual table struct
            ///
            /// This is the type which provides the base methods of the query
            /// builder, such as `.select` and `.filter`.
            pub struct table;
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for table {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "table")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::clone::Clone for table {
                #[inline]
                fn clone(&self) -> table {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::marker::Copy for table {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for table {
                    type QueryId = table;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::default::Default for table {
                #[inline]
                fn default() -> table {
                    table {}
                }
            }
            impl table {
                #[allow(dead_code)]
                /// Represents `table_name.*`, which is sometimes necessary
                /// for efficient count queries. It cannot be used in place of
                /// `all_columns`
                pub fn star(&self) -> star {
                    star
                }
            }
            /// The SQL type of all of the columns on this table
            pub type SqlType = (Int4, Int4, Varchar, Varchar);
            /// Helper type for representing a boxed query from this table
            pub type BoxedQuery<'a, DB, ST = SqlType> = diesel::internal::table_macro::BoxedSelectStatement<
                'a,
                ST,
                diesel::internal::table_macro::FromClause<table>,
                DB,
            >;
            impl diesel::QuerySource for table {
                type FromClause = diesel::internal::table_macro::StaticQueryFragmentInstance<
                    table,
                >;
                type DefaultSelection = <Self as diesel::Table>::AllColumns;
                fn from_clause(&self) -> Self::FromClause {
                    diesel::internal::table_macro::StaticQueryFragmentInstance::new()
                }
                fn default_selection(&self) -> Self::DefaultSelection {
                    use diesel::Table;
                    Self::all_columns()
                }
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for table
            where
                DB: diesel::backend::Backend,
                <table as diesel::internal::table_macro::StaticQueryFragment>::Component: diesel::query_builder::QueryFragment<
                    DB,
                >,
            {
                fn walk_ast<'b>(
                    &'b self,
                    __diesel_internal_pass: diesel::query_builder::AstPass<'_, 'b, DB>,
                ) -> diesel::result::QueryResult<()> {
                    <table as diesel::internal::table_macro::StaticQueryFragment>::STATIC_COMPONENT
                        .walk_ast(__diesel_internal_pass)
                }
            }
            impl diesel::internal::table_macro::StaticQueryFragment for table {
                type Component = diesel::internal::table_macro::Identifier<'static>;
                const STATIC_COMPONENT: &'static Self::Component = &diesel::internal::table_macro::Identifier(
                    "users_extra",
                );
            }
            impl diesel::query_builder::AsQuery for table {
                type SqlType = SqlType;
                type Query = diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<Self>,
                >;
                fn as_query(self) -> Self::Query {
                    diesel::internal::table_macro::SelectStatement::simple(self)
                }
            }
            impl diesel::Table for table {
                type PrimaryKey = id;
                type AllColumns = (id, user_id, first_launch_path, first_launch_scene);
                fn primary_key(&self) -> Self::PrimaryKey {
                    id
                }
                fn all_columns() -> Self::AllColumns {
                    (id, user_id, first_launch_path, first_launch_scene)
                }
            }
            impl diesel::associations::HasTable for table {
                type Table = Self;
                fn table() -> Self::Table {
                    table
                }
            }
            impl diesel::query_builder::IntoUpdateTarget for table {
                type WhereClause = <<Self as diesel::query_builder::AsQuery>::Query as diesel::query_builder::IntoUpdateTarget>::WhereClause;
                fn into_update_target(
                    self,
                ) -> diesel::query_builder::UpdateTarget<
                    Self::Table,
                    Self::WhereClause,
                > {
                    use diesel::query_builder::AsQuery;
                    let q: diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<table>,
                    > = self.as_query();
                    q.into_update_target()
                }
            }
            impl diesel::query_source::AppearsInFromClause<table> for table {
                type Count = diesel::query_source::Once;
            }
            impl<S> diesel::internal::table_macro::AliasAppearsInFromClause<S, table>
            for table
            where
                S: diesel::query_source::AliasSource<Target = table>,
            {
                type Count = diesel::query_source::Never;
            }
            impl<
                S1,
                S2,
            > diesel::internal::table_macro::AliasAliasAppearsInFromClause<table, S2, S1>
            for table
            where
                S1: diesel::query_source::AliasSource<Target = table>,
                S2: diesel::query_source::AliasSource<Target = table>,
                S1: diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<
                    S2,
                    table,
                >,
            {
                type Count = <S1 as diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<
                    S2,
                    table,
                >>::Count;
            }
            impl<
                S,
            > diesel::query_source::AppearsInFromClause<diesel::query_source::Alias<S>>
            for table
            where
                S: diesel::query_source::AliasSource,
            {
                type Count = diesel::query_source::Never;
            }
            impl<
                S,
                C,
            > diesel::internal::table_macro::FieldAliasMapperAssociatedTypesDisjointnessTrick<
                table,
                S,
                C,
            > for table
            where
                S: diesel::query_source::AliasSource<Target = table>
                    + ::std::clone::Clone,
                C: diesel::query_source::Column<Table = table>,
            {
                type Out = diesel::query_source::AliasedField<S, C>;
                fn map(
                    __diesel_internal_column: C,
                    __diesel_internal_alias: &diesel::query_source::Alias<S>,
                ) -> Self::Out {
                    __diesel_internal_alias.field(__diesel_internal_column)
                }
            }
            impl diesel::query_source::AppearsInFromClause<table>
            for diesel::internal::table_macro::NoFromClause {
                type Count = diesel::query_source::Never;
            }
            impl<
                Left,
                Right,
                Kind,
            > diesel::JoinTo<diesel::internal::table_macro::Join<Left, Right, Kind>>
            for table
            where
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    Kind,
                >: diesel::JoinTo<table>,
                Left: diesel::query_source::QuerySource,
                Right: diesel::query_source::QuerySource,
            {
                type FromClause = diesel::internal::table_macro::Join<Left, Right, Kind>;
                type OnClause = <diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    Kind,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        Kind,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::Join::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                Join,
                On,
            > diesel::JoinTo<diesel::internal::table_macro::JoinOn<Join, On>> for table
            where
                diesel::internal::table_macro::JoinOn<Join, On>: diesel::JoinTo<table>,
            {
                type FromClause = diesel::internal::table_macro::JoinOn<Join, On>;
                type OnClause = <diesel::internal::table_macro::JoinOn<
                    Join,
                    On,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::JoinOn<
                        Join,
                        On,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::JoinOn::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                F,
                S,
                D,
                W,
                O,
                L,
                Of,
                G,
            > diesel::JoinTo<
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >,
            > for table
            where
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >: diesel::JoinTo<table>,
                F: diesel::query_source::QuerySource,
            {
                type FromClause = diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >;
                type OnClause = <diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<F>,
                        S,
                        D,
                        W,
                        O,
                        L,
                        Of,
                        G,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::SelectStatement::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                'a,
                QS,
                ST,
                DB,
            > diesel::JoinTo<
                diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >,
            > for table
            where
                diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >: diesel::JoinTo<table>,
                QS: diesel::query_source::QuerySource,
            {
                type FromClause = diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >;
                type OnClause = <diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::BoxedSelectStatement<
                        'a,
                        diesel::internal::table_macro::FromClause<QS>,
                        ST,
                        DB,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::BoxedSelectStatement::join_target(
                        table,
                    );
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<S> diesel::JoinTo<diesel::query_source::Alias<S>> for table
            where
                diesel::query_source::Alias<S>: diesel::JoinTo<table>,
            {
                type FromClause = diesel::query_source::Alias<S>;
                type OnClause = <diesel::query_source::Alias<
                    S,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::query_source::Alias<S>,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::query_source::Alias::<
                        S,
                    >::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<T> diesel::insertable::Insertable<T> for table
            where
                <table as diesel::query_builder::AsQuery>::Query: diesel::insertable::Insertable<
                    T,
                >,
            {
                type Values = <<table as diesel::query_builder::AsQuery>::Query as diesel::insertable::Insertable<
                    T,
                >>::Values;
                fn values(self) -> Self::Values {
                    use diesel::query_builder::AsQuery;
                    self.as_query().values()
                }
            }
            impl<'a, T> diesel::insertable::Insertable<T> for &'a table
            where
                table: diesel::insertable::Insertable<T>,
            {
                type Values = <table as diesel::insertable::Insertable<T>>::Values;
                fn values(self) -> Self::Values {
                    (*self).values()
                }
            }
            impl<S> diesel::JoinTo<diesel::query_builder::Only<S>> for table
            where
                diesel::query_builder::Only<S>: diesel::JoinTo<table>,
            {
                type FromClause = diesel::query_builder::Only<S>;
                type OnClause = <diesel::query_builder::Only<
                    S,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::query_builder::Only<S>,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::query_builder::Only::<
                        S,
                    >::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Only<table>,
            > for table {
                type Count = diesel::query_source::Once;
            }
            impl diesel::query_source::AppearsInFromClause<table>
            for diesel::query_builder::Only<table> {
                type Count = diesel::query_source::Once;
            }
            impl<S, TSM> diesel::JoinTo<diesel::query_builder::Tablesample<S, TSM>>
            for table
            where
                diesel::query_builder::Tablesample<S, TSM>: diesel::JoinTo<table>,
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type FromClause = diesel::query_builder::Tablesample<S, TSM>;
                type OnClause = <diesel::query_builder::Tablesample<
                    S,
                    TSM,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::query_builder::Tablesample<S, TSM>,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::query_builder::Tablesample::<
                        S,
                        TSM,
                    >::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<
                TSM,
            > diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<table, TSM>,
            > for table
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<TSM> diesel::query_source::AppearsInFromClause<table>
            for diesel::query_builder::Tablesample<table, TSM>
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            /// Contains all of the columns of this table
            pub mod columns {
                use ::diesel;
                use super::table;
                use diesel::sql_types::*;
                #[allow(non_camel_case_types, dead_code)]
                /// Represents `table_name.*`, which is sometimes needed for
                /// efficient count queries. It cannot be used in place of
                /// `all_columns`, and has a `SqlType` of `()` to prevent it
                /// being used that way
                pub struct star;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for star {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "star")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for star {
                    #[inline]
                    fn clone(&self) -> star {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for star {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for star {
                        type QueryId = star;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                impl<__GB> diesel::expression::ValidGrouping<__GB> for star
                where
                    (
                        id,
                        user_id,
                        first_launch_path,
                        first_launch_scene,
                    ): diesel::expression::ValidGrouping<__GB>,
                {
                    type IsAggregate = <(
                        id,
                        user_id,
                        first_launch_path,
                        first_launch_scene,
                    ) as diesel::expression::ValidGrouping<__GB>>::IsAggregate;
                }
                impl diesel::Expression for star {
                    type SqlType = diesel::expression::expression_types::NotSelectable;
                }
                impl<
                    DB: diesel::backend::Backend,
                > diesel::query_builder::QueryFragment<DB> for star
                where
                    <table as diesel::QuerySource>::FromClause: diesel::query_builder::QueryFragment<
                        DB,
                    >,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        use diesel::QuerySource;
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_sql("*");
                        Ok(())
                    }
                }
                impl diesel::SelectableExpression<table> for star {}
                impl diesel::AppearsOnTable<table> for star {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct id;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for id {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "id")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for id {
                    #[inline]
                    fn clone(&self) -> id {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for id {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for id {
                        type QueryId = id;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for id {
                    #[inline]
                    fn default() -> id {
                        id {}
                    }
                }
                impl diesel::expression::Expression for id {
                    type SqlType = Int4;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for id
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("id")
                    }
                }
                impl diesel::SelectableExpression<super::table> for id {}
                impl<QS> diesel::AppearsOnTable<QS> for id
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for id
                where
                    id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for id
                where
                    id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for id
                where
                    id: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for id
                where
                    From: diesel::query_source::QuerySource,
                    id: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for id
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        id,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for id {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for id {
                    type Table = super::table;
                    const NAME: &'static str = "id";
                }
                impl<T> diesel::EqAll<T> for id
                where
                    T: diesel::expression::AsExpression<Int4>,
                    diesel::dsl::Eq<
                        id,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Div<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Div<
                        Self,
                        Rhs::Expression,
                    >;
                    fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Div::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Mul<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Mul<
                        Self,
                        Rhs::Expression,
                    >;
                    fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Mul::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for id {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for id {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct user_id;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for user_id {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "user_id")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for user_id {
                    #[inline]
                    fn clone(&self) -> user_id {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for user_id {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for user_id {
                        type QueryId = user_id;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for user_id {
                    #[inline]
                    fn default() -> user_id {
                        user_id {}
                    }
                }
                impl diesel::expression::Expression for user_id {
                    type SqlType = Int4;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for user_id
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("user_id")
                    }
                }
                impl diesel::SelectableExpression<super::table> for user_id {}
                impl<QS> diesel::AppearsOnTable<QS> for user_id
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for user_id
                where
                    user_id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for user_id
                where
                    user_id: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for user_id
                where
                    user_id: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for user_id
                where
                    From: diesel::query_source::QuerySource,
                    user_id: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for user_id
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        user_id,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for user_id {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<user_id> for user_id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for user_id {
                    type Table = super::table;
                    const NAME: &'static str = "user_id";
                }
                impl<T> diesel::EqAll<T> for user_id
                where
                    T: diesel::expression::AsExpression<Int4>,
                    diesel::dsl::Eq<
                        user_id,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for user_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<user_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Add<
                        Self,
                        Rhs::Expression,
                    >;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for user_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<user_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<
                        Self,
                        Rhs::Expression,
                    >;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Div<Rhs> for user_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<user_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Div<
                        Self,
                        Rhs::Expression,
                    >;
                    fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Div::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl<Rhs> ::std::ops::Mul<Rhs> for user_id
                where
                    Rhs: diesel::expression::AsExpression<
                        <<user_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                    >,
                {
                    type Output = diesel::internal::table_macro::ops::Mul<
                        Self,
                        Rhs::Expression,
                    >;
                    fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Mul::new(
                            self,
                            __diesel_internal_rhs.as_expression(),
                        )
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for user_id {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for user_id {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for user_id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for user_id
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct first_launch_path;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for first_launch_path {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "first_launch_path")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for first_launch_path {
                    #[inline]
                    fn clone(&self) -> first_launch_path {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for first_launch_path {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for first_launch_path {
                        type QueryId = first_launch_path;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for first_launch_path {
                    #[inline]
                    fn default() -> first_launch_path {
                        first_launch_path {}
                    }
                }
                impl diesel::expression::Expression for first_launch_path {
                    type SqlType = Varchar;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for first_launch_path
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("first_launch_path")
                    }
                }
                impl diesel::SelectableExpression<super::table> for first_launch_path {}
                impl<QS> diesel::AppearsOnTable<QS> for first_launch_path
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for first_launch_path
                where
                    first_launch_path: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for first_launch_path
                where
                    first_launch_path: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for first_launch_path
                where
                    first_launch_path: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for first_launch_path
                where
                    From: diesel::query_source::QuerySource,
                    first_launch_path: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for first_launch_path
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        first_launch_path,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for first_launch_path {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<first_launch_path>
                for first_launch_path {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for first_launch_path {
                    type Table = super::table;
                    const NAME: &'static str = "first_launch_path";
                }
                impl<T> diesel::EqAll<T> for first_launch_path
                where
                    T: diesel::expression::AsExpression<Varchar>,
                    diesel::dsl::Eq<
                        first_launch_path,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for first_launch_path {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for first_launch_path {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for first_launch_path
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for first_launch_path
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct first_launch_scene;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for first_launch_scene {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "first_launch_scene")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for first_launch_scene {
                    #[inline]
                    fn clone(&self) -> first_launch_scene {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for first_launch_scene {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for first_launch_scene {
                        type QueryId = first_launch_scene;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for first_launch_scene {
                    #[inline]
                    fn default() -> first_launch_scene {
                        first_launch_scene {}
                    }
                }
                impl diesel::expression::Expression for first_launch_scene {
                    type SqlType = Varchar;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for first_launch_scene
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    >: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<
                            '_,
                            'b,
                            DB,
                        >,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                                table,
                            > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("first_launch_scene")
                    }
                }
                impl diesel::SelectableExpression<super::table> for first_launch_scene {}
                impl<QS> diesel::AppearsOnTable<QS> for first_launch_scene
                where
                    QS: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Once,
                    >,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for first_launch_scene
                where
                    first_launch_scene: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::LeftOuter,
                        >,
                    >,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<
                            super::table,
                            Count = diesel::query_source::Never,
                        > + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {}
                impl<
                    Left,
                    Right,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for first_launch_scene
                where
                    first_launch_scene: diesel::AppearsOnTable<
                        diesel::internal::table_macro::Join<
                            Left,
                            Right,
                            diesel::internal::table_macro::Inner,
                        >,
                    >,
                    Left: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table>
                        + diesel::query_source::QuerySource,
                    (
                        Left::Count,
                        Right::Count,
                    ): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<
                        <(
                            Left::Count,
                            Right::Count,
                        ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                    >,
                {}
                impl<
                    Join,
                    On,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::JoinOn<Join, On>,
                > for first_launch_scene
                where
                    first_launch_scene: diesel::SelectableExpression<Join>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::JoinOn<Join, On>,
                        >,
                {}
                impl<
                    From,
                > diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for first_launch_scene
                where
                    From: diesel::query_source::QuerySource,
                    first_launch_scene: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<
                            diesel::internal::table_macro::SelectStatement<
                                diesel::internal::table_macro::FromClause<From>,
                            >,
                        >,
                {}
                impl<__GB> diesel::expression::ValidGrouping<__GB> for first_launch_scene
                where
                    __GB: diesel::expression::IsContainedInGroupBy<
                        first_launch_scene,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for first_launch_scene {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<first_launch_scene>
                for first_launch_scene {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for first_launch_scene {
                    type Table = super::table;
                    const NAME: &'static str = "first_launch_scene";
                }
                impl<T> diesel::EqAll<T> for first_launch_scene
                where
                    T: diesel::expression::AsExpression<Varchar>,
                    diesel::dsl::Eq<
                        first_launch_scene,
                        T::Expression,
                    >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Only<super::table>,
                > for first_launch_scene {
                    type Count = diesel::query_source::Once;
                }
                impl diesel::SelectableExpression<
                    diesel::query_builder::Only<super::table>,
                > for first_launch_scene {}
                impl<
                    TSM,
                > diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for first_launch_scene
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {
                    type Count = diesel::query_source::Once;
                }
                impl<
                    TSM,
                > diesel::SelectableExpression<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for first_launch_scene
                where
                    TSM: diesel::internal::table_macro::TablesampleMethod,
                {}
                impl diesel::expression::IsContainedInGroupBy<id> for user_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<user_id> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for first_launch_path {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<first_launch_path> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id>
                for first_launch_scene {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<first_launch_scene>
                for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<user_id>
                for first_launch_path {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<first_launch_path>
                for user_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<user_id>
                for first_launch_scene {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<first_launch_scene>
                for user_id {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<first_launch_path>
                for first_launch_scene {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<first_launch_scene>
                for first_launch_path {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
            }
        }
        impl ::diesel::query_source::TableNotEqual<downlogs::table>
        for resources::table {}
        impl ::diesel::query_source::TableNotEqual<resources::table>
        for downlogs::table {}
        impl ::diesel::query_source::TableNotEqual<downlogs::table>
        for ::diesel::query_builder::Only<resources::table> {}
        impl ::diesel::query_source::TableNotEqual<resources::table>
        for ::diesel::query_builder::Only<downlogs::table> {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<downlogs::table>,
        > for resources::table {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<resources::table>,
        > for downlogs::table {}
        impl<TSM> ::diesel::query_source::TableNotEqual<downlogs::table>
        for ::diesel::query_builder::Tablesample<resources::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<TSM> ::diesel::query_source::TableNotEqual<resources::table>
        for ::diesel::query_builder::Tablesample<downlogs::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<downlogs::table, TSM>,
        > for resources::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<resources::table, TSM>,
        > for downlogs::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl ::diesel::query_source::TableNotEqual<downlogs::table> for rooms::table {}
        impl ::diesel::query_source::TableNotEqual<rooms::table> for downlogs::table {}
        impl ::diesel::query_source::TableNotEqual<downlogs::table>
        for ::diesel::query_builder::Only<rooms::table> {}
        impl ::diesel::query_source::TableNotEqual<rooms::table>
        for ::diesel::query_builder::Only<downlogs::table> {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<downlogs::table>,
        > for rooms::table {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<rooms::table>,
        > for downlogs::table {}
        impl<TSM> ::diesel::query_source::TableNotEqual<downlogs::table>
        for ::diesel::query_builder::Tablesample<rooms::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<TSM> ::diesel::query_source::TableNotEqual<rooms::table>
        for ::diesel::query_builder::Tablesample<downlogs::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<downlogs::table, TSM>,
        > for rooms::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<rooms::table, TSM>,
        > for downlogs::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl ::diesel::query_source::TableNotEqual<downlogs::table> for users::table {}
        impl ::diesel::query_source::TableNotEqual<users::table> for downlogs::table {}
        impl ::diesel::query_source::TableNotEqual<downlogs::table>
        for ::diesel::query_builder::Only<users::table> {}
        impl ::diesel::query_source::TableNotEqual<users::table>
        for ::diesel::query_builder::Only<downlogs::table> {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<downlogs::table>,
        > for users::table {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<users::table>,
        > for downlogs::table {}
        impl<TSM> ::diesel::query_source::TableNotEqual<downlogs::table>
        for ::diesel::query_builder::Tablesample<users::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<TSM> ::diesel::query_source::TableNotEqual<users::table>
        for ::diesel::query_builder::Tablesample<downlogs::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<downlogs::table, TSM>,
        > for users::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<users::table, TSM>,
        > for downlogs::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl ::diesel::query_source::TableNotEqual<downlogs::table>
        for users_extra::table {}
        impl ::diesel::query_source::TableNotEqual<users_extra::table>
        for downlogs::table {}
        impl ::diesel::query_source::TableNotEqual<downlogs::table>
        for ::diesel::query_builder::Only<users_extra::table> {}
        impl ::diesel::query_source::TableNotEqual<users_extra::table>
        for ::diesel::query_builder::Only<downlogs::table> {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<downlogs::table>,
        > for users_extra::table {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<users_extra::table>,
        > for downlogs::table {}
        impl<TSM> ::diesel::query_source::TableNotEqual<downlogs::table>
        for ::diesel::query_builder::Tablesample<users_extra::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<TSM> ::diesel::query_source::TableNotEqual<users_extra::table>
        for ::diesel::query_builder::Tablesample<downlogs::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<downlogs::table, TSM>,
        > for users_extra::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<users_extra::table, TSM>,
        > for downlogs::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl ::diesel::query_source::TableNotEqual<resources::table> for rooms::table {}
        impl ::diesel::query_source::TableNotEqual<rooms::table> for resources::table {}
        impl ::diesel::query_source::TableNotEqual<resources::table>
        for ::diesel::query_builder::Only<rooms::table> {}
        impl ::diesel::query_source::TableNotEqual<rooms::table>
        for ::diesel::query_builder::Only<resources::table> {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<resources::table>,
        > for rooms::table {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<rooms::table>,
        > for resources::table {}
        impl<TSM> ::diesel::query_source::TableNotEqual<resources::table>
        for ::diesel::query_builder::Tablesample<rooms::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<TSM> ::diesel::query_source::TableNotEqual<rooms::table>
        for ::diesel::query_builder::Tablesample<resources::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<resources::table, TSM>,
        > for rooms::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<rooms::table, TSM>,
        > for resources::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl ::diesel::query_source::TableNotEqual<resources::table> for users::table {}
        impl ::diesel::query_source::TableNotEqual<users::table> for resources::table {}
        impl ::diesel::query_source::TableNotEqual<resources::table>
        for ::diesel::query_builder::Only<users::table> {}
        impl ::diesel::query_source::TableNotEqual<users::table>
        for ::diesel::query_builder::Only<resources::table> {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<resources::table>,
        > for users::table {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<users::table>,
        > for resources::table {}
        impl<TSM> ::diesel::query_source::TableNotEqual<resources::table>
        for ::diesel::query_builder::Tablesample<users::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<TSM> ::diesel::query_source::TableNotEqual<users::table>
        for ::diesel::query_builder::Tablesample<resources::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<resources::table, TSM>,
        > for users::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<users::table, TSM>,
        > for resources::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl ::diesel::query_source::TableNotEqual<resources::table>
        for users_extra::table {}
        impl ::diesel::query_source::TableNotEqual<users_extra::table>
        for resources::table {}
        impl ::diesel::query_source::TableNotEqual<resources::table>
        for ::diesel::query_builder::Only<users_extra::table> {}
        impl ::diesel::query_source::TableNotEqual<users_extra::table>
        for ::diesel::query_builder::Only<resources::table> {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<resources::table>,
        > for users_extra::table {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<users_extra::table>,
        > for resources::table {}
        impl<TSM> ::diesel::query_source::TableNotEqual<resources::table>
        for ::diesel::query_builder::Tablesample<users_extra::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<TSM> ::diesel::query_source::TableNotEqual<users_extra::table>
        for ::diesel::query_builder::Tablesample<resources::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<resources::table, TSM>,
        > for users_extra::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<users_extra::table, TSM>,
        > for resources::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl ::diesel::query_source::TableNotEqual<rooms::table> for users::table {}
        impl ::diesel::query_source::TableNotEqual<users::table> for rooms::table {}
        impl ::diesel::query_source::TableNotEqual<rooms::table>
        for ::diesel::query_builder::Only<users::table> {}
        impl ::diesel::query_source::TableNotEqual<users::table>
        for ::diesel::query_builder::Only<rooms::table> {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<rooms::table>,
        > for users::table {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<users::table>,
        > for rooms::table {}
        impl<TSM> ::diesel::query_source::TableNotEqual<rooms::table>
        for ::diesel::query_builder::Tablesample<users::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<TSM> ::diesel::query_source::TableNotEqual<users::table>
        for ::diesel::query_builder::Tablesample<rooms::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<rooms::table, TSM>,
        > for users::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<users::table, TSM>,
        > for rooms::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl ::diesel::query_source::TableNotEqual<rooms::table> for users_extra::table {}
        impl ::diesel::query_source::TableNotEqual<users_extra::table> for rooms::table {}
        impl ::diesel::query_source::TableNotEqual<rooms::table>
        for ::diesel::query_builder::Only<users_extra::table> {}
        impl ::diesel::query_source::TableNotEqual<users_extra::table>
        for ::diesel::query_builder::Only<rooms::table> {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<rooms::table>,
        > for users_extra::table {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<users_extra::table>,
        > for rooms::table {}
        impl<TSM> ::diesel::query_source::TableNotEqual<rooms::table>
        for ::diesel::query_builder::Tablesample<users_extra::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<TSM> ::diesel::query_source::TableNotEqual<users_extra::table>
        for ::diesel::query_builder::Tablesample<rooms::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<rooms::table, TSM>,
        > for users_extra::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<users_extra::table, TSM>,
        > for rooms::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl ::diesel::query_source::TableNotEqual<users::table> for users_extra::table {}
        impl ::diesel::query_source::TableNotEqual<users_extra::table> for users::table {}
        impl ::diesel::query_source::TableNotEqual<users::table>
        for ::diesel::query_builder::Only<users_extra::table> {}
        impl ::diesel::query_source::TableNotEqual<users_extra::table>
        for ::diesel::query_builder::Only<users::table> {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<users::table>,
        > for users_extra::table {}
        impl ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Only<users_extra::table>,
        > for users::table {}
        impl<TSM> ::diesel::query_source::TableNotEqual<users::table>
        for ::diesel::query_builder::Tablesample<users_extra::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<TSM> ::diesel::query_source::TableNotEqual<users_extra::table>
        for ::diesel::query_builder::Tablesample<users::table, TSM>
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<users::table, TSM>,
        > for users_extra::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
        impl<
            TSM,
        > ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<users_extra::table, TSM>,
        > for users::table
        where
            TSM: ::diesel::internal::table_macro::TablesampleMethod,
        {}
    }
}
mod modules {
    pub mod user {
        use nidrs::macros::module;
        pub mod controller {
            use nidrs::macros::{controller, get};
            use nidrs::openapi::api;
            use nidrs::{post, AppResult, Inject};
            use nidrs_extern::axum::Json;
            use nidrs_macro::meta;
            use crate::models::dao::users::{CreateUser, User};
            use super::dto::LoginDto;
            use super::service::UserService;
            pub struct UserController {
                user_service: Inject<UserService>,
            }
            #[automatically_derived]
            impl ::core::default::Default for UserController {
                #[inline]
                fn default() -> UserController {
                    UserController {
                        user_service: ::core::default::Default::default(),
                    }
                }
            }
            impl nidrs::Controller for UserController {}
            impl nidrs::Service for UserController {
                fn inject(
                    &self,
                    ctx: nidrs::ModuleCtx,
                    module_name: &str,
                ) -> nidrs::ModuleCtx {
                    let service = ctx
                        .get_service::<UserService>(&module_name, "UserService");
                    self.user_service.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for UserController {
                fn __meta() -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                    meta.set("service", "UserController");
                    meta.set_data(nidrs::datasets::ServiceName::from("UserController"));
                    meta.set_data(nidrs::datasets::ControllerPath::from("/user"));
                    meta.set("module", "UserModule");
                    meta.set("global", "app");
                    meta
                }
            }
            impl UserController {
                pub async fn register(
                    &self,
                    dto: Json<LoginDto>,
                ) -> AppResult<Json<User>> {
                    let user = self.user_service.login(dto.openid.to_owned()).await?;
                    return Ok(Json(user));
                }
                pub fn __meta_register(&self) -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set_data(nidrs::datasets::RouterName::from("register"));
                    meta.set_data(
                        nidrs::openapi::RouterIn(
                            nidrs::openapi::RouterParams::default()
                                .merge_type::<Json<LoginDto>>(),
                        ),
                    );
                    meta.set("handler", "register");
                    meta.set_data(nidrs::datasets::RouterMethod::from("post"));
                    meta.set_data(nidrs::datasets::RouterPath::from("/register"));
                    meta.set("disable_auto_json", true);
                    meta.set_data(
                        nidrs::openapi::RouterOut(
                            nidrs::openapi::RouterParams::default()
                                .merge_type::<AppResult<Json<User>>>(),
                        ),
                    );
                    meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                    meta.set("service", "UserController");
                    meta.set_data(nidrs::datasets::ServiceName::from("UserController"));
                    meta.set_data(nidrs::datasets::ControllerPath::from("/user"));
                    meta.set("module", "UserModule");
                    meta.set("global", "app");
                    meta
                }
                pub fn __route_register(
                    &self,
                    mut ctx: nidrs::ModuleCtx,
                ) -> nidrs::ModuleCtx {
                    use nidrs::externs::axum;
                    use axum::response::IntoResponse;
                    use nidrs::externs::axum::{extract::Query, Json};
                    use nidrs::externs::meta::{InnerMeta, Meta};
                    use nidrs::Interceptor;
                    use serde_json::Value;
                    let mut meta = self.__meta_register();
                    let router_info = ctx.get_router_full(&meta);
                    if let Err(e) = router_info {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!("[{0}] {1:?}", "__route_register", e),
                            );
                        };
                    }
                    let full_path = router_info.unwrap();
                    {
                        ::std::io::_print(
                            format_args!(
                                "{0} ",
                                nidrs_extern::colored::Colorize::green("[nidrs]"),
                            ),
                        );
                    };
                    {
                        ::std::io::_print(
                            format_args!(
                                "Registering router \'{0} {1}\'.\n",
                                "post".to_uppercase(),
                                full_path,
                            ),
                        );
                    };
                    meta.set_data(nidrs::datasets::RouterFullPath(full_path.clone()));
                    let meta = Meta::new(meta);
                    let module_name = meta.get::<&str>("module").unwrap();
                    let controller_name = meta
                        .get_data::<nidrs::datasets::ServiceName>()
                        .unwrap()
                        .value();
                    let t_controller = ctx
                        .get_controller::<Self>(module_name, controller_name);
                    let router = nidrs::externs::axum::Router::new()
                        .route(
                            &full_path,
                            nidrs::externs::axum::routing::post(|p0| async move {
                                let r = t_controller.register(p0).await;
                                r
                            }),
                        )
                        .route_layer(nidrs::externs::axum::Extension(meta.clone()));
                    ctx.routers.push(nidrs::MetaRouter::new(router, meta));
                    ctx
                }
            }
        }
        pub mod dto {
            use nidrs::openapi::schema;
            use serde::{Deserialize, Serialize};
            pub struct LoginDto {
                pub openid: String,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for LoginDto {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "LoginDto",
                        "openid",
                        &&self.openid,
                    )
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for LoginDto {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "LoginDto",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "openid",
                            &self.openid,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for LoginDto {
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
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
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
                                    "openid" => _serde::__private::Ok(__Field::__field0),
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
                                    b"openid" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
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
                            marker: _serde::__private::PhantomData<LoginDto>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = LoginDto;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct LoginDto",
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
                                                &"struct LoginDto with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(LoginDto { openid: __field0 })
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
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("openid"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
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
                                        _serde::__private::de::missing_field("openid")?
                                    }
                                };
                                _serde::__private::Ok(LoginDto { openid: __field0 })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["openid"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "LoginDto",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<LoginDto>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            impl utoipa::IntoParams for LoginDto {
                fn into_params(
                    parameter_in_provider: impl Fn(
                    ) -> Option<utoipa::openapi::path::ParameterIn>,
                ) -> Vec<utoipa::openapi::path::Parameter> {
                    [
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("openid")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(
                                Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::SchemaType::String),
                                ),
                            )
                            .build(),
                    ]
                        .to_vec()
                }
            }
            impl<'__s> utoipa::ToSchema<'__s> for LoginDto {
                fn schema() -> (
                    &'__s str,
                    utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
                ) {
                    (
                        "LoginDto",
                        utoipa::openapi::ObjectBuilder::new()
                            .property(
                                "openid",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::SchemaType::String),
                            )
                            .required("openid")
                            .into(),
                    )
                }
            }
            impl nidrs::openapi::ToParamDto for LoginDto {
                fn to_param_dto(
                    dto_type: nidrs::openapi::ParamDtoIn,
                ) -> nidrs::openapi::ParamDto {
                    use nidrs::openapi::utoipa::IntoParams;
                    use nidrs::openapi::utoipa::ToSchema;
                    match dto_type {
                        nidrs::openapi::ParamDtoIn::Param(p) => {
                            nidrs::openapi::ParamDto::ParamList(
                                Self::into_params(|| Some(p.clone())),
                            )
                        }
                        nidrs::openapi::ParamDtoIn::Body => {
                            nidrs::openapi::ParamDto::BodySchema(Self::schema())
                        }
                    }
                }
            }
        }
        pub mod service {
            use std::sync::{Arc, Mutex};
            use nidrs::macros::injectable;
            use nidrs::{AppResult, Inject};
            use crate::app::service::AppService;
            use crate::models::dao::users::{CreateUser, User, UserEntity};
            pub struct UserService {
                user_entity: Inject<UserEntity>,
            }
            #[automatically_derived]
            impl ::core::default::Default for UserService {
                #[inline]
                fn default() -> UserService {
                    UserService {
                        user_entity: ::core::default::Default::default(),
                    }
                }
            }
            impl nidrs::Service for UserService {
                fn inject(
                    &self,
                    ctx: nidrs::ModuleCtx,
                    module_name: &str,
                ) -> nidrs::ModuleCtx {
                    let service = ctx
                        .get_service::<UserEntity>(&module_name, "UserEntity");
                    self.user_entity.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for UserService {
                fn __meta() -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set_data(nidrs::datasets::ServiceName::from("UserService"));
                    meta.set("service", "UserService");
                    meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                    meta.set("module", "UserModule");
                    meta.set("global", "app");
                    meta
                }
            }
            impl UserService {
                pub async fn login(&self, openid: String) -> AppResult<User> {
                    let res = self.user_entity.create(openid.to_string()).await?;
                    let user = self.user_entity.find_by_openid(openid).await?;
                    return Ok(user);
                }
            }
        }
        use crate::models::dao::users::UserEntity;
        use controller::UserController;
        use service::UserService;
        pub struct UserModule;
        #[automatically_derived]
        impl ::core::default::Default for UserModule {
            #[inline]
            fn default() -> UserModule {
                UserModule {}
            }
        }
        impl nidrs::Module for UserModule {
            fn init(self, mut ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
                use nidrs::{
                    Service, Controller, Interceptor, InterCtx, InterceptorHandler,
                    ModuleCtx, StateCtx, ImplMeta,
                };
                if ctx.modules.contains_key("UserModule") {
                    return ctx;
                }
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Registering module {0}.\n", "UserModule"),
                    );
                };
                ctx.modules.insert("UserModule".to_string(), Box::new(self));
                ctx.imports.insert("UserModule".to_string(), Vec::from([]));
                ctx.append_exports("UserModule", Vec::from(["UserService"]), false);
                if ctx
                    .register_controller(
                        "UserModule",
                        "UserController",
                        Box::new(
                            std::sync::Arc::new(controller::UserController::default()),
                        ),
                    )
                {
                    let t_controller = ctx
                        .get_controller::<
                            controller::UserController,
                        >("UserModule", "UserController");
                    ctx = t_controller.__route_register(ctx);
                }
                let svc = std::sync::Arc::new(UserService::default());
                ctx.register_service("UserModule", "UserService", Box::new(svc));
                let svc = std::sync::Arc::new(UserEntity::default());
                ctx.register_service("UserModule", "UserEntity", Box::new(svc));
                let t = ctx.get_service::<UserService>("UserModule", "UserService");
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Injecting {0}::{1}.\n",
                            "UserModule",
                            "UserService",
                        ),
                    );
                };
                let ctx = t.inject(ctx, &"UserModule");
                let t = ctx.get_service::<UserEntity>("UserModule", "UserEntity");
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Injecting {0}::{1}.\n", "UserModule", "UserEntity"),
                    );
                };
                let ctx = t.inject(ctx, &"UserModule");
                let t = ctx
                    .get_controller::<UserController>("UserModule", "UserController");
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Injecting {0}::{1}.\n",
                            "UserModule",
                            "UserController",
                        ),
                    );
                };
                let ctx = t.inject(ctx, &"UserModule");
                ctx
            }
            fn destroy(&self, ctx: &nidrs::ModuleCtx) {
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Destroying module {0}.\n", "UserModule"),
                    );
                };
            }
        }
        impl nidrs::ImplMeta for UserModule {
            fn __meta() -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set_data(nidrs::datasets::ServiceName::from("UserService"));
                meta.set("service", "UserService");
                meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                meta.set("module", "UserModule");
                meta.set("global", "app");
                meta
            }
        }
    }
}
pub use nidrs::AppError;
pub use nidrs::AppResult;
fn main() {
    dotenvy::dotenv();
    let app = nidrs::NidrsFactory::create(app::AppModule);
    let app = app.default_prefix("/api/{version}");
    let app = app.default_version("v1");
    app.listen(3000).block();
}
pub mod import {
    pub use crate::modules::user::service::UserService;
    pub use crate::models::dao::users_extra::UserExtraEntity;
    pub use crate::app::service::AppService;
    pub use crate::models::dao::rooms::RoomEntity;
    pub use crate::models::dao::users::UserEntity;
    pub use crate::models::dao::downlogs::DownlogEntity;
    pub use crate::models::dao::resources::ResourceEntity;
    pub use crate::modules::user::controller::UserController;
    pub use crate::app::controller::AppController;
}
extern crate alloc;