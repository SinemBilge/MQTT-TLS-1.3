use rumqttc::{MqttOptions, TlsConfiguration, Transport, Client, QoS};
use rumqttc::tokio_rustls::rustls::{ClientConfig, RootCertStore, Certificate};
use std::sync::Arc;
use std::time::Duration;
use std::thread;

fn main() {
    // Load certificates to the trust store
    let mut root_store = RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().expect("could not load platform certs") {
        root_store
            .add(&Certificate(cert.0))
            .unwrap();
    }
    

    // Build client config and set TLS version to 1.3
    let client_config = ClientConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[&rumqttc::tokio_rustls::rustls::version::TLS13])
        .expect("It looks like your system doesn't support TLS1.3")
        .with_root_certificates(root_store)
        .with_no_client_auth();
        


    let mut options = MqttOptions::new("tls13-test", "test.mosquitto.org", 8883);
        // .set_ca(items[0])
    options.set_transport(Transport::tls_with_config(TlsConfiguration::Rustls(Arc::new(client_config))));
    
    let (mut client, mut connection) = Client::new(options, 10);
    client.subscribe("hello/rumqtt", QoS::AtMostOnce).unwrap();
    thread::spawn(move || for i in 0..10 {
    client.publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize]).unwrap();
    thread::sleep(Duration::from_millis(100));
    });

    // Iterate to poll the eventloop for connection progress
    for (_i, notification) in connection.iter().enumerate() {
        println!("Notification = {:?}", notification);
    }

}
