#!/usr/bin/env python3
"""
How to directly use ALL Shimmy-DS narrative intelligence functions
within your own program/application.
"""

import requests
import json
from typing import Dict, Any, List, Optional

class ShimmyDSNarrativeEngine:
    """Direct interface to use ALL Shimmy-DS narrative intelligence functions"""

    def __init__(self, shimmy_url="http://127.0.0.1:11435"):
        self.base_url = shimmy_url.rstrip('/')

    # ========================
    # NARRATIVE DNA FUNCTIONS
    # ========================

    def analyze_capr_loops(self, text: str) -> Dict[str, Any]:
        """Analyze Contradiction→Action→Pressure→Return loops in text"""
        response = requests.post(f"{self.base_url}/v1/chat/completions", json={
            "model": "analysis",
            "messages": [
                {"role": "system", "content": "Analyze this text for CAPR loops (Contradiction→Action→Pressure→Return patterns)."},
                {"role": "user", "content": text}
            ],
            "max_tokens": 200
        })

        # Get narrative analysis
        analysis = requests.get(f"{self.base_url}/narrative/analyze").json()

        return {
            "capr_analysis": response.json()["choices"][0]["message"]["content"],
            "loop_strength": analysis["narrative_health"]["system_scores"].get("dna_tracker", 0),
            "active_patterns": [p for p in analysis["active_patterns"] if p.get("type") == "CAPR_loop"]
        }

    # ========================
    # CHARACTER CONSISTENCY FUNCTIONS
    # ========================

    def check_character_consistency(self, character_name: str, new_dialogue: str) -> Dict[str, Any]:
        """Check if new dialogue/action is consistent with character"""
        response = requests.post(f"{self.base_url}/v1/chat/completions", json={
            "model": "analysis",
            "messages": [
                {"role": "system", "content": f"Analyze if this dialogue is consistent with {character_name}'s established character."},
                {"role": "user", "content": new_dialogue}
            ],
            "max_tokens": 150
        })

        analysis = requests.get(f"{self.base_url}/narrative/analyze").json()

        return {
            "consistency_score": analysis["narrative_health"]["system_scores"].get("character_consistency", 0),
            "analysis": response.json()["choices"][0]["message"]["content"],
            "character_data": [p for p in analysis["active_patterns"] if p.get("type") == "character_arc"]
        }

    def get_character_voice_profile(self, character_name: str) -> Dict[str, Any]:
        """Get character voice and personality profile"""
        analysis = requests.get(f"{self.base_url}/narrative/analyze").json()

        character_patterns = [p for p in analysis["active_patterns"]
                            if p.get("type") == "character_arc" and character_name.lower() in p.get("character", "").lower()]

        return {
            "character_name": character_name,
            "arc_progress": character_patterns[0].get("progress", 0) if character_patterns else 0,
            "arc_type": character_patterns[0].get("arc_type", "") if character_patterns else "",
            "consistency_score": analysis["narrative_health"]["system_scores"].get("character_consistency", 0)
        }

    # ========================
    # CONSTRAINT SPACE FUNCTIONS
    # ========================

    def get_narrative_possibilities(self) -> Dict[str, Any]:
        """Get current narrative possibility space and constraints"""
        analysis = requests.get(f"{self.base_url}/narrative/analyze").json()

        constraints = analysis.get("constraints", {})

        return {
            "freedom_score": constraints.get("freedom_score", 0),
            "active_constraints": constraints.get("active", 0),
            "resolved_constraints": constraints.get("resolved", 0),
            "pressure_points": constraints.get("pressure_points", []),
            "narrative_paths_open": constraints.get("freedom_score", 0) > 0.7
        }

    def check_constraint_violation(self, proposed_action: str) -> Dict[str, Any]:
        """Check if a proposed story action would violate narrative constraints"""
        response = requests.post(f"{self.base_url}/v1/chat/completions", json={
            "model": "analysis",
            "messages": [
                {"role": "system", "content": "Analyze if this action would violate established narrative constraints."},
                {"role": "user", "content": f"Proposed action: {proposed_action}"}
            ],
            "max_tokens": 100
        })

        analysis = requests.get(f"{self.base_url}/narrative/analyze").json()

        return {
            "violation_risk": 1.0 - analysis["narrative_health"]["system_scores"].get("constraint_space", 1.0),
            "analysis": response.json()["choices"][0]["message"]["content"],
            "current_freedom": analysis.get("constraints", {}).get("freedom_score", 0)
        }

    # ========================
    # ENGAGEMENT TRACKING FUNCTIONS
    # ========================

    def get_reader_engagement_metrics(self) -> Dict[str, Any]:
        """Get current reader engagement analysis"""
        analysis = requests.get(f"{self.base_url}/narrative/analyze").json()

        engagement = analysis.get("engagement_metrics", {})

        return {
            "curiosity_score": engagement.get("curiosity_score", 0),
            "investment_score": engagement.get("investment_score", 0),
            "tension_level": engagement.get("tension_level", 0),
            "overall_engagement": sum(engagement.values()) / len(engagement) if engagement else 0,
            "engagement_recommendations": self._generate_engagement_recommendations(engagement)
        }

    def _generate_engagement_recommendations(self, engagement: Dict[str, float]) -> List[str]:
        """Generate recommendations to improve engagement"""
        recommendations = []

        if engagement.get("curiosity_score", 0) < 0.7:
            recommendations.append("Add mystery or unanswered questions")
        if engagement.get("tension_level", 0) < 0.6:
            recommendations.append("Increase dramatic tension or conflict")
        if engagement.get("investment_score", 0) < 0.7:
            recommendations.append("Develop character stakes and emotional investment")

        return recommendations

    # ========================
    # DRIFT STABILIZATION FUNCTIONS
    # ========================

    def check_narrative_drift(self) -> Dict[str, Any]:
        """Check for narrative drift and get stabilization recommendations"""
        analysis = requests.get(f"{self.base_url}/narrative/analyze").json()

        drift_score = 1.0 - analysis["narrative_health"]["system_scores"].get("drift_stability", 1.0)

        return {
            "drift_detected": drift_score > 0.3,
            "drift_severity": drift_score,
            "stability_score": analysis["narrative_health"]["system_scores"].get("drift_stability", 0),
            "warnings": self._get_drift_warnings(drift_score),
            "stabilization_needed": drift_score > 0.5
        }

    def _get_drift_warnings(self, drift_score: float) -> List[str]:
        """Get drift warning messages based on severity"""
        warnings = []

        if drift_score > 0.7:
            warnings.append("Severe narrative drift detected - major course correction needed")
        elif drift_score > 0.5:
            warnings.append("Moderate drift detected - review recent narrative choices")
        elif drift_score > 0.3:
            warnings.append("Minor drift detected - monitor narrative consistency")

        return warnings

    # ========================
    # MULTI-LEVEL RECURSION FUNCTIONS
    # ========================

    def analyze_recursive_patterns(self, text: str) -> Dict[str, Any]:
        """Analyze recursive patterns across narrative scales"""
        response = requests.post(f"{self.base_url}/v1/chat/completions", json={
            "model": "analysis",
            "messages": [
                {"role": "system", "content": "Analyze recursive patterns and cross-scale echoes in this narrative."},
                {"role": "user", "content": text}
            ],
            "max_tokens": 200
        })

        analysis = requests.get(f"{self.base_url}/narrative/analyze").json()

        return {
            "recursion_strength": analysis["narrative_health"]["system_scores"].get("recursion_tracker", 0),
            "pattern_analysis": response.json()["choices"][0]["message"]["content"],
            "scale_coherence": analysis["narrative_health"]["system_scores"].get("recursion_tracker", 0) > 0.7
        }

    # ========================
    # COMPREHENSIVE NARRATIVE INTELLIGENCE
    # ========================

    def get_full_narrative_intelligence(self) -> Dict[str, Any]:
        """Get complete narrative intelligence analysis from all systems"""
        analysis = requests.get(f"{self.base_url}/narrative/analyze").json()
        report = requests.get(f"{self.base_url}/narrative/report").json()

        return {
            "overall_health": analysis["narrative_health"]["overall_score"],
            "system_breakdown": {
                "dna_tracker": analysis["narrative_health"]["system_scores"].get("dna_tracker", 0),
                "constraint_space": analysis["narrative_health"]["system_scores"].get("constraint_space", 0),
                "recursion_tracker": analysis["narrative_health"]["system_scores"].get("recursion_tracker", 0),
                "character_consistency": analysis["narrative_health"]["system_scores"].get("character_consistency", 0),
                "engagement_tracker": analysis["narrative_health"]["system_scores"].get("engagement_tracker", 0),
                "drift_stability": analysis["narrative_health"]["system_scores"].get("drift_stability", 0)
            },
            "active_patterns": analysis["active_patterns"],
            "constraints": analysis["constraints"],
            "engagement_metrics": analysis["engagement_metrics"],
            "adaptive_intelligence": report.get("adaptive_intelligence", {}),
            "recommendations": self._generate_comprehensive_recommendations(analysis)
        }

    def _generate_comprehensive_recommendations(self, analysis: Dict[str, Any]) -> List[str]:
        """Generate comprehensive recommendations based on all systems"""
        recommendations = []

        health = analysis["narrative_health"]["overall_score"]
        systems = analysis["narrative_health"]["system_scores"]

        if health < 0.7:
            recommendations.append("Overall narrative health needs attention")

        # Check each system
        if systems.get("dna_tracker", 0) < 0.6:
            recommendations.append("Strengthen CAPR loop development")
        if systems.get("character_consistency", 0) < 0.7:
            recommendations.append("Improve character consistency")
        if systems.get("constraint_space", 0) < 0.6:
            recommendations.append("Address narrative constraint issues")

        return recommendations

    # ========================
    # ADAPTIVE INTELLIGENCE FUNCTIONS
    # ========================

    def configure_adaptive_intelligence(self, config: Dict[str, Any]) -> bool:
        """Configure adaptive intelligence parameters"""
        response = requests.post(f"{self.base_url}/narrative/config", json=config)
        return response.status_code == 200

    def get_adaptive_status(self) -> Dict[str, Any]:
        """Get current adaptive intelligence status"""
        report = requests.get(f"{self.base_url}/narrative/report").json()

        adaptive = report.get("adaptive_intelligence", {})

        return {
            "adapt_iq": adaptive.get("adapt_iq", {}),
            "qualitier": adaptive.get("qualitier", {}),
            "obli_select": adaptive.get("obli_select", {}),
            "profile_mesh": adaptive.get("profile_mesh", {})
        }

# ========================
# USAGE EXAMPLES
# ========================

def example_program_integration():
    """Example of how to use ALL functions within your program"""

    # Initialize the engine
    engine = ShimmyDSNarrativeEngine()

    print("🧠 Using ALL Shimmy-DS Functions in Your Program")
    print("=" * 60)

    # Example story text
    story_text = """
    Elena stood before the mirror, but her reflection showed a different room entirely.
    She reached out to touch the glass, and her reflection did the same, but their
    hands met in the middle of what should have been solid silver.
    """

    # 1. Analyze CAPR loops
    print("\n1. CAPR Loop Analysis:")
    capr = engine.analyze_capr_loops(story_text)
    print(f"   Loop Strength: {capr['loop_strength']:.2f}")
    print(f"   Active Patterns: {len(capr['active_patterns'])}")

    # 2. Check character consistency
    print("\n2. Character Consistency:")
    character = engine.check_character_consistency("Elena", "Elena laughed maniacally")
    print(f"   Consistency Score: {character['consistency_score']:.2f}")

    # 3. Get narrative possibilities
    print("\n3. Narrative Possibilities:")
    possibilities = engine.get_narrative_possibilities()
    print(f"   Freedom Score: {possibilities['freedom_score']:.2f}")
    print(f"   Paths Open: {possibilities['narrative_paths_open']}")

    # 4. Check engagement
    print("\n4. Reader Engagement:")
    engagement = engine.get_reader_engagement_metrics()
    print(f"   Overall Engagement: {engagement['overall_engagement']:.2f}")
    print(f"   Recommendations: {engagement['engagement_recommendations'][:2]}")

    # 5. Check for drift
    print("\n5. Narrative Drift:")
    drift = engine.check_narrative_drift()
    print(f"   Drift Detected: {drift['drift_detected']}")
    print(f"   Stability Score: {drift['stability_score']:.2f}")

    # 6. Analyze recursive patterns
    print("\n6. Recursive Patterns:")
    recursion = engine.analyze_recursive_patterns(story_text)
    print(f"   Recursion Strength: {recursion['recursion_strength']:.2f}")
    print(f"   Scale Coherence: {recursion['scale_coherence']}")

    # 7. Get full intelligence report
    print("\n7. Full Intelligence Report:")
    full_report = engine.get_full_narrative_intelligence()
    print(f"   Overall Health: {full_report['overall_health']:.2f}")
    print(f"   Systems Active: {len([s for s in full_report['system_breakdown'].values() if s > 0.5])}/6")
    print(f"   Recommendations: {len(full_report['recommendations'])}")

    # 8. Configure and adapt
    print("\n8. Adaptive Configuration:")
    config = {
        "assertiveness_level": "moderate",
        "adaptive_settings": {"adaptation_enabled": True}
    }
    configured = engine.configure_adaptive_intelligence(config)
    print(f"   Configuration Success: {configured}")

    adaptive_status = engine.get_adaptive_status()
    print(f"   AdaptIQ Depth: {adaptive_status['adapt_iq'].get('current_depth', 'N/A')}")

if __name__ == "__main__":
    try:
        example_program_integration()
        print("\n✅ All functions successfully accessed!")
    except requests.exceptions.ConnectionError:
        print("❌ Error: Start Shimmy-DS with: ./target/release/shimmy.exe serve")
    except Exception as e:
        print(f"❌ Error: {e}")