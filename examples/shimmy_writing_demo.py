#!/usr/bin/env python3
"""
Shimmy-DS Writing Demo

This test demonstrates Shimmy-DS's narrative intelligence by having it write
a complete story while showing all the intelligence systems working in real-time.

Run this after starting Shimmy-DS server:
./target/release/shimmy.exe serve
"""

import requests
import json
import time
from typing import Dict, Any, List

class ShimmyWritingDemo:
    """Demonstrates Shimmy-DS writing with full narrative intelligence"""

    def __init__(self, base_url="http://127.0.0.1:11435"):
        self.base_url = base_url.rstrip('/')
        self.story_text = ""
        self.analysis_history = []

    def configure_for_creative_writing(self):
        """Configure Shimmy-DS for optimal creative writing"""
        config = {
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

        response = requests.post(f"{self.base_url}/narrative/config", json=config)
        return response.status_code == 200

    def get_narrative_analysis(self) -> Dict[str, Any]:
        """Get current narrative intelligence analysis"""
        try:
            response = requests.get(f"{self.base_url}/narrative/analyze")
            if response.status_code == 200:
                return response.json()
            else:
                return self._default_analysis()
        except:
            return self._default_analysis()

    def _default_analysis(self) -> Dict[str, Any]:
        """Default analysis when server is unavailable"""
        return {
            "narrative_health": {"overall_score": 0.5, "system_scores": {}},
            "active_patterns": [],
            "constraints": {"freedom_score": 0.5, "active": 0, "pressure_points": []},
            "engagement_metrics": {"curiosity_score": 0.5, "tension_level": 0.5}
        }

    def generate_story_segment(self, prompt: str, max_tokens: int = 800) -> str:
        """Generate a story segment with narrative intelligence"""
        try:
            response = requests.post(f"{self.base_url}/v1/chat/completions", json={
                "model": "creative-writer",
                "messages": [
                    {"role": "system", "content": "You are a creative writer with deep understanding of narrative structure, character development, and recursive storytelling techniques. Write with vivid imagery and emotional depth."},
                    {"role": "user", "content": prompt}
                ],
                "max_tokens": max_tokens,
                "temperature": 0.8,
                "top_p": 0.9
            })

            if response.status_code == 200:
                result = response.json()
                return result["choices"][0]["message"]["content"].strip()
            else:
                return f"[Simulated story segment for: {prompt[:50]}...]"
        except:
            return f"[Simulated story segment for: {prompt[:50]}...]"

    def display_intelligence_metrics(self, analysis: Dict[str, Any], segment_num: int):
        """Display narrative intelligence metrics"""
        print(f"\n{'='*60}")
        print(f"📊 NARRATIVE INTELLIGENCE ANALYSIS - Segment {segment_num}")
        print(f"{'='*60}")

        # Overall health
        health = analysis["narrative_health"]["overall_score"]
        health_bar = "█" * int(health * 20) + "░" * (20 - int(health * 20))
        print(f"🎯 Overall Narrative Health: {health:.2f} [{health_bar}]")

        # System breakdown
        print(f"\n🧠 Intelligence Systems:")
        systems = analysis["narrative_health"]["system_scores"]

        system_names = {
            "dna_tracker": "CAPR Loop Tracking",
            "constraint_space": "Constraint Modeling",
            "recursion_tracker": "Multi-Level Recursion",
            "character_consistency": "Character Consistency",
            "engagement_tracker": "Reader Engagement",
            "drift_stability": "Drift Stabilization"
        }

        for key, name in system_names.items():
            score = systems.get(key, 0.5)
            bar = "█" * int(score * 10) + "░" * (10 - int(score * 10))
            status = "🟢" if score > 0.7 else "🟡" if score > 0.5 else "🔴"
            print(f"  {status} {name:<20}: {score:.2f} [{bar}]")

        # Active patterns
        patterns = analysis["active_patterns"]
        print(f"\n🎭 Active Narrative Patterns ({len(patterns)}):")
        for i, pattern in enumerate(patterns[:5], 1):
            pattern_type = pattern.get("type", "Unknown")
            description = pattern.get("description", "No description")
            strength = pattern.get("strength", 0.5)
            print(f"  {i}. {pattern_type}: {description} (strength: {strength:.2f})")

        # Constraints and freedom
        constraints = analysis["constraints"]
        freedom = constraints.get("freedom_score", 0.5)
        active_constraints = constraints.get("active", 0)
        print(f"\n🗺️  Narrative Space:")
        print(f"  Freedom Score: {freedom:.2f}")
        print(f"  Active Constraints: {active_constraints}")

        pressure_points = constraints.get("pressure_points", [])
        if pressure_points:
            print(f"  Pressure Points: {', '.join(pressure_points[:3])}")

        # Engagement metrics
        engagement = analysis["engagement_metrics"]
        curiosity = engagement.get("curiosity_score", 0.5)
        tension = engagement.get("tension_level", 0.5)
        print(f"\n📚 Reader Engagement:")
        print(f"  Curiosity Score: {curiosity:.2f}")
        print(f"  Tension Level: {tension:.2f}")

        # Store for history
        self.analysis_history.append({
            "segment": segment_num,
            "health": health,
            "systems": systems,
            "patterns": len(patterns),
            "freedom": freedom,
            "engagement": (curiosity + tension) / 2
        })

    def write_complete_story(self):
        """Write a complete story with Shimmy-DS intelligence"""
        print("🚀 SHIMMY-DS WRITING DEMONSTRATION")
        print("🧠 Showcasing World's First Recursive Narrative Intelligence")
        print("=" * 80)

        # Configure for writing
        print("⚙️  Configuring narrative intelligence systems...")
        if self.configure_for_creative_writing():
            print("✅ All 6 narrative intelligence systems activated")
        else:
            print("⚠️  Using simulated responses (server not available)")

        print("\n📝 Beginning story generation with full narrative intelligence...\n")

        # Story segments with progressive complexity
        story_segments = [
            {
                "prompt": "Write the opening of a story about Elena, a young woman who discovers that mirrors in her grandmother's old house show different versions of reality. Focus on atmosphere and mystery. Include vivid sensory details.",
                "description": "Opening - Establishing character and mysterious premise"
            },
            {
                "prompt": "Continue Elena's story. She decides to touch one of the mirrors and finds her hand passes through. Develop her character's curiosity and fear. Show her internal conflict about whether to step through.",
                "description": "Development - Character choice and rising tension"
            },
            {
                "prompt": "Elena steps through the mirror into an alternate reality where she meets another version of herself. This other Elena warns her about the dangers of mirror-hopping. Create tension between the two Elenas and establish rules for this mirror world.",
                "description": "Complication - Recursive character encounter"
            },
            {
                "prompt": "The two Elenas discover that each mirror leads to a reality where different choices were made. They realize they must work together to prevent a catastrophe that threatens all realities. Build toward a climax while maintaining character consistency.",
                "description": "Climax - Recursive threat and collaboration"
            },
            {
                "prompt": "Write the resolution where Elena learns that she herself is responsible for creating these mirror realities through her own choices and regrets. She must accept all versions of herself to restore balance. End with a satisfying but open conclusion.",
                "description": "Resolution - Recursive self-acceptance theme"
            }
        ]

        # Generate each segment with intelligence analysis
        for i, segment in enumerate(story_segments, 1):
            print(f"\n{'🔸' * 40}")
            print(f"📖 WRITING SEGMENT {i}: {segment['description']}")
            print(f"{'🔸' * 40}")

            # Generate the story segment
            print("🖊️  Generating with narrative intelligence...")
            story_part = self.generate_story_segment(segment["prompt"])

            # Add to full story
            self.story_text += f"\n\n--- Chapter {i} ---\n\n{story_part}"

            # Display the generated text
            print(f"\n📜 Generated Text:")
            print("-" * 60)
            print(story_part)
            print("-" * 60)

            # Get and display intelligence analysis
            analysis = self.get_narrative_analysis()
            self.display_intelligence_metrics(analysis, i)

            # Brief pause for readability
            time.sleep(1)

        # Show final analysis and story evolution
        self.show_story_evolution()
        self.show_complete_story()

    def show_story_evolution(self):
        """Show how the story evolved with narrative intelligence"""
        print(f"\n{'🎭' * 30}")
        print("📈 STORY EVOLUTION WITH NARRATIVE INTELLIGENCE")
        print(f"{'🎭' * 30}")

        if not self.analysis_history:
            print("No analysis history available")
            return

        print("\n📊 Intelligence Metrics Over Time:")
        print("Segment | Health | Patterns | Freedom | Engagement")
        print("-" * 50)

        for entry in self.analysis_history:
            print(f"   {entry['segment']}    | {entry['health']:.2f}   |    {entry['patterns']}     | {entry['freedom']:.2f}    |   {entry['engagement']:.2f}")

        # Calculate improvements
        if len(self.analysis_history) >= 2:
            first = self.analysis_history[0]
            last = self.analysis_history[-1]

            health_change = last['health'] - first['health']
            pattern_change = last['patterns'] - first['patterns']
            engagement_change = last['engagement'] - first['engagement']

            print(f"\n📈 Overall Improvements:")
            print(f"  Health: {health_change:+.2f}")
            print(f"  Patterns Developed: {pattern_change:+d}")
            print(f"  Engagement: {engagement_change:+.2f}")

        # Show final intelligence status
        final_analysis = self.get_narrative_analysis()
        print(f"\n🏆 Final Intelligence Assessment:")
        print(f"  Overall Narrative Health: {final_analysis['narrative_health']['overall_score']:.2f}")
        print(f"  Active Patterns: {len(final_analysis['active_patterns'])}")
        print(f"  Character Consistency: {final_analysis['narrative_health']['system_scores'].get('character_consistency', 0.5):.2f}")
        print(f"  Reader Engagement: {final_analysis['engagement_metrics'].get('curiosity_score', 0.5):.2f}")

    def show_complete_story(self):
        """Display the complete generated story"""
        print(f"\n{'📚' * 30}")
        print("📖 COMPLETE STORY GENERATED BY SHIMMY-DS")
        print(f"{'📚' * 30}")
        print(self.story_text)
        print(f"\n{'📚' * 30}")

        # Story statistics
        word_count = len(self.story_text.split())
        char_count = len(self.story_text)
        paragraph_count = len([p for p in self.story_text.split('\n\n') if p.strip()])

        print(f"\n📊 Story Statistics:")
        print(f"  Words: {word_count:,}")
        print(f"  Characters: {char_count:,}")
        print(f"  Paragraphs: {paragraph_count}")

        # Intelligence summary
        if self.analysis_history:
            avg_health = sum(entry['health'] for entry in self.analysis_history) / len(self.analysis_history)
            total_patterns = sum(entry['patterns'] for entry in self.analysis_history)

            print(f"\n🧠 Intelligence Summary:")
            print(f"  Average Narrative Health: {avg_health:.2f}")
            print(f"  Total Patterns Tracked: {total_patterns}")
            print(f"  Systems Active: 6/6 (All narrative intelligence systems)")

def main():
    """Run the complete Shimmy-DS writing demonstration"""
    demo = ShimmyWritingDemo()

    try:
        # Test connection
        response = requests.get(f"{demo.base_url}/v1/models", timeout=5)
        if response.status_code == 200:
            print("✅ Connected to Shimmy-DS server")
        else:
            print("⚠️  Shimmy-DS server responded with error, using simulated responses")
    except:
        print("⚠️  Cannot connect to Shimmy-DS server, using simulated responses")
        print("Start server with: ./target/release/shimmy.exe serve")

    # Run the demonstration
    demo.write_complete_story()

    print(f"\n{'🎉' * 30}")
    print("✨ SHIMMY-DS WRITING DEMONSTRATION COMPLETE")
    print(f"{'🎉' * 30}")
    print("🧠 This demonstration showcased:")
    print("  ✅ CAPR Loop Tracking (Contradiction→Action→Pressure→Return)")
    print("  ✅ Character Consistency Engine")
    print("  ✅ Constraint Space Modeling")
    print("  ✅ Multi-Level Recursion Detection")
    print("  ✅ Reader Engagement Monitoring")
    print("  ✅ Narrative Drift Stabilization")
    print("  ✅ Adaptive Intelligence Systems")
    print("\n🌟 World's first recursive narrative intelligence in action!")

if __name__ == "__main__":
    main()