wget https://github.com/etcd-io/etcd/releases/download/v3.5.24/etcd-v3.5.24-linux-amd64.tar.gz
wget https://github.com/etcd-io/etcd/releases/download/v3.5.24/etcd-v3.5.24-linux-arm64.tar.gz

tar -xvf etcd-v3.5.24-linux-amd64.tar.gz
tar -xvf etcd-v3.5.24-linux-arm64.tar.gz

cp etcd-v3.5.24-linux-amd64/etcdctl ./etcdctl-amd64
cp etcd-v3.5.24-linux-arm64/etcdctl ./etcdctl-arm64

rm -rf etcd-*
