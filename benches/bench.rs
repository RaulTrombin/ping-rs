use std::{net::SocketAddr, str::FromStr};

use bluerobotics_ping::{device::Common, message::ProtocolMessage};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use tokio_serial::{SerialPort, SerialPortBuilderExt};
// use criterion::async_executor::FuturesExecutor;
// use criterion::async_executor::AsyncExecutor;
use tokio::runtime::Runtime;
use udp_stream::UdpStream;

fn rt() -> Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}

fn sensor_benchmark(c: &mut Criterion) {

    // some previous required setup
    // let port = tokio_serial::new("/dev/ttyUSB0".to_string(), 115200)
    //     .open_native_async()
    //     .map_err(|e| {
    //         eprintln!("Error opening serial port: {}", e);
    //         e
    //     })
    //     .unwrap();
    // port.clear(tokio_serial::ClearBuffer::All).unwrap();




    c.bench_function("bench", move |b| {
        b.to_async(rt()).iter(|| async {
            let socket_addr = SocketAddr::from_str(&format!("192.168.0.191:8080"))
            .map_err(|e| {
                eprintln!("Error parsing UDP address: {}", e);
                e
            })
            .unwrap();
let port = UdpStream::connect(socket_addr)
            .await
            .map_err(|e| {
                eprintln!("Error connecting to UDP socket: {}", e);
                e
            })
            .unwrap();

let ping1d = Common::new(port);
let request = bluerobotics_ping::common::Messages::GeneralRequest(
    bluerobotics_ping::common::GeneralRequestStruct { requested_id: 5 },
);
let mut package = bluerobotics_ping::message::ProtocolMessage::new();
package.set_message(&request);
            ping1d.send_message(package.clone()).await.unwrap();

        })
    });
}

criterion_group!(benches, sensor_benchmark);
criterion_main!(benches);
