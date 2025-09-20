# Shimmy-DS Function Reference

Quick reference for accessing ALL Shimmy-DS narrative intelligence functions within your programs.

## 🎯 How to Access Functions

### Option 1: REST API (Any Language)
```python
import requests

# Start Shimmy-DS server first
# ./target/release/shimmy.exe serve

base_url = "http://127.0.0.1:11435"
```

### Option 2: Direct Rust Module Import
```rust
use shimmy::{
    narrative_dna::NarrativeDNATracker,
    character_consistency::CharacterConsistencyEngine,
    // ... other modules
};
```

## 📋 All Available Functions

### NARRATIVE DNA (CAPR Loops)
```python
# Get CAPR loop analysis
response = requests.get(f"{base_url}/narrative/analyze")
capr_data = response.json()["active_patterns"]

# Filter for CAPR loops
capr_loops = [p for p in capr_data if p.get("type") == "CAPR_loop"]
```

### CHARACTER CONSISTENCY
```python
# Check character consistency score
response = requests.get(f"{base_url}/narrative/analyze")
character_score = response.json()["narrative_health"]["system_scores"]["character_consistency"]

# Validate character action
requests.post(f"{base_url}/v1/chat/completions", json={
    "model": "analysis",
    "messages": [
        {"role": "system", "content": "Check if this action is consistent with Elena's character."},
        {"role": "user", "content": "Elena suddenly started laughing maniacally"}
    ]
})
```

### CONSTRAINT SPACE
```python
# Get narrative possibilities
response = requests.get(f"{base_url}/narrative/analyze")
constraints = response.json()["constraints"]

freedom_score = constraints["freedom_score"]  # 0.0-1.0
active_constraints = constraints["active"]
pressure_points = constraints["pressure_points"]
```

### ENGAGEMENT TRACKING
```python
# Get reader engagement metrics
response = requests.get(f"{base_url}/narrative/analyze")
engagement = response.json()["engagement_metrics"]

curiosity_score = engagement["curiosity_score"]
investment_score = engagement["investment_score"]
tension_level = engagement["tension_level"]
```

### DRIFT STABILIZATION
```python
# Check for narrative drift
response = requests.get(f"{base_url}/narrative/analyze")
drift_score = 1.0 - response.json()["narrative_health"]["system_scores"]["drift_stability"]

drift_detected = drift_score > 0.3
```

### MULTI-LEVEL RECURSION
```python
# Analyze recursive patterns
response = requests.get(f"{base_url}/narrative/analyze")
recursion_strength = response.json()["narrative_health"]["system_scores"]["recursion_tracker"]

# Check for cross-scale patterns
patterns = response.json()["active_patterns"]
recursive_patterns = [p for p in patterns if "recursive" in p.get("description", "").lower()]
```

### ADAPTIVE INTELLIGENCE
```python
# Get adaptive status
response = requests.get(f"{base_url}/narrative/report")
adaptive = response.json()["adaptive_intelligence"]

adapt_iq = adaptive["adapt_iq"]
qualitier = adaptive["qualitier"]
obli_select = adaptive["obli_select"]
profile_mesh = adaptive["profile_mesh"]
```

### OBLIGATION MANAGEMENT
```python
# Configure smart obligations
config = {
    "obli_select": {
        "max_obligations": 5,
        "urgency_weight": 0.3,
        "salience_weight": 0.4
    }
}
requests.post(f"{base_url}/narrative/config", json=config)
```

### CONFIGURATION
```python
# Configure all systems
config = {
    "assertiveness_level": "moderate",  # passive/moderate/active
    "systems_enabled": {
        "dna_tracking": True,
        "constraint_modeling": True,
        "recursion_tracking": True,
        "character_consistency": True,
        "engagement_loops": True,
        "drift_stabilization": True
    },
    "sensitivity_thresholds": {
        "constraint_pressure": 0.7,
        "character_drift": 0.8,
        "unresolved_loops": 0.6,
        "engagement_drops": 0.7,
        "pattern_breaks": 0.5
    },
    "adaptive_settings": {
        "adaptation_enabled": True,
        "learning_rate": 0.15,
        "quality_tier": "Enhanced"
    }
}

requests.post(f"{base_url}/narrative/config", json=config)
```

## 🔥 Complete Usage Example

```python
class NarrativeIntelligence:
    def __init__(self):
        self.base_url = "http://127.0.0.1:11435"

    def analyze_story(self, text):
        """Get complete narrative analysis"""

        # Configure for story analysis
        self.configure({
            "assertiveness_level": "moderate",
            "sensitivity_thresholds": {"character_drift": 0.9}
        })

        # Get full analysis
        analysis = requests.get(f"{self.base_url}/narrative/analyze").json()

        return {
            "health": analysis["narrative_health"]["overall_score"],
            "capr_loops": len([p for p in analysis["active_patterns"]
                             if p.get("type") == "CAPR_loop"]),
            "character_consistency": analysis["narrative_health"]["system_scores"]["character_consistency"],
            "engagement": analysis["engagement_metrics"]["curiosity_score"],
            "freedom": analysis["constraints"]["freedom_score"],
            "recommendations": self.get_recommendations(analysis)
        }

    def generate_enhanced(self, prompt, max_tokens=1000):
        """Generate text with narrative intelligence"""

        # Analyze prompt first
        analysis = self.analyze_story(prompt)

        # Generate with narrative intelligence automatically applied
        response = requests.post(f"{self.base_url}/v1/chat/completions", json={
            "model": "default",
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": max_tokens
        })

        generated = response.json()["choices"][0]["message"]["content"]

        # Post-analyze
        final_analysis = self.analyze_story(generated)

        return {
            "text": generated,
            "before_health": analysis["health"],
            "after_health": final_analysis["health"],
            "improvement": final_analysis["health"] - analysis["health"]
        }

    def configure(self, config):
        """Configure narrative intelligence"""
        requests.post(f"{self.base_url}/narrative/config", json=config)

    def get_recommendations(self, analysis):
        """Generate recommendations based on analysis"""
        recs = []

        if analysis["narrative_health"]["overall_score"] < 0.7:
            recs.append("Improve overall narrative coherence")

        if analysis["constraints"]["freedom_score"] < 0.5:
            recs.append("Story has limited paths - consider opening possibilities")

        if analysis["engagement_metrics"]["tension_level"] < 0.6:
            recs.append("Increase dramatic tension")

        return recs

# Usage
ni = NarrativeIntelligence()

# Analyze existing story
story_analysis = ni.analyze_story("Elena looked into the mirror and saw another world...")
print(f"Story health: {story_analysis['health']:.2f}")

# Generate enhanced content
result = ni.generate_enhanced("Continue Elena's mirror story with recursive themes")
print(f"Generated text: {result['text']}")
print(f"Health improvement: {result['improvement']:+.2f}")
```

## 🎯 Function Categories

| Category | Functions | Purpose |
|----------|-----------|---------|
| **Core Analysis** | `/narrative/analyze`, `/narrative/report` | Get current narrative state |
| **Configuration** | `/narrative/config` | Configure all systems |
| **Generation** | `/v1/chat/completions`, `/v1/completions` | Generate with intelligence |
| **CAPR Tracking** | Pattern analysis in responses | Track narrative loops |
| **Character Engine** | Character consistency scores | Maintain character integrity |
| **Constraint Space** | Freedom scores, pressure points | Map story possibilities |
| **Engagement** | Curiosity, investment, tension metrics | Monitor reader engagement |
| **Drift Protection** | Stability scores, drift detection | Prevent narrative drift |
| **Recursion** | Cross-scale pattern detection | Find recursive themes |
| **Adaptive Intelligence** | AdaptIQ, Qualitier, ObliSelect, ProfileMesh | Personalized intelligence |

## 🚀 Integration Patterns

### Pattern 1: Analysis + Generation
```python
# 1. Analyze current state
analysis = get_narrative_analysis()

# 2. Generate with intelligence applied
text = generate_with_config(prompt, analysis)

# 3. Post-analyze results
final_analysis = get_narrative_analysis()
```

### Pattern 2: Validation + Enhancement
```python
# 1. Validate proposed action
validation = validate_story_action("Elena jumps through mirror")

# 2. Enhance if needed
if not validation["is_valid"]:
    enhanced_action = enhance_action_with_intelligence(action)

# 3. Apply validated action
result = apply_narrative_action(enhanced_action)
```

### Pattern 3: Continuous Monitoring
```python
# 1. Set up monitoring
configure_narrative_intelligence({"assertiveness_level": "moderate"})

# 2. Generate content
for chapter in story_chapters:
    text = generate_chapter(chapter)

    # 3. Monitor health
    health = get_narrative_health()
    if health < 0.6:
        apply_corrective_measures()
```

This reference shows you exactly how to access and use every single narrative intelligence function within your own programs!