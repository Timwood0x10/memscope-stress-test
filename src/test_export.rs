use memscope_rs::lockfree::{
    export_comprehensive_analysis, finalize_thread_tracker, init_thread_tracker,
    track_allocation_lockfree, IntegratedProfilingSession,
};

use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = "./test_output";
    std::fs::create_dir_all(output_dir)?;

    // Initialize session
    let mut session = IntegratedProfilingSession::new(Path::new(output_dir))?;
    session.start_profiling()?;

    // Initialize tracker
    init_thread_tracker(Path::new(output_dir), None)?;

    // Track a simple allocation
    let data = vec![1u8; 100];
    let ptr = data.as_ptr() as usize;
    track_allocation_lockfree(ptr, 100, &[0x1000])?;

    // Finalize
    finalize_thread_tracker()?;
    let analysis = session.stop_profiling_and_analyze()?;

    // Try to export
    export_comprehensive_analysis(&analysis, Path::new(output_dir), "test")?;

    println!("Export successful!");
    Ok(())
}
