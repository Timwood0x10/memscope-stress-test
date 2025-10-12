use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::collections::HashMap;
use memscope_rs::{init, track_var};
use memscope_rs::export::fixed_hybrid_template::{FixedHybridTemplate, RenderMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Deep Inspector Real Data Verification Test");
    println!("Testing different variable types and sizes...\n");
    
    init();

    // Test Case 1:Large memory buffers
    let large_image_buffer = vec![0u8; 1024 * 512]; // 512KB
    track_var!(large_image_buffer);
    
    let video_frame_buffer = vec![0u32; 1920 * 1080]; // ~8MB
    track_var!(video_frame_buffer);
    
    // Test Case 2: Network buffers
    let tcp_recv_buffer = vec![0u8; 8192]; // 8KB
    track_var!(tcp_recv_buffer);
    
    let http_response_cache = vec![0u8; 65536]; // 64KB
    track_var!(http_response_cache);
    
    // Test Case 3: 计算相关数据 (Computation data)
    let matrix_data = vec![0f64; 1000 * 1000]; // 8MB matrix
    track_var!(matrix_data);
    
    let fft_coefficients = vec![0f32; 4096]; // 16KB
    track_var!(fft_coefficients);
    
    // Test Case 4: 多线程场景 (Multi-threading scenarios)
    let shared_counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for thread_id in 0..5 {
        let counter = Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            // local variables in each thread
            match thread_id {
                0 => {
                    let database_cache = vec![0u8; 256 * 1024]; // 256KB
                    track_var!(database_cache);
                    thread::sleep(Duration::from_millis(100));
                },
                1 => {
                    let crypto_key_buffer = vec![0u8; 4096]; // 4KB
                    track_var!(crypto_key_buffer);
                    let signature_data = vec![0u8; 2048]; // 2KB
                    track_var!(signature_data);
                    thread::sleep(Duration::from_millis(150));
                },
                2 => {
                    let audio_sample_buffer = vec![0i16; 44100]; // ~88KB
                    track_var!(audio_sample_buffer);
                    thread::sleep(Duration::from_millis(80));
                },
                3 => {
                    let file_read_buffer = vec![0u8; 128 * 1024]; // 128KB
                    track_var!(file_read_buffer);
                    let compression_workspace = vec![0u8; 64 * 1024]; // 64KB
                    track_var!(compression_workspace);
                    thread::sleep(Duration::from_millis(120));
                },
                4 => {
                    let gpu_texture_data = vec![0u8; 2048 * 2048 * 4]; // 16MB
                    track_var!(gpu_texture_data);
                    thread::sleep(Duration::from_millis(200));
                },
                _ => {}
            }
            
            // update shared counter
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    // Test Case 5: Dynamic allocation/deallocation
    println!("Creating dynamic allocations...");
    for i in 0..3 {
        let dynamic_buffer = vec![0u8; (i + 1) * 32 * 1024]; // 32KB, 64KB, 96KB
        track_var!(dynamic_buffer);
        thread::sleep(Duration::from_millis(50));
        // Buffer will be dropped here
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Test Case 6: Special data types
    let string_collection: Vec<String> = (0..1000)
        .map(|i| format!("data_entry_{}", i))
        .collect();
    track_var!(string_collection);
    
    let nested_structure = vec![vec![0i32; 100]; 50]; 
    track_var!(nested_structure);
    
    println!("\n📊 Generating comprehensive Deep Inspector report...");
    
    // Deep Inspector
    generate_deep_inspector_verification_report()?;
    
    println!("\n🎯 Verification checklist:");
    println!("   □ Variable names show real identifiers (not 'Vec<u8> allocated')");
    println!("   □ Memory sizes match actual allocations");  
    println!("   □ Timeline shows track_var! registration events");
    println!("   □ No JavaScript syntax errors in browser console");
    println!("   □ Deep Inspector pages navigate properly");
    println!("   □ Memory attribution percentages are calculated (not hardcoded 15%)");
    
    Ok(())
}

fn generate_deep_inspector_verification_report() -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::HashMap;
    
    println!("\n📊 Generating Deep Inspector verification report...");
    
    
    let real_variables = memscope_rs::variable_registry::VariableRegistry::get_all_variables();
    println!("🔢 Variables tracked: {}", real_variables.len());
    
    let total_memory: u64 = real_variables.values().map(|v| v.memory_usage).sum();
    println!("💾 Total memory tracked: {:.2} MB", total_memory as f64 / (1024.0 * 1024.0));
    
    let mut lockfree_analysis = memscope_rs::lockfree::analysis::LockfreeAnalysis::new();
    lockfree_analysis.summary.peak_memory_usage = total_memory as usize;
    lockfree_analysis.summary.total_allocations = real_variables.len() as u64;
    
    let variable_details: HashMap<String, memscope_rs::export::fixed_hybrid_template::VariableDetail> = 
        real_variables.into_iter().map(|(addr, var_info)| {
            (
                format!("{}_{:x}", var_info.var_name, addr),
                memscope_rs::export::fixed_hybrid_template::VariableDetail {
                    name: var_info.var_name.clone(),
                    type_info: var_info.type_name.clone(),
                    thread_id: var_info.thread_id,
                    task_id: Some(var_info.thread_id * 100 + (addr % 100)),
                    allocation_count: 1,
                    memory_usage: var_info.memory_usage,
                    lifecycle_stage: memscope_rs::export::fixed_hybrid_template::LifecycleStage::Active,
                },
            )
        }).collect();
    
    let hybrid_data = memscope_rs::export::fixed_hybrid_template::HybridAnalysisData {
        variable_registry: variable_details.clone(),
        lockfree_analysis: Some(lockfree_analysis),
        thread_task_mapping: HashMap::new(),
        visualization_config: Default::default(),
        performance_metrics: memscope_rs::export::fixed_hybrid_template::PerformanceTimeSeries {
            cpu_usage: Vec::new(),
            memory_usage: Vec::new(),
            io_operations: Vec::new(),
            network_bytes: Vec::new(),
            timestamps: Vec::new(),
            thread_cpu_breakdown: HashMap::new(),
            thread_memory_breakdown: HashMap::new(),
        },
    };
    
    
    let template = FixedHybridTemplate::new(5, 25)
        .with_render_mode(RenderMode::Comprehensive)
        .with_variable_details(true)
        .with_enhanced_insights(true);
    
    let html_content = template.generate_hybrid_dashboard(&hybrid_data)?;
    std::fs::write("deep_inspector_real_data_verification.html", html_content)?;
    
    println!("✅ Deep Inspector verification report generated!");
    println!("📁 File: deep_inspector_real_data_verification.html");
    
    // 验证报告内容
    verify_report_content(&variable_details)?;
    
    Ok(())
}

fn verify_report_content(variable_details: &HashMap<String, memscope_rs::export::fixed_hybrid_template::VariableDetail>) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    
    let html_content = fs::read_to_string("deep_inspector_real_data_verification.html")?;
    
    // check real allocations
    let real_variable_names = [
        "large_image_buffer",
        "video_frame_buffer", 
        "tcp_recv_buffer",
        "http_response_cache",
        "matrix_data",
        "fft_coefficients",
        "database_cache",
        "crypto_key_buffer",
        "audio_sample_buffer",
        "gpu_texture_data"
    ];
    
    let mut real_data_count = 0;
    for var_name in &real_variable_names {
        if html_content.contains(var_name) {
            real_data_count += 1;
            println!("✅ real var name: {}", var_name);
        }
    }

    println!("📊 real variable data statistics:");
    println!("   • real variable names: {}/{}", real_data_count, real_variable_names.len());
    println!("   • total tracked variables: {}", variable_details.len());

    if real_data_count == real_variable_names.len() {
        println!("✅ All real variable names are correctly displayed");
    } else {
        println!("❌ Some variable names are missing or incorrect");
    }
    if html_content.contains("track_var!(") {
        println!("✅ track_var! call is correctly displayed");
    }
    
    if html_content.contains("Variable Tracking Timeline") {
        println!("✅ Interface title has been updated to the real description");
    }
    
    Ok(())
}