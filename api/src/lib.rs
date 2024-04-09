pub mod proto {
    pub mod apiserver {
        tonic::include_proto!("apiserver");
        pub mod request {
            tonic::include_proto!("apiserver.request");
        }
        pub mod updateworkload {
            tonic::include_proto!("apiserver.updateworkload");
        }
    }
    pub mod statemanager {
        tonic::include_proto!("statemanager");
    }
}
