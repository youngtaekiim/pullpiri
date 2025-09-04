# PICCOLO Clustering System

**Document No.**: PICCOLO-CLUSTERING-LLD-2025-001  
**Version**: 1.0  
**Date**: 2025-09-04  
**Author**: PICCOLO Team  
**Classification**: LLD (Low-Level Design)

## 1. Overview

The PICCOLO framework implements an efficient clustering mechanism between master nodes and sub (worker) nodes, enabling monitoring and management in distributed environments. This clustering system is designed with the specific characteristics of embedded environments in mind, optimized for limited resources, cloud connectivity, and small-scale clusters.

### 1.1 Purpose and Scope

This document includes:
- Detailed description of the PICCOLO clustering architecture
- Clustering-related functions and interfaces of the API Server and NodeAgent
- Cluster configuration, deployment, and management processes
- Special implementations optimized for embedded environments

### 1.2 Clustering Goals and Principles

1. **Minimal Architecture**
   - Lightweight design optimized for embedded environments
   - Support for small clusters of 2–10 nodes
   - Simplified master-sub node structure without leader election
   - Only NodeAgent runs on sub nodes to minimize resource load

2. **Hybrid Connectivity**
   - Support connections between embedded nodes and cloud nodes
   - Operates under various network conditions (unstable connections, limited bandwidth)
   - Local operation in offline state and synchronization upon reconnection

3. **Centralized State Management**
   - Store container monitoring data in etcd
   - Efficiently deliver state changes to StateManager
   - Concentrate API Server, FilterGateway, ActionController, StateManager, and MonitoringServer on the master node

4. **Resource Efficiency**
   - Podman-based container management using a daemonless architecture
   - Minimal overhead on limited hardware specs
   - Agent design optimized for low memory usage

## 2. API Server Clustering Functions

The API Server is the core component of the master node in the PICCOLO cluster, responsible for cluster configuration, node management, and artifact distribution.

### 2.1 Key Functions

1. **Node Management**
   - Handle node registration and authentication
   - Maintain and update cluster configuration information
   - Monitor node status and verify activity

2. **Artifact Distribution**
   - Manage artifacts to be deployed to sub nodes
   - Send artifact information to NodeAgent
   - Track and report deployment status

3. **Cluster Configuration Management**
   - Manage cluster topology information
   - Set master-sub node relationships
   - Manage node roles and permissions

### 2.2 System Architecture

Clustering-related modules of the API Server:

```text
apiserver
└── src
    ├── grpc
    │   └── sender
    │       ├── nodeagent.rs
    │       └── notification.rs
    └── node
        ├── manager.rs
        ├── registry.rs
        └── status.rs
```

### 2.3 Core Interfaces

#### 2.3.1 gRPC Communication with NodeAgent

The API Server communicates with NodeAgent via gRPC to deliver artifact information, check node status, etc.

```rust
/// Send artifact information to NodeAgent via gRPC
///
/// ### Parameters
/// * `artifact: ArtifactInfo` - Artifact information
/// * `metadata: Option<Metadata>` - Optional request metadata
/// ### Returns
/// * `Result<Response<ArtifactResponse>, Status>` - Response from NodeAgent
/// ### Description
/// Sends artifact information to NodeAgent using the gRPC client
/// Handles connection management and retries automatically
/// Includes security context and tracing information when available
...
```

#### 2.3.2 Node Registration and Management

The API Server manages the cluster’s node configuration and processes registration requests from new nodes.

```rust
/// Register a new node in the cluster
///
/// ### Parameters
/// * `node_info: NodeRegistrationRequest` - Node information and credentials
/// ### Returns
/// * `Result<NodeRegistrationResponse, NodeRegistrationError>` - Registration result
...
```

## 3. NodeAgent Clustering Functions

NodeAgent runs on each sub node, handling communication with the master node and local node management.

### 3.1 Key Functions

1. **Node Identification and Registration**
   - Assign and manage unique node ID
   - Collect and report system information
   - Request node registration with API Server

2. **Cluster Connection Management**
   - Maintain connection with master node
   - Attempt reconnection on network failure
   - Report active status via heartbeat mechanism

3. **System Readiness Check**
   - Verify system readiness before joining cluster
   - Check hardware resources, essential services, and network availability
   - Perform lightweight checks optimized for embedded environments

### 3.2 Clustering Process

#### 3.2.1 Node Discovery Phase

1. **Master Node Configuration**
   - API Server reads `node.yaml` config file to identify sub nodes to manage
   - Config file includes hostname, IP address, role (embedded/cloud), and access credentials
   - Static configuration by default, dynamic discovery for cloud nodes
   - Automatic sub node registration process on embedded system boot

#### 3.2.2 NodeAgent Deployment Phase

NodeAgent is deployed to sub nodes via an installation script (example provided in Bash).

#### 3.2.3 System Readiness Check Phase

Before joining the cluster, a readiness check script verifies node status (example provided in Bash).

#### 3.2.4 Node Connection and Authentication Phase

NodeAgent connects to the master node using Rust code (example provided).

### 3.3 Clustering Architecture

PICCOLO’s clustering architecture is designed for small-scale clusters optimized for embedded environments:

1. **Simplified Master-Sub Structure**
   - All core services on a single master node
   - Only NodeAgent runs on sub nodes
   - Predefined master node without leader election
   - Lightweight state management

2. **Podman-Based Container Management**
   - Daemonless architecture to minimize resource usage
   - Rootless mode for enhanced security
   - Lightweight runtime suitable for embedded devices
   - OCI standard compatibility

3. **etcd-Based State Storage**
   - Store container monitoring data in etcd
   - Ensure data consistency with distributed key-value store
   - Lightweight configuration for embedded environments
   - Data retention policy considering limited storage

4. **Hybrid Connectivity Model**
   - Integrated structure between embedded and cloud nodes
   - Operates in various network environments (wired, wireless, cellular)
   - Robust synchronization under intermittent connectivity
   - Extended features leveraging cloud connectivity

### 3.4 Cluster Topology Types

1. **Basic Embedded Topology**
   - Single master node with a few sub nodes
   - All core services on master node
   - Simple structure optimized for limited resources
   - Suitable for 2–5 node systems

2. **Edge-Cloud Hybrid Topology**
   - Connect local embedded cluster with cloud nodes
   - Combine fast edge processing with cloud scalability
   - Local operation possible under intermittent cloud connectivity
   - Distribute data processing load between edge and cloud

3. **Multi-Embedded Cluster Topology**
   - Multiple embedded clusters connected to an upper master node
   - Each cluster can operate independently
   - Hierarchical management in distributed environments
   - Isolation and resource management between clusters

4. **Geographically Distributed Topology**
   - Integrate geographically dispersed embedded systems
   - Dynamic configuration changes based on connectivity
   - Ensure autonomy of local clusters
   - Balance between centralized management and distributed processing

## 4. Cluster State Management

### 4.1 Node Status Monitoring

1. **Heartbeat Mechanism**
   - Regular heartbeat checks to verify node activity
   - Detect unresponsive nodes and update status
   - Automatic recovery procedure upon reconnection

2. **Resource Monitoring**
   - Monitor Podman container status and store in etcd
   - Monitor embedded device resources (CPU, memory, disk, power)
   - Set resource constraints and alert thresholds

3. **State Change Notification**
   - Immediately notify StateManager of critical state changes
   - Automatic recovery on node disconnection/reconnection
   - Log and analyze state events

### 4.2 Cluster Configuration Synchronization

1. **Configuration Distribution**
   - Propagate configuration changes from master to sub nodes
   - Support partial updates to minimize network traffic
   - Manage configuration versions and resolve conflicts

2. **Policy Synchronization**
   - Synchronize security policies, monitoring settings, resource constraints
   - Apply differentiated policies based on node type and role
   - Verify and report policy application status

## 5. Deployment and Operations

### 5.1 Initial Cluster Setup

1. **Master Node Configuration**
   - Install and configure API Server, FilterGateway, ActionController, StateManager, MonitoringServer
   - Set up and initialize etcd
   - Create cluster configuration file

2. **Sub Node Registration**
   - Run NodeAgent installation script
   - Specify master node IP and node type
   - Automatic service registration and startup

### 5.2 Cluster Expansion

1. **Adding New Nodes**
   - Pre-register node info on master or enable dynamic discovery
   - Install and register NodeAgent
   - Automatically update cluster topology

2. **Removing Nodes**
   - Safe node shutdown and removal procedure
   - Update cluster configuration
   - Release and clean up node resources

### 5.3 Fault Recovery

1. **Node Failure Detection**
   - Update node status on heartbeat failure
   - Log failures and generate alerts
   - Start automatic recovery procedure

2. **Recovery Procedure**
   - Check node status and attempt restart
   - Isolate node and notify admin on permanent failure
   - Reconfigure cluster and redistribute workloads

## 6. Security

### 6.1 Node Authentication

1. **Initial Authentication**
   - TLS-based node certificates
   - Use master node’s certificate authority
   - Secure initial authentication process

2. **Ongoing Authentication**
   - Periodic certificate renewal
   - Token-based session maintenance
   - Detect and block suspicious activity

### 6.2 Communication Security

1. **Encrypted Communication**
   - TLS encryption for gRPC communication
   - Secure API endpoints
   - Verify data integrity

2. **Access Control**
   - Role-based permissions for nodes
   - Apply principle of least privilege
   - Restrict and monitor resource access

## 7. References and Appendix

### 7.1 Related Documents

- HLD/base/piccolo_network(pharos).md – PICCOLO Network Architecture
- 3.LLD/apiserver.md – API Server Detailed Design
- 3.LLD/nodeagent.md – NodeAgent Detailed Design

### 7.2 Glossary

| Term | Definition |
|------|------------|
| Master Node | Central management node running core services like API Server, FilterGateway, ActionController |
| Sub Node | Worker node running only NodeAgent, managed by master node |
| NodeAgent | Agent running on each node, responsible for node status monitoring and communication with master node |
| Embedded Environment | Device environment with limited resources (CPU, memory, storage) |
| Podman | Daemonless container management tool, used as a Docker alternative |
| etcd | Distributed key-value store used for storing cluster state information |

