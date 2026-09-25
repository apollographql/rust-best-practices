use std::{num::NonZeroU16, path::PathBuf};

use serde::{Deserialize, Serialize};
use url::Host;

/// Wire shape: mirrors the external format and admits invalid combinations.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct RawListener {
    host: String,
    port: u16,
    tls_cert: Option<PathBuf>,
    tls_key: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "RawListener", into = "RawListener")]
struct Listener {
    /// Domain, IPv4, or bracketed IPv6, per the URL host grammar.
    host: Host,
    port: NonZeroU16,
    transport: Transport,
}

/// Certificate and key are required together, so they share a variant.
#[derive(Clone, Debug, PartialEq)]
enum Transport {
    Plain,
    Tls { cert: PathBuf, key: PathBuf },
}

#[derive(Debug, PartialEq, thiserror::Error)]
enum ListenerError {
    #[error("invalid host")]
    InvalidHost(#[from] url::ParseError),
    #[error("port must be non-zero")]
    ZeroPort,
    #[error("tls_cert and tls_key must be set together")]
    PartialTls,
}

impl TryFrom<RawListener> for Listener {
    type Error = ListenerError;

    fn try_from(raw: RawListener) -> Result<Self, Self::Error> {
        let transport = match (raw.tls_cert, raw.tls_key) {
            (None, None) => Transport::Plain,
            (Some(cert), Some(key)) => Transport::Tls { cert, key },
            (Some(_), None) | (None, Some(_)) => return Err(ListenerError::PartialTls),
        };
        Ok(Self {
            host: Host::parse(&raw.host)?,
            port: NonZeroU16::new(raw.port).ok_or(ListenerError::ZeroPort)?,
            transport,
        })
    }
}

impl From<Listener> for RawListener {
    fn from(listener: Listener) -> Self {
        let (tls_cert, tls_key) = match listener.transport {
            Transport::Plain => (None, None),
            Transport::Tls { cert, key } => (Some(cert), Some(key)),
        };
        Self {
            host: listener.host.to_string(),
            port: listener.port.get(),
            tls_cert,
            tls_key,
        }
    }
}

/// Interior code takes the parsed type and never re-checks its invariants.
fn url(listener: &Listener) -> String {
    let scheme = match listener.transport {
        Transport::Plain => "http",
        Transport::Tls { .. } => "https",
    };
    // `Host`'s `Display` brackets IPv6, so the authority is well-formed.
    format!("{scheme}://{}:{}", listener.host, listener.port)
}

fn main() -> Result<(), serde_json::Error> {
    let input = r#"{"host":"localhost","port":8443,"tls_cert":"cert.pem","tls_key":"key.pem"}"#;
    let listener: Listener = serde_json::from_str(input)?;
    println!("{}", url(&listener));
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};

    use proptest::prelude::*;
    use url::Url;

    use super::*;

    fn raw(host: &str, port: u16, tls_cert: Option<&str>, tls_key: Option<&str>) -> RawListener {
        RawListener {
            host: host.to_owned(),
            port,
            tls_cert: tls_cert.map(PathBuf::from),
            tls_key: tls_key.map(PathBuf::from),
        }
    }

    #[test]
    fn rejects_invalid_wire_shapes() {
        let cases = [
            (raw("localhost", 0, None, None), ListenerError::ZeroPort),
            (
                raw("localhost", 80, Some("cert.pem"), None),
                ListenerError::PartialTls,
            ),
            (
                raw("localhost", 80, None, Some("key.pem")),
                ListenerError::PartialTls,
            ),
        ];
        for (input, expected) in cases {
            assert_eq!(Listener::try_from(input), Err(expected));
        }
    }

    #[test]
    fn rejects_hosts_that_would_corrupt_the_authority() {
        for host in [
            "",
            " ",
            "localhost/path",
            "localhost:80",
            "user@localhost",
            "::1",
        ] {
            assert!(
                matches!(
                    Listener::try_from(raw(host, 80, None, None)),
                    Err(ListenerError::InvalidHost(_))
                ),
                "{host:?} should be rejected"
            );
        }
    }

    #[test]
    fn brackets_ipv6_in_urls() {
        let listener = Listener::try_from(raw("[::1]", 80, None, None)).expect("bracketed IPv6");
        assert_eq!(url(&listener), "http://[::1]:80");
    }

    fn listener() -> impl Strategy<Value = Listener> {
        let host = prop_oneof![
            "[a-z][a-z0-9-]{0,8}(\\.[a-z][a-z0-9-]{0,8}){0,2}".prop_map(Host::Domain),
            any::<Ipv4Addr>().prop_map(Host::Ipv4),
            any::<Ipv6Addr>().prop_map(Host::Ipv6),
        ];
        let transport = prop_oneof![
            Just(Transport::Plain),
            ("[a-z]{1,8}", "[a-z]{1,8}").prop_map(|(cert, key)| Transport::Tls {
                cert: cert.into(),
                key: key.into(),
            }),
        ];
        (host, 1..=u16::MAX, transport).prop_map(|(host, port, transport)| Listener {
            host,
            port: NonZeroU16::new(port).expect("strategy excludes zero"),
            transport,
        })
    }

    proptest! {
        #[test]
        fn json_round_trip_preserves_domain_value(listener in listener()) {
            let json = serde_json::to_string(&listener).expect("domain value serializes");
            let parsed: Listener = serde_json::from_str(&json).expect("serialized value parses");
            prop_assert_eq!(parsed, listener);
        }

        /// Semantic check: URL parsing recovers the host and port (shared host grammar).
        #[test]
        fn url_preserves_host_and_port(listener in listener()) {
            let parsed = Url::parse(&url(&listener)).expect("url() emits a valid URL");
            prop_assert_eq!(parsed.host().map(|host| host.to_owned()), Some(listener.host.clone()));
            prop_assert_eq!(parsed.port_or_known_default(), Some(listener.port.get()));
        }
    }
}
