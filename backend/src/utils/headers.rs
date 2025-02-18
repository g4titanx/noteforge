use axum::http::HeaderMap;

#[macro_export]
macro_rules! headers_map {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut headers = HeaderMap::new();
        $(
            headers.insert(
                $key.parse().unwrap(),
                $value.parse().unwrap(),
            );
        )*
        headers
    }};
}
