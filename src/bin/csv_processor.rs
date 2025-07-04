use clap::{Arg, Command};
use doc_loader::{processors::csv::CsvProcessor, ProcessingParams, DocumentProcessor};
use std::path::Path;
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let matches = Command::new("CSV Processor")
        .version("1.0.0")
        .about("Extract and process content from CSV files into universal JSON format")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Input CSV file path")
                .required(true)
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output JSON file path (optional, defaults to stdout)")
        )
        .arg(
            Arg::new("chunk-size")
                .long("chunk-size")
                .value_name("SIZE")
                .help("Maximum chunk size in characters")
                .default_value("1000")
        )
        .arg(
            Arg::new("chunk-overlap")
                .long("chunk-overlap")
                .value_name("SIZE")
                .help("Overlap between chunks in characters")
                .default_value("100")
        )
        .arg(
            Arg::new("no-cleaning")
                .long("no-cleaning")
                .help("Disable text cleaning")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("detect-language")
                .long("detect-language")
                .help("Enable language detection")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("pretty")
                .long("pretty")
                .help("Pretty print JSON output")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    // Parse arguments
    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output");
    let chunk_size: usize = matches.get_one::<String>("chunk-size").unwrap().parse()?;
    let chunk_overlap: usize = matches.get_one::<String>("chunk-overlap").unwrap().parse()?;
    let text_cleaning = !matches.get_flag("no-cleaning");
    let language_detection = matches.get_flag("detect-language");
    let pretty_print = matches.get_flag("pretty");

    // Validate input file
    let input_path = Path::new(input_file);
    if !input_path.exists() {
        eprintln!("Error: Input file '{}' not found", input_file);
        std::process::exit(1);
    }

    if !input_path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase() == "csv")
        .unwrap_or(false) 
    {
        eprintln!("Error: Input file must have .csv extension");
        std::process::exit(1);
    }

    // Create processing parameters
    let params = ProcessingParams {
        max_chunk_size: chunk_size,
        chunk_overlap,
        text_cleaning,
        language_detection,
        format_specific: serde_json::Value::Null,
    };

    // Process the CSV file
    println!("Processing CSV file: {}", input_file);
    let processor = CsvProcessor::new();
    
    let result = match processor.process_file(input_path, &params) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Error processing CSV file: {}", e);
            std::process::exit(1);
        }
    };

    // Serialize output
    let json_output = if pretty_print {
        serde_json::to_string_pretty(&result)?
    } else {
        serde_json::to_string(&result)?
    };

    // Write output
    match output_file {
        Some(output_path) => {
            std::fs::write(output_path, json_output)?;
            println!("Results written to: {}", output_path);
        }
        None => {
            println!("{}", json_output);
        }
    }

    // Print summary
    eprintln!("✅ Processing completed successfully!");
    eprintln!("   📄 Document: {}", result.document_metadata.filename);
    eprintln!("   🧩 Chunks extracted: {}", result.processing_info.total_chunks);
    eprintln!("   📊 Total content size: {} characters", result.processing_info.total_content_size);
    eprintln!("   ⏱️  Processing time: {}ms", result.processing_info.processing_time_ms);

    // Display CSV-specific metadata
    if let Some(csv_meta) = result.document_metadata.format_metadata["csv_metadata"].as_object() {
        if let Some(total_rows) = csv_meta["total_rows"].as_u64() {
            eprintln!("   📏 Total rows: {}", total_rows);
        }
        if let Some(total_columns) = csv_meta["total_columns"].as_u64() {
            eprintln!("   📐 Total columns: {}", total_columns);
        }
        if let Some(completeness) = csv_meta["data_completeness"].as_f64() {
            eprintln!("   ✅ Data completeness: {:.1}%", completeness * 100.0);
        }
        
        // Display headers
        if let Some(headers) = csv_meta["headers"].as_array() {
            eprintln!("   🏷️  Column headers: {}", 
                headers.iter()
                    .filter_map(|h| h.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }

    if let Some(language) = result.document_metadata.format_metadata["detected_language"].as_str() {
        eprintln!("   🌐 Detected language: {}", language);
    }

    Ok(())
}
