# Shimmy-DS Features

## 🧠 Adaptive Intelligence System
- **AdaptIQ Engine**: Dynamic recursion depth and pathogen sensitivity adaptation
- **Qualitier**: 4-tier quality management (Minimal, Standard, Enhanced, Premium)
- **ObliSelect**: Smart obligation prioritization preventing prompt bloat
- **ProfileMesh**: Cross-session user preference tracking with 6-dimensional taste vectors
- **PulseTrace**: Real-time telemetry and performance monitoring
- **CacheMind**: Intelligent narrative state caching with LRU eviction

## 🧬 Recursive Narrative Intelligence
- **CAPR DNA Tracking**: Contradiction→Action→Pressure→Return loop analysis
- **Constraint Space Modeling**: Dynamic constraint graph with freedom scoring
- **Multi-Level Recursion**: Cross-scale pattern detection (sentence→story)
- **Character Consistency**: Deep personality tracking with dialogue fingerprinting
- **Reader Engagement**: Psychology-based engagement loop detection
- **RIP+RIC Protocol**: Cross-language narrative integrity system

## 🎯 Core Engine Features

### Auto-Discovery
- Automatically finds GGUF and SafeTensors models
- Scans common directories and environment variables
- Use `cargo run --features llama -- list` to see discovered models

## API Enhancements
- Proper HTTP status codes (404 for missing models, 502 for generation failures)
- `/metrics` endpoint for monitoring
- Enhanced error messages

## RustChain Integration
- Compatible as RustChain LLM provider
- See `docs/rustchain-provider.md` for configuration

## CLI Commands
- `serve` - Start HTTP server with all features
- `list` - Show discovered models
- `probe` - Test model loading
- `generate` - Quick CLI generation

## Environment Variables
- `SHIMMY_BASE_GGUF` - Primary model file
- `SHIMMY_LORA_GGUF` - Optional LoRA adapter
- Models also auto-discovered in:
  - `~/.cache/huggingface/`
  - `~/models/`
  - Parent directory of SHIMMY_BASE_GGUF

## 📊 Adaptive Intelligence Configuration
- **ProfileMesh Storage**: `.shimmy/user_mesh.json` for cross-session persistence
- **Quality Thresholds**: Configurable memory and time limits in `shimmy-ds.toml`
- **Obligation Limits**: Smart injection count limits (default: top 8 obligations)
- **Telemetry Settings**: Real-time performance monitoring configuration
- **Cache Management**: Intelligent state persistence with configurable LRU limits

## 🎛️ Quality Management Tiers
- **Minimal**: Obligation injection only, 4 max recursion depth
- **Standard**: + Emotion tracking, 6 max recursion depth
- **Enhanced**: + Spatial validation, CAPR depth, 10 max recursion depth
- **Premium**: Full recursive intelligence, 14 max recursion depth

## 🔍 Smart Obligation Features
- **Multi-Factor Scoring**: Urgency × Salience × Freshness × Tension Balance
- **FPD Integration**: Setup/payoff relationship tracking
- **Prompt Bloat Prevention**: Intelligent top-N selection
- **Mistake Learning**: Recurring error pattern detection and avoidance
