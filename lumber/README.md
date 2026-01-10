# Lumber - Advanced Log File Parser

A robust, non-trivial log file parser built in Rust that handles Docker container logs, application logs, and poorly formatted logs with intelligent parsing strategies.

## Project Overview

Lumber is designed to parse, analyze, and transform log files from various sources into structured, queryable data. It handles the chaos of real-world logging with grace and performance.

### Supported Log Sources
- **Docker Container Logs**: JSON-formatted logs with container metadata
- **Application Logs**: Structured logs (syslog, log4j, custom formats)
- **Poorly Formatted Logs**: Unstructured or inconsistent log entries

### Key Features
- Automatic format detection
- Multi-format output (JSON, CSV, pretty-print)
- Time-range filtering
- Log level filtering
- Keyword/regex searching
- Performance optimized for large files (streaming, zero-copy where possible)
- Error resilience and recovery

---

## Project Structure

```
lumber/
├── Cargo.toml                    # Project manifest and dependencies
├── Cargo.lock                    # Dependency lock file
├── README.md                     # This file
├── LICENSE-MIT                   # MIT license
├── LICENSE-APACHE                # Apache 2.0 license
│
├── src/
│   ├── main.rs                   # Entry point, CLI setup, orchestration
│   │
│   ├── cli/
│   │   ├── mod.rs                # CLI module exports
│   │   ├── args.rs               # Command-line argument definitions (clap)
│   │   └── config.rs             # Configuration file parser (YAML/TOML)
│   │
│   ├── core/
│   │   ├── mod.rs                # Core module exports
│   │   ├── log_entry.rs          # LogEntry struct and core data types
│   │   ├── log_level.rs          # LogLevel enum and normalization
│   │   └── error.rs              # Custom error types and Result aliases
│   │
│   ├── io/
│   │   ├── mod.rs                # I/O module exports
│   │   ├── reader.rs             # LogReader trait and implementations
│   │   ├── buffered_reader.rs    # Buffered file reader
│   │   ├── mmap_reader.rs        # Memory-mapped file reader
│   │   ├── compressed_reader.rs  # Gzip/bzip2 reader
│   │   └── stdin_reader.rs       # Standard input reader
│   │
│   ├── parsers/
│   │   ├── mod.rs                # Parser module exports and registry
│   │   ├── traits.rs             # Parser trait definitions
│   │   ├── detector.rs           # Format detection logic
│   │   ├── docker_json.rs        # Docker JSON log parser
│   │   ├── syslog.rs             # Syslog format parser
│   │   ├── json_lines.rs         # JSON Lines parser
│   │   ├── common_log.rs         # Apache/nginx common log format
│   │   ├── log4j.rs              # Log4j pattern parser
│   │   ├── structured.rs         # Generic structured log parser
│   │   └── unstructured.rs       # Fallback parser for poorly formatted logs
│   │
│   ├── extractors/
│   │   ├── mod.rs                # Extractor module exports
│   │   ├── timestamp.rs          # Multi-format timestamp extraction
│   │   ├── level.rs              # Log level detection and normalization
│   │   ├── metadata.rs           # Key-value pair extraction
│   │   └── patterns.rs           # Common regex patterns (lazy_static)
│   │
│   ├── filters/
│   │   ├── mod.rs                # Filter module exports
│   │   ├── traits.rs             # Filter trait definition
│   │   ├── chain.rs              # Filter chain orchestration
│   │   ├── time_range.rs         # Time-based filtering
│   │   ├── level.rs              # Log level filtering
│   │   ├── keyword.rs            # Keyword/regex search
│   │   ├── metadata.rs           # Metadata field filtering
│   │   └── composite.rs          # Composite filters (AND, OR, NOT)
│   │
│   ├── analyzers/
│   │   ├── mod.rs                # Analyzer module exports
│   │   ├── statistics.rs         # Statistics collection
│   │   ├── aggregator.rs         # Log aggregation and grouping
│   │   ├── correlator.rs         # Cross-source log correlation
│   │   └── patterns.rs           # Pattern detection (error clustering)
│   │
│   ├── output/
│   │   ├── mod.rs                # Output module exports
│   │   ├── traits.rs             # Writer trait definition
│   │   ├── json.rs               # JSON output writer
│   │   ├── json_lines.rs         # JSON Lines output
│   │   ├── csv.rs                # CSV output writer
│   │   ├── pretty.rs             # Pretty-print/human-readable output
│   │   ├── table.rs              # Table format output
│   │   └── template.rs           # Custom template engine
│   │
│   └── utils/
│       ├── mod.rs                # Utility module exports
│       ├── performance.rs        # Performance optimization helpers
│       ├── progress.rs           # Progress bar integration
│       └── compression.rs        # Compression detection and handling
│
├── tests/
│   ├── integration_tests.rs      # Integration tests
│   ├── parser_tests.rs           # Parser-specific tests
│   ├── filter_tests.rs           # Filter logic tests
│   └── fixtures/                 # Test data fixtures
│       ├── docker_logs.json      # Sample Docker logs
│       ├── syslog_samples.log    # Sample syslog entries
│       ├── mixed_format.log      # Mixed format logs
│       └── malformed.log         # Poorly formatted logs
│
├── benches/
│   ├── parsing_benchmark.rs      # Parser performance benchmarks
│   ├── filtering_benchmark.rs    # Filter performance benchmarks
│   └── io_benchmark.rs           # I/O performance benchmarks
│
├── examples/
│   ├── basic_usage.rs            # Simple usage example
│   ├── custom_parser.rs          # Creating custom parser
│   ├── advanced_filtering.rs    # Complex filter chains
│   └── output_formats.rs         # Using different output formats
│
└── docs/
    ├── architecture.md           # Architecture documentation
    ├── parser_guide.md           # Parser development guide
    ├── performance.md            # Performance tuning guide
    └── api/                      # Generated API docs (rustdoc)
```

---

## Product Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                          CLI Entry Point                            │
│                         (main.rs, cli/)                             │
│  • Parse command-line arguments                                     │
│  • Load configuration file                                          │
│  • Set up logging/error handling                                    │
└────────────────────────┬────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      Pipeline Orchestrator                          │
│                         (main.rs)                                   │
│  • Coordinate I/O → Parse → Filter → Analyze → Output              │
│  • Manage error recovery                                            │
│  • Track statistics                                                 │
└────────────┬────────────────────────┬────────────────┬──────────────┘
             │                        │                │
             ▼                        ▼                ▼
┌────────────────────┐   ┌────────────────────┐   ┌────────────────┐
│   Input Layer      │   │  Processing Layer  │   │  Output Layer  │
│     (io/)          │   │                    │   │   (output/)    │
└────────────────────┘   └────────────────────┘   └────────────────┘
```

### Detailed Component Architecture

```
┌───────────────────────────────────────────────────────────────────────────┐
│                              INPUT LAYER                                  │
├───────────────────────────────────────────────────────────────────────────┤
│                                                                           │
│   ┌──────────────────┐      ┌─────────────────┐    ┌─────────────────┐  │
│   │  File Reader     │      │ Compressed      │    │  Stdin Reader   │  │
│   │                  │      │ Reader          │    │                 │  │
│   │  • BufReader     │      │ • Gzip          │    │  • Pipe support │  │
│   │  • Memory-mapped │      │ • Bzip2         │    │  • Stream mode  │  │
│   │  • Read-ahead    │      │ • Auto-detect   │    │                 │  │
│   └──────────────────┘      └─────────────────┘    └─────────────────┘  │
│                                                                           │
│   All implement: Iterator<Item = String> (line-by-line)                  │
└─────────────────────────────┬─────────────────────────────────────────────┘
                              │
                              ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                           PARSING LAYER                                   │
├───────────────────────────────────────────────────────────────────────────┤
│                                                                           │
│   ┌─────────────────────────────────────────────────────────────────┐    │
│   │              Format Detector (detector.rs)                      │    │
│   │  • Sample first N lines                                         │    │
│   │  • Try each parser, score confidence                            │    │
│   │  • Select best-match format                                     │    │
│   └────────────────────────┬────────────────────────────────────────┘    │
│                            │                                             │
│                            ▼                                             │
│   ┌───────────────────────────────────────────────────────────────┐     │
│   │                  Parser Registry                               │     │
│   │  Routes to appropriate parser based on format                  │     │
│   └───┬───────────┬───────────┬──────────┬──────────┬─────────────┘     │
│       │           │           │          │          │                    │
│   ┌───▼───┐   ┌───▼───┐  ┌────▼────┐ ┌──▼──┐  ┌────▼────────┐          │
│   │Docker │   │Syslog │  │  JSON   │ │Log4j│  │Unstructured │          │
│   │ JSON  │   │       │  │  Lines  │ │     │  │  (fallback) │          │
│   └───────┘   └───────┘  └─────────┘ └─────┘  └─────────────┘          │
│                                                                           │
│   All implement: Parser trait                                            │
│   fn parse(line: &str) -> Result<LogEntry>                               │
│   fn matches(line: &str) -> bool                                         │
└─────────────────────────────┬─────────────────────────────────────────────┘
                              │
                              ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                      NORMALIZATION LAYER                                  │
├───────────────────────────────────────────────────────────────────────────┤
│                                                                           │
│   ┌─────────────────┐   ┌─────────────────┐   ┌──────────────────┐      │
│   │   Timestamp     │   │  Level          │   │   Metadata       │      │
│   │   Extractor     │   │  Normalizer     │   │   Extractor      │      │
│   │                 │   │                 │   │                  │      │
│   │ • Multi-format  │   │ • ERROR → Error │   │ • Key-value pairs│      │
│   │ • Timezone      │   │ • WARN → Warn   │   │ • Container ID   │      │
│   │ • Fallback      │   │ • INFO → Info   │   │ • Custom fields  │      │
│   └─────────────────┘   └─────────────────┘   └──────────────────┘      │
│                                                                           │
│   Produces: Normalized LogEntry with consistent fields                   │
└─────────────────────────────┬─────────────────────────────────────────────┘
                              │
                              ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                         FILTERING LAYER                                   │
├───────────────────────────────────────────────────────────────────────────┤
│                                                                           │
│   ┌──────────────────────────────────────────────────────────────┐       │
│   │               Filter Chain (filters/chain.rs)                │       │
│   │  • Applies filters in sequence                               │       │
│   │  • Short-circuit evaluation (AND logic)                      │       │
│   │  • Optimizes filter order by selectivity                     │       │
│   └──────┬───────────┬──────────┬──────────┬───────────┬─────────┘       │
│          │           │          │          │           │                 │
│   ┌──────▼──┐  ┌─────▼────┐ ┌──▼─────┐ ┌──▼──────┐ ┌──▼────────┐       │
│   │  Time   │  │  Level   │ │Keyword │ │Metadata │ │ Composite │       │
│   │  Range  │  │  Filter  │ │ Filter │ │ Filter  │ │ (AND/OR)  │       │
│   └─────────┘  └──────────┘ └────────┘ └─────────┘ └───────────┘       │
│                                                                           │
│   All implement: Filter trait                                            │
│   fn matches(&self, entry: &LogEntry) -> bool                            │
└─────────────────────────────┬─────────────────────────────────────────────┘
                              │
                              ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                         ANALYSIS LAYER                                    │
├───────────────────────────────────────────────────────────────────────────┤
│                                                                           │
│   ┌────────────────┐   ┌────────────────┐   ┌──────────────────┐        │
│   │  Statistics    │   │  Aggregator    │   │  Correlator      │        │
│   │  Collector     │   │                │   │                  │        │
│   │                │   │ • Group by     │   │ • Time-window    │        │
│   │ • Count by     │   │   field        │   │   correlation    │        │
│   │   level        │   │ • Time-series  │   │ • Multi-source   │        │
│   │ • Time range   │   │ • Top N        │   │   matching       │        │
│   │ • Error        │   │                │   │ • Trace ID       │        │
│   │   patterns     │   │                │   │   tracking       │        │
│   └────────────────┘   └────────────────┘   └──────────────────┘        │
│                                                                           │
│   Updates incrementally as logs are processed                            │
└─────────────────────────────┬─────────────────────────────────────────────┘
                              │
                              ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                          OUTPUT LAYER                                     │
├───────────────────────────────────────────────────────────────────────────┤
│                                                                           │
│   ┌──────────────────────────────────────────────────────────────┐       │
│   │            Output Format Router                              │       │
│   │  Selects writer based on --output flag                       │       │
│   └───┬───────────┬─────────┬──────────┬──────────┬──────────────┘       │
│       │           │         │          │          │                      │
│   ┌───▼──┐   ┌────▼───┐ ┌──▼─────┐ ┌──▼──┐  ┌────▼─────┐               │
│   │ JSON │   │  CSV   │ │ Pretty │ │Table│  │ Template │               │
│   │      │   │        │ │ Print  │ │     │  │  Custom  │               │
│   │• Full│   │• Flat  │ │• Color │ │• CLI│  │ • User   │               │
│   │  data│   │  table │ │• Human │ │  UI │  │   format │               │
│   └──────┘   └────────┘ └────────┘ └─────┘  └──────────┘               │
│                                                                           │
│   All implement: Writer trait                                            │
│   fn write_entry(&mut self, entry: &LogEntry)                            │
│   fn write_header(&mut self)                                             │
│   fn write_footer(&mut self)                                             │
└───────────────────────────────────────────────────────────────────────────┘
```

### Data Flow Diagram

```
┌──────────────┐
│  Log File(s) │
└──────┬───────┘
       │
       ▼
┌──────────────────────────┐
│  Read (buffered/mmap)    │  ───────┐
└──────┬───────────────────┘         │
       │                             │
       ▼                             │  Streaming
┌──────────────────────────┐         │  (line-by-line)
│  Format Detection        │         │
│  (sample first N lines)  │         │
└──────┬───────────────────┘         │
       │                             │
       ▼                             │
┌──────────────────────────┐         │
│  Parse (format-specific) │  ◄──────┘
│  → Raw LogEntry          │
└──────┬───────────────────┘
       │
       ▼
┌──────────────────────────┐
│  Normalize               │
│  • Extract timestamp     │
│  • Normalize level       │
│  • Extract metadata      │
└──────┬───────────────────┘
       │
       ▼
┌──────────────────────────┐
│  Filter Chain            │
│  (time, level, keyword)  │
└──────┬───────────────────┘
       │
       ├─── Matches? No ────► Drop entry
       │
       ▼ Yes
┌──────────────────────────┐
│  Analyze (optional)      │
│  • Update statistics     │
│  • Aggregate data        │
└──────┬───────────────────┘
       │
       ▼
┌──────────────────────────┐
│  Format & Write          │
│  (JSON/CSV/Pretty/etc)   │
└──────┬───────────────────┘
       │
       ▼
┌──────────────────────────┐
│  Output (file/stdout)    │
└──────────────────────────┘
```

### Core Data Structure

```rust
// Central data structure that flows through the pipeline
struct LogEntry {
    // Original raw line
    raw_line: String,
    line_number: u64,

    // Normalized fields
    timestamp: Option<DateTime<Utc>>,
    level: Option<LogLevel>,
    message: String,

    // Additional extracted data
    metadata: HashMap<String, String>,

    // Parsing metadata
    format: LogFormat,
    parse_errors: Vec<String>,
}

// Flows through:
// Reader → Parser → Normalizer → Filter → Analyzer → Writer
```

### Threading & Concurrency Model

```
Single File (< 100MB):
┌────────────────────────────────────────────┐
│  Single-threaded sequential processing     │
│  Main thread: Read → Parse → Filter → Write│
└────────────────────────────────────────────┘

Single Large File (> 100MB):
┌────────────────────────────────────────────┐
│  Main thread: Read lines                   │
│       │                                     │
│       ▼                                     │
│  Rayon: Parallel parse + filter            │
│       │                                     │
│       ▼                                     │
│  Main thread: Aggregate + write (ordered)  │
└────────────────────────────────────────────┘

Multiple Files:
┌────────────────────────────────────────────┐
│  Rayon: Process files in parallel          │
│       │                                     │
│  ┌────┼────┬────────┬────────┐            │
│  │    │    │        │        │            │
│  ▼    ▼    ▼        ▼        ▼            │
│ File1 File2 File3  File4  File5           │
│  │    │    │        │        │            │
│  └────┴────┴────────┴────────┘            │
│       │                                     │
│       ▼                                     │
│  Main thread: Merge + correlate + write    │
└────────────────────────────────────────────┘
```

### Error Handling Flow

```
┌─────────────────┐
│  Parse Error    │
└────────┬────────┘
         │
         ▼
┌─────────────────────────────┐
│  Error Recovery Handler     │
│  • Log to stderr            │
│  • Increment error count    │
│  • Create partial LogEntry? │
└────────┬────────────────────┘
         │
         ├─── Count < threshold ───► Continue processing
         │
         └─── Count ≥ threshold ───► Abort with error
```

### Performance Optimization Strategy

```
┌──────────────────────────────────────────────┐
│  File Size Assessment                        │
└─────────┬────────────────────────────────────┘
          │
    ┌─────┴──────┐
    │            │
< 100MB      > 100MB
    │            │
    ▼            ▼
┌──────┐    ┌─────────┐
│BufRead│    │  mmap   │
└──────┘    └─────────┘
    │            │
    ▼            ▼
Sequential   Parallel
Processing   Processing
    │            │
    └─────┬──────┘
          │
          ▼
┌──────────────────┐
│  Optimization    │
│  • Lazy parsing  │
│  • Zero-copy     │
│  • Regex caching │
│  • Filter order  │
└──────────────────┘
```

### Extension Points

The architecture supports extensibility through:

1. **Custom Parsers**: Implement `Parser` trait
2. **Custom Filters**: Implement `Filter` trait
3. **Custom Output Writers**: Implement `Writer` trait
4. **Plugin System** (future): Dynamic loading of parsers/filters
5. **Configuration Files**: YAML/TOML for custom patterns and formats

---

## Implementation Phases

### Phase 1: Core Input/Output & Basic Parsing Infrastructure

**Goal**: Establish the foundation for reading log files and basic line processing.

#### Key Components
1. File/Stream reader with buffering
2. Line-by-line iterator
3. Basic output formatter
4. CLI argument parsing
5. Error handling framework

#### Pseudocode

```
STRUCT LogEntry:
    raw_line: String
    line_number: u64
    timestamp: Option<DateTime>
    level: Option<LogLevel>
    message: String
    metadata: HashMap<String, String>

STRUCT LogReader:
    source: BufReader
    current_line: u64

    FUNCTION new(path: String) -> Result<LogReader>:
        file = open_file(path)?
        RETURN LogReader {
            source: BufReader::new(file),
            current_line: 0
        }

    FUNCTION next_line() -> Option<String>:
        IF source.read_line(&mut buffer) > 0:
            current_line += 1
            RETURN Some(buffer.trim())
        RETURN None

STRUCT LogWriter:
    output_format: OutputFormat
    destination: Writer

    FUNCTION write_entry(entry: LogEntry):
        formatted = format_entry(entry, output_format)
        destination.write(formatted)

FUNCTION main():
    args = parse_cli_args()
    reader = LogReader::new(args.input_file)?
    writer = LogWriter::new(args.output_format, args.output_file)?

    WHILE line = reader.next_line():
        entry = parse_line(line, reader.current_line)
        writer.write_entry(entry)
```

#### Implementation Details
- Use `std::io::BufReader` for efficient file reading (8KB-64KB buffer)
- Use `memmap2` for memory-mapped files (large files >100MB)
- Implement iterator pattern for lazy line processing
- Support stdin as input source for piping
- Use `clap` or `structopt` for CLI parsing
- Implement custom error types with `thiserror` or `anyhow`

---

### Phase 2: Log Format Detection & Pattern Recognition

**Goal**: Automatically detect and classify log formats using heuristics and pattern matching.

#### Key Components
1. Format detector (tries multiple parsers)
2. Pattern matchers for common formats
3. Confidence scoring system
4. Format-specific parsers (Docker, syslog, JSON, etc.)

#### Pseudocode

```
ENUM LogFormat:
    DockerJson,
    Syslog,
    CommonLog,
    JsonLines,
    CustomStructured,
    Unstructured

STRUCT FormatDetector:
    samples: Vec<String>

    FUNCTION detect(lines: Vec<String>) -> LogFormat:
        scores = HashMap::new()

        FOR format IN all_formats:
            score = 0
            FOR line IN lines.take(100):  // Sample first 100 lines
                IF format.matches(line):
                    score += 1
            scores[format] = score / lines.len()

        best_format = scores.max_by_key(|k, v| v)
        RETURN best_format.key

STRUCT DockerJsonParser:
    FUNCTION matches(line: String) -> bool:
        // Docker logs: {"log":"...", "stream":"stdout", "time":"2024-..."}
        IF line.starts_with("{") AND line.contains("\"log\""):
            TRY:
                json = parse_json(line)
                RETURN json.has_key("log") AND json.has_key("stream")
            CATCH:
                RETURN false
        RETURN false

    FUNCTION parse(line: String) -> Result<LogEntry>:
        json = parse_json(line)?
        RETURN LogEntry {
            raw_line: line,
            timestamp: parse_rfc3339(json["time"])?,
            message: json["log"],
            metadata: {
                "stream": json["stream"],
                "container_id": extract_from_source()
            }
        }

STRUCT SyslogParser:
    // Pattern: <priority>timestamp hostname app[pid]: message
    regex: Regex = r"^<(\d+)>(\w{3}\s+\d+\s+\d+:\d+:\d+)\s+(\S+)\s+(\S+)\[(\d+)\]:\s+(.+)$"

    FUNCTION matches(line: String) -> bool:
        RETURN regex.is_match(line)

    FUNCTION parse(line: String) -> Result<LogEntry>:
        captures = regex.captures(line)?
        priority = captures[1]
        level = extract_level_from_priority(priority)

        RETURN LogEntry {
            timestamp: parse_syslog_timestamp(captures[2]),
            level: level,
            message: captures[6],
            metadata: {
                "hostname": captures[3],
                "app": captures[4],
                "pid": captures[5]
            }
        }

STRUCT UnstructuredParser:
    // Fallback for poorly formatted logs
    timestamp_patterns: Vec<Regex>
    level_patterns: Vec<Regex>

    FUNCTION parse(line: String) -> LogEntry:
        entry = LogEntry::default()
        entry.raw_line = line

        // Try to extract timestamp
        FOR pattern IN timestamp_patterns:
            IF match = pattern.find(line):
                TRY:
                    entry.timestamp = parse_timestamp(match)
                    BREAK

        // Try to extract log level
        FOR pattern IN level_patterns:
            IF match = pattern.find(line):
                entry.level = match_to_level(match)
                BREAK

        // Everything else is message
        entry.message = line
        RETURN entry
```

#### Implementation Details
- Use `regex` crate for pattern matching
- Use `serde_json` for JSON parsing
- Common timestamp patterns to detect:
  - RFC3339: `2024-01-15T10:30:45.123Z`
  - RFC2822: `Mon, 15 Jan 2024 10:30:45 +0000`
  - Syslog: `Jan 15 10:30:45`
  - Common: `2024-01-15 10:30:45.123`
  - UNIX timestamps: `1705318245`
- Log level patterns: `ERROR|ERR|FATAL|WARN|WARNING|INFO|DEBUG|TRACE` (case-insensitive)
- Implement confidence scoring (0.0-1.0) based on successful parses
- Support format hints via CLI flags (--format docker, --format syslog)

---

### Phase 3: Structured Data Extraction & Normalization

**Goal**: Extract meaningful fields from parsed logs and normalize into consistent structure.

#### Key Components
1. Timestamp parser (multiple formats)
2. Log level normalizer
3. Metadata extractor
4. Field mapping system
5. Data validation

#### Pseudocode

```
STRUCT TimestampParser:
    formats: Vec<String>

    FUNCTION new() -> TimestampParser:
        RETURN TimestampParser {
            formats: [
                "%Y-%m-%dT%H:%M:%S%.fZ",           // RFC3339
                "%Y-%m-%d %H:%M:%S%.f",            // Common
                "%b %d %H:%M:%S",                   // Syslog
                "%d/%b/%Y:%H:%M:%S %z",            // Apache
                "%s",                               // UNIX timestamp
                "%Y-%m-%d %H:%M:%S,%f",            // Log4j
            ]
        }

    FUNCTION parse(input: String) -> Option<DateTime>:
        FOR format IN formats:
            TRY:
                dt = DateTime::parse_from_str(input, format)
                RETURN Some(dt)
            CATCH:
                CONTINUE

        // Try regex-based extraction for embedded timestamps
        FOR pattern IN timestamp_regex_patterns:
            IF match = pattern.find(input):
                RETURN parse(match)

        RETURN None

ENUM LogLevel:
    Fatal,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
    Unknown

STRUCT LevelNormalizer:
    mappings: HashMap<String, LogLevel>

    FUNCTION new() -> LevelNormalizer:
        mappings = {
            "FATAL|CRIT|CRITICAL": LogLevel::Fatal,
            "ERROR|ERR|SEVERE": LogLevel::Error,
            "WARN|WARNING": LogLevel::Warn,
            "INFO|INFORMATION|NOTICE": LogLevel::Info,
            "DEBUG": LogLevel::Debug,
            "TRACE|VERBOSE": LogLevel::Trace,
        }
        RETURN LevelNormalizer { mappings }

    FUNCTION normalize(input: String) -> LogLevel:
        input_upper = input.to_uppercase()
        FOR pattern, level IN mappings:
            IF input_upper MATCHES pattern:
                RETURN level
        RETURN LogLevel::Unknown

STRUCT MetadataExtractor:
    // Extract key-value pairs from logs
    kv_patterns: Vec<Regex>

    FUNCTION extract(line: String) -> HashMap<String, String>:
        metadata = HashMap::new()

        // Pattern: key=value or key="value" or key:value
        patterns = [
            r"(\w+)=([^\s]+)",
            r"(\w+)=\"([^\"]+)\"",
            r"(\w+):\s*([^\s,]+)",
        ]

        FOR pattern IN patterns:
            FOR match IN pattern.find_all(line):
                key = match[1]
                value = match[2]
                metadata[key] = value

        // Docker-specific metadata
        IF line.contains("container_id") OR line.contains("container_name"):
            metadata.merge(extract_docker_metadata(line))

        RETURN metadata

FUNCTION normalize_entry(entry: LogEntry, format: LogFormat) -> LogEntry:
    normalized = entry.clone()

    // Ensure timestamp is set
    IF normalized.timestamp.is_none():
        timestamp_parser = TimestampParser::new()

        // Try extracting from message
        IF ts = timestamp_parser.parse(normalized.message):
            normalized.timestamp = Some(ts)
        ELSE IF ts = timestamp_parser.parse(normalized.raw_line):
            normalized.timestamp = Some(ts)
        ELSE:
            // Use current time as fallback
            normalized.timestamp = Some(DateTime::now())
            normalized.metadata["timestamp_assumed"] = "true"

    // Normalize log level
    IF normalized.level.is_none():
        level_normalizer = LevelNormalizer::new()
        normalized.level = level_normalizer.normalize(normalized.message)

    // Extract additional metadata
    extractor = MetadataExtractor::new()
    extracted = extractor.extract(normalized.raw_line)
    normalized.metadata.merge(extracted)

    RETURN normalized
```

#### Implementation Details
- Use `chrono` crate for timestamp parsing and manipulation
- Use `chrono-tz` for timezone support
- Implement fallback chains for timestamp parsing (try specific -> try generic -> use current time)
- Store timezone information in metadata
- Use lazy_static for compiled regex patterns
- Implement field validation (timestamp ranges, level values)
- Support custom field extractors via configuration
- Handle multiline logs (stack traces, JSON spanning lines)

---

### Phase 4: Filtering, Querying & Analysis

**Goal**: Enable powerful filtering and analysis capabilities on parsed logs.

#### Key Components
1. Filter chain system
2. Time range filters
3. Level filters
4. Keyword/regex search
5. Aggregation engine
6. Statistics collector

#### Pseudocode

```
TRAIT Filter:
    FUNCTION matches(entry: LogEntry) -> bool

STRUCT TimeRangeFilter:
    start: Option<DateTime>
    end: Option<DateTime>

    FUNCTION matches(entry: LogEntry) -> bool:
        IF entry.timestamp.is_none():
            RETURN false

        ts = entry.timestamp.unwrap()

        IF start.is_some() AND ts < start.unwrap():
            RETURN false

        IF end.is_some() AND ts > end.unwrap():
            RETURN false

        RETURN true

STRUCT LevelFilter:
    levels: Set<LogLevel>
    min_level: Option<LogLevel>

    FUNCTION matches(entry: LogEntry) -> bool:
        IF entry.level.is_none():
            RETURN false

        level = entry.level.unwrap()

        IF !levels.is_empty() AND !levels.contains(level):
            RETURN false

        IF min_level.is_some() AND level.severity() < min_level.severity():
            RETURN false

        RETURN true

STRUCT KeywordFilter:
    keywords: Vec<String>
    regex_patterns: Vec<Regex>
    case_sensitive: bool

    FUNCTION matches(entry: LogEntry) -> bool:
        search_text = IF case_sensitive:
            entry.message
        ELSE:
            entry.message.to_lowercase()

        // Check keywords
        FOR keyword IN keywords:
            search_keyword = IF case_sensitive:
                keyword
            ELSE:
                keyword.to_lowercase()

            IF search_text.contains(search_keyword):
                RETURN true

        // Check regex patterns
        FOR pattern IN regex_patterns:
            IF pattern.is_match(entry.message):
                RETURN true

        RETURN false

STRUCT MetadataFilter:
    conditions: HashMap<String, String>

    FUNCTION matches(entry: LogEntry) -> bool:
        FOR key, expected_value IN conditions:
            IF !entry.metadata.contains_key(key):
                RETURN false

            IF entry.metadata[key] != expected_value:
                RETURN false

        RETURN true

STRUCT FilterChain:
    filters: Vec<Box<dyn Filter>>

    FUNCTION add_filter(filter: Box<dyn Filter>):
        filters.push(filter)

    FUNCTION matches(entry: LogEntry) -> bool:
        FOR filter IN filters:
            IF !filter.matches(entry):
                RETURN false
        RETURN true

STRUCT LogAnalyzer:
    stats: Statistics

    FUNCTION analyze(entries: Iterator<LogEntry>):
        FOR entry IN entries:
            stats.total_count += 1
            stats.count_by_level[entry.level] += 1

            IF entry.timestamp.is_some():
                ts = entry.timestamp.unwrap()
                IF ts < stats.earliest_timestamp:
                    stats.earliest_timestamp = ts
                IF ts > stats.latest_timestamp:
                    stats.latest_timestamp = ts

            // Track error patterns
            IF entry.level == LogLevel::Error:
                stats.error_patterns[entry.message.substring(0, 50)] += 1

            // Track sources
            IF entry.metadata.contains_key("container_id"):
                container = entry.metadata["container_id"]
                stats.count_by_container[container] += 1

    FUNCTION get_summary() -> Summary:
        RETURN Summary {
            total_entries: stats.total_count,
            time_range: (stats.earliest_timestamp, stats.latest_timestamp),
            level_distribution: stats.count_by_level,
            top_errors: stats.error_patterns.top(10),
            top_sources: stats.count_by_container.top(10),
            entries_per_second: calculate_rate(stats),
        }

FUNCTION process_with_filters(reader: LogReader, filters: FilterChain, writer: LogWriter):
    analyzer = LogAnalyzer::new()
    processed_count = 0
    filtered_count = 0

    FOR line IN reader:
        entry = parse_and_normalize(line)

        IF filters.matches(entry):
            writer.write_entry(entry)
            analyzer.analyze(entry)
            processed_count += 1
        ELSE:
            filtered_count += 1

    summary = analyzer.get_summary()
    print_summary(summary, processed_count, filtered_count)
```

#### Implementation Details
- Implement short-circuit evaluation in filter chains (stop at first non-match)
- Use `rayon` for parallel processing of large log files
- Implement filter optimization (reorder filters by selectivity)
- Support complex queries: `--level ERROR --since "2024-01-15 10:00" --until "2024-01-15 11:00" --grep "timeout"`
- Add support for container name/ID filtering: `--container myapp-container`
- Implement negation filters: `--exclude-keyword "health-check"`
- Add tail mode: `--tail 100` (last N entries)
- Add follow mode: `--follow` (like `tail -f`)
- Store statistics in efficient data structures (BTreeMap for time-ordered data)

---

### Phase 5: Advanced Features & Output Formats

**Goal**: Polish the parser with advanced features, multiple output formats, and production-ready performance.

#### Key Components
1. Multi-format output (JSON, CSV, table, pretty-print)
2. Log correlation across sources
3. Performance optimization
4. Error recovery and resilience
5. Configuration file support
6. Export and reporting

#### Pseudocode

```
ENUM OutputFormat:
    Json,
    JsonLines,
    Csv,
    Table,
    PrettyPrint,
    Custom(String)

STRUCT JsonOutputWriter:
    writer: BufWriter
    is_first: bool

    FUNCTION write_header():
        writer.write("[\n")

    FUNCTION write_entry(entry: LogEntry):
        IF !is_first:
            writer.write(",\n")

        json = serialize_to_json(entry)
        writer.write("  " + json)
        is_first = false

    FUNCTION write_footer():
        writer.write("\n]\n")

STRUCT CsvOutputWriter:
    writer: csv::Writer

    FUNCTION write_header():
        writer.write_record([
            "timestamp", "level", "message",
            "container_id", "stream", "metadata"
        ])

    FUNCTION write_entry(entry: LogEntry):
        writer.write_record([
            entry.timestamp.to_string(),
            entry.level.to_string(),
            entry.message,
            entry.metadata.get("container_id").unwrap_or(""),
            entry.metadata.get("stream").unwrap_or(""),
            serialize_metadata(entry.metadata),
        ])

STRUCT PrettyPrintWriter:
    colorize: bool
    show_metadata: bool

    FUNCTION write_entry(entry: LogEntry):
        output = ""

        // Timestamp in cyan
        IF entry.timestamp.is_some():
            ts = entry.timestamp.unwrap().format("%Y-%m-%d %H:%M:%S%.3f")
            IF colorize:
                output += colorize(ts, Color::Cyan)
            ELSE:
                output += ts
            output += " "

        // Level with color
        level_str = format!("[{:5}]", entry.level)
        IF colorize:
            color = match entry.level:
                Fatal, Error => Color::Red,
                Warn => Color::Yellow,
                Info => Color::Green,
                Debug => Color::Blue,
                Trace => Color::Magenta,
            output += colorize(level_str, color)
        ELSE:
            output += level_str
        output += " "

        // Container/source info
        IF entry.metadata.contains_key("container_id"):
            container = entry.metadata["container_id"].substring(0, 12)
            output += format!("[{}] ", container)

        // Message
        output += entry.message

        // Metadata on separate line if requested
        IF show_metadata AND !entry.metadata.is_empty():
            output += "\n  " + format_metadata(entry.metadata)

        println(output)

STRUCT LogCorrelator:
    // Correlate logs from multiple sources/containers
    window_size: Duration
    entries_by_time: BTreeMap<DateTime, Vec<LogEntry>>

    FUNCTION add_entry(entry: LogEntry):
        timestamp = entry.timestamp.unwrap_or(DateTime::now())
        entries_by_time[timestamp].push(entry)

    FUNCTION get_correlated(timestamp: DateTime) -> Vec<LogEntry>:
        start = timestamp - window_size / 2
        end = timestamp + window_size / 2

        result = Vec::new()
        FOR (ts, entries) IN entries_by_time.range(start..end):
            result.extend(entries)

        // Sort by timestamp
        result.sort_by_key(|e| e.timestamp)
        RETURN result

STRUCT PerformanceOptimizer:
    FUNCTION optimize_for_large_file(path: String) -> Reader:
        file_size = get_file_size(path)

        IF file_size > 100_MB:
            // Use memory mapping for large files
            RETURN MmapReader::new(path)
        ELSE:
            // Use buffered reader for smaller files
            RETURN BufReader::new(path)

    FUNCTION parallel_process(entries: Vec<LogEntry>, workers: usize) -> Vec<LogEntry>:
        USE rayon::prelude::*

        RETURN entries
            .par_iter()
            .map(|entry| parse_and_normalize(entry))
            .filter(|entry| apply_filters(entry))
            .collect()

STRUCT ErrorRecovery:
    max_errors: usize
    error_count: usize

    FUNCTION handle_parse_error(line: String, error: Error) -> Option<LogEntry>:
        error_count += 1

        IF error_count > max_errors:
            eprintln!("Too many parse errors, aborting")
            EXIT(1)

        // Log the error
        eprintln!("Parse error on line {}: {}", line_number, error)

        // Return a partial entry
        RETURN Some(LogEntry {
            raw_line: line,
            message: line,
            level: Some(LogLevel::Unknown),
            timestamp: None,
            metadata: {
                "parse_error": error.to_string()
            }
        })

STRUCT ConfigFile:
    // Support YAML/TOML configuration
    format_hints: HashMap<String, LogFormat>
    custom_patterns: Vec<PatternConfig>
    filter_presets: HashMap<String, FilterChain>

    FUNCTION load(path: String) -> Result<ConfigFile>:
        content = read_file(path)?
        config = parse_yaml(content)?
        RETURN config

    FUNCTION apply_to_parser(parser: &mut Parser):
        FOR source, format IN format_hints:
            parser.set_format_hint(source, format)

        FOR pattern_config IN custom_patterns:
            parser.add_custom_pattern(pattern_config)

FUNCTION main():
    args = parse_args()

    // Load configuration if provided
    IF args.config_file.is_some():
        config = ConfigFile::load(args.config_file)?

    // Set up reader with performance optimization
    reader = PerformanceOptimizer::optimize_for_large_file(args.input_file)

    // Set up format detection
    format = IF args.format_hint.is_some():
        args.format_hint
    ELSE:
        detect_format(reader.peek_lines(100))

    // Set up filter chain
    filters = FilterChain::new()
    IF args.time_range.is_some():
        filters.add(TimeRangeFilter::new(args.time_range))
    IF args.levels.is_some():
        filters.add(LevelFilter::new(args.levels))
    IF args.keywords.is_some():
        filters.add(KeywordFilter::new(args.keywords))

    // Set up output writer
    writer = match args.output_format:
        OutputFormat::Json => JsonOutputWriter::new(args.output_file),
        OutputFormat::Csv => CsvOutputWriter::new(args.output_file),
        OutputFormat::PrettyPrint => PrettyPrintWriter::new(args.colorize),
        _ => DefaultWriter::new()

    writer.write_header()

    // Set up error recovery
    error_handler = ErrorRecovery::new(max_errors: 100)

    // Process logs
    stats = Statistics::new()
    FOR line IN reader:
        TRY:
            entry = parse_and_normalize(line, format)
        CATCH error:
            entry = error_handler.handle_parse_error(line, error)?

        IF filters.matches(entry):
            writer.write_entry(entry)
            stats.update(entry)

    writer.write_footer()

    // Print summary
    IF args.show_summary:
        print_summary(stats)
```

#### Implementation Details
- Use `serde` for JSON serialization
- Use `csv` crate for CSV output
- Use `colored` or `termcolor` for colorized output
- Implement streaming JSON output (JSON Lines format) for large datasets
- Add progress bar using `indicatif` for large files
- Support gzip/bzip2 compressed logs using `flate2`
- Implement log rotation detection (handle multiple log files in sequence)
- Add export formats:
  - HTML report with charts
  - SQLite database for further analysis
  - Elasticsearch bulk import format
- Implement custom output templates (user-defined format strings)
- Add metrics export (Prometheus format)
- Support parallel processing with `rayon` (process multiple files concurrently)
- Implement zero-copy parsing where possible (use `&str` instead of `String`)
- Add benchmarking suite to track performance

---

## Technical Stack

### Core Dependencies
- **clap**: CLI argument parsing
- **serde**: Serialization/deserialization
- **serde_json**: JSON support
- **chrono**: Date/time handling
- **regex**: Pattern matching
- **anyhow/thiserror**: Error handling
- **csv**: CSV output
- **rayon**: Parallel processing
- **memmap2**: Memory-mapped file I/O

### Optional Dependencies
- **colored**: Terminal colors
- **indicatif**: Progress bars
- **flate2**: Compression support
- **rusqlite**: SQLite export
- **quick-xml**: XML log parsing
- **yaml-rust**: YAML config files
- **toml**: TOML config files

---

## CLI Design

```bash
# Basic usage
lumber /var/log/app.log

# Docker container logs
lumber --format docker /var/lib/docker/containers/*/container.log

# Time-based filtering
lumber --since "2024-01-15 10:00" --until "2024-01-15 11:00" app.log

# Level filtering
lumber --level ERROR --level FATAL app.log
lumber --min-level WARN app.log

# Keyword search
lumber --grep "timeout" --grep "connection refused" app.log
lumber --grep-regex "error.*database" app.log

# Output formats
lumber --output json app.log
lumber --output csv --output-file results.csv app.log
lumber --pretty --colorize app.log

# Container-specific
lumber --container myapp-container /var/log/docker/*

# Analysis
lumber --summary --stats app.log
lumber --top-errors 10 app.log

# Follow mode (like tail -f)
lumber --follow app.log

# Multiple files
lumber /var/log/app.*.log --correlate

# Configuration file
lumber --config lumber.yaml app.log
```

---

## Performance Considerations

1. **Memory Efficiency**
   - Use streaming parsers (don't load entire file into memory)
   - Use memory-mapped I/O for large files (>100MB)
   - Implement line-by-line processing with iterators
   - Use `Cow<str>` to avoid unnecessary allocations

2. **CPU Efficiency**
   - Compile regex patterns once (use `lazy_static`)
   - Use parallel processing for multi-file scenarios
   - Implement filter short-circuiting
   - Use zero-copy parsing where possible

3. **I/O Efficiency**
   - Use buffered readers/writers
   - Batch output writes
   - Support compressed logs without decompression to disk
   - Implement read-ahead buffering

4. **Scalability**
   - Handle files >10GB
   - Support streaming from stdin
   - Implement chunked processing
   - Add sampling mode for very large files

---

## Error Handling Strategy

1. **Parse Errors**
   - Log failed lines to stderr
   - Continue processing (don't abort)
   - Track error count, abort if threshold exceeded
   - Option to output unparseable lines separately

2. **I/O Errors**
   - Graceful handling of missing files
   - Handle permission errors
   - Detect truncated files
   - Support log rotation mid-read

3. **Resource Errors**
   - Handle out-of-memory conditions
   - Implement backpressure for streaming
   - Graceful degradation when limits hit

---

## Testing Strategy

1. **Unit Tests**
   - Test each parser individually
   - Test filter logic
   - Test timestamp parsing edge cases
   - Test normalization functions

2. **Integration Tests**
   - Test with real Docker logs
   - Test with real application logs
   - Test with malformed logs
   - Test with mixed formats

3. **Performance Tests**
   - Benchmark with 1GB+ files
   - Memory usage profiling
   - CPU profiling
   - I/O profiling

4. **Fuzz Testing**
   - Random input generation
   - Edge case discovery
   - Crash testing

---

## Learning Resources

### Rust Fundamentals
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - Essential reading
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Hands-on examples
- [Rust Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html) - Critical for parsers

### CLI Applications in Rust
- [Command Line Apps in Rust](https://rust-cli.github.io/book/) - Comprehensive CLI guide
- [clap Documentation](https://docs.rs/clap/) - Argument parsing
- [Building a CLI Tool with Rust](https://kerkour.com/rust-cli-tutorial) - Practical tutorial

### Parsing & Text Processing
- [Regex in Rust](https://docs.rs/regex/) - Pattern matching
- [nom Parser Combinators](https://github.com/rust-bakery/nom) - Advanced parsing (optional)
- [Pest Parser](https://pest.rs/) - PEG parsing for complex formats
- [Parsing Text with nom](https://blog.logrocket.com/parsing-in-rust-with-nom/)

### Performance Optimization
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/) - Optimization techniques
- [Rayon Data Parallelism](https://github.com/rayon-rs/rayon) - Parallel processing
- [Zero-Copy Parsing in Rust](https://blog.adamchalmers.com/nom-chars/)
- [Memory-Mapped I/O](https://docs.rs/memmap2/)

### Date/Time Handling
- [Chrono Documentation](https://docs.rs/chrono/) - Comprehensive date/time library
- [Time Formatting in Rust](https://time-rs.github.io/book/)

### Log Format Specifications
- [Docker Logging Drivers](https://docs.docker.com/config/containers/logging/configure/) - Docker log formats
- [Syslog Protocol RFC 5424](https://tools.ietf.org/html/rfc5424) - Syslog specification
- [Common Log Format](https://en.wikipedia.org/wiki/Common_Log_Format) - Web server logs
- [JSON Lines Format](https://jsonlines.org/) - Streaming JSON

### Error Handling
- [anyhow Documentation](https://docs.rs/anyhow/) - Flexible error handling
- [thiserror Documentation](https://docs.rs/thiserror/) - Custom error types
- [Error Handling in Rust](https://www.shuttle.rs/blog/2022/06/30/error-handling)

### Testing
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [cargo-fuzz](https://rust-fuzz.github.io/book/) - Fuzz testing
- [Proptest](https://proptest-rs.github.io/proptest/) - Property-based testing
- [Criterion.rs](https://github.com/bheisler/criterion.rs) - Benchmarking

### Data Structures
- [BTreeMap vs HashMap](https://doc.rust-lang.org/std/collections/) - Choosing collections
- [Efficient String Handling](https://deterministic.space/secret-life-of-cows.html) - Cow<str> usage

### Real-World Examples
- [ripgrep Source Code](https://github.com/BurntSushi/ripgrep) - High-performance text search
- [fd Source Code](https://github.com/sharkdp/fd) - Fast file finder
- [bat Source Code](https://github.com/sharkdp/bat) - Cat clone with syntax highlighting
- [lnav - Log Navigator](https://lnav.org/) - Inspiration for features (C++)

### Articles & Tutorials
- [Writing a Command Line Tool in Rust](https://mattgathu.dev/2017/08/29/writing-cli-app-rust.html)
- [Parsing Logs 230x Faster with Rust](https://andre.arko.net/2018/10/25/parsing-logs-230x-faster-with-rust/)
- [Building Fast Interpreters in Rust](https://blog.cloudflare.com/building-fast-interpreters-in-rust/)
- [Rust Stream Processing](https://blog.adamchalmers.com/streaming-data-rust/)

### Documentation Tools
- [rustdoc](https://doc.rust-lang.org/rustdoc/) - Document your code
- [mdBook](https://rust-lang.github.io/mdBook/) - Create user guides

### Community Resources
- [r/rust](https://www.reddit.com/r/rust/) - Rust community
- [users.rust-lang.org](https://users.rust-lang.org/) - Rust forums
- [This Week in Rust](https://this-week-in-rust.org/) - Weekly newsletter
- [Rust Discord](https://discord.gg/rust-lang) - Real-time help

---

## Future Enhancements

- **Machine Learning Integration**: Anomaly detection, log clustering
- **Real-time Processing**: WebSocket streaming, live dashboards
- **Distributed Parsing**: Process logs across multiple machines
- **Smart Correlation**: Automatic trace ID detection, request flow tracking
- **Database Integration**: Direct insert into ClickHouse, TimescaleDB
- **Alert System**: Trigger webhooks/emails on patterns
- **Interactive Mode**: TUI for exploring logs (using `tui-rs`)
- **Plugin System**: User-defined parsers and formatters
- **Cloud Integration**: S3, GCS, Azure Blob log parsing
- **Metrics & Visualization**: Built-in charts and graphs

---

## License

MIT or Apache-2.0 (dual license, Rust convention)

---

## Contributing

Contributions welcome! This is a learning project focused on building non-trivial Rust applications.

Focus areas:
1. Parser improvements for new log formats
2. Performance optimizations
3. Additional output formats
4. Better error recovery
5. Documentation and examples
