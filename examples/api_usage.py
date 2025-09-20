#!/usr/bin/env python3
"""
Examples of how to use Shimmy-DS narrative intelligence functions
via API calls with different parameters and configurations.
"""

import requests
import json
from typing import Dict, List, Any

class ShimmyDSInterface:
    """Interface to Shimmy-DS narrative intelligence functions"""

    def __init__(self, base_url="http://127.0.0.1:11435"):
        self.base_url = base_url.rstrip('/')

    def generate_with_narrative_intelligence(self,
                                           prompt: str,
                                           max_tokens: int = 1000,
                                           temperature: float = 0.7,
                                           narrative_config: Dict = None) -> Dict[str, Any]:
        """Generate text with full narrative intelligence"""

        # Configure narrative intelligence if provided
        if narrative_config:
            self.configure_narrative(narrative_config)

        # Generate with narrative intelligence automatically applied
        response = requests.post(f"{self.base_url}/v1/chat/completions", json={
            "model": "default",
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": max_tokens,
            "temperature": temperature
        })

        result = response.json()

        # Get narrative analysis of the generation
        analysis = self.get_narrative_analysis()

        return {
            "generated_text": result["choices"][0]["message"]["content"],
            "narrative_analysis": analysis,
            "generation_metadata": result.get("usage", {})
        }

    def get_narrative_analysis(self) -> Dict[str, Any]:
        """Get current narrative intelligence analysis"""
        response = requests.get(f"{self.base_url}/narrative/analyze")
        return response.json()

    def configure_narrative(self, config: Dict[str, Any]) -> Dict[str, Any]:
        """Configure narrative intelligence parameters"""
        response = requests.post(f"{self.base_url}/narrative/config", json=config)
        return response.json()

    def get_system_report(self) -> Dict[str, Any]:
        """Get comprehensive system status"""
        response = requests.get(f"{self.base_url}/narrative/report")
        return response.json()

    def test_narrative_functions(self):
        """Test all narrative intelligence functions with different parameters"""

        print("🧠 Testing Narrative Intelligence Functions")
        print("=" * 50)

        # Test 1: Basic generation with default settings
        print("\n1. Basic Generation (Default Settings)")
        result1 = self.generate_with_narrative_intelligence(
            "Write a story about a character who discovers mirrors that show different realities"
        )
        print(f"   Health Score: {result1['narrative_analysis']['narrative_health']['overall_score']:.2f}")
        print(f"   Active Patterns: {len(result1['narrative_analysis']['active_patterns'])}")

        # Test 2: High assertiveness configuration
        print("\n2. High Assertiveness Configuration")
        high_assertiveness_config = {
            "assertiveness_level": "active",
            "sensitivity_thresholds": {
                "constraint_pressure": 0.9,
                "character_drift": 0.95,
                "unresolved_loops": 0.8
            }
        }
        result2 = self.generate_with_narrative_intelligence(
            "Continue the mirror story with deeper recursive elements",
            narrative_config=high_assertiveness_config
        )
        print(f"   Health Score: {result2['narrative_analysis']['narrative_health']['overall_score']:.2f}")
        print(f"   System Scores: {result2['narrative_analysis']['narrative_health']['system_scores']}")

        # Test 3: Creative writing mode
        print("\n3. Creative Writing Mode")
        creative_config = {
            "assertiveness_level": "moderate",
            "systems_enabled": {
                "dna_tracking": True,
                "constraint_modeling": True,
                "character_consistency": True,
                "engagement_loops": True
            }
        }
        result3 = self.generate_with_narrative_intelligence(
            "Write an interactive fiction scenario where choices affect the story",
            max_tokens=1500,
            temperature=0.8,
            narrative_config=creative_config
        )
        print(f"   Engagement Score: {result3['narrative_analysis']['engagement_metrics']['curiosity_score']:.2f}")
        print(f"   Constraints: {result3['narrative_analysis']['constraints']['active']} active")

        # Test 4: Get comprehensive system report
        print("\n4. System Status Report")
        report = self.get_system_report()
        print(f"   Modules Loaded: {report['system_status']['modules_loaded']}")
        print(f"   Memory Usage: {report['system_status']['memory_usage']}")
        print(f"   Adaptive Intelligence: {report['adaptive_intelligence']['adapt_iq']['current_depth']}")

        # Test 5: Different narrative approaches
        print("\n5. Testing Different Narrative Approaches")

        approaches = [
            ("Long-form", {"assertiveness_level": "passive"}, 2000),
            ("Interactive", {"assertiveness_level": "moderate"}, 1000),
            ("Structured", {"assertiveness_level": "active"}, 800)
        ]

        for name, config, tokens in approaches:
            result = self.generate_with_narrative_intelligence(
                f"Write a {name.lower()} story about recursive time loops",
                max_tokens=tokens,
                narrative_config=config
            )
            health = result['narrative_analysis']['narrative_health']['overall_score']
            patterns = len(result['narrative_analysis']['active_patterns'])
            print(f"   {name}: Health={health:.2f}, Patterns={patterns}")

def main():
    """Example usage of all narrative intelligence functions"""

    # Create interface
    shimmy = ShimmyDSInterface()

    try:
        # Test all functions
        shimmy.test_narrative_functions()

        print("\n" + "=" * 50)
        print("✅ All narrative intelligence functions tested successfully!")

    except requests.exceptions.ConnectionError:
        print("❌ Error: Could not connect to Shimmy-DS server")
        print("Please ensure shimmy.exe serve is running on port 11435")
    except Exception as e:
        print(f"❌ Error: {e}")

if __name__ == "__main__":
    main()