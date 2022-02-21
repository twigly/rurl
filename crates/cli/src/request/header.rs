pub const ACCEPT: &str = "accept";
pub const CONTENT_TYPE: &str = "content-type";
pub const USER_AGENT: &str = "user-agent";

pub trait StandardHeader {
    fn is_standard(&self) -> bool;
}

impl StandardHeader for reqwest::header::HeaderName {
    fn is_standard(&self) -> bool {
        let header_name = self.as_str();
        [
            ACCEPT,
            "accept-ch",
            "accept-ch-lifetime",
            "accept-encoding",
            "accept-language",
            "accept-push-policy",
            "accept-ranges",
            "accept-signature",
            "access-control-allow-credentials",
            "access-control-allow-headers",
            "access-control-allow-methods",
            "access-control-allow-origin",
            "access-control-expose-headers",
            "access-control-max-age",
            "access-control-request-headers",
            "access-control-request-method",
            "age",
            "allow",
            "alt-svc",
            "authorization",
            "cache-control",
            "clear-site-data",
            "connection",
            "content-disposition",
            "content-dpr",
            "content-encoding",
            "content-language",
            "content-length",
            "content-location",
            "content-range",
            "content-security-policy",
            "content-security-policy-report-only",
            CONTENT_TYPE,
            "cookie",
            "cookie2",
            "cross-origin-embedder-policy",
            "cross-origin-opener-policy",
            "cross-origin-resource-policy",
            "date",
            "device-memory",
            "downlink",
            "dpr",
            "early-data",
            "ect",
            "etag",
            "expect",
            "expect-ct",
            "expires",
            "feature-policy",
            "forwarded",
            "from",
            "host",
            "if-match",
            "if-modified-since",
            "if-none-match",
            "if-range",
            "if-unmodified-since",
            "keep-alive",
            "large-allocation",
            "last-event-id",
            "last-modified",
            "link",
            "location",
            "max-forwards",
            "nel",
            "origin",
            "origin-isolation",
            "ping-from",
            "ping-to",
            "pragma",
            "proxy-authenticate",
            "proxy-authorization",
            "public-key-pins",
            "public-key-pins-report-only",
            "push-policy",
            "range",
            "referer",
            "referrer-policy",
            "report-to",
            "retry-after",
            "rtt",
            "save-data",
            "sec-ch-ua",
            "sec-ch-ua-arch",
            "sec-ch-ua-bitness",
            "sec-ch-ua-full-version",
            "sec-ch-ua-full-version-list",
            "sec-ch-ua-mobile",
            "sec-ch-ua-model",
            "sec-ch-ua-platform",
            "sec-ch-ua-platform-version",
            "sec-fetch-dest",
            "sec-fetch-mode",
            "sec-fetch-site",
            "sec-fetch-user",
            "sec-websocket-accept",
            "sec-websocket-extensions",
            "sec-websocket-key",
            "sec-websocket-protocol",
            "sec-websocket-version",
            "server",
            "server-timing",
            "service-worker-allowed",
            "set-cookie",
            "set-cookie2",
            "signature",
            "signed-headers",
            "sourcemap",
            "strict-transport-security",
            "te",
            "timing-allow-origin",
            "trailer",
            "transfer-encoding",
            "upgrade",
            "upgrade-insecure-requests",
            USER_AGENT,
            "vary",
            "via",
            "viewport-width",
            "warning",
            "width",
            "www-authenticate",
            "x-content-type-options",
            "x-dns-prefetch-control",
            "x-download-options",
            "x-firefox-spdy",
            "x-forwarded-for",
            "x-forwarded-host",
            "x-forwarded-proto",
            "x-frame-options",
            "x-permitted-cross-domain-policies",
            "x-pingback",
            "x-powered-by",
            "x-requested-with",
            "x-robots-tag",
            "x-ua-compatible",
            "x-xss-protection",
        ]
        .contains(&header_name)
    }
}