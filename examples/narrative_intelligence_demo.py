#!/usr/bin/env python3
"""
Shimmy-DS Narrative Intelligence Demo

This demonstrates the narrative intelligence capabilities by simulating
the analysis and showing how the systems would work.
"""

import json
import time
from typing import Dict, Any, List

class NarrativeIntelligenceDemo:
    """Demonstrates Shimmy-DS narrative intelligence without requiring server"""

    def __init__(self):
        self.story_segments = []
        self.analysis_history = []

    def simulate_capr_analysis(self, text: str) -> Dict[str, Any]:
        """Simulate CAPR (Contradiction→Action→Pressure→Return) loop analysis"""
        # Look for narrative patterns that indicate CAPR loops
        contradictions = []
        actions = []
        pressures = []
        returns = []

        # Simple pattern detection
        if any(word in text.lower() for word in ['but', 'however', 'contradiction', 'impossible', 'paradox']):
            contradictions.append("Logical contradiction detected")

        if any(word in text.lower() for word in ['decided', 'chose', 'action', 'stepped', 'reached']):
            actions.append("Character action taken")

        if any(word in text.lower() for word in ['tension', 'pressure', 'conflict', 'danger', 'threat']):
            pressures.append("Narrative pressure building")

        if any(word in text.lower() for word in ['return', 'back', 'again', 'cycle', 'repeat']):
            returns.append("Return/circular element detected")

        # Calculate CAPR strength
        capr_elements = len(contradictions) + len(actions) + len(pressures) + len(returns)
        capr_strength = min(capr_elements / 8.0, 1.0)  # Normalize to 0-1

        return {
            "contradictions": contradictions,
            "actions": actions,
            "pressures": pressures,
            "returns": returns,
            "capr_strength": capr_strength,
            "complete_loops": capr_elements >= 4
        }

    def simulate_character_consistency(self, text: str, character_name: str = "Elena") -> Dict[str, Any]:
        """Simulate character consistency analysis"""
        consistency_score = 0.8  # Base score

        # Check for character development markers
        development_markers = 0
        if character_name.lower() in text.lower():
            development_markers += 1

        if any(word in text.lower() for word in ['thought', 'felt', 'realized', 'understood']):
            development_markers += 1

        if any(word in text.lower() for word in ['curious', 'afraid', 'determined', 'cautious']):
            development_markers += 1

        # Adjust score based on development
        consistency_score = min(0.7 + (development_markers * 0.1), 1.0)

        return {
            "character_name": character_name,
            "consistency_score": consistency_score,
            "development_markers": development_markers,
            "voice_coherence": consistency_score > 0.75,
            "personality_traits": ["curious", "cautious", "determined"] if development_markers > 1 else ["basic"]
        }

    def simulate_constraint_analysis(self, text: str) -> Dict[str, Any]:
        """Simulate narrative constraint space analysis"""
        # Analyze narrative freedom and constraints
        constraint_indicators = 0
        freedom_indicators = 0

        # Look for constraint indicators
        if any(word in text.lower() for word in ['must', 'cannot', 'impossible', 'trapped', 'limited']):
            constraint_indicators += 1

        if any(word in text.lower() for word in ['rule', 'law', 'boundary', 'restriction']):
            constraint_indicators += 1

        # Look for freedom indicators
        if any(word in text.lower() for word in ['choice', 'option', 'possibility', 'could', 'might']):
            freedom_indicators += 1

        if any(word in text.lower() for word in ['open', 'free', 'unlimited', 'explore']):
            freedom_indicators += 1

        # Calculate freedom score
        total_indicators = constraint_indicators + freedom_indicators
        freedom_score = freedom_indicators / max(total_indicators, 1)

        return {
            "freedom_score": freedom_score,
            "constraint_indicators": constraint_indicators,
            "freedom_indicators": freedom_indicators,
            "narrative_paths_open": freedom_score > 0.5,
            "pressure_points": ["character_choices", "world_rules"] if constraint_indicators > 0 else []
        }

    def simulate_engagement_analysis(self, text: str) -> Dict[str, Any]:
        """Simulate reader engagement analysis"""
        curiosity_score = 0.5
        tension_score = 0.5

        # Curiosity indicators
        curiosity_words = ['mystery', 'strange', 'unknown', 'wonder', 'question', 'curious', 'discover']
        curiosity_count = sum(1 for word in curiosity_words if word in text.lower())
        curiosity_score = min(0.4 + (curiosity_count * 0.15), 1.0)

        # Tension indicators
        tension_words = ['danger', 'fear', 'threat', 'urgent', 'crisis', 'conflict', 'tension']
        tension_count = sum(1 for word in tension_words if word in text.lower())
        tension_score = min(0.3 + (tension_count * 0.2), 1.0)

        return {
            "curiosity_score": curiosity_score,
            "tension_score": tension_score,
            "overall_engagement": (curiosity_score + tension_score) / 2,
            "engagement_hooks": curiosity_count + tension_count,
            "reader_investment": "high" if (curiosity_score + tension_score) > 1.4 else "moderate"
        }

    def simulate_recursion_analysis(self, text: str) -> Dict[str, Any]:
        """Simulate multi-level recursion analysis"""
        recursion_indicators = 0

        # Look for recursive themes
        recursive_words = ['mirror', 'reflect', 'echo', 'repeat', 'pattern', 'cycle', 'recursive', 'infinite']
        recursion_count = sum(1 for word in recursive_words if word in text.lower())

        # Look for scale-crossing patterns
        scale_words = ['level', 'layer', 'dimension', 'world', 'reality', 'version']
        scale_count = sum(1 for word in scale_words if word in text.lower())

        recursion_strength = min((recursion_count + scale_count) / 10.0, 1.0)

        return {
            "recursion_strength": recursion_strength,
            "recursive_themes": recursion_count,
            "scale_elements": scale_count,
            "cross_scale_patterns": recursion_strength > 0.6,
            "recursive_depth": min(recursion_count, 5)
        }

    def simulate_drift_analysis(self, current_text: str, previous_texts: List[str]) -> Dict[str, Any]:
        """Simulate narrative drift analysis"""
        if not previous_texts:
            return {
                "drift_detected": False,
                "drift_score": 0.0,
                "stability_score": 1.0,
                "consistency_rating": "stable"
            }

        # Simple drift detection based on tone and character consistency
        drift_score = 0.0

        # Check for major tone shifts
        current_tone_words = set()
        for word in ['dark', 'light', 'serious', 'playful', 'mysterious', 'clear']:
            if word in current_text.lower():
                current_tone_words.add(word)

        previous_tone_words = set()
        for prev_text in previous_texts[-2:]:  # Check last 2 segments
            for word in ['dark', 'light', 'serious', 'playful', 'mysterious', 'clear']:
                if word in prev_text.lower():
                    previous_tone_words.add(word)

        # Calculate tone consistency
        if previous_tone_words and current_tone_words:
            tone_overlap = len(current_tone_words & previous_tone_words) / len(previous_tone_words | current_tone_words)
            drift_score = 1.0 - tone_overlap
        else:
            drift_score = 0.2  # Minor drift if no clear tone

        stability_score = 1.0 - drift_score

        return {
            "drift_detected": drift_score > 0.3,
            "drift_score": drift_score,
            "stability_score": stability_score,
            "consistency_rating": "stable" if drift_score < 0.3 else "minor_drift" if drift_score < 0.6 else "major_drift"
        }

    def analyze_story_segment(self, text: str, segment_number: int) -> Dict[str, Any]:
        """Complete narrative intelligence analysis of a story segment"""

        print(f"\n🧠 ANALYZING STORY SEGMENT {segment_number}")
        print("=" * 60)
        print(f"Text: {text[:100]}...")

        # Run all narrative intelligence systems
        capr_analysis = self.simulate_capr_analysis(text)
        character_analysis = self.simulate_character_consistency(text)
        constraint_analysis = self.simulate_constraint_analysis(text)
        engagement_analysis = self.simulate_engagement_analysis(text)
        recursion_analysis = self.simulate_recursion_analysis(text)
        drift_analysis = self.simulate_drift_analysis(text, self.story_segments)

        # Calculate overall narrative health
        system_scores = {
            "capr_loops": capr_analysis["capr_strength"],
            "character_consistency": character_analysis["consistency_score"],
            "constraint_space": constraint_analysis["freedom_score"],
            "reader_engagement": engagement_analysis["overall_engagement"],
            "recursion_tracking": recursion_analysis["recursion_strength"],
            "drift_stability": drift_analysis["stability_score"]
        }

        overall_health = sum(system_scores.values()) / len(system_scores)

        analysis_result = {
            "segment": segment_number,
            "overall_health": overall_health,
            "system_scores": system_scores,
            "capr_analysis": capr_analysis,
            "character_analysis": character_analysis,
            "constraint_analysis": constraint_analysis,
            "engagement_analysis": engagement_analysis,
            "recursion_analysis": recursion_analysis,
            "drift_analysis": drift_analysis,
            "text": text
        }

        # Store for history
        self.analysis_history.append(analysis_result)
        self.story_segments.append(text)

        return analysis_result

    def display_analysis_results(self, analysis: Dict[str, Any]):
        """Display formatted analysis results"""
        print(f"\n📊 NARRATIVE INTELLIGENCE REPORT - Segment {analysis['segment']}")
        print("=" * 60)

        # Overall health
        health = analysis["overall_health"]
        health_bar = "█" * int(health * 20) + "░" * (20 - int(health * 20))
        health_color = "🟢" if health > 0.7 else "🟡" if health > 0.5 else "🔴"
        print(f"{health_color} Overall Health: {health:.2f} [{health_bar}]")

        # System breakdown
        print(f"\n🔧 System Analysis:")
        systems = analysis["system_scores"]
        for system, score in systems.items():
            bar = "█" * int(score * 10) + "░" * (10 - int(score * 10))
            color = "🟢" if score > 0.7 else "🟡" if score > 0.5 else "🔴"
            system_name = system.replace("_", " ").title()
            print(f"  {color} {system_name:<18}: {score:.2f} [{bar}]")

        # Detailed insights
        print(f"\n🎭 CAPR Loop Analysis:")
        capr = analysis["capr_analysis"]
        print(f"  Strength: {capr['capr_strength']:.2f}")
        print(f"  Complete Loops: {'Yes' if capr['complete_loops'] else 'No'}")
        if capr["contradictions"]:
            print(f"  Contradictions: {', '.join(capr['contradictions'])}")

        print(f"\n👤 Character Analysis:")
        char = analysis["character_analysis"]
        print(f"  Consistency: {char['consistency_score']:.2f}")
        print(f"  Voice Coherence: {'Yes' if char['voice_coherence'] else 'No'}")
        print(f"  Traits: {', '.join(char['personality_traits'])}")

        print(f"\n🗺️  Constraint Space:")
        constraint = analysis["constraint_analysis"]
        print(f"  Freedom Score: {constraint['freedom_score']:.2f}")
        print(f"  Paths Open: {'Yes' if constraint['narrative_paths_open'] else 'Limited'}")

        print(f"\n📚 Reader Engagement:")
        engagement = analysis["engagement_analysis"]
        print(f"  Curiosity: {engagement['curiosity_score']:.2f}")
        print(f"  Tension: {engagement['tension_score']:.2f}")
        print(f"  Investment: {engagement['reader_investment']}")

        print(f"\n🔄 Recursion Analysis:")
        recursion = analysis["recursion_analysis"]
        print(f"  Strength: {recursion['recursion_strength']:.2f}")
        print(f"  Cross-Scale: {'Yes' if recursion['cross_scale_patterns'] else 'No'}")
        print(f"  Depth: {recursion['recursive_depth']}")

        print(f"\n⚖️  Drift Analysis:")
        drift = analysis["drift_analysis"]
        print(f"  Stability: {drift['stability_score']:.2f}")
        print(f"  Status: {drift['consistency_rating']}")
        if drift["drift_detected"]:
            print(f"  ⚠️  Drift detected!")

    def run_complete_demo(self):
        """Run complete narrative intelligence demonstration"""
        print("🧠 SHIMMY-DS NARRATIVE INTELLIGENCE DEMONSTRATION")
        print("🌟 World's First Recursive Narrative Intelligence System")
        print("=" * 80)

        # Sample story segments that demonstrate different narrative elements
        story_segments = [
            "Elena stood before the antique mirror in her grandmother's attic, but instead of her own reflection, she saw a different room entirely—one where the furniture was arranged differently and strange symbols glowed softly on the walls.",

            "Curious despite her fear, Elena reached out to touch the glass. Her hand passed through as if the mirror were made of water, sending ripples across the silvered surface. She realized this wasn't just a mirror—it was a doorway.",

            "Elena stepped through into the mirror world and immediately encountered another version of herself. This other Elena warned her urgently: 'You shouldn't be here. Each mirror leads to a different choice, a different life. But something is hunting between the worlds.'",

            "The two Elenas discovered that every mirror in every reality was connected, forming an infinite network of possibilities. But a dark presence was moving through the mirror maze, seeking to collapse all realities into one. They had to work together to stop it.",

            "In the climactic confrontation, Elena realized the dark presence was her own regret and self-doubt made manifest. To save all realities, she had to accept every version of herself—every choice she'd made and hadn't made. The mirrors finally showed her true reflection: complete and whole."
        ]

        segment_descriptions = [
            "Opening - Mysterious premise with recursive mirrors",
            "Development - Character choice and world-building",
            "Complication - Recursive character encounter",
            "Climax - Universal threat and collaboration",
            "Resolution - Self-acceptance and recursive themes"
        ]

        # Analyze each segment
        for i, (text, description) in enumerate(zip(story_segments, segment_descriptions), 1):
            print(f"\n{'🔸' * 25}")
            print(f"📖 SEGMENT {i}: {description}")
            print(f"{'🔸' * 25}")

            analysis = self.analyze_story_segment(text, i)
            self.display_analysis_results(analysis)

            # Brief pause for readability
            time.sleep(1)

        # Show evolution summary
        self.show_evolution_summary()

    def show_evolution_summary(self):
        """Show how narrative intelligence evolved across the story"""
        print(f"\n{'🎭' * 30}")
        print("📈 NARRATIVE INTELLIGENCE EVOLUTION")
        print(f"{'🎭' * 30}")

        if len(self.analysis_history) < 2:
            return

        print("\n📊 Intelligence Metrics Over Time:")
        print("Segment | Health | CAPR | Character | Engagement | Recursion")
        print("-" * 60)

        for analysis in self.analysis_history:
            seg = analysis["segment"]
            health = analysis["overall_health"]
            capr = analysis["system_scores"]["capr_loops"]
            character = analysis["system_scores"]["character_consistency"]
            engagement = analysis["system_scores"]["reader_engagement"]
            recursion = analysis["system_scores"]["recursion_tracking"]

            print(f"   {seg}    | {health:.2f}   | {capr:.2f} |   {character:.2f}    |    {engagement:.2f}     |   {recursion:.2f}")

        # Calculate improvements
        first = self.analysis_history[0]
        last = self.analysis_history[-1]

        improvements = {
            "health": last["overall_health"] - first["overall_health"],
            "capr": last["system_scores"]["capr_loops"] - first["system_scores"]["capr_loops"],
            "character": last["system_scores"]["character_consistency"] - first["system_scores"]["character_consistency"],
            "engagement": last["system_scores"]["reader_engagement"] - first["system_scores"]["reader_engagement"],
            "recursion": last["system_scores"]["recursion_tracking"] - first["system_scores"]["recursion_tracking"]
        }

        print(f"\n📈 Overall System Improvements:")
        for metric, improvement in improvements.items():
            symbol = "📈" if improvement > 0 else "📉" if improvement < 0 else "➡️"
            print(f"  {symbol} {metric.title()}: {improvement:+.2f}")

        # Final assessment
        final_health = last["overall_health"]
        print(f"\n🏆 FINAL ASSESSMENT:")
        print(f"  Overall Narrative Health: {final_health:.2f}/1.0")
        print(f"  Systems Operational: 6/6")
        print(f"  Recursive Themes: Strong" if last["system_scores"]["recursion_tracking"] > 0.7 else "  Recursive Themes: Moderate")
        print(f"  Character Consistency: {'Excellent' if last['system_scores']['character_consistency'] > 0.8 else 'Good'}")

        print(f"\n✨ DEMONSTRATION COMPLETE!")
        print("🧠 This showcased all 6 narrative intelligence systems:")
        print("  ✅ CAPR Loop Tracking")
        print("  ✅ Character Consistency Engine")
        print("  ✅ Constraint Space Modeling")
        print("  ✅ Reader Engagement Monitoring")
        print("  ✅ Multi-Level Recursion Detection")
        print("  ✅ Narrative Drift Stabilization")
        print("\n🌟 World's first recursive narrative intelligence system!")

def main():
    """Run the narrative intelligence demonstration"""
    demo = NarrativeIntelligenceDemo()
    demo.run_complete_demo()

if __name__ == "__main__":
    main()