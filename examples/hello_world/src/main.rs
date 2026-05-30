#![forbid(unsafe_code)]

use mdux::{run_hello_world_demo, HelloWorldDemoConfig};

fn main() {
    let run = run_hello_world_demo(HelloWorldDemoConfig::default())
        .expect("hello world demo should build and run");

    let greeting = run
        .framework
        .ui_runtime()
        .components()
        .first()
        .map(|component| component.label.as_str())
        .unwrap_or("Hello world");

    println!("{greeting} from MduX-rust");
    println!("{}", run.framework.release_summary());
    println!(
        "preview_frame index={} draw_calls={} frame_time_ms={} dynamic_allocations={}",
        run.frame.frame_index,
        run.frame.draw_calls,
        run.frame.frame_time_ms,
        run.frame.dynamic_allocations
    );
    println!("trace_matrix\n{}", run.framework.trace_matrix_export());
    println!("audit_log\n{}", run.framework.audit_export());
}
