# Shimmy-DS API Reference

## Overview

Shimmy-DS provides a comprehensive API with 100% OpenAI compatibility plus advanced narrative intelligence capabilities. The system features 75 compiled modules (35,162 lines of code) providing both standard LLM inference and world-first recursive narrative intelligence.

## Base URL

```
http://127.0.0.1:11435
```

## Authentication

No authentication required for local usage. For production deployments, implement authentication at the reverse proxy level.

## OpenAI Compatible Endpoints

### Chat Completions

**Endpoint:** `POST /v1/chat/completions`

**Description:** OpenAI-compatible chat completions with automatic narrative intelligence enhancement.

**Request:**
```json
{
  "model": "microsoft/Phi-3.5-mini-instruct",
  "messages": [
    {"role": "system", "content": "You are a creative writing assistant."},
    {"role": "user", "content": "Write a story about recursive mirrors."}
  ],
  "max_tokens": 1000,
  "temperature": 0.7,
  "stream": false
}
```

**Response:**
```json
{
  "id": "chatcmpl-123",
  "object": "chat.completion",
  "created": 1677652288,
  "model": "microsoft/Phi-3.5-mini-instruct",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "Elena approached the antique mirror..."
      },
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 25,
    "completion_tokens": 250,
    "total_tokens": 275
  }
}
```

### Legacy Completions

**Endpoint:** `POST /v1/completions`

**Description:** OpenAI-compatible text completions with narrative intelligence.

**Request:**
```json
{
  "model": "microsoft/Phi-3.5-mini-instruct",
  "prompt": "Once upon a time in a land of recursive dreams,",
  "max_tokens": 500,
  "temperature": 0.8
}
```

### Models List

**Endpoint:** `GET /v1/models`

**Description:** List all available models.

**Response:**
```json
{
  "object": "list",
  "data": [
    {
      "id": "microsoft/Phi-3.5-mini-instruct",
      "object": "model",
      "created": 1677610602,
      "owned_by": "microsoft",
      "permission": [],
      "root": "microsoft/Phi-3.5-mini-instruct",
      "parent": null
    }
  ]
}
```

## Narrative Intelligence Endpoints

### Narrative Analysis

**Endpoint:** `GET /narrative/analyze`

**Description:** Get current narrative state analysis from all intelligence systems.

**Response:**
```json
{
  "narrative_health": {
    "overall_score": 0.85,
    "system_scores": {
      "dna_tracker": 0.90,
      "constraint_space": 0.82,
      "recursion_tracker": 0.88,
      "character_consistency": 0.91,
      "engagement_tracker": 0.79,
      "drift_stability": 0.84
    }
  },
  "active_patterns": [
    {
      "type": "CAPR_loop",
      "description": "Mirror reflection → Identity crisis → Self-discovery → Acceptance",
      "strength": 0.78,
      "completion": 0.45
    },
    {
      "type": "character_arc",
      "character": "Elena",
      "arc_type": "Self-discovery",
      "progress": 0.62
    }
  ],
  "constraints": {
    "active": 12,
    "resolved": 8,
    "freedom_score": 0.73,
    "pressure_points": ["character_motivation", "setting_consistency"]
  },
  "engagement_metrics": {
    "curiosity_score": 0.81,
    "investment_score": 0.77,
    "tension_level": 0.69
  }
}
```

### Comprehensive System Report

**Endpoint:** `GET /narrative/report`

**Description:** Full system status report with detailed telemetry.

**Response:**
```json
{
  "system_status": {
    "compilation_status": "100% successful",
    "modules_loaded": 75,
    "lines_of_code": 35162,
    "test_coverage": "comprehensive",
    "uptime": "2h 34m",
    "memory_usage": "127MB"
  },
  "adaptive_intelligence": {
    "adapt_iq": {
      "current_depth": 3,
      "sensitivity": 0.7,
      "adaptations_made": 15
    },
    "qualitier": {
      "current_tier": "Enhanced",
      "resource_usage": 0.62,
      "efficiency_score": 0.84
    },
    "obli_select": {
      "obligations_active": 8,
      "priority_score": 0.79,
      "injection_rate": 0.23
    },
    "profile_mesh": {
      "sessions_tracked": 1,
      "taste_confidence": 0.71,
      "learning_rate": 0.15
    }
  },
  "narrative_systems": {
    "dna_tracker": {
      "active_loops": 3,
      "completion_rate": 0.67,
      "transformation_score": 0.82
    },
    "constraint_space": {
      "total_constraints": 20,
      "violation_rate": 0.05,
      "freedom_index": 0.73
    },
    "recursion_tracker": {
      "scales_active": 5,
      "pattern_density": 0.71,
      "echo_strength": 0.68
    },
    "character_engine": {
      "characters_tracked": 2,
      "consistency_score": 0.91,
      "voice_stability": 0.87
    },
    "engagement_loops": {
      "active_hooks": 4,
      "satisfaction_rate": 0.79,
      "tension_curve": "ascending"
    }
  },
  "stability_metrics": {
    "drift_warnings": 0,
    "pathogen_detections": 0,
    "intervention_count": 2,
    "stability_score": 0.89
  }
}
```

### Runtime Configuration

**Endpoint:** `GET /narrative/config`

**Description:** Get current narrative intelligence configuration.

**Response:**
```json
{
  "assertiveness_level": "moderate",
  "systems_enabled": {
    "dna_tracking": true,
    "constraint_modeling": true,
    "recursion_tracking": true,
    "character_consistency": true,
    "engagement_loops": true,
    "drift_stabilization": true
  },
  "sensitivity_thresholds": {
    "constraint_pressure": 0.7,
    "character_drift": 0.8,
    "unresolved_loops": 0.6,
    "engagement_drops": 0.7,
    "pattern_breaks": 0.5
  },
  "adaptive_settings": {
    "adaptation_enabled": true,
    "learning_rate": 0.15,
    "quality_tier": "Enhanced"
  }
}
```

**Endpoint:** `POST /narrative/config`

**Description:** Update narrative intelligence configuration.

**Request:**
```json
{
  "assertiveness_level": "active",
  "sensitivity_thresholds": {
    "constraint_pressure": 0.8,
    "character_drift": 0.9
  }
}
```

## Shimmy-DS CLI Commands

### Server Commands

```bash
# Start server with full narrative intelligence
shimmy.exe serve

# Start with specific bind address
shimmy.exe serve --bind 0.0.0.0:11435

# Start with custom configuration
shimmy.exe serve --config custom-config.toml
```

### Model Management

```bash
# List all available models
shimmy.exe list

# Discover models in filesystem
shimmy.exe discover

# Register a new model
shimmy.exe register "my-model" "/path/to/model.gguf" --template chatml

# Test model loading
shimmy.exe probe "my-model"

# Benchmark model performance
shimmy.exe bench "my-model" --max-tokens 100
```

### Generation Commands

```bash
# Generate text with narrative intelligence
shimmy.exe generate "my-model" --prompt "Write a recursive story" --max-tokens 500

# Generate with specific configuration
shimmy.exe generate "my-model" --prompt "Story prompt" --config narrative-config.toml
```

## Streaming Support

All generation endpoints support streaming responses. Add `"stream": true` to enable:

```json
{
  "model": "microsoft/Phi-3.5-mini-instruct",
  "messages": [{"role": "user", "content": "Write a story"}],
  "stream": true
}
```

Streaming responses use Server-Sent Events (SSE) format:

```
data: {"choices":[{"delta":{"content":"Once"}}]}

data: {"choices":[{"delta":{"content":" upon"}}]}

data: {"choices":[{"delta":{"content":" a"}}]}

data: [DONE]
```

## Error Handling

### Standard HTTP Status Codes

- `200 OK`: Request successful
- `400 Bad Request`: Invalid request format
- `404 Not Found`: Model or endpoint not found
- `500 Internal Server Error`: Server error
- `503 Service Unavailable`: Model loading error

### Error Response Format

```json
{
  "error": {
    "type": "invalid_request_error",
    "message": "Model 'nonexistent-model' not found",
    "code": "model_not_found"
  }
}
```

## Integration Examples

### Python with OpenAI Library

```python
import openai

# Configure for local Shimmy-DS
openai.api_base = "http://127.0.0.1:11435/v1"
openai.api_key = "not-needed"

# Generate with narrative intelligence
response = openai.ChatCompletion.create(
    model="microsoft/Phi-3.5-mini-instruct",
    messages=[
        {"role": "user", "content": "Write a recursive narrative about time loops"}
    ],
    max_tokens=1000
)

print(response.choices[0].message.content)

# Get narrative analysis
import requests
analysis = requests.get("http://127.0.0.1:11435/narrative/analyze").json()
print(f"Narrative health: {analysis['narrative_health']['overall_score']}")
```

### JavaScript/Node.js

```javascript
// Using fetch API
async function generateStory() {
  const response = await fetch('http://127.0.0.1:11435/v1/chat/completions', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      model: 'microsoft/Phi-3.5-mini-instruct',
      messages: [
        {role: 'user', content: 'Write a story with recursive themes'}
      ],
      max_tokens: 500
    })
  });

  const data = await response.json();
  return data.choices[0].message.content;
}

// Get narrative analysis
async function getNarrativeHealth() {
  const response = await fetch('http://127.0.0.1:11435/narrative/analyze');
  const analysis = await response.json();
  return analysis.narrative_health.overall_score;
}
```

### cURL Examples

```bash
# Generate text
curl -X POST http://127.0.0.1:11435/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "microsoft/Phi-3.5-mini-instruct",
    "messages": [{"role": "user", "content": "Write a recursive story"}],
    "max_tokens": 500
  }'

# Get narrative analysis
curl http://127.0.0.1:11435/narrative/analyze

# Update configuration
curl -X POST http://127.0.0.1:11435/narrative/config \
  -H "Content-Type: application/json" \
  -d '{"assertiveness_level": "active"}'
```

## Performance Optimization

### Recommended Settings

For optimal performance with narrative intelligence:

```json
{
  "adaptive_settings": {
    "quality_tier": "Enhanced",
    "adaptation_enabled": true,
    "cache_enabled": true
  },
  "narrative_settings": {
    "max_analysis_depth": 5,
    "concurrent_systems": true,
    "cache_insights": true
  }
}
```

### Resource Usage

| Component | Memory Usage | CPU Usage | Latency |
|-----------|-------------|-----------|---------|
| Base System | ~100MB | Low | <10ms |
| Narrative Intelligence | +50MB | Moderate | <15ms |
| Model (Phi-3.5-mini) | ~4GB | High | Variable |
| **Total** | **~4.15GB** | **Variable** | **<25ms** |

## Rate Limiting

Default rate limits (configurable):

- Chat completions: 60 requests/minute
- Narrative analysis: 120 requests/minute
- Configuration updates: 10 requests/minute

## WebSocket Support

Real-time narrative intelligence streaming:

```javascript
const ws = new WebSocket('ws://127.0.0.1:11435/narrative/stream');

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Narrative update:', data);
};
```

## Security Considerations

- All endpoints are HTTP-only by default (use reverse proxy for HTTPS)
- No authentication required for local usage
- Input validation and sanitization enabled
- Rate limiting available for production use
- CORS headers configurable

## Troubleshooting

### Common Issues

1. **Model Not Found**
   - Check `shimmy.exe list` for available models
   - Use `shimmy.exe discover` to find models
   - Verify model path and permissions

2. **High Memory Usage**
   - Reduce `quality_tier` in adaptive settings
   - Disable unused narrative systems
   - Use smaller models

3. **Slow Response Times**
   - Enable caching in configuration
   - Reduce `max_analysis_depth`
   - Use faster hardware

### Debug Endpoints

```bash
# System health check
curl http://127.0.0.1:11435/health

# Detailed diagnostics
curl http://127.0.0.1:11435/debug/stats

# Memory usage
curl http://127.0.0.1:11435/debug/memory
```

## Version Information

- **Shimmy-DS Version**: 1.3.3+recursive-intelligence
- **API Version**: v1 (OpenAI compatible)
- **Narrative Intelligence Version**: 2.0
- **Compilation Status**: ✅ 100% successful
- **Modules**: 75 files, 35,162 lines of code
- **Test Coverage**: Comprehensive (200+ tests)

## Support

For technical support:
- GitHub Issues: [Shimmy-DS Issues](https://github.com/zamfir70/Shimmy-DS/issues)
- Documentation: See `/docs` directory
- System Audit: See `SYSTEM_AUDIT.txt`
- Deployment Guide: See `DEPLOYMENT.md`