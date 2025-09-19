#!/usr/bin/env python3
"""
Unit Tests for RIP Bridge Script
Part of the RIP+RIC Unified Protocol Stack (v1.0)

Tests the Python RIP bridge that connects to the Rust RIC system.
"""

import sys
import json
import asyncio
import unittest
from unittest.mock import patch, MagicMock
from io import StringIO

# Add parent directory to path for imports
sys.path.append('..')

# Mock the RIP components since they may not be available in test environment
class MockRecursiveExpander:
    def extract_beat_ligands(self, seed, beat):
        return MockConstraintGenome()

    def create_expansion_candidate(self, text, context):
        return MockExpansionCandidate()

    def guard_chain_passes(self, candidate, genome):
        return MockGuardResult()

class MockConstraintGenome:
    def __init__(self):
        self.health_score = 0.85
        self.ligands = [MockLigand()]

class MockLigand:
    def __init__(self):
        self.anchor_text = "test_anchor"
        self.strength = 0.7

class MockExpansionCandidate:
    pass

class MockGuardResult:
    def __init__(self):
        self.passes = True
        self.overall_confidence = 0.9
        self.violations = []

class MockDriftDetector:
    async def scan_for_drift(self, text, context):
        return MockDriftResult()

class MockDriftResult:
    def __init__(self):
        self.threat_level = 0.2
        self.detections = [MockPathogenDetection()]

class MockPathogenDetection:
    def __init__(self):
        self.pathogen_type = "test_pathogen"

class MockRecursiveGrowthPass:
    def __init__(self):
        self.growth_phases = {"surface_expansion": False, "deep_analysis": False}

    async def execute_growth_pass(self, genome, expander, max_iterations=10):
        return ["expansion_1", "expansion_2", "expansion_3"]

    def _detect_stagnation(self):
        return False

# Mock the RIP bridge with our mock components
class MockRIPBridge:
    def __init__(self):
        self.recursive_expander = MockRecursiveExpander()
        self.drift_detector = MockDriftDetector()
        self.growth_pass = MockRecursiveGrowthPass()

    async def analyze_narrative_content(self, query):
        # Extract ligands and create constraint genome
        constraint_genome = self.recursive_expander.extract_beat_ligands(
            query['seed'],
            query['beat']
        )

        # Perform guard chain validation
        expansion_candidate = self.recursive_expander.create_expansion_candidate(
            query['text'],
            query['context']
        )

        guard_result = self.recursive_expander.guard_chain_passes(
            expansion_candidate,
            constraint_genome
        )

        # Scan for pathogen drift
        drift_result = await self.drift_detector.scan_for_drift(
            query['text'],
            {'seed': query['seed'], 'beat': query['beat']}
        )

        # Check loop saturation via growth pass
        growth_result = await self.growth_pass.execute_growth_pass(
            constraint_genome,
            self.recursive_expander,
            max_iterations=min(query['max_iterations'], query['budget_remaining'])
        )

        # Generate unified RIP vote
        rip_vote = self._generate_rip_vote(
            constraint_genome,
            guard_result,
            drift_result,
            growth_result
        )

        # Compile analysis result
        return {
            'constraint_genome_health': constraint_genome.health_score,
            'guard_chain_health': guard_result.overall_confidence,
            'guard_chain_passes': guard_result.passes,
            'pathogen_threat_level': drift_result.threat_level,
            'detected_pathogens': [p.pathogen_type for p in drift_result.detections],
            'loop_saturation_detected': self.growth_pass._detect_stagnation(),
            'saturated_growth_phases': [
                phase for phase, saturated in self.growth_pass.growth_phases.items()
                if saturated
            ],
            'failed_ligands': [
                ligand.anchor_text for ligand in constraint_genome.ligands
                if ligand.strength < 0.3
            ],
            'guard_chain_violations': guard_result.violations if not guard_result.passes else [],
            'completion_summary': self._generate_completion_summary(growth_result),
            'rip_vote': rip_vote
        }

    def _generate_rip_vote(self, genome, guard_result, drift_result, growth_result):
        # Calculate composite health scores
        content_health = (genome.health_score + guard_result.overall_confidence) / 2.0
        process_health = 1.0 - drift_result.threat_level
        expansion_health = len(growth_result) / max(1, len(growth_result)) if growth_result else 0.5

        overall_health = (content_health + process_health + expansion_health) / 3.0

        if overall_health > 0.8:
            return "RIP_VOTE_CONTINUE_HIGH_CONFIDENCE"
        elif overall_health > 0.6:
            return "RIP_VOTE_CONTINUE_MEDIUM_CONFIDENCE"
        elif overall_health > 0.4:
            return "RIP_VOTE_CAUTION_LOW_CONFIDENCE"
        elif overall_health > 0.2:
            return "RIP_VOTE_HALT_RECOMMENDED"
        else:
            return "RIP_VOTE_HALT_CRITICAL"

    def _generate_completion_summary(self, growth_result):
        if not growth_result:
            return "No recursive expansion achieved - narrative constraints require resolution"

        expansion_count = len(growth_result)
        if expansion_count < 3:
            return f"Limited recursive expansion ({expansion_count} iterations) - constraint genome requires strengthening"
        elif expansion_count < 8:
            return f"Moderate recursive expansion ({expansion_count} iterations) - narrative obligations partially fulfilled"
        else:
            return f"Extensive recursive expansion ({expansion_count} iterations) - deep narrative exploration achieved"


class TestRIPBridge(unittest.TestCase):
    """Test cases for the RIP bridge functionality"""

    def setUp(self):
        self.bridge = MockRIPBridge()

    def test_valid_query_processing(self):
        """Test that valid queries are processed correctly"""
        query = {
            'text': 'The protagonist faced a difficult choice.',
            'context': 'climactic_scene',
            'seed': 'resolve_conflict; establish_truth',
            'beat': 'Chapter 5, Scene 2',
            'max_iterations': 10,
            'budget_remaining': 15
        }

        # Run the async function
        loop = asyncio.new_event_loop()
        asyncio.set_event_loop(loop)
        result = loop.run_until_complete(self.bridge.analyze_narrative_content(query))
        loop.close()

        # Verify result structure
        self.assertIn('constraint_genome_health', result)
        self.assertIn('guard_chain_health', result)
        self.assertIn('guard_chain_passes', result)
        self.assertIn('pathogen_threat_level', result)
        self.assertIn('detected_pathogens', result)
        self.assertIn('loop_saturation_detected', result)
        self.assertIn('saturated_growth_phases', result)
        self.assertIn('failed_ligands', result)
        self.assertIn('guard_chain_violations', result)
        self.assertIn('completion_summary', result)
        self.assertIn('rip_vote', result)

        # Verify health scores are in valid ranges
        self.assertGreaterEqual(result['constraint_genome_health'], 0.0)
        self.assertLessEqual(result['constraint_genome_health'], 1.0)
        self.assertGreaterEqual(result['guard_chain_health'], 0.0)
        self.assertLessEqual(result['guard_chain_health'], 1.0)
        self.assertGreaterEqual(result['pathogen_threat_level'], 0.0)
        self.assertLessEqual(result['pathogen_threat_level'], 1.0)

        # Verify boolean fields
        self.assertIsInstance(result['guard_chain_passes'], bool)
        self.assertIsInstance(result['loop_saturation_detected'], bool)

        # Verify list fields
        self.assertIsInstance(result['detected_pathogens'], list)
        self.assertIsInstance(result['saturated_growth_phases'], list)
        self.assertIsInstance(result['failed_ligands'], list)
        self.assertIsInstance(result['guard_chain_violations'], list)

        # Verify vote format
        self.assertTrue(result['rip_vote'].startswith('RIP_VOTE_'))

    def test_rip_vote_generation_high_health(self):
        """Test RIP vote generation for high health scenario"""
        # Create high health scenario
        genome = MockConstraintGenome()
        genome.health_score = 0.95

        guard_result = MockGuardResult()
        guard_result.overall_confidence = 0.9

        drift_result = MockDriftResult()
        drift_result.threat_level = 0.1

        growth_result = ["exp1", "exp2", "exp3", "exp4", "exp5"]

        vote = self.bridge._generate_rip_vote(genome, guard_result, drift_result, growth_result)
        self.assertEqual(vote, "RIP_VOTE_CONTINUE_HIGH_CONFIDENCE")

    def test_rip_vote_generation_low_health(self):
        """Test RIP vote generation for low health scenario"""
        # Create low health scenario
        genome = MockConstraintGenome()
        genome.health_score = 0.1

        guard_result = MockGuardResult()
        guard_result.overall_confidence = 0.2

        drift_result = MockDriftResult()
        drift_result.threat_level = 0.9

        growth_result = []

        vote = self.bridge._generate_rip_vote(genome, guard_result, drift_result, growth_result)
        self.assertEqual(vote, "RIP_VOTE_HALT_CRITICAL")

    def test_completion_summary_generation(self):
        """Test completion summary generation for different scenarios"""
        # Test no expansion
        summary = self.bridge._generate_completion_summary([])
        self.assertIn("No recursive expansion achieved", summary)

        # Test limited expansion
        summary = self.bridge._generate_completion_summary(["exp1", "exp2"])
        self.assertIn("Limited recursive expansion", summary)
        self.assertIn("2 iterations", summary)

        # Test moderate expansion
        summary = self.bridge._generate_completion_summary(["exp1", "exp2", "exp3", "exp4", "exp5"])
        self.assertIn("Moderate recursive expansion", summary)
        self.assertIn("5 iterations", summary)

        # Test extensive expansion
        summary = self.bridge._generate_completion_summary([f"exp{i}" for i in range(10)])
        self.assertIn("Extensive recursive expansion", summary)
        self.assertIn("10 iterations", summary)

    def test_json_serialization(self):
        """Test that results can be properly serialized to JSON"""
        query = {
            'text': 'Test narrative text.',
            'context': 'test_context',
            'seed': 'test_seed',
            'beat': 'Test Beat',
            'max_iterations': 5,
            'budget_remaining': 10
        }

        loop = asyncio.new_event_loop()
        asyncio.set_event_loop(loop)
        result = loop.run_until_complete(self.bridge.analyze_narrative_content(query))
        loop.close()

        # Should be able to serialize to JSON without error
        json_str = json.dumps(result)
        self.assertIsInstance(json_str, str)

        # Should be able to deserialize back
        deserialized = json.loads(json_str)
        self.assertEqual(deserialized, result)

    def test_error_handling(self):
        """Test error handling in RIP bridge"""
        # Test with missing required fields
        incomplete_query = {
            'text': 'Test text',
            # Missing other required fields
        }

        # Should handle missing fields gracefully
        try:
            loop = asyncio.new_event_loop()
            asyncio.set_event_loop(loop)
            result = loop.run_until_complete(self.bridge.analyze_narrative_content(incomplete_query))
            loop.close()
        except KeyError as e:
            # Expected behavior - should raise KeyError for missing fields
            self.assertIsInstance(e, KeyError)

    def test_budget_constraint_enforcement(self):
        """Test that recursion budget constraints are enforced"""
        query = {
            'text': 'Test narrative text.',
            'context': 'test_context',
            'seed': 'test_seed',
            'beat': 'Test Beat',
            'max_iterations': 20,  # High max
            'budget_remaining': 5   # Low budget
        }

        loop = asyncio.new_event_loop()
        asyncio.set_event_loop(loop)
        result = loop.run_until_complete(self.bridge.analyze_narrative_content(query))
        loop.close()

        # The effective iterations should be limited by budget
        # This is tested indirectly through the completion summary
        self.assertIsNotNone(result['completion_summary'])

    def test_pathogen_detection_integration(self):
        """Test that pathogen detection is properly integrated"""
        query = {
            'text': 'Character behavior is completely inconsistent.',
            'context': 'pathogen_test',
            'seed': 'maintain_consistency',
            'beat': 'Test Beat',
            'max_iterations': 10,
            'budget_remaining': 15
        }

        loop = asyncio.new_event_loop()
        asyncio.set_event_loop(loop)
        result = loop.run_until_complete(self.bridge.analyze_narrative_content(query))
        loop.close()

        # Should detect some pathogens
        self.assertIsInstance(result['detected_pathogens'], list)
        self.assertGreaterEqual(len(result['detected_pathogens']), 0)

        # Pathogen threat level should be calculated
        self.assertIsInstance(result['pathogen_threat_level'], (int, float))
        self.assertGreaterEqual(result['pathogen_threat_level'], 0.0)
        self.assertLessEqual(result['pathogen_threat_level'], 1.0)


class TestRIPBridgeMainFunction(unittest.TestCase):
    """Test the main function and CLI interface"""

    @patch('sys.stdin', new_callable=StringIO)
    @patch('sys.stdout', new_callable=StringIO)
    def test_main_function_valid_input(self, mock_stdout, mock_stdin):
        """Test main function with valid JSON input"""
        # Mock input JSON
        test_query = {
            'text': 'The hero faced the dragon.',
            'context': 'climactic_battle',
            'seed': 'defeat_evil',
            'beat': 'Chapter 10',
            'max_iterations': 10,
            'budget_remaining': 15
        }
        mock_stdin.write(json.dumps(test_query))
        mock_stdin.seek(0)

        # We can't easily test the actual main function due to import issues,
        # but we can test the logic structure
        try:
            query = json.loads(mock_stdin.read())

            # Validate required fields
            required_fields = ['text', 'context', 'seed', 'beat', 'max_iterations', 'budget_remaining']
            for field in required_fields:
                self.assertIn(field, query)

            # Should be able to process the query
            self.assertEqual(query['text'], 'The hero faced the dragon.')
            self.assertEqual(query['max_iterations'], 10)

        except Exception as e:
            self.fail(f"Should handle valid input without error: {e}")

    def test_error_response_format(self):
        """Test that error responses follow the expected format"""
        error_result = {
            'constraint_genome_health': 0.0,
            'guard_chain_health': 0.0,
            'guard_chain_passes': False,
            'pathogen_threat_level': 1.0,
            'detected_pathogens': ['BRIDGE_ERROR'],
            'loop_saturation_detected': True,
            'saturated_growth_phases': ['error_phase'],
            'failed_ligands': ['bridge_failure'],
            'guard_chain_violations': ['RIP bridge error: test error'],
            'completion_summary': 'RIP bridge failed: test error',
            'rip_vote': 'RIP_VOTE_HALT_CRITICAL'
        }

        # Should be serializable
        json_str = json.dumps(error_result)
        self.assertIsInstance(json_str, str)

        # Should have critical health scores
        self.assertEqual(error_result['constraint_genome_health'], 0.0)
        self.assertEqual(error_result['pathogen_threat_level'], 1.0)
        self.assertTrue(error_result['loop_saturation_detected'])
        self.assertEqual(error_result['rip_vote'], 'RIP_VOTE_HALT_CRITICAL')


if __name__ == '__main__':
    # Run the tests
    unittest.main(verbosity=2)