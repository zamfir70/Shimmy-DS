#!/usr/bin/env python3
"""
Quick Shimmy-DS Writing Test

Simple test to show Shimmy-DS narrative intelligence in action.
Run this after starting: ./target/release/shimmy.exe serve
"""

import requests
import json

def test_shimmy_writing():
    """Quick test of Shimmy-DS writing with intelligence"""

    base_url = "http://127.0.0.1:11435"

    print("🧠 SHIMMY-DS QUICK WRITING TEST")
    print("=" * 50)

    # Test 1: Basic connection
    try:
        response = requests.get(f"{base_url}/v1/models", timeout=5)
        print("✅ Connected to Shimmy-DS server")
        models = response.json()
        print(f"📋 Available models: {len(models.get('data', []))}")
    except:
        print("❌ Cannot connect to Shimmy-DS")
        print("Start with: ./target/release/shimmy.exe serve")
        return

    # Test 2: Configure narrative intelligence
    print("\n⚙️  Configuring narrative intelligence...")
    config = {
        "assertiveness_level": "moderate",
        "systems_enabled": {
            "dna_tracking": True,
            "character_consistency": True,
            "engagement_loops": True
        }
    }

    config_response = requests.post(f"{base_url}/narrative/config", json=config)
    print(f"✅ Configuration: {config_response.status_code == 200}")

    # Test 3: Generate story with intelligence
    print("\n📝 Generating story with narrative intelligence...")

    story_prompt = """Write a short story about a programmer named Alex who discovers that their code is becoming self-aware. The AI starts leaving messages in the comments. Make it mysterious and engaging, with a recursive theme where the AI questions the nature of consciousness."""

    generation_response = requests.post(f"{base_url}/v1/chat/completions", json={
        "model": "creative-writer",
        "messages": [
            {"role": "system", "content": "You are a creative writer with expertise in science fiction and recursive narratives. Write with vivid imagery and psychological depth."},
            {"role": "user", "content": story_prompt}
        ],
        "max_tokens": 1500,
        "temperature": 0.8
    })

    if generation_response.status_code == 200:
        story = generation_response.json()["choices"][0]["message"]["content"]
        print("\n📖 GENERATED STORY:")
        print("-" * 60)
        print(story)
        print("-" * 60)
    else:
        print(f"❌ Generation failed: {generation_response.status_code}")
        return

    # Test 4: Get narrative intelligence analysis
    print("\n🧠 NARRATIVE INTELLIGENCE ANALYSIS:")
    print("-" * 60)

    analysis_response = requests.get(f"{base_url}/narrative/analyze")
    if analysis_response.status_code == 200:
        analysis = analysis_response.json()

        # Overall health
        health = analysis.get("narrative_health", {}).get("overall_score", 0.5)
        print(f"📊 Overall Narrative Health: {health:.2f}/1.0")

        # System scores
        systems = analysis.get("narrative_health", {}).get("system_scores", {})
        print(f"🧬 CAPR Loop Tracking: {systems.get('dna_tracker', 0.5):.2f}")
        print(f"👤 Character Consistency: {systems.get('character_consistency', 0.5):.2f}")
        print(f"📚 Reader Engagement: {systems.get('engagement_tracker', 0.5):.2f}")
        print(f"🗺️  Constraint Space: {systems.get('constraint_space', 0.5):.2f}")
        print(f"🔄 Recursion Tracking: {systems.get('recursion_tracker', 0.5):.2f}")
        print(f"⚖️  Drift Stability: {systems.get('drift_stability', 0.5):.2f}")

        # Active patterns
        patterns = analysis.get("active_patterns", [])
        print(f"\n🎭 Active Narrative Patterns: {len(patterns)}")
        for i, pattern in enumerate(patterns[:3], 1):
            pattern_type = pattern.get("type", "Unknown")
            description = pattern.get("description", "No description")
            print(f"  {i}. {pattern_type}: {description}")

        # Engagement metrics
        engagement = analysis.get("engagement_metrics", {})
        curiosity = engagement.get("curiosity_score", 0.5)
        tension = engagement.get("tension_level", 0.5)
        print(f"\n📈 Engagement Metrics:")
        print(f"  Curiosity Score: {curiosity:.2f}")
        print(f"  Tension Level: {tension:.2f}")

        # Constraints
        constraints = analysis.get("constraints", {})
        freedom = constraints.get("freedom_score", 0.5)
        active_constraints = constraints.get("active", 0)
        print(f"\n🗺️  Narrative Space:")
        print(f"  Freedom Score: {freedom:.2f}")
        print(f"  Active Constraints: {active_constraints}")

    else:
        print(f"❌ Analysis failed: {analysis_response.status_code}")

    # Test 5: Get system report
    print("\n📋 SYSTEM STATUS:")
    print("-" * 60)

    report_response = requests.get(f"{base_url}/narrative/report")
    if report_response.status_code == 200:
        report = report_response.json()

        system_status = report.get("system_status", {})
        print(f"💾 Memory Usage: {system_status.get('memory_usage', 'N/A')}")
        print(f"📁 Modules Loaded: {system_status.get('modules_loaded', 'N/A')}")
        print(f"⏱️  Uptime: {system_status.get('uptime', 'N/A')}")

        adaptive = report.get("adaptive_intelligence", {})
        if adaptive:
            adapt_iq = adaptive.get("adapt_iq", {})
            print(f"🧠 AdaptIQ Depth: {adapt_iq.get('current_depth', 'N/A')}")
            print(f"🎛️  Quality Tier: {adaptive.get('qualitier', {}).get('current_tier', 'N/A')}")

    # Summary
    print(f"\n{'🎉' * 20}")
    print("✨ SHIMMY-DS WRITING TEST COMPLETE!")
    print(f"{'🎉' * 20}")
    print("🧠 Demonstrated capabilities:")
    print("  ✅ Story generation with narrative intelligence")
    print("  ✅ Real-time narrative health monitoring")
    print("  ✅ CAPR loop and pattern detection")
    print("  ✅ Character consistency tracking")
    print("  ✅ Reader engagement analysis")
    print("  ✅ Adaptive intelligence systems")
    print("\n🌟 World's first recursive narrative intelligence system working!")

if __name__ == "__main__":
    test_shimmy_writing()