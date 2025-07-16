use lazy_static::lazy_static;
use prometheus::{histogram_opts, CounterVec, Encoder, Histogram, Registry, TextEncoder};
use rocket::get;
use rocket::response::content::RawText;

lazy_static! {
    pub static ref REGISTRY: Registry =
        Registry::new_custom(Some("centre_logistique".into()), None).unwrap();
    pub static ref HTTP_REQUESTS_TOTAL: CounterVec = CounterVec::new(
        prometheus::Opts::new("http_requests_total", "Nombre total de requêtes HTTP"),
        &["status"]
    )
    .unwrap();
    pub static ref HTTP_REQ_DURATION_SECONDS: Histogram = Histogram::with_opts(histogram_opts!(
        "http_request_duration_seconds",
        "Durée des requêtes HTTP"
    ))
    .unwrap();
}

pub fn init_metrics() {
    REGISTRY
        .register(Box::new(HTTP_REQUESTS_TOTAL.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(HTTP_REQ_DURATION_SECONDS.clone()))
        .unwrap();
}

#[get("/metrics")]
pub fn metrics() -> RawText<String> {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    RawText(String::from_utf8(buffer).unwrap())
}
