extern crate tls_api_test;
extern crate tls_api_native_tls;

#[test]
fn test_google() {
    tls_api_test::test_google::<tls_api_native_tls::TlsConnector>();
}

#[test]
fn connect_bad_hostname() {
    tls_api_test::connect_bad_hostname::<tls_api_native_tls::TlsConnector>();
}

#[test]
fn connect_bad_hostname_ignored() {
    tls_api_test::connect_bad_hostname_ignored::<tls_api_native_tls::TlsConnector>();
}

#[test]
fn server() {
    tls_api_test::server::<
        tls_api_native_tls::TlsConnector,
        tls_api_native_tls::TlsAcceptor>();
}

#[test]
fn tokio_fetch_google() {
    tls_api_test::tokio_fetch_google::<tls_api_native_tls::TlsConnector>();
}
