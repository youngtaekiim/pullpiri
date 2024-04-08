use common::etcd;

fn main() {
    let handle = std::thread::spawn(|| {
        std::process::Command::new("/usr/bin/etcd")
            .arg("--listen-peer-urls")
            .arg(etcd::LISTEN_PEER_URLS)
            .arg("--listen-client-urls")
            .arg(etcd::LISTEN_CLIENT_URLS)
            .arg("--advertise-client-urls")
            .arg(etcd::ADVERTISE_CLIENT_URLS)
            .stdout(std::process::Stdio::null())
            .output()
    });

    let _ = handle.join();
}
