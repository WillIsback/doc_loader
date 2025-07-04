use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Select, Input, Confirm};
use crate::{PdfProcessor, ProcessingOptions, OutputFormat};
use crate::splitter::SplitOptions;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use tokio::fs;

/// Doc Loader CLI - A comprehensive toolkit for PDF document processing
#[derive(Parser)]
#[command(name = "doc_loader")]
#[command(about = "Extract, process, and structure content from PDF documents")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    
    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
    
    /// Output directory for processed files
    #[arg(short, long, default_value = ".")]
    pub output_dir: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Extract text from PDF files
    Extract {
        /// Input PDF file(s)
        #[arg(value_name = "FILES")]
        files: Vec<String>,
        
        /// Output format (json, json-pretty, yaml)
        #[arg(short, long, default_value = "json-pretty")]
        format: String,
        
        /// Maximum chunk size in characters
        #[arg(long, default_value = "1000")]
        max_chunk_size: usize,
        
        /// Chunk overlap in characters
        #[arg(long, default_value = "100")]
        chunk_overlap: usize,
        
        /// Continue processing on errors
        #[arg(long)]
        continue_on_error: bool,
    },
    
    /// Interactive menu for processing options
    Interactive,
    
    /// Process files in batch
    Batch {
        /// Directory containing PDF files
        directory: String,
        
        /// File pattern to match (default: *.pdf)
        #[arg(short, long, default_value = "*.pdf")]
        pattern: String,
        
        /// Maximum chunk size
        #[arg(long, default_value = "1000")]
        max_chunk_size: usize,
        
        /// Enable recursive directory search
        #[arg(short, long)]
        recursive: bool,
    },
    
    /// Show information about a PDF file
    Info {
        /// PDF file to analyze
        file: String,
    },
}

impl Cli {
    /// Execute the CLI command
    pub async fn execute(self) -> crate::Result<()> {
        // Initialize logging
        if self.verbose {
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
        } else {
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
        }

        // Ensure output directory exists
        fs::create_dir_all(&self.output_dir).await?;

        match &self.command {
            Some(Commands::Extract { files, format, max_chunk_size, chunk_overlap, continue_on_error }) => {
                self.handle_extract(files.clone(), format, *max_chunk_size, *chunk_overlap, *continue_on_error).await
            }
            Some(Commands::Interactive) => {
                self.handle_interactive().await
            }
            Some(Commands::Batch { directory, pattern, max_chunk_size, recursive }) => {
                self.handle_batch(directory.clone(), pattern, *max_chunk_size, *recursive).await
            }
            Some(Commands::Info { file }) => {
                self.handle_info(file).await
            }
            None => {
                // No command provided, show interactive menu
                self.handle_interactive().await
            }
        }
    }

    async fn handle_extract(
        &self,
        files: Vec<String>,
        format: &str,
        max_chunk_size: usize,
        chunk_overlap: usize,
        continue_on_error: bool,
    ) -> crate::Result<()> {
        let processor = PdfProcessor::new();
        let output_format = self.parse_output_format(&format)?;
        
        let options = ProcessingOptions {
            split_options: SplitOptions {
                max_chunk_size,
                chunk_overlap,
                ..Default::default()
            },
            continue_on_error,
            output_format: output_format.clone(),
            include_metadata: true,
        };

        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );

        for file in &files {
            pb.set_message(format!("Processing {}", file));
            
            match processor.process_file(file, &options).await {
                Ok(output) => {
                    let output_path = self.generate_output_path(file, &format);
                    self.save_output(&output, &output_path, &output_format).await?;
                    
                    let summary = output.get_summary();
                    println!("✓ {}", summary);
                }
                Err(e) => {
                    eprintln!("✗ Failed to process {}: {}", file, e);
                    if !continue_on_error {
                        return Err(e);
                    }
                }
            }
            
            pb.inc(1);
        }

        pb.finish_with_message("Processing complete");
        Ok(())
    }

    async fn handle_interactive(&self) -> crate::Result<()> {
        println!("🚀 Welcome to Doc Loader Interactive Mode!");
        
        loop {
            let options = &[
                "Extract text from PDF",
                "Batch process directory",
                "Show PDF information",
                "Exit",
            ];

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("What would you like to do?")
                .default(0)
                .items(options)
                .interact()
                .unwrap();

            match selection {
                0 => self.interactive_extract().await?,
                1 => self.interactive_batch().await?,
                2 => self.interactive_info().await?,
                3 => {
                    println!("👋 Goodbye!");
                    break;
                }
                _ => unreachable!(),
            }

            if !Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to perform another operation?")
                .default(true)
                .interact()
                .unwrap()
            {
                break;
            }
        }

        Ok(())
    }

    async fn interactive_extract(&self) -> crate::Result<()> {
        let file: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter PDF file path")
            .interact_text()
            .unwrap();

        let max_chunk_size: usize = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Maximum chunk size (characters)")
            .default(1000)
            .interact_text()
            .unwrap();

        let chunk_overlap: usize = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Chunk overlap (characters)")
            .default(100)
            .interact_text()
            .unwrap();

        let format_options = &["json-pretty", "json", "yaml"];
        let format_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Output format")
            .default(0)
            .items(format_options)
            .interact()
            .unwrap();

        let format = format_options[format_selection].to_string();

        self.handle_extract(
            vec![file],
            &format,
            max_chunk_size,
            chunk_overlap,
            false,
        ).await
    }

    async fn interactive_batch(&self) -> crate::Result<()> {
        let directory: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter directory path")
            .interact_text()
            .unwrap();

        let recursive = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Search subdirectories recursively?")
            .default(false)
            .interact()
            .unwrap();

        let max_chunk_size: usize = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Maximum chunk size (characters)")
            .default(1000)
            .interact_text()
            .unwrap();

        self.handle_batch(directory, "*.pdf", max_chunk_size, recursive).await
    }

    async fn interactive_info(&self) -> crate::Result<()> {
        let file: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter PDF file path")
            .interact_text()
            .unwrap();

        self.handle_info(&file).await
    }

    async fn handle_batch(
        &self,
        directory: String,
        pattern: &str,
        max_chunk_size: usize,
        recursive: bool,
    ) -> crate::Result<()> {
        let files = self.find_pdf_files(&directory, &pattern, recursive).await?;
        
        if files.is_empty() {
            println!("No PDF files found in {}", directory);
            return Ok(());
        }

        println!("Found {} PDF files", files.len());
        
        self.handle_extract(
            files,
            "json-pretty",
            max_chunk_size,
            100, // default overlap
            true, // continue on error for batch
        ).await
    }

    async fn handle_info(&self, file: &str) -> crate::Result<()> {
        let processor = PdfProcessor::new();
        let options = ProcessingOptions::default();
        
        println!("📊 Analyzing PDF file: {}", file);
        
        let output = processor.process_file(&file, &options).await?;
        let summary = output.get_summary();
        
        println!("\n{}", summary);
        
        // Show metadata details
        println!("\n📝 Document Metadata:");
        if let Some(title) = &output.metadata.title {
            println!("  Title: {}", title);
        }
        if let Some(author) = &output.metadata.author {
            println!("  Author: {}", author);
        }
        if let Some(subject) = &output.metadata.subject {
            println!("  Subject: {}", subject);
        }
        if let Some(version) = &output.metadata.pdf_version {
            println!("  PDF Version: {}", version);
        }
        
        // Show chunk information
        println!("\n📄 Content Analysis:");
        println!("  Largest chunk: {} characters", 
            output.chunks.iter().map(|c| c.metadata.char_count).max().unwrap_or(0));
        println!("  Smallest chunk: {} characters", 
            output.chunks.iter().map(|c| c.metadata.char_count).min().unwrap_or(0));
        
        Ok(())
    }

    async fn find_pdf_files(&self, directory: &str, _pattern: &str, recursive: bool) -> crate::Result<Vec<String>> {
        let mut files = Vec::new();
        
        if recursive {
            self.find_pdf_files_recursive(directory, &mut files).await?;
        } else {
            let mut entries = fs::read_dir(directory).await?;
            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("pdf") {
                    files.push(path.to_string_lossy().to_string());
                }
            }
        }
        
        Ok(files)
    }

    fn find_pdf_files_recursive<'a>(&'a self, directory: &'a str, files: &'a mut Vec<String>) -> std::pin::Pin<Box<dyn std::future::Future<Output = crate::Result<()>> + 'a>> {
        Box::pin(async move {
            let mut entries = fs::read_dir(directory).await?;
            
            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                
                if path.is_dir() {
                    if let Some(dir_str) = path.to_str() {
                        self.find_pdf_files_recursive(dir_str, files).await?;
                    }
                } else if path.extension().and_then(|s| s.to_str()) == Some("pdf") {
                    files.push(path.to_string_lossy().to_string());
                }
            }
            
            Ok(())
        })
    }

    fn parse_output_format(&self, format: &str) -> crate::Result<OutputFormat> {
        match format.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "json-pretty" => Ok(OutputFormat::JsonPretty),
            "yaml" => Ok(OutputFormat::Yaml),
            "csv" => Ok(OutputFormat::Csv),
            _ => Err(crate::DocLoaderError::InvalidFormat(format.to_string())),
        }
    }

    fn generate_output_path(&self, input_file: &str, format: &str) -> String {
        let path = Path::new(input_file);
        let filename = path.file_stem().unwrap().to_str().unwrap();
        let extension = match format {
            "yaml" => "yaml",
            "csv" => "csv",
            _ => "json",
        };
        
        Path::new(&self.output_dir)
            .join(format!("{}_processed.{}", filename, extension))
            .to_string_lossy()
            .to_string()
    }

    async fn save_output(
        &self,
        output: &crate::JsonOutput,
        path: &str,
        format: &OutputFormat,
    ) -> crate::Result<()> {
        let content = match format {
            OutputFormat::Json => output.to_json(false)?,
            OutputFormat::JsonPretty => output.to_json(true)?,
            OutputFormat::Yaml => {
                return Err(crate::DocLoaderError::Processing(
                    "YAML output not yet implemented".to_string()
                ));
            }
            OutputFormat::Csv => {
                return Err(crate::DocLoaderError::Processing(
                    "CSV output not yet implemented".to_string()
                ));
            }
        };

        fs::write(path, content).await?;
        Ok(())
    }
}
