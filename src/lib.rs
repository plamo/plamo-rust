use std::os::raw::{c_char, c_uchar, c_uint, c_void};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PlamoScheme {
    PlamoSchemeHttp,
    PlamoSchemeHttps,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PlamoHttpVersion {
    PlamoHttpVersionHttp09,
    PlamoHttpVersionHttp10,
    PlamoHttpVersionHttp11,
    PlamoHttpVersionHttp20,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlamoString {
    _unused: [u8; 0],
}

extern "C" {
    pub fn plamo_string_new(value: *const c_char) -> *mut PlamoString;
    pub fn plamo_string_destroy(plamo_string: *mut PlamoString);
    pub fn plamo_string_get_char(plamo_string: *const PlamoString) -> *const c_char;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlamoStringArray {
    _unused: [u8; 0],
}

extern "C" {
    pub fn plamo_string_array_new() -> *mut PlamoStringArray;
    pub fn plamo_string_array_destroy(plamo_string_array: *mut PlamoStringArray);
    pub fn plamo_string_array_length(plamo_string_array: *const PlamoStringArray) -> usize;
    pub fn plamo_string_array_for_each(
        plamo_string_array: *const PlamoStringArray,
        callback: Option<unsafe extern "C" fn(arg1: *const c_char)>,
    );
    pub fn plamo_string_array_get_at(
        plamo_string_array: *const PlamoStringArray,
        index: usize,
    ) -> *const c_char;
    pub fn plamo_string_array_get_first(
        plamo_string_array: *const PlamoStringArray,
    ) -> *const c_char;
    pub fn plamo_string_array_get_last(
        plamo_string_array: *const PlamoStringArray,
    ) -> *const c_char;
    pub fn plamo_string_array_add(
        plamo_string_array: *mut PlamoStringArray,
        value: *const c_char,
    );
    pub fn plamo_string_array_remove_at(
        plamo_string_array: *mut PlamoStringArray,
        index: usize,
    ) -> bool;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlamoByteArray {
    _unused: [u8; 0],
}

extern "C" {
    pub fn plamo_byte_array_new(
        body: *const c_uchar,
        length: usize,
    ) -> *mut PlamoByteArray;
    pub fn plamo_byte_array_destroy(plamo_byte_array: *mut PlamoByteArray);
    pub fn plamo_byte_array_get_body(
        plamo_byte_array: *const PlamoByteArray,
    ) -> *const c_uchar;
    pub fn plamo_byte_array_get_body_size(plamo_byte_array: *const PlamoByteArray) -> usize;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlamoHttpHeader {
    _unused: [u8; 0],
}

extern "C" {
    pub fn plamo_http_header_new() -> *mut PlamoHttpHeader;
    pub fn plamo_http_header_destroy(plamo_http_header: *mut PlamoHttpHeader);
    pub fn plamo_http_header_for_each(
        plamo_http_header: *mut PlamoHttpHeader,
        callback: Option<
            unsafe extern "C" fn(arg1: *const c_char, arg2: *const c_char),
        >,
    );
    pub fn plamo_http_header_get(
        plamo_http_header: *mut PlamoHttpHeader,
        key: *const c_char,
    ) -> *mut PlamoStringArray;
    pub fn plamo_http_header_add(
        plamo_http_header: *mut PlamoHttpHeader,
        key: *const c_char,
        value: *const c_char,
    );
    pub fn plamo_http_header_remove(
        plamo_http_header: *mut PlamoHttpHeader,
        key: *const c_char,
    ) -> bool;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlamoHttpQuery {
    _unused: [u8; 0],
}

extern "C" {
    pub fn plamo_http_query_new() -> *mut PlamoHttpQuery;
    pub fn plamo_http_query_destroy(plamo_http_query: *mut PlamoHttpQuery);
    pub fn plamo_http_query_for_each(
        plamo_http_query: *mut PlamoHttpQuery,
        callback: Option<
            unsafe extern "C" fn(arg1: *const c_char, arg2: *const c_char),
        >,
    );
    pub fn plamo_http_query_get(
        plamo_http_query: *mut PlamoHttpQuery,
        key: *const c_char,
    ) -> *mut PlamoStringArray;
    pub fn plamo_http_query_add(
        plamo_http_query: *mut PlamoHttpQuery,
        key: *const c_char,
        value: *const c_char,
    );
    pub fn plamo_http_query_remove(
        plamo_http_query: *mut PlamoHttpQuery,
        key: *const c_char,
    ) -> bool;
}

pub type PlamoDefinedHttpMethod = usize;

pub const PLAMO_HTTP_METHOD_GET: PlamoDefinedHttpMethod = usize::max_value();
pub const PLAMO_HTTP_METHOD_POST: PlamoDefinedHttpMethod = usize::max_value() - 1;
pub const PLAMO_HTTP_METHOD_PUT: PlamoDefinedHttpMethod = usize::max_value() - 2;
pub const PLAMO_HTTP_METHOD_DELETE: PlamoDefinedHttpMethod = usize::max_value() - 3;
pub const PLAMO_HTTP_METHOD_HEAD: PlamoDefinedHttpMethod = usize::max_value() - 4;
pub const PLAMO_HTTP_METHOD_CONNECT: PlamoDefinedHttpMethod = usize::max_value() - 5;
pub const PLAMO_HTTP_METHOD_OPTIONS: PlamoDefinedHttpMethod = usize::max_value() - 6;
pub const PLAMO_HTTP_METHOD_TRACE: PlamoDefinedHttpMethod = usize::max_value() - 7;
pub const PLAMO_HTTP_METHOD_PATCH: PlamoDefinedHttpMethod = usize::max_value() - 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub union PlamoHttpMethod {
    pub defined_http_method: PlamoDefinedHttpMethod,
    pub undefined_http_method: *mut c_char,
}

unsafe impl Send for PlamoHttpMethod {}
unsafe impl Sync for PlamoHttpMethod {}

extern "C" {
    pub fn plamo_http_method_new(method: usize) -> PlamoHttpMethod;
    pub fn plamo_http_method_destroy(plamo_http_method: *mut PlamoHttpMethod);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PlamoRequest {
    pub scheme: PlamoScheme,
    pub version: PlamoHttpVersion,
    pub method: PlamoHttpMethod,
    pub path: *mut PlamoString,
    pub query: *mut PlamoHttpQuery,
    pub header: *mut PlamoHttpHeader,
    pub body: *mut PlamoByteArray,
}

extern "C" {
    pub fn plamo_request_new(
        scheme: PlamoScheme,
        version: PlamoHttpVersion,
        method: PlamoHttpMethod,
        path: *const c_char,
        query: *mut PlamoHttpQuery,
        header: *mut PlamoHttpHeader,
        body: *mut PlamoByteArray,
    ) -> *mut PlamoRequest;
    pub fn plamo_request_destroy(plamo_request: *mut PlamoRequest);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlamoResponse {
    pub status_code: c_uint,
    pub header: *mut PlamoHttpHeader,
    pub body: *mut PlamoByteArray,
}

extern "C" {
    pub fn plamo_response_new() -> *mut PlamoResponse;
    pub fn plamo_response_destroy(plamo_response: *mut PlamoResponse);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlamoMiddleware {
    pub config: *const c_void,
    pub callback: Option<
        unsafe extern "C" fn(
            arg1: *const c_void,
            arg2: *const PlamoRequest,
            arg3: *mut PlamoResponse,
        ),
    >,
}

extern "C" {
    pub fn plamo_middleware_new(
        config: *const c_void,
        callback: Option<
            unsafe extern "C" fn(
                arg1: *const c_void,
                arg2: *const PlamoRequest,
                arg3: *mut PlamoResponse,
            ),
        >,
    ) -> *mut PlamoMiddleware;
    pub fn plamo_middleware_destroy(plamo_middleware: *mut PlamoMiddleware);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlamoApp {
    _unused: [u8; 0],
}

extern "C" {
    pub fn plamo_app_new() -> *mut PlamoApp;
    pub fn plamo_app_destroy(plamo_app: *mut PlamoApp);
    pub fn plamo_app_add_middleware(
        plamo_app: *mut PlamoApp,
        plamo_middleware: *const PlamoMiddleware,
    );
    pub fn plamo_app_execute(
        plamo_app: *const PlamoApp,
        plamo_request: *const PlamoRequest,
    ) -> *mut PlamoResponse;
}
