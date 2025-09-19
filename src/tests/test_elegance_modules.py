"""
Tests for Elegance Modules (EAT + FPD + RIE-lite)
================================================

Targeted tests for the minimal elegance upgrade modules to ensure
proper integration with the existing RIP+RIC pipeline.
"""

import unittest
import asyncio
from datetime import datetime
from typing import Dict, List, Any

# Import the elegance modules
import sys
import os
sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'dreamspiral'))

from emotional_arc_tracker import (
    EmotionalArcTracker, EmotionalPoint, EmotionalBeatState,
    EmotionalIntensity, EmotionalValence
)
from foreshadowing_detector import (
    ForeshadowingPayoffDetector, ForeshadowingSetup, NarrativePayoff,
    SetupType, PayoffType, PayoffTiming
)
from inquiry_bank import (
    RecursiveInquiryEngine, InquiryTemplate, ActiveInquiry,
    InquiryType, InquiryDepth
)


class TestEmotionalArcTracker(unittest.TestCase):
    """Test EAT (Emotional Arc Tracker) functionality"""

    def setUp(self):
        self.eat = EmotionalArcTracker()

    def test_emotional_point_creation(self):
        """Test creating emotional points"""
        point = self.eat.track_emotional_point(
            character_name="Elena",
            beat_id="scene_1",
            emotion_type="fear",
            intensity=EmotionalIntensity.STRONG,
            valence=EmotionalValence.NEGATIVE,
            context="Discovering the cracked mirror"
        )

        self.assertEqual(point.character_name, "Elena")
        self.assertEqual(point.emotion_type, "fear")
        self.assertEqual(point.intensity, EmotionalIntensity.STRONG)
        self.assertEqual(point.valence, EmotionalValence.NEGATIVE)

    def test_beat_state_creation(self):
        """Test creating complete beat states"""
        # Create primary emotion
        primary = self.eat.track_emotional_point(
            character_name="Elena",
            beat_id="scene_1",
            emotion_type="curiosity",
            intensity=EmotionalIntensity.MODERATE,
            valence=EmotionalValence.POSITIVE,
            context="Examining the mirror"
        )

        # Create beat state
        beat_state = self.eat.create_beat_state(
            character_name="Elena",
            beat_id="scene_1",
            primary_emotion=primary,
            emotional_shift=("neutral", "curious")
        )

        self.assertEqual(beat_state.character_name, "Elena")
        self.assertEqual(beat_state.primary_emotion.emotion_type, "curiosity")
        self.assertEqual(beat_state.emotional_shift, ("neutral", "curious"))
        self.assertGreaterEqual(beat_state.calculate_complexity_score(), 0.0)

    def test_character_arc_analysis(self):
        """Test analyzing complete character emotional arcs"""
        # Create multiple emotional states for a character
        emotions = [
            ("curiosity", EmotionalIntensity.MODERATE, EmotionalValence.POSITIVE),
            ("fear", EmotionalIntensity.STRONG, EmotionalValence.NEGATIVE),
            ("determination", EmotionalIntensity.STRONG, EmotionalValence.POSITIVE)
        ]

        for i, (emotion, intensity, valence) in enumerate(emotions):
            point = self.eat.track_emotional_point(
                character_name="Elena",
                beat_id=f"scene_{i+1}",
                emotion_type=emotion,
                intensity=intensity,
                valence=valence,
                context=f"Beat {i+1} context"
            )

            self.eat.create_beat_state(
                character_name="Elena",
                beat_id=f"scene_{i+1}",
                primary_emotion=point
            )

        # Analyze the arc
        analysis = self.eat.analyze_character_arc("Elena")

        self.assertEqual(analysis['character_name'], "Elena")
        self.assertEqual(analysis['total_beats'], 3)
        self.assertIn('emotional_range', analysis)
        self.assertIn('dominant_emotions', analysis)

    def test_beat_emotional_summary(self):
        """Test getting emotional summary for a beat"""
        # Add multiple characters to same beat
        characters = ["Elena", "Marcus", "Sofia"]
        emotions = ["curiosity", "skepticism", "excitement"]

        for char, emotion in zip(characters, emotions):
            point = self.eat.track_emotional_point(
                character_name=char,
                beat_id="scene_1",
                emotion_type=emotion,
                intensity=EmotionalIntensity.MODERATE,
                valence=EmotionalValence.POSITIVE,
                context="Group discovery scene"
            )

            self.eat.create_beat_state(
                character_name=char,
                beat_id="scene_1",
                primary_emotion=point
            )

        summary = self.eat.get_beat_emotional_summary("scene_1")

        self.assertEqual(summary['beat_id'], "scene_1")
        self.assertEqual(len(summary['characters']), 3)
        self.assertGreaterEqual(summary['overall_intensity'], 0.0)

    def test_rip_integration_export(self):
        """Test RIP integration export functionality"""
        # Create some emotional data
        point = self.eat.track_emotional_point(
            character_name="Elena",
            beat_id="scene_1",
            emotion_type="confusion",
            intensity=EmotionalIntensity.OVERWHELMING,
            valence=EmotionalValence.CONFLICTED,
            context="Major revelation"
        )

        self.eat.create_beat_state(
            character_name="Elena",
            beat_id="scene_1",
            primary_emotion=point
        )

        export_data = self.eat.export_for_rip_integration()

        self.assertEqual(export_data['tracker_type'], 'EAT')
        self.assertIn('character_summaries', export_data)
        self.assertIn('continuity_flags', export_data)
        self.assertIn('pattern_violations', export_data)

    def test_tracker_health(self):
        """Test tracker health metrics"""
        health = self.eat.get_tracker_health()

        self.assertEqual(health['tracker_type'], 'EAT')
        self.assertIn('total_characters', health)
        self.assertIn('health_score', health)


class TestForeshadowingPayoffDetector(unittest.TestCase):
    """Test FPD (Foreshadowing & Payoff Detector) functionality"""

    def setUp(self):
        self.fpd = ForeshadowingPayoffDetector()

    def test_automatic_setup_detection(self):
        """Test automatic detection of narrative setups"""
        setup = self.fpd.detect_setup(
            beat_id="scene_1",
            content="I promise I'll find the truth about what happened to grandmother",
            characters=["Elena"]
        )

        self.assertIsNotNone(setup)
        self.assertEqual(setup.setup_type, SetupType.CHARACTER_PROMISE)
        self.assertIn("Elena", setup.characters_involved)

    def test_manual_setup_registration(self):
        """Test manual setup registration"""
        setup = self.fpd.register_manual_setup(
            beat_id="scene_2",
            setup_type=SetupType.MYSTERY_ELEMENT,
            content="What secrets is the mirror hiding?",
            characters=["Elena"],
            emotional_weight=0.8,
            expected_timing=PayoffTiming.MEDIUM_TERM
        )

        self.assertEqual(setup.setup_type, SetupType.MYSTERY_ELEMENT)
        self.assertEqual(setup.emotional_weight, 0.8)
        self.assertEqual(setup.expected_timing, PayoffTiming.MEDIUM_TERM)

    def test_automatic_payoff_detection(self):
        """Test automatic payoff detection and matching"""
        # First create a setup
        setup = self.fpd.register_manual_setup(
            beat_id="scene_1",
            setup_type=SetupType.CHARACTER_PROMISE,
            content="Elena promised to uncover the truth",
            characters=["Elena"],
            emotional_weight=0.7,
            expected_timing=PayoffTiming.SHORT_TERM
        )

        # Now detect payoff
        payoffs = self.fpd.detect_payoff(
            beat_id="scene_3",
            content="Elena discovered the hidden letters that revealed everything",
            characters=["Elena"]
        )

        self.assertGreater(len(payoffs), 0)
        payoff = payoffs[0]
        self.assertEqual(payoff.setup_id, setup.setup_id)
        self.assertGreater(payoff.satisfaction_score, 0.0)

    def test_setup_payoff_lattice_health(self):
        """Test lattice health analysis"""
        # Create some setups and payoffs
        setup1 = self.fpd.register_manual_setup(
            beat_id="scene_1",
            setup_type=SetupType.CHARACTER_PROMISE,
            content="Promise 1",
            characters=["Elena"],
            emotional_weight=0.6,
            expected_timing=PayoffTiming.SHORT_TERM
        )

        payoff1 = self.fpd.register_manual_payoff(
            beat_id="scene_2",
            setup_id=setup1.setup_id,
            payoff_type=PayoffType.DIRECT_FULFILLMENT,
            content="Promise fulfilled",
            characters=["Elena"],
            satisfaction_score=0.9
        )

        health = self.fpd.analyze_lattice_health()

        self.assertGreater(health['total_setups'], 0)
        self.assertGreater(health['total_payoffs'], 0)
        self.assertGreater(health['connection_health'], 0.5)

    def test_timing_accuracy_calculation(self):
        """Test payoff timing accuracy"""
        # Create setup with short-term expectation
        setup = self.fpd.register_manual_setup(
            beat_id="scene_1",
            setup_type=SetupType.THREAT_ESTABLISHMENT,
            content="Danger is coming",
            characters=["Elena"],
            emotional_weight=0.8,
            expected_timing=PayoffTiming.SHORT_TERM
        )

        # Create payoff in expected timeframe
        payoff = self.fpd.register_manual_payoff(
            beat_id="scene_3",  # 2 beats later
            setup_id=setup.setup_id,
            payoff_type=PayoffType.DIRECT_FULFILLMENT,
            content="Danger arrives",
            characters=["Elena"],
            satisfaction_score=0.8
        )

        self.assertGreater(payoff.timing_accuracy, 0.5)

    def test_rip_integration_export(self):
        """Test RIP integration export"""
        # Create setup without payoff (orphaned)
        self.fpd.register_manual_setup(
            beat_id="scene_1",
            setup_type=SetupType.MYSTERY_ELEMENT,
            content="Unsolved mystery",
            characters=["Elena"],
            emotional_weight=0.5,
            expected_timing=PayoffTiming.MEDIUM_TERM
        )

        export_data = self.fpd.export_for_rip_integration()

        self.assertEqual(export_data['tracker_type'], 'FPD')
        self.assertIn('lattice_health', export_data)
        self.assertIn('orphaned_setups', export_data)

    def test_tracker_health(self):
        """Test tracker health metrics"""
        health = self.fpd.get_tracker_health()

        self.assertEqual(health['tracker_type'], 'FPD')
        self.assertIn('connection_health', health)
        self.assertIn('health_score', health)


class TestRecursiveInquiryEngine(unittest.TestCase):
    """Test RIE-lite (Recursive Inquiry Engine) functionality"""

    def setUp(self):
        self.rie = RecursiveInquiryEngine()

    def test_inquiry_generation(self):
        """Test generating contextual inquiries"""
        context = {
            'character': 'Elena',
            'action': 'examining the mirror',
            'emotion': 'fearful',
            'location': 'attic'
        }

        inquiry = self.rie.generate_inquiry(
            seed_id="seed_1",
            beat_id="scene_1",
            context=context,
            inquiry_type=InquiryType.CHARACTER_DEPTH,
            depth=InquiryDepth.MEDIUM
        )

        self.assertIsNotNone(inquiry)
        self.assertEqual(inquiry.inquiry_type, InquiryType.CHARACTER_DEPTH)
        self.assertEqual(inquiry.depth, InquiryDepth.MEDIUM)
        self.assertIn('Elena', inquiry.question)

    def test_followup_inquiry_generation(self):
        """Test generating followup inquiries based on answers"""
        # Create initial inquiry
        context = {
            'character': 'Elena',
            'emotion': 'confused'
        }

        inquiry = self.rie.generate_inquiry(
            seed_id="seed_1",
            beat_id="scene_1",
            context=context,
            inquiry_type=InquiryType.EMOTIONAL_EXPLORATION
        )

        self.assertIsNotNone(inquiry)

        # Generate followups based on answer
        answer = "Elena felt confused because Marcus had betrayed her trust"
        followups = self.rie.generate_followup_inquiry(inquiry.inquiry_id, answer)

        # Should detect new exploration paths (Marcus, betrayal)
        self.assertGreater(len(inquiry.exploration_paths), 0)

    def test_inquiry_type_suggestion(self):
        """Test inquiry type suggestion based on context"""
        # Character depth context
        context1 = {'character': 'Elena', 'emotion': 'angry'}
        suggestion1 = self.rie.suggest_inquiry_type(context1)
        self.assertEqual(suggestion1, InquiryType.EMOTIONAL_EXPLORATION)

        # Relationship context
        context2 = {'character1': 'Elena', 'character2': 'Marcus'}
        suggestion2 = self.rie.suggest_inquiry_type(context2)
        self.assertEqual(suggestion2, InquiryType.RELATIONSHIP_DYNAMICS)

        # World building context
        context3 = {'location': 'mirror_realm', 'world': 'fantasy'}
        suggestion3 = self.rie.suggest_inquiry_type(context3)
        self.assertEqual(suggestion3, InquiryType.WORLD_BUILDING)

    def test_seed_and_beat_inquiry_retrieval(self):
        """Test retrieving inquiries by seed and beat"""
        context = {'character': 'Elena'}

        # Generate inquiries for multiple beats
        inquiry1 = self.rie.generate_inquiry("seed_1", "beat_1", context)
        inquiry2 = self.rie.generate_inquiry("seed_1", "beat_2", context)
        inquiry3 = self.rie.generate_inquiry("seed_2", "beat_1", context)

        # Test seed retrieval
        seed1_inquiries = self.rie.get_seed_inquiries("seed_1")
        self.assertEqual(len(seed1_inquiries), 2)

        # Test beat retrieval
        beat1_inquiries = self.rie.get_beat_inquiries("beat_1")
        self.assertEqual(len(beat1_inquiries), 2)

    def test_inquiry_pattern_analysis(self):
        """Test analysis of inquiry patterns"""
        # Generate multiple inquiries
        contexts = [
            {'character': 'Elena', 'emotion': 'fear'},
            {'character': 'Marcus', 'action': 'hiding'},
            {'character1': 'Elena', 'character2': 'Marcus'}
        ]

        for i, context in enumerate(contexts):
            self.rie.generate_inquiry(f"seed_{i}", f"beat_{i}", context)

        analysis = self.rie.analyze_inquiry_patterns()

        self.assertGreater(analysis['total_inquiries'], 0)
        self.assertIn('type_distribution', analysis)
        self.assertIn('depth_distribution', analysis)

    def test_rip_integration_export(self):
        """Test RIP integration export"""
        # Generate some inquiries
        context = {'character': 'Elena', 'emotion': 'confused'}
        inquiry = self.rie.generate_inquiry("seed_1", "beat_1", context, depth=InquiryDepth.RECURSIVE)

        export_data = self.rie.export_for_rip_integration()

        self.assertEqual(export_data['tracker_type'], 'RIE-lite')
        self.assertIn('inquiry_analysis', export_data)
        self.assertIn('unanswered_inquiries', export_data)

    def test_tracker_health(self):
        """Test tracker health metrics"""
        health = self.rie.get_tracker_health()

        self.assertEqual(health['tracker_type'], 'RIE-lite')
        self.assertIn('total_inquiries', health)
        self.assertIn('health_score', health)

    def test_inquiry_reset(self):
        """Test resetting inquiries for a seed"""
        context = {'character': 'Elena'}

        # Generate inquiries
        self.rie.generate_inquiry("seed_1", "beat_1", context)
        self.rie.generate_inquiry("seed_1", "beat_2", context)

        self.assertEqual(len(self.rie.get_seed_inquiries("seed_1")), 2)

        # Reset seed inquiries
        self.rie.reset_seed_inquiries("seed_1")

        self.assertEqual(len(self.rie.get_seed_inquiries("seed_1")), 0)


class TestEleganceModulesIntegration(unittest.TestCase):
    """Test integration between elegance modules"""

    def setUp(self):
        self.eat = EmotionalArcTracker()
        self.fpd = ForeshadowingPayoffDetector()
        self.rie = RecursiveInquiryEngine()

    def test_cross_module_data_flow(self):
        """Test data flow between modules"""
        # Simulate a narrative beat with all three modules
        beat_id = "integration_scene_1"
        character = "Elena"

        # 1. EAT: Track emotional state
        emotional_point = self.eat.track_emotional_point(
            character_name=character,
            beat_id=beat_id,
            emotion_type="determination",
            intensity=EmotionalIntensity.STRONG,
            valence=EmotionalValence.POSITIVE,
            context="Deciding to confront the truth"
        )

        beat_state = self.eat.create_beat_state(
            character_name=character,
            beat_id=beat_id,
            primary_emotion=emotional_point
        )

        # 2. FPD: Create setup based on emotional state
        setup = self.fpd.register_manual_setup(
            beat_id=beat_id,
            setup_type=SetupType.CHARACTER_PROMISE,
            content=f"{character} promises to face whatever truth the mirror reveals",
            characters=[character],
            emotional_weight=emotional_point.intensity.value,
            expected_timing=PayoffTiming.SHORT_TERM
        )

        # 3. RIE: Generate inquiry based on setup and emotion
        context = {
            'character': character,
            'emotion': emotional_point.emotion_type,
            'promise': setup.content,
            'beat_id': beat_id
        }

        inquiry = self.rie.generate_inquiry(
            seed_id="integration_seed",
            beat_id=beat_id,
            context=context,
            inquiry_type=InquiryType.CHARACTER_DEPTH
        )

        # Verify integration
        self.assertIsNotNone(emotional_point)
        self.assertIsNotNone(setup)
        self.assertIsNotNone(inquiry)
        self.assertIn(character, inquiry.question)

    def test_combined_health_metrics(self):
        """Test combined health metrics from all modules"""
        # Add some data to each module
        self.eat.track_emotional_point("Elena", "scene_1", "joy",
                                     EmotionalIntensity.MODERATE, EmotionalValence.POSITIVE, "test")
        self.fpd.register_manual_setup("scene_1", SetupType.CHARACTER_PROMISE, "test", ["Elena"], 0.5, PayoffTiming.SHORT_TERM)
        self.rie.generate_inquiry("seed_1", "scene_1", {'character': 'Elena'})

        # Get health from all modules
        eat_health = self.eat.get_tracker_health()
        fpd_health = self.fpd.get_tracker_health()
        rie_health = self.rie.get_tracker_health()

        combined_health = {
            'eat': eat_health,
            'fpd': fpd_health,
            'rie': rie_health,
            'overall_score': (eat_health['health_score'] + fpd_health['health_score'] + rie_health['health_score']) / 3
        }

        self.assertIn('eat', combined_health)
        self.assertIn('fpd', combined_health)
        self.assertIn('rie', combined_health)
        self.assertGreaterEqual(combined_health['overall_score'], 0.0)

    def test_rip_constraint_compatibility(self):
        """Test that all modules generate compatible RIP constraints"""
        # Generate RIP export data from all modules
        eat_export = self.eat.export_for_rip_integration()
        fpd_export = self.fpd.export_for_rip_integration()
        rie_export = self.rie.export_for_rip_integration()

        # Verify all exports have required structure
        for export_data in [eat_export, fpd_export, rie_export]:
            self.assertIn('tracker_type', export_data)
            self.assertIn(export_data['tracker_type'], ['EAT', 'FPD', 'RIE-lite'])

        # Verify tracker types are unique
        tracker_types = {export_data['tracker_type'] for export_data in [eat_export, fpd_export, rie_export]}
        self.assertEqual(len(tracker_types), 3)


if __name__ == '__main__':
    # Run all tests
    unittest.main(verbosity=2)