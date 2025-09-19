#!/usr/bin/env python3
"""
RIP Bridge Script - Connects Rust RIC system to Python RIP components
Part of the RIP+RIC Unified Protocol Stack (v1.0)

This script acts as a bridge between the Rust RecursiveNarrativeAssistant
and the Python RIP (Recursive Integrity Protocol) components.
"""

import sys
import json
import asyncio
import os
from pathlib import Path
from typing import Dict, List, Any, Optional

# Import RIP components
sys.path.append('../dreamspiral')
from recursive_expander import RecursiveExpander, ConstraintGenome
from drift_detector import DriftDetector, PathogenFingerprint
from recursive_growth_pass import RecursiveGrowthPass, RecursionBudget


class RIPBridge:
    """Bridge between Rust RIC and Python RIP components"""

    def __init__(self):
        self.recursive_expander = RecursiveExpander()
        self.drift_detector = DriftDetector()
        self.growth_pass = RecursiveGrowthPass()

    async def analyze_narrative_content(self, query: Dict[str, Any]) -> Dict[str, Any]:
        """
        Perform unified RIP analysis on narrative content

        Args:
            query: Dictionary containing:
                - text: Text to analyze
                - context: Narrative context
                - seed: Seed constraints/obligations
                - beat: Current narrative beat
                - max_iterations: Maximum recursive iterations
                - budget_remaining: Current recursion budget

        Returns:
            Dictionary containing RIP analysis results
        """

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
        """Generate unified RIP vote based on all subsystem analysis"""

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
        """Generate completion summary for continuity floor scenarios"""
        if not growth_result:
            return "No recursive expansion achieved - narrative constraints require resolution"

        expansion_count = len(growth_result)
        if expansion_count < 3:
            return f"Limited recursive expansion ({expansion_count} iterations) - constraint genome requires strengthening"
        elif expansion_count < 8:
            return f"Moderate recursive expansion ({expansion_count} iterations) - narrative obligations partially fulfilled"
        else:
            return f"Extensive recursive expansion ({expansion_count} iterations) - deep narrative exploration achieved"

    def cache_constraint_snapshot(self, chapter: int, scene: Optional[int],
                                freedom_score: float, constraints: List[str],
                                pressures: Dict[str, float]) -> str:
        """Cache a constraint snapshot for cross-language access"""
        snapshot = {
            'freedom_score': freedom_score,
            'active_constraints': constraints,
            'constraint_pressures': pressures,
            'timestamp': int(__import__('time').time()),
            'chapter': chapter,
            'scene': scene
        }

        cache_dir = Path.home() / '.shimmy'
        cache_dir.mkdir(exist_ok=True)

        key = f"ch{chapter}_sc{scene}"
        cache_file = cache_dir / f"constraint_{key}.json"

        with open(cache_file, 'w') as f:
            json.dump(snapshot, f, indent=2)

        return str(cache_file)

    def cache_capr_summary(self, chapter: int, scene: Optional[int],
                          loop_count: int, return_vector: List[str],
                          contradictions: List[str], pressure_points: List[str],
                          loop_quality: float) -> str:
        """Cache a CAPR path summary for cross-language access"""
        summary = {
            'loop_count': loop_count,
            'last_return_vector': return_vector,
            'active_contradictions': contradictions,
            'pressure_points': pressure_points,
            'avg_loop_duration': 0.0,  # Would need timing data
            'last_loop_quality': loop_quality,
            'chapter': chapter,
            'scene': scene,
            'timestamp': int(__import__('time').time())
        }

        cache_dir = Path.home() / '.shimmy'
        cache_dir.mkdir(exist_ok=True)

        key = f"capr_ch{chapter}_sc{scene}"
        cache_file = cache_dir / f"capr_{key}.json"

        with open(cache_file, 'w') as f:
            json.dump(summary, f, indent=2)

        return str(cache_file)

    def cache_character_emotions(self, character: str, chapter: int, scene: Optional[int],
                               valence_seq: List[float], intensity_seq: List[float],
                               emotions: List[str], turning_points: List[int]) -> str:
        """Cache character emotion arc for cross-language access"""
        arc = {
            'character': character,
            'valence_sequence': valence_seq,
            'intensity_sequence': intensity_seq,
            'dominant_emotions': emotions,
            'turning_points': turning_points,
            'arc_trend': 'stable',  # Simplified
            'chapter': chapter,
            'scene_range': [scene, scene],
            'timestamp': int(__import__('time').time())
        }

        cache_dir = Path.home() / '.shimmy'
        cache_dir.mkdir(exist_ok=True)

        key = f"{character}ch{chapter}_sc{scene}"
        cache_file = cache_dir / f"emotion_{key}.json"

        with open(cache_file, 'w') as f:
            json.dump(arc, f, indent=2)

        return str(cache_file)

    def load_cached_data(self, cache_type: str, key: str) -> Optional[Dict[str, Any]]:
        """Load cached data by type and key"""
        cache_dir = Path.home() / '.shimmy'
        cache_file = cache_dir / f"{cache_type}_{key}.json"

        if cache_file.exists():
            try:
                with open(cache_file, 'r') as f:
                    return json.load(f)
            except Exception:
                return None
        return None

    async def cache_operation(self, operation: Dict[str, Any]) -> Dict[str, Any]:
        """Handle cache operations from Rust side"""
        op_type = operation.get('type')

        if op_type == 'cache_constraint':
            file_path = self.cache_constraint_snapshot(
                operation['chapter'],
                operation.get('scene'),
                operation['freedom_score'],
                operation['constraints'],
                operation['pressures']
            )
            return {'success': True, 'file_path': file_path}

        elif op_type == 'cache_capr':
            file_path = self.cache_capr_summary(
                operation['chapter'],
                operation.get('scene'),
                operation['loop_count'],
                operation['return_vector'],
                operation['contradictions'],
                operation['pressure_points'],
                operation['loop_quality']
            )
            return {'success': True, 'file_path': file_path}

        elif op_type == 'cache_emotion':
            file_path = self.cache_character_emotions(
                operation['character'],
                operation['chapter'],
                operation.get('scene'),
                operation['valence_sequence'],
                operation['intensity_sequence'],
                operation['emotions'],
                operation['turning_points']
            )
            return {'success': True, 'file_path': file_path}

        elif op_type == 'load_cache':
            data = self.load_cached_data(
                operation['cache_type'],
                operation['key']
            )
            return {'success': data is not None, 'data': data}

        else:
            return {'success': False, 'error': f'Unknown cache operation: {op_type}'}


async def main():
    """Main entry point for RIP bridge"""
    try:
        # Read query from stdin
        query_json = sys.stdin.read().strip()
        if not query_json:
            raise ValueError("No input received")

        query = json.loads(query_json)

        # Initialize bridge
        bridge = RIPBridge()

        # Check if this is a cache operation
        if 'operation_type' in query and query['operation_type'] == 'cache':
            result = await bridge.cache_operation(query)
        else:
            # Validate required fields for narrative analysis
            required_fields = ['text', 'context', 'seed', 'beat', 'max_iterations', 'budget_remaining']
            for field in required_fields:
                if field not in query:
                    raise ValueError(f"Missing required field: {field}")

            # Perform narrative analysis
            result = await bridge.analyze_narrative_content(query)

        # Output result as JSON
        print(json.dumps(result, indent=2))

    except Exception as e:
        # Output error in JSON format for consistent parsing
        error_result = {
            'constraint_genome_health': 0.0,
            'guard_chain_health': 0.0,
            'guard_chain_passes': False,
            'pathogen_threat_level': 1.0,
            'detected_pathogens': ['BRIDGE_ERROR'],
            'loop_saturation_detected': True,
            'saturated_growth_phases': ['error_phase'],
            'failed_ligands': ['bridge_failure'],
            'guard_chain_violations': [f'RIP bridge error: {str(e)}'],
            'completion_summary': f'RIP bridge failed: {str(e)}',
            'rip_vote': 'RIP_VOTE_HALT_CRITICAL'
        }
        print(json.dumps(error_result, indent=2))
        sys.exit(1)


if __name__ == "__main__":
    asyncio.run(main())