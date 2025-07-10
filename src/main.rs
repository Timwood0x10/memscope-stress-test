use std::sync::{Arc, Mutex, RwLock};
use std::collections::{HashMap, BTreeMap, VecDeque};
use std::rc::{Rc, Weak};
use tokio;
use tokio::time::{sleep, Duration};
use memscope_rs::{track_var, get_global_tracker};
use memscope_rs::export::export_to_svg;
use rayon::prelude::*;

// Macro to handle track_var results and suppress warnings
macro_rules! track_var_safe {
    ($var:ident) => {
        let _ = track_var!($var);
    };
}

mod data_structures;

use data_structures::DataManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting COMPREHENSIVE memscope-rs stress test and pressure testing...");
    println!("{}", "=".repeat(100));
    
    // Phase 1: Basic data structure stress test
    println!("Phase 1: Basic Data Structure Stress Test");
    basic_data_structure_stress_test().await?;
    
    // Phase 2: Complex nested structures
    println!("\nPhase 2: Complex Nested Structure Test");
    complex_nested_structure_test().await?;
    
    // Phase 3: Concurrent memory operations
    println!("\nPhase 3: Concurrent Memory Operations Test");
    concurrent_memory_operations_test().await?;
    
    // Phase 4: Large scale data processing
    println!("\nPhase 4: Large Scale Data Processing Test");
    large_scale_data_processing_test().await?;
    
    // Phase 5: Memory fragmentation test
    println!("\nPhase 5: Memory Fragmentation Test");
    memory_fragmentation_test().await?;
    
    // Phase 6: Circular reference stress test
    println!("\nPhase 6: Circular Reference Stress Test");
    circular_reference_stress_test().await?;
    
    // Phase 7: Mixed workload simulation
    println!("\nPhase 7: Mixed Workload Simulation");
    mixed_workload_simulation().await?;
    
    // Phase 8: Memory leak detection test
    println!("\nPhase 8: Memory Leak Detection Test");
    memory_leak_detection_test().await?;
    
    println!("\nAll stress tests completed successfully!");
    println!("This comprehensive test demonstrates memscope-rs capabilities under extreme conditions:");
    println!("   • {} different test phases", 8);
    println!("   • Thousands of tracked allocations");
    println!("   • Complex data structure hierarchies");
    println!("   • Concurrent memory operations");
    println!("   • Large-scale data processing");
    println!("   • Memory fragmentation scenarios");
    println!("   • Circular reference patterns");
    println!("   • Mixed workload simulations");
    println!("   • Memory leak detection");
    
    // Export memory tracking data to SVG
    println!("\nGenerating comprehensive SVG visualization...");
    let tracker = get_global_tracker();
    match export_to_svg(&tracker, "memory_analysis.svg") {
        Ok(_) => println!("SVG visualization generated: memory_analysis.svg"),
        Err(e) => println!("Failed to generate SVG: {}", e),
    }
    
    println!("Check the generated SVG file for detailed memory analysis!");
    
    Ok(())
}

// Phase 1: Basic data structure stress test
async fn basic_data_structure_stress_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Creating massive basic data structures...");
    
    // Create thousands of vectors with different sizes
    for i in 0..1000 {
        let size = 100 + (i % 1000);
        let vec_data: Vec<u64> = (0..size).map(|x| x as u64 * i as u64).collect();
        track_var_safe!(vec_data);
        
        if i % 100 == 0 {
            println!("    Created {} vectors", i + 1);
        }
    }
    
    // Create large string collections
    for batch in 0..50 {
        let strings: Vec<String> = (0..200)
            .map(|i| format!("Batch {batch} - String {i} - {}", "x".repeat(50 + i % 100)))
            .collect();
        track_var_safe!(strings);
    }
    
    // Create nested hashmaps
    for i in 0..100 {
        let mut nested_map: HashMap<String, HashMap<u32, Vec<String>>> = HashMap::new();
        for j in 0..50 {
            let mut inner_map: HashMap<u32, Vec<String>> = HashMap::new();
            for k in 0..20 {
                let vec_strings: Vec<String> = (0..10)
                    .map(|l| format!("nested_{i}_{j}_{k}_{l}"))
                    .collect();
                inner_map.insert(k, vec_strings);
            }
            nested_map.insert(format!("key_{i}_{j}"), inner_map);
        }
        let nested_map_box = Box::new(nested_map);
        track_var_safe!(nested_map_box);
    }
    
    println!("  Basic data structure stress test completed");
    Ok(())
}

// Phase 2: Complex nested structures
async fn complex_nested_structure_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Creating deeply nested complex structures...");
    
    // Create complex nested structures
    for i in 0..50 {
        let complex_structure: BTreeMap<String, Arc<Mutex<VecDeque<Box<HashMap<u64, Vec<String>>>>>>> = 
            (0..20).map(|j| {
                let key = format!("complex_key_{i}_{j}");
                let deque: VecDeque<Box<HashMap<u64, Vec<String>>>> = (0..10).map(|k| {
                    let mut map = HashMap::new();
                    for l in 0..15 {
                        let strings: Vec<String> = (0..8)
                            .map(|m| format!("deep_nested_{i}_{j}_{k}_{l}_{m}"))
                            .collect();
                        map.insert(l as u64, strings);
                    }
                    Box::new(map)
                }).collect();
                (key, Arc::new(Mutex::new(deque)))
            }).collect();
        let complex_structure_box = Box::new(complex_structure);
        track_var_safe!(complex_structure_box);
    }
    
    // Create recursive data structures
    #[derive(Clone)]
    #[allow(dead_code)]
    struct RecursiveNode {
        id: u64,
        data: Vec<String>,
        children: Vec<Box<RecursiveNode>>,
    }
    
    fn create_recursive_tree(depth: u32, id: u64) -> Box<RecursiveNode> {
        let data: Vec<String> = (0..20)
            .map(|i| format!("recursive_data_{id}_{i}"))
            .collect();
        
        let children = if depth > 0 {
            (0..5).map(|i| create_recursive_tree(depth - 1, id * 10 + i)).collect()
        } else {
            Vec::new()
        };
        
        Box::new(RecursiveNode { id, data, children })
    }
    
    for i in 0..20 {
        let tree = create_recursive_tree(4, i);
        track_var_safe!(tree);
    }
    
    println!("  Complex nested structure test completed");
    Ok(())
}

// Phase 3: Concurrent memory operations
async fn concurrent_memory_operations_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Running concurrent memory operations...");
    
    let shared_data = Arc::new(RwLock::new(HashMap::<String, Vec<u8>>::new()));
    track_var_safe!(shared_data);
    
    let mut handles = Vec::new();
    
    // Spawn multiple concurrent tasks
    for task_id in 0..100 {
        let shared_clone = Arc::clone(&shared_data);
        let handle = tokio::spawn(async move {
            for i in 0..50 {
                let key = format!("task_{task_id}_{i}");
                let data: Vec<u8> = (0..1000).map(|_| rand::random()).collect();
                
                // Write operation
                {
                    let mut map = shared_clone.write().unwrap();
                    map.insert(key.clone(), data.clone());
                }
                
                // Create local data structures
                let local_data: Vec<Vec<String>> = (0..20).map(|j| {
                    (0..30).map(|k| format!("concurrent_{task_id}_{i}_{j}_{k}")).collect()
                }).collect();
                track_var_safe!(local_data);
                
                // Small delay to simulate work
                sleep(Duration::from_millis(1)).await;
            }
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        handle.await?;
    }
    
    println!("  Concurrent memory operations test completed");
    Ok(())
}

// Phase 4: Large scale data processing
async fn large_scale_data_processing_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Processing large scale data sets...");
    
    // Create massive data sets
    let large_dataset: Vec<Vec<f64>> = (0..1000).map(|i| {
        (0..2000).map(|j| (i as f64 * j as f64).sin()).collect()
    }).collect();
    track_var_safe!(large_dataset);
    
    // Process data in parallel
    let processed_data: Vec<Vec<f64>> = (0..500).into_par_iter().map(|batch| {
        let batch_data: Vec<f64> = (0..5000).map(|i| {
            let x = (batch * 5000 + i) as f64;
            x.sin() * x.cos() + x.sqrt()
        }).collect();
        batch_data
    }).collect();
    track_var_safe!(processed_data);
    
    // Create multiple data managers
    for i in 0..20 {
        let mut data_manager = DataManager::new();
        for user_id in 0..100 {
            let _ = data_manager.create_user(
                user_id, 
                format!("LargeScaleUser_{i}_{user_id}"), 
                format!("user_{i}_{user_id}@largescale.com")
            );
            
            for post_id in 0..10 {
                let _ = data_manager.create_post(
                    user_id,
                    user_id * 1000 + post_id,
                    format!("Large Scale Post {post_id} by User {user_id}"),
                    format!("This is a large scale post content for batch {i} user {user_id} post {post_id}"),
                    vec![format!("tag_{i}"), format!("user_{user_id}"), "largescale".to_string()]
                );
            }
        }
        let data_manager_box = Box::new(data_manager);
        track_var_safe!(data_manager_box);
    }
    
    println!("  Large scale data processing test completed");
    Ok(())
}

// Phase 5: Memory fragmentation test
async fn memory_fragmentation_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Testing memory fragmentation patterns...");
    
    // Create fragmented memory patterns
    let mut fragmented_data = Vec::new();
    
    for cycle in 0..100 {
        // Allocate various sized chunks
        for size in [10, 100, 1000, 10000, 50000] {
            let chunk: Vec<u8> = (0..size).map(|_| rand::random()).collect();
            fragmented_data.push(chunk);
        }
        
        // Periodically clear some data to create fragmentation
        if cycle % 10 == 0 && fragmented_data.len() > 50 {
            fragmented_data.drain(0..25);
        }
    }
    track_var_safe!(fragmented_data);
    
    // Create interleaved data structures
    for i in 0..200 {
        let small_vec: Vec<u32> = (0..10).collect();
        track_var_safe!(small_vec);
        
        let medium_string = "x".repeat(500 + i % 1000);
        track_var_safe!(medium_string);
        
        let large_map: HashMap<String, Vec<f64>> = (0..50).map(|j| {
            let key = format!("frag_{i}_{j}");
            let values: Vec<f64> = (0..100).map(|k| (i * j * k) as f64).collect();
            (key, values)
        }).collect();
        let large_map_box = Box::new(large_map);
        track_var_safe!(large_map_box);
    }
    
    println!("  Memory fragmentation test completed");
    Ok(())
}

// Phase 6: Circular reference stress test
async fn circular_reference_stress_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Creating circular reference patterns...");
    
    #[derive(Debug)]
    #[allow(dead_code)]
    struct CircularNode {
        id: u64,
        data: Vec<String>,
        parent: Option<Weak<CircularNode>>,
        children: Vec<Rc<CircularNode>>,
    }
    
    // Create complex circular reference patterns
    for tree_id in 0..50 {
        let mut nodes = Vec::new();
        
        // Create nodes
        for i in 0..20 {
            let data: Vec<String> = (0..50)
                .map(|j| format!("circular_data_{tree_id}_{i}_{j}"))
                .collect();
            
            let node = Rc::new(CircularNode {
                id: tree_id * 100 + i,
                data,
                parent: None,
                children: Vec::new(),
            });
            nodes.push(node);
        }
        
        // Create circular references (carefully to avoid memory leaks)
        for i in 0..nodes.len() {
            let child_indices: Vec<usize> = (0..3)
                .map(|_| rand::random::<usize>() % nodes.len())
                .filter(|&idx| idx != i)
                .collect();
            
            for &child_idx in &child_indices {
                // This creates a controlled circular reference pattern
                unsafe {
                    let node_ptr = Rc::as_ptr(&nodes[i]) as *mut CircularNode;
                    (*node_ptr).children.push(Rc::clone(&nodes[child_idx]));
                }
            }
        }
        
        track_var_safe!(nodes);
    }
    
    println!("  Circular reference stress test completed");
    Ok(())
}

// Phase 7: Mixed workload simulation
async fn mixed_workload_simulation() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Running mixed workload simulation...");
    
    // Simulate a complex application workload
    let mut handles = Vec::new();
    
    // Database simulation
    let handle1 = tokio::spawn(async {
        for i in 0..100 {
            let mut database_records: BTreeMap<u64, HashMap<String, String>> = BTreeMap::new();
            for record_id in 0..500 {
                let mut record = HashMap::new();
                record.insert("id".to_string(), record_id.to_string());
                record.insert("name".to_string(), format!("Record_{record_id}"));
                record.insert("data".to_string(), "x".repeat(100 + record_id % 200));
                record.insert("timestamp".to_string(), format!("{}", i * 1000 + record_id));
                database_records.insert(record_id as u64, record);
            }
            let database_records_box = Box::new(database_records);
            track_var_safe!(database_records_box);
            sleep(Duration::from_millis(10)).await;
        }
    });
    handles.push(handle1);
    
    // Cache simulation
    let handle2 = tokio::spawn(async {
        let cache = Arc::new(Mutex::new(HashMap::<String, Vec<u8>>::new()));
        for i in 0..200 {
            let key = format!("cache_key_{i}");
            let value: Vec<u8> = (0..2000).map(|_| rand::random()).collect();
            {
                let mut cache_guard = cache.lock().unwrap();
                cache_guard.insert(key, value);
                
                // Simulate cache eviction
                if cache_guard.len() > 100 {
                    let keys_to_remove: Vec<String> = cache_guard.keys().take(20).cloned().collect();
                    for key in keys_to_remove {
                        cache_guard.remove(&key);
                    }
                }
            }
            sleep(Duration::from_millis(5)).await;
        }
        track_var_safe!(cache);
    });
    handles.push(handle2);
    
    // Message queue simulation
    let handle3 = tokio::spawn(async {
        for batch in 0..50 {
            let message_queue: VecDeque<HashMap<String, Vec<String>>> = (0..100).map(|i| {
                let mut message = HashMap::new();
                message.insert("id".to_string(), vec![format!("msg_{batch}_{i}")]);
                message.insert("payload".to_string(), (0..20).map(|j| format!("data_{i}_{j}")).collect());
                message.insert("metadata".to_string(), vec!["urgent".to_string(), "processed".to_string()]);
                message
            }).collect();
            let message_queue_vec = Vec::from(message_queue);
            track_var_safe!(message_queue_vec);
            sleep(Duration::from_millis(20)).await;
        }
    });
    handles.push(handle3);
    
    // Wait for all simulations to complete
    for handle in handles {
        handle.await?;
    }
    
    println!("  Mixed workload simulation completed");
    Ok(())
}

// Phase 8: Memory leak detection test
async fn memory_leak_detection_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Running memory leak detection test...");
    
    // Simulate potential memory leak scenarios (but track them properly)
    for iteration in 0..100 {
        // Scenario 1: Growing collections
        let mut growing_collection = Vec::new();
        for i in 0..1000 {
            let data: Vec<String> = (0..50)
                .map(|j| format!("leak_test_{iteration}_{i}_data_{j}"))
                .collect();
            growing_collection.push(data);
        }
        track_var_safe!(growing_collection);
        
        // Scenario 2: Nested allocations
        let nested_allocations: Vec<Box<Vec<Arc<String>>>> = (0..100).map(|i| {
            let inner: Vec<Arc<String>> = (0..50)
                .map(|j| Arc::new(format!("nested_alloc_{iteration}_{i}_{j}")))
                .collect();
            Box::new(inner)
        }).collect();
        track_var_safe!(nested_allocations);
        
        // Scenario 3: Temporary large allocations
        if iteration % 10 == 0 {
            let temp_large_data: Vec<Vec<u8>> = (0..50).map(|_| {
                (0..10000).map(|_| rand::random()).collect()
            }).collect();
            track_var_safe!(temp_large_data);
        }
        
        if iteration % 20 == 0 {
            println!("    Completed {} leak detection iterations", iteration + 1);
        }
    }
    
    println!("  Memory leak detection test completed");
    Ok(())
}