/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

pub mod proto {
    pub mod apiserver {
        tonic::include_proto!("apiserver");
        pub mod scenario {
            tonic::include_proto!("apiserver.scenario");
        }
    }
    pub mod gateway {
        tonic::include_proto!("piccologatewaypackage");
    }
    pub mod statemanager {
        tonic::include_proto!("statemanager");
    }
    pub mod yamlparser {
        tonic::include_proto!("yamlparser");
    }
    pub mod constants {
        tonic::include_proto!("constants");
    }
}
