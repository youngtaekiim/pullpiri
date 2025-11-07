/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use rocksdb::{IteratorMode, Options, WriteBatch, DB};
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;

// Global RocksDB instance using safe OnceLock
static DB_INSTANCE: OnceLock<Arc<Mutex<DB>>> = OnceLock::new();

pub struct KV {
    pub key: String,
    pub value: String,
}

// Initialize RocksDB (call once at startup)
pub fn init_db(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "[ROCKSDB_INIT_DEBUG] Initializing RocksDB at path: '{}'",
        path
    );

    let mut opts = Options::default();
    opts.create_if_missing(true);

    // Performance optimizations
    opts.set_max_background_jobs(4); // 백그라운드 작업 증가
    opts.set_write_buffer_size(64 * 1024 * 1024); // 64MB 버퍼
    opts.set_max_write_buffer_number(3); // 버퍼 개수
    opts.set_target_file_size_base(64 * 1024 * 1024); // SST 파일 크기
    opts.increase_parallelism(4); // CPU 병렬 처리 (4 코어)

    println!("[ROCKSDB_INIT_DEBUG] Opening RocksDB with optimized settings...");

    let db = DB::open(&opts, path)?;

    // Safe initialization using OnceLock
    DB_INSTANCE.set(Arc::new(Mutex::new(db))).map_err(|_| {
        println!("[ROCKSDB_INIT_DEBUG] ERROR: RocksDB already initialized");
        format!("RocksDB already initialized")
    })?;

    println!(
        "[ROCKSDB_INIT_DEBUG] RocksDB successfully initialized at path: '{}'",
        path
    );

    Ok(())
}

// Get DB instance safely
fn get_db() -> Result<Arc<Mutex<DB>>, String> {
    DB_INSTANCE
        .get()
        .ok_or_else(|| "RocksDB not initialized. Call init_db() first.".to_string())
        .map(|db| db.clone())
}

// Check if RocksDB is initialized and accessible
pub async fn is_db_alive() -> bool {
    match get_db() {
        Ok(db) => {
            // Try to acquire lock to ensure DB is accessible
            match db.try_lock() {
                Ok(_) => true,
                Err(_) => false, // DB is locked, but it exists
            }
        }
        Err(_) => false,
    }
}

// Get detailed DB status
pub async fn get_db_status() -> Result<String, String> {
    let db = get_db()?;
    let db_lock = db.lock().await;

    // Get some basic stats from RocksDB
    match db_lock.property_value("rocksdb.stats") {
        Ok(Some(stats)) => Ok(stats),
        Ok(None) => Ok("RocksDB is alive but stats not available".to_string()),
        Err(e) => Err(format!("Failed to get DB stats: {}", e)),
    }
}

// Comprehensive health check
pub async fn health_check() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // Check if DB is initialized
    if !is_db_alive().await {
        return Err("RocksDB is not initialized or accessible".into());
    }

    // Try a simple read/write operation
    let test_key = "__health_check_test__";
    let test_value = "test_value";

    // Test write
    put(test_key, test_value).await?;

    // Test read
    let retrieved_value = get(test_key).await?;

    // Cleanup test data
    delete(test_key).await?;

    // Verify the operation worked
    if retrieved_value == test_value {
        Ok("RocksDB health check passed - read/write operations working".to_string())
    } else {
        Err("RocksDB health check failed - data integrity issue".into())
    }
}

// etcd.rs와 동일한 API 제공
pub fn open_server() -> String {
    let config = crate::setting::get_config();
    if config.host.ip.is_empty() {
        panic!("Host IP is missing in the configuration.");
    }

    // Validate the IP format
    if config.host.ip.parse::<std::net::IpAddr>().is_err() {
        panic!("Invalid IP address format: {}", config.host.ip);
    }

    // RocksDB는 로컬 DB이므로 IP 반환 (호환성을 위해)
    format!("{}:2379", config.host.ip)
}

pub async fn put(key: &str, value: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // DEBUG LOG: Track RocksDB put operations
    println!(
        "[ROCKSDB_PUT_DEBUG] Called with key='{}', value_len={}",
        key,
        value.len()
    );

    // Validate key length
    if key.len() > 1024 {
        println!(
            "[ROCKSDB_PUT_DEBUG] ERROR: Key too long: {} characters",
            key.len()
        );
        return Err("Key exceeds maximum allowed length of 1024 characters".into());
    }

    // Validate key for invalid special characters
    if key.contains(['<', '>', '?', '{', '}']) {
        println!(
            "[ROCKSDB_PUT_DEBUG] ERROR: Key contains invalid characters: {}",
            key
        );
        return Err("Key contains invalid special characters".into());
    }

    let db = get_db()?;
    let db_lock = db.lock().await;

    println!("[ROCKSDB_PUT_DEBUG] About to write to RocksDB...");

    db_lock.put(key.as_bytes(), value.as_bytes()).map_err(|e| {
        println!("[ROCKSDB_PUT_DEBUG] ERROR: RocksDB put failed: {}", e);
        format!("RocksDB put error: {}", e)
    })?;

    println!(
        "[ROCKSDB_PUT_DEBUG] Successfully wrote key='{}' to RocksDB",
        key
    );

    Ok(())
}

pub async fn get(key: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // DEBUG LOG: Track RocksDB get operations
    println!("[ROCKSDB_GET_DEBUG] Called with key='{}'", key);

    // Validate key length
    if key.is_empty() {
        println!("[ROCKSDB_GET_DEBUG] ERROR: Key is empty");
        return Err("Key cannot be empty".into());
    }

    if key.len() > 1024 {
        println!(
            "[ROCKSDB_GET_DEBUG] ERROR: Key too long: {} characters",
            key.len()
        );
        return Err("Key exceeds maximum allowed length of 1024 characters".into());
    }

    // Validate key for invalid special characters
    if key.contains(['<', '>', '?', '{', '}']) {
        println!(
            "[ROCKSDB_GET_DEBUG] ERROR: Key contains invalid characters: {}",
            key
        );
        return Err("Key contains invalid special characters".into());
    }

    let db = get_db()?;
    let db_lock = db.lock().await;

    println!("[ROCKSDB_GET_DEBUG] About to read from RocksDB...");

    match db_lock.get(key.as_bytes())? {
        Some(value) => {
            let result = String::from_utf8(value).map_err(
                |e| -> Box<dyn std::error::Error + Send + Sync> {
                    println!("[ROCKSDB_GET_DEBUG] ERROR: UTF-8 conversion failed: {}", e);
                    format!("UTF-8 conversion error: {}", e).into()
                },
            )?;
            println!(
                "[ROCKSDB_GET_DEBUG] Successfully read key='{}', value_len={}",
                key,
                result.len()
            );
            Ok(result)
        }
        None => {
            println!("[ROCKSDB_GET_DEBUG] Key not found: '{}'", key);
            Err("Key not found".into())
        }
    }
}

pub async fn get_all_with_prefix(
    prefix: &str,
) -> Result<Vec<KV>, Box<dyn std::error::Error + Send + Sync>> {
    // DEBUG LOG: Track RocksDB prefix searches
    println!("[ROCKSDB_PREFIX_DEBUG] Called with prefix='{}'", prefix);

    let db = get_db()?;
    let db_lock = db.lock().await;

    let mut results = Vec::new();
    let iter = db_lock.iterator(IteratorMode::Start);

    for item in iter {
        let (key_bytes, value_bytes) = item?;
        let key = String::from_utf8(key_bytes.to_vec())?;

        if key.starts_with(prefix) {
            let value = String::from_utf8(value_bytes.to_vec())?;
            results.push(KV {
                key: key.clone(),
                value,
            });
            println!("[ROCKSDB_PREFIX_DEBUG] Found matching key: '{}'", key);
        }
    }

    println!(
        "[ROCKSDB_PREFIX_DEBUG] Found {} keys with prefix '{}'",
        results.len(),
        prefix
    );

    Ok(results)
}

pub async fn delete(key: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Validate key length
    if key.len() > 1024 {
        return Err("Key exceeds maximum allowed length of 1024 characters".into());
    }

    // Validate key for invalid special characters
    if key.contains(['<', '>', '?', '{', '}']) {
        return Err("Key contains invalid special characters".into());
    }

    let db = get_db()?;
    let db_lock = db.lock().await;

    db_lock
        .delete(key.as_bytes())
        .map_err(|e| format!("RocksDB delete error: {}", e))?;

    Ok(())
}

// High-performance batch operations for metrics and logs
pub async fn put_batch(
    items: Vec<(String, String)>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if items.is_empty() {
        return Ok(());
    }

    let db = get_db()?;
    let db_lock = db.lock().await;

    let mut batch = WriteBatch::default();
    for (key, value) in items {
        // Validate each key
        if key.len() > 1024 || key.contains(['<', '>', '?', '{', '}']) {
            return Err(format!("Invalid key: {}", key).into());
        }
        batch.put(key.as_bytes(), value.as_bytes());
    }

    db_lock
        .write(batch)
        .map_err(|e| format!("RocksDB batch write error: {}", e))?;

    Ok(())
}

// Optimized for metrics storage (time-series data)
pub async fn put_metric(
    metric_type: &str,
    timestamp: u64,
    node_id: &str,
    data: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let key = format!("/metrics/{}/{}/{}", metric_type, node_id, timestamp);
    put(&key, data).await
}

// Optimized for log storage
pub async fn put_log(
    component: &str,
    level: &str,
    timestamp: u64,
    message: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let key = format!("/logs/{}/{}/{}", component, level, timestamp);
    put(&key, message).await
}

// Get time-range data (for metrics/logs)
pub async fn get_range(
    prefix: &str,
    start_time: u64,
    end_time: u64,
) -> Result<Vec<KV>, Box<dyn std::error::Error + Send + Sync>> {
    let all_data = get_all_with_prefix(prefix).await?;

    let mut filtered_data = Vec::new();
    for kv in all_data {
        // Extract timestamp from key (assuming format: /prefix/component/timestamp)
        if let Some(timestamp_str) = kv.key.split('/').last() {
            if let Ok(timestamp) = timestamp_str.parse::<u64>() {
                if timestamp >= start_time && timestamp <= end_time {
                    filtered_data.push(kv);
                }
            }
        }
    }

    // Sort by timestamp
    filtered_data.sort_by(|a, b| {
        let a_time = a
            .key
            .split('/')
            .last()
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0);
        let b_time = b
            .key
            .split('/')
            .last()
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0);
        a_time.cmp(&b_time)
    });

    Ok(filtered_data)
}

pub async fn delete_all_with_prefix(
    prefix: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db = get_db()?;
    let db_lock = db.lock().await;

    let mut keys_to_delete = Vec::new();
    let iter = db_lock.iterator(IteratorMode::Start);

    // 먼저 삭제할 키들을 수집
    for item in iter {
        let (key_bytes, _) = item?;
        let key = String::from_utf8(key_bytes.to_vec())?;

        if key.starts_with(prefix) {
            keys_to_delete.push(key);
        }
    }

    // 수집된 키들을 삭제
    for key in keys_to_delete {
        db_lock.delete(key.as_bytes())?;
    }

    Ok(())
}
