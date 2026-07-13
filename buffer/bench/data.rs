// Shared benchmark data types and builders.
#![allow(dead_code)]

#[derive(Value)]
pub struct OwnedData {
    id: i32,
    title: String,
    attributes: Vec<String>,
}

pub fn owned_data() -> OwnedData {
    OwnedData {
        id: 42,
        title: "A very important document".to_owned(),
        attributes: vec!["#1".to_owned(), "#2".to_owned(), "#3".to_owned()],
    }
}

#[derive(Value)]
pub struct BorrowedData<'a> {
    id: i32,
    title: &'a str,
    attributes: &'a [&'a str],
}

pub fn borrowed_data() -> BorrowedData<'static> {
    BorrowedData {
        id: 42,
        title: "A very important document",
        attributes: &["#1", "#2", "#3"],
    }
}

#[derive(Value)]
pub struct ExportLogsServiceRequest<'a> {
    #[sval(index = 1)]
    resource_logs: &'a [ResourceLogs<'a>],
}

#[derive(Value)]
pub struct ResourceLogs<'a> {
    #[sval(index = 1)]
    resource: Option<Resource<'a>>,
    #[sval(index = 2)]
    scope_logs: &'a [ScopeLogs<'a>],
    #[sval(index = 3)]
    schema_url: &'a str,
}

#[derive(Value)]
pub struct Resource<'a> {
    #[sval(index = 1)]
    attributes: &'a [KeyValue<'a>],
    #[sval(index = 2)]
    dropped_attribute_count: u32,
}

#[derive(Value)]
pub struct ScopeLogs<'a> {
    #[sval(index = 1)]
    scope: Option<InstrumentationScope<'a>>,
    #[sval(index = 2)]
    log_records: &'a [LogRecord<'a>],
    #[sval(index = 3)]
    schema_url: &'a str,
}

#[derive(Value)]
pub struct InstrumentationScope<'a> {
    #[sval(index = 1)]
    name: &'a str,
    #[sval(index = 2)]
    version: &'a str,
    #[sval(index = 3)]
    attributes: &'a [KeyValue<'a>],
    #[sval(index = 4)]
    dropped_attribute_count: u32,
}

#[derive(Value)]
#[repr(i32)]
pub enum SeverityNumber {
    Unspecified = 0,
    Trace = 1,
    Debug = 5,
    Info = 9,
    Warn = 13,
    Error = 17,
    Fatal = 21,
}

#[derive(Value)]
pub enum AnyValue<'a> {
    #[sval(index = 1)]
    String(&'a str),
    #[sval(index = 2)]
    Bool(bool),
    #[sval(index = 3)]
    Int(i64),
    #[sval(index = 4)]
    Double(f64),
    #[sval(index = 5)]
    Array(&'a [AnyValue<'a>]),
    #[sval(index = 6)]
    Kvlist(KvList<'a>),
    #[sval(index = 7)]
    Bytes(&'a sval::BinarySlice),
}

#[derive(Value)]
pub struct KvList<'a> {
    #[sval(index = 1)]
    values: &'a [KeyValue<'a>],
}

#[derive(Value)]
pub struct KeyValue<'a> {
    #[sval(index = 1)]
    key: &'a str,
    #[sval(index = 2)]
    value: Option<AnyValue<'a>>,
}

#[derive(Value)]
pub struct LogRecord<'a> {
    #[sval(index = 1)]
    time_unix_nano: u64,
    #[sval(index = 2)]
    severity_number: SeverityNumber,
    #[sval(index = 3)]
    severity_text: &'a str,
    #[sval(index = 5)]
    body: Option<AnyValue<'a>>,
    #[sval(index = 6)]
    attributes: &'a [KeyValue<'a>],
    #[sval(index = 7)]
    dropped_attributes_count: u32,
    #[sval(index = 8)]
    flags: u32,
    #[sval(index = 9)]
    trace_id: Option<&'a sval::BinarySlice>,
    #[sval(index = 10)]
    span_id: Option<&'a sval::BinarySlice>,
    #[sval(index = 11)]
    observed_time_unix_nano: u64,
}

pub const fn log_record1_data() -> LogRecord<'static> {
    LogRecord {
        time_unix_nano: 1696310935000000000,
        observed_time_unix_nano: 1696310935000000000,
        severity_number: SeverityNumber::Info,
        severity_text: "Info",
        body: Some(AnyValue::String("Added 1 x product {\"Name\":\"Rocket Ship Dark Roast, Whole Beans\",\"SizeInGrams\":100} to order")),
        attributes: &[
            KeyValue {
                key: "Action",
                value: Some(AnyValue::String("AddItem")),
            },
            KeyValue {
                key: "Controller",
                value: Some(AnyValue::String("OrdersController")),
            },
            KeyValue {
                key: "Application",
                value: Some(AnyValue::String("Roastery Web Frontend")),
            },
            KeyValue {
                key: "OrderId",
                value: Some(AnyValue::String("order-154613823ae87469e1edc0")),
            },
            KeyValue {
                key: "Origin",
                value: Some(AnyValue::String("seqcli sample ingest")),
            },
            KeyValue {
                key: "Product",
                value: Some(AnyValue::Kvlist(KvList {
                    values: &[
                        KeyValue {
                            key: "Name",
                            value: Some(AnyValue::String("Rocket Ship Dark Roast, Whole Beans")),
                        },
                        KeyValue {
                            key: "SizeInGrams",
                            value: Some(AnyValue::Int(100)),
                        },
                    ],
                })),
            },
            KeyValue {
                key: "ProductId",
                value: Some(AnyValue::String("product-8908fd0sa")),
            },
            KeyValue {
                key: "RequestId",
                value: Some(AnyValue::String("b249369fe6c70fa27e4897")),
            },
            KeyValue {
                key: "SourceContext",
                value: Some(AnyValue::String("Roastery.Api.OrdersController")),
            },
        ],
        dropped_attributes_count: 0,
        flags: 0,
        trace_id: None,
        span_id: None,
    }
}

pub const fn log_record2_data() -> LogRecord<'static> {
    LogRecord {
        time_unix_nano: 1696311466000000000,
        observed_time_unix_nano: 1696311466000000000,
        severity_number: SeverityNumber::Debug,
        severity_text: "Debug",
        body: Some(AnyValue::String("Execution of insert into roastery.orderitem (orderid, productid) values ('order-724537b0f2348b2d60aabf', 'product-cvsad9033') returning id; affected 1 rows in 9.241 ms")),
        attributes: &[
            KeyValue {
                key: "Action",
                value: Some(AnyValue::String("AddItem")),
            },
            KeyValue {
                key: "Controller",
                value: Some(AnyValue::String("OrdersController")),
            },
            KeyValue {
                key: "Application",
                value: Some(AnyValue::String("Roastery Web Frontend")),
            },
            KeyValue {
                key: "Elapsed",
                value: Some(AnyValue::Double(9.241)),
            },
            KeyValue {
                key: "OrderId",
                value: Some(AnyValue::String("order-724537b0f2348b2d60aabf")),
            },
            KeyValue {
                key: "Origin",
                value: Some(AnyValue::String("seqcli sample ingest")),
            },
            KeyValue {
                key: "ProductId",
                value: Some(AnyValue::String("product-cvsad9033")),
            },
            KeyValue {
                key: "RequestId",
                value: Some(AnyValue::String("044ca890649f3111d0bddb")),
            },
            KeyValue {
                key: "RowCount",
                value: Some(AnyValue::Int(1)),
            },
            KeyValue {
                key: "SourceContext",
                value: Some(AnyValue::String("Roastery.Data.Database")),
            },
            KeyValue {
                key: "Sql",
                value: Some(AnyValue::String("insert into roastery.orderitem (orderid, productid) values ('order-724537b0f2348b2d60aabf', 'product-cvsad9033') returning id;")),
            },
        ],
        dropped_attributes_count: 0,
        flags: 0,
        trace_id: None,
        span_id: None,
    }
}

pub const fn log_record3_data() -> LogRecord<'static> {
    LogRecord {
        time_unix_nano: 1696311899000000000,
        observed_time_unix_nano: 1696311899000000000,
        severity_number: SeverityNumber::Error,
        severity_text: "Error",
        body: Some(AnyValue::String("HTTP POST /api/orders/order-ad424d996f277c10e38056/items responded 500 in 10.211 ms")),
        attributes: &[
            KeyValue {
                key: "Exception",
                value: Some(AnyValue::String("System.OperationCanceledException: A deadlock was detected and the transaction chosen as the deadlock victim.\n   at Roastery.Data.Database.LogExecAsync(String sql, Int32 rowCount) in /Users/user/dl/osx/seqcli/src/Roastery/Data/Database.cs:line 139\n   at Roastery.Data.Database.SelectAsync[T](Func`2 predicate, String where) in /Users/user/dl/osx/seqcli/src/Roastery/Data/Database.cs:line 55\n   at Roastery.Api.OrdersController.AddItem(HttpRequest request) in /Users/user/dl/osx/seqcli/src/Roastery/Api/OrdersController.cs:line 98\n   at Roastery.Web.Router.InvokeAsync(HttpRequest request) in /Users/user/dl/osx/seqcli/src/Roastery/Web/Router.cs:line 94\n   at Roastery.Web.FaultInjectionMiddleware.InvokeAsync(HttpRequest request) in /Users/user/dl/osx/seqcli/src/Roastery/Web/FaultInjectionMiddleware.cs:line 64\n   at Roastery.Web.SchedulingLatencyMiddleware.InvokeAsync(HttpRequest request) in /Users/user/dl/osx/seqcli/src/Roastery/Web/SchedulingLatencyMiddleware.cs:line 32\n   at Roastery.Web.RequestLoggingMiddleware.InvokeAsync(HttpRequest request) in /Users/user/dl/osx/seqcli/src/Roastery/Web/RequestLoggingMiddleware.cs:line 29")),
            },
            KeyValue {
                key: "Application",
                value: Some(AnyValue::String("Roastery Web Frontend")),
            },
            KeyValue {
                key: "Elapsed",
                value: Some(AnyValue::Double(10.2111)),
            },
            KeyValue {
                key: "Origin",
                value: Some(AnyValue::String("seqcli sample ingest")),
            },
            KeyValue {
                key: "RequestId",
                value: Some(AnyValue::String("4e48b8a4a87cd9ecfb6e37")),
            },
            KeyValue {
                key: "RequestMethod",
                value: Some(AnyValue::String("POST")),
            },
            KeyValue {
                key: "RequestPath",
                value: Some(AnyValue::String("/api/orders/order-ad424d996f277c10e38056/items")),
            },
            KeyValue {
                key: "SourceContext",
                value: Some(AnyValue::String("Roastery.Web.RequestLoggingMiddleware")),
            },
            KeyValue {
                key: "StatusCode",
                value: Some(AnyValue::Int(500)),
            },
        ],
        dropped_attributes_count: 0,
        flags: 0,
        trace_id: None,
        span_id: None,
    }
}

pub fn export_logs_service_request_data() -> ExportLogsServiceRequest<'static> {
    ExportLogsServiceRequest {
        resource_logs: &[ResourceLogs {
            resource: Some(Resource {
                attributes: &[KeyValue {
                    key: "service.name",
                    value: Some(AnyValue::String("sval_protobuf_tests")),
                }],
                dropped_attribute_count: 0,
            }),
            scope_logs: {
                const SCOPE_LOGS: &'static [ScopeLogs<'static>] = &[ScopeLogs {
                    scope: None,
                    log_records: &[log_record1_data(), log_record2_data(), log_record3_data()],
                    schema_url: "",
                }];

                SCOPE_LOGS
            },
            schema_url: "",
        }],
    }
}

// An exponential histogram: bucket midpoints with their counts.
//
// The values themselves don't matter; the shape does — a long seq of small
// float/int pairs, where per-container costs dominate the payload bytes
pub fn histogram_data() -> Vec<(f64, u32)> {
    (0..160)
        .map(|i| (2f64.powf(i as f64 / 20.0), i as u32 * 37 % 256))
        .collect()
}

// A map where all keys are small borrowed strings: the target case for
// inlining small borrowed fragments so `into_owned` stays free
pub fn small_key_map_data() -> std::collections::BTreeMap<String, u64> {
    (0..32).map(|i| (format!("key-{i}"), i as u64)).collect()
}
