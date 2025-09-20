#!/usr/bin/env python3
"""
Examples of how to configure and pass parameters to Shimmy-DS
narrative intelligence functions for different use cases.
"""

import requests
import json
from typing import Dict, Any, List

def configure_for_creative_writing():
    """Configuration optimized for creative fiction writing"""
    return {
        "assertiveness_level": "moderate",
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

def configure_for_technical_writing():
    """Configuration optimized for technical/structured writing"""
    return {
        "assertiveness_level": "active",
        "systems_enabled": {
            "constraint_modeling": True,
            "recursion_tracking": False,  # Less important for technical
            "character_consistency": False,
            "engagement_loops": True,
            "drift_stabilization": True
        },
        "sensitivity_thresholds": {
            "constraint_pressure": 0.9,
            "unresolved_loops": 0.8,
            "engagement_drops": 0.6
        },
        "adaptive_settings": {
            "quality_tier": "Premium"
        }
    }

def configure_for_interactive_fiction():
    """Configuration optimized for interactive/branching narratives"""
    return {
        "assertiveness_level": "active",
        "systems_enabled": {
            "dna_tracking": True,
            "constraint_modeling": True,
            "recursion_tracking": True,
            "character_consistency": True,
            "engagement_loops": True,
            "drift_stabilization": True
        },
        "sensitivity_thresholds": {
            "constraint_pressure": 0.6,  # Allow more freedom for branching
            "character_drift": 0.9,      # Strict character consistency
            "unresolved_loops": 0.5,     # Allow open loops for choices
            "engagement_drops": 0.8      # High engagement requirement
        }
    }

def generate_with_parameters(prompt: str,
                           config: Dict[str, Any],
                           generation_params: Dict[str, Any] = None):
    """Generate text with specific narrative intelligence parameters"""

    base_url = "http://127.0.0.1:11435"

    # Configure narrative intelligence
    config_response = requests.post(f"{base_url}/narrative/config", json=config)
    print(f"Configuration applied: {config_response.status_code}")

    # Set default generation parameters
    if generation_params is None:
        generation_params = {
            "max_tokens": 1000,
            "temperature": 0.7,
            "top_p": 0.9
        }

    # Generate with configured narrative intelligence
    response = requests.post(f"{base_url}/v1/chat/completions", json={
        "model": "default",
        "messages": [{"role": "user", "content": prompt}],
        **generation_params
    })

    result = response.json()

    # Get narrative analysis
    analysis_response = requests.get(f"{base_url}/narrative/analyze")
    analysis = analysis_response.json()

    return {
        "text": result["choices"][0]["message"]["content"],
        "analysis": analysis,
        "config_used": config,
        "generation_params": generation_params
    }

def example_use_cases():
    """Demonstrate different parameter configurations for various use cases"""

    print("🎭 Shimmy-DS Parameter Configuration Examples")
    print("=" * 60)

    # Example 1: Creative Fiction
    print("\n1. Creative Fiction Writing")
    print("-" * 30)
    creative_result = generate_with_parameters(
        prompt="Write a fantasy story about a library where books rewrite themselves based on the reader's deepest fears",
        config=configure_for_creative_writing(),
        generation_params={"max_tokens": 1500, "temperature": 0.8}
    )

    print(f"Health Score: {creative_result['analysis']['narrative_health']['overall_score']:.2f}")
    print(f"Active Patterns: {len(creative_result['analysis']['active_patterns'])}")
    print(f"Character Consistency: {creative_result['analysis']['narrative_health']['system_scores'].get('character_consistency', 'N/A')}")

    # Example 2: Technical Documentation
    print("\n2. Technical Documentation")
    print("-" * 30)
    technical_result = generate_with_parameters(
        prompt="Explain how to implement a recursive algorithm for tree traversal with error handling and optimization techniques",
        config=configure_for_technical_writing(),
        generation_params={"max_tokens": 1200, "temperature": 0.6}
    )

    print(f"Health Score: {technical_result['analysis']['narrative_health']['overall_score']:.2f}")
    print(f"Constraint Pressure: {technical_result['analysis']['constraints'].get('freedom_score', 'N/A')}")

    # Example 3: Interactive Fiction
    print("\n3. Interactive Fiction")
    print("-" * 30)
    interactive_result = generate_with_parameters(
        prompt="Create a branching story scenario where the protagonist must choose between three doors, each leading to a different recursive challenge",
        config=configure_for_interactive_fiction(),
        generation_params={"max_tokens": 1800, "temperature": 0.75}
    )

    print(f"Health Score: {interactive_result['analysis']['narrative_health']['overall_score']:.2f}")
    print(f"Engagement Score: {interactive_result['analysis']['engagement_metrics'].get('curiosity_score', 'N/A')}")

    # Example 4: Custom Fine-tuned Configuration
    print("\n4. Custom Fine-tuned Configuration")
    print("-" * 30)

    custom_config = {
        "assertiveness_level": "moderate",
        "sensitivity_thresholds": {
            "constraint_pressure": 0.85,
            "character_drift": 0.7,
            "unresolved_loops": 0.9
        },
        "adaptive_settings": {
            "adaptation_enabled": True,
            "quality_tier": "Enhanced"
        }
    }

    custom_result = generate_with_parameters(
        prompt="Write a story that demonstrates recursive narrative techniques while maintaining reader engagement",
        config=custom_config,
        generation_params={"max_tokens": 1000, "temperature": 0.7, "top_p": 0.85}
    )

    print(f"Health Score: {custom_result['analysis']['narrative_health']['overall_score']:.2f}")
    print(f"Systems Active: {len([k for k, v in custom_result['analysis']['narrative_health']['system_scores'].items() if v > 0.5])}")

def parameter_reference():
    """Show all available parameters and their effects"""

    print("\n📋 Parameter Reference Guide")
    print("=" * 60)

    parameters = {
        "assertiveness_level": {
            "options": ["passive", "moderate", "active"],
            "description": "How actively the system intervenes in generation",
            "passive": "Minimal intervention, preserves user intent",
            "moderate": "Balanced intervention with suggestions",
            "active": "Strong intervention for narrative quality"
        },
        "sensitivity_thresholds": {
            "constraint_pressure": "0.0-1.0: How sensitive to narrative constraints",
            "character_drift": "0.0-1.0: How sensitive to character inconsistencies",
            "unresolved_loops": "0.0-1.0: How sensitive to unresolved plot threads",
            "engagement_drops": "0.0-1.0: How sensitive to engagement drops",
            "pattern_breaks": "0.0-1.0: How sensitive to narrative pattern breaks"
        },
        "systems_enabled": {
            "dna_tracking": "CAPR loop analysis (Contradiction→Action→Pressure→Return)",
            "constraint_modeling": "Narrative possibility space mapping",
            "recursion_tracking": "Cross-scale pattern detection",
            "character_consistency": "Character voice and development tracking",
            "engagement_loops": "Reader psychology and engagement monitoring",
            "drift_stabilization": "Long-term narrative coherence protection"
        },
        "adaptive_settings": {
            "adaptation_enabled": "Enable cross-session learning",
            "learning_rate": "0.0-1.0: How quickly system adapts to patterns",
            "quality_tier": "Minimal/Standard/Enhanced/Premium: Resource allocation"
        }
    }

    for category, details in parameters.items():
        print(f"\n{category.upper()}:")
        if isinstance(details, dict):
            for key, value in details.items():
                if key in ["options", "description"]:
                    print(f"  {key}: {value}")
                else:
                    print(f"  • {key}: {value}")
        print()

def main():
    """Run all parameter examples"""
    try:
        example_use_cases()
        parameter_reference()

        print("\n✅ All parameter examples completed!")
        print("\n💡 Tips:")
        print("• Use 'passive' for preserving user style")
        print("• Use 'active' for maximum narrative intelligence")
        print("• Adjust thresholds based on content type")
        print("• Enable adaptive settings for personalization")

    except requests.exceptions.ConnectionError:
        print("❌ Error: Shimmy-DS server not running")
        print("Start with: ./target/release/shimmy.exe serve")
    except Exception as e:
        print(f"❌ Error: {e}")

if __name__ == "__main__":
    main()