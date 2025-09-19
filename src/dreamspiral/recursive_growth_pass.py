"""
🔄 Recursive Growth Pass - Loop Saturation Control
=================================================

Implements budget-based recursion control for the RIP+RIC unified protocol.
Manages recursive expansion cycles with ZC (Zero-Continuation) gates to prevent
infinite loops while allowing organic narrative growth.

Key Features:
- Budget tracking for recursion depth
- Insight-based budget reset mechanism
- Saturation detection and prevention
- Integration with RIC arbitration system
"""

import asyncio
import logging
from typing import Dict, List, Optional, Tuple, NamedTuple
from dataclasses import dataclass
from enum import Enum
from datetime import datetime, timedelta
import json

# Minimal Elegance Modules (EAT + FPD + RIE-lite) - Light Integration
try:
    from .emotional_arc_tracker import EmotionalArcTracker
    from .foreshadowing_detector import ForeshadowingPayoffDetector
    from .inquiry_bank import RecursiveInquiryEngine
    ELEGANCE_MODULES_AVAILABLE = True
except ImportError:
    ELEGANCE_MODULES_AVAILABLE = False
    logger.warning("Elegance modules (EAT/FPD/RIE-lite) not available - operating in basic mode")

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class GrowthPhase(Enum):
    """Phases of recursive growth"""
    SEEDING = "seeding"          # Initial expansion from seed
    ELABORATION = "elaboration"  # Deepening existing content
    INTEGRATION = "integration"  # Connecting elements
    SATURATION = "saturation"    # No new insights possible
    TERMINATION = "termination"  # Natural completion


class RecursionBudget(Enum):
    """Budget levels for different recursion types"""
    SURFACE_EXPANSION = 5   # Simple elaborations
    DEEP_ANALYSIS = 10      # Complex narrative analysis
    CHARACTER_DEVELOPMENT = 8  # Character-focused expansion
    THEMATIC_EXPLORATION = 12  # Theme and meaning development
    OBLIGATION_RESOLUTION = 15  # Resolving narrative obligations


@dataclass
class GrowthMetrics:
    """Metrics tracking recursive growth quality"""
    total_iterations: int
    successful_expansions: int
    failed_expansions: int
    insights_generated: int
    budget_resets: int
    saturation_events: int
    average_expansion_quality: float
    growth_velocity: float  # Expansions per iteration


@dataclass
class RecursionState:
    """Current state of recursive expansion"""
    current_phase: GrowthPhase
    iteration_count: int
    budget_remaining: int
    initial_budget: int
    last_insight_time: datetime
    stagnation_counter: int
    quality_trend: List[float]  # Recent quality scores
    expansion_history: List[str]
    saturation_detected: bool


class ZCGate:
    """Zero-Continuation Gate - Controls recursive flow"""

    def __init__(self, budget_type: RecursionBudget, subsystem_name: str):
        self.budget_type = budget_type
        self.subsystem_name = subsystem_name
        self.budget_remaining = budget_type.value
        self.initial_budget = budget_type.value
        self.iteration_count = 0
        self.last_reset_time = datetime.now()
        self.stagnation_threshold = 3  # Iterations without insight before considering stagnation

    def can_continue(self) -> bool:
        """Check if recursion can continue"""
        return self.budget_remaining > 0

    def consume_budget(self, amount: int = 1) -> bool:
        """Consume budget for one iteration"""
        if self.budget_remaining >= amount:
            self.budget_remaining -= amount
            self.iteration_count += 1
            return True
        return False

    def reset_on_insight(self, insight_quality: float = 1.0) -> None:
        """Reset budget when new insight is gained"""
        # Partial reset based on insight quality
        reset_amount = int(self.initial_budget * insight_quality)
        self.budget_remaining = min(self.initial_budget, self.budget_remaining + reset_amount)
        self.last_reset_time = datetime.now()
        logger.info(f"ZC Gate reset for {self.subsystem_name}: +{reset_amount} budget")

    def get_saturation_level(self) -> float:
        """Get current saturation level (0.0 = fresh, 1.0 = saturated)"""
        return 1.0 - (self.budget_remaining / self.initial_budget)

    def time_since_last_reset(self) -> timedelta:
        """Time since last budget reset"""
        return datetime.now() - self.last_reset_time


class RecursiveGrowthPass:
    """
    Manages recursive narrative expansion with saturation control.
    Integrates with RIC arbitration and RIP constraint validation.
    """

    def __init__(self, ric_integration=None, drift_detector=None):
        self.ric_integration = ric_integration
        self.drift_detector = drift_detector
        self.zc_gates: Dict[str, ZCGate] = {}
        self.recursion_state = RecursionState(
            current_phase=GrowthPhase.SEEDING,
            iteration_count=0,
            budget_remaining=0,
            initial_budget=0,
            last_insight_time=datetime.now(),
            stagnation_counter=0,
            quality_trend=[],
            expansion_history=[],
            saturation_detected=False
        )

        # Initialize Elegance modules (EAT + FPD + RIE-lite) if available
        self.elegance_modules = {}
        if ELEGANCE_MODULES_AVAILABLE:
            self.elegance_modules = {
                'eat': EmotionalArcTracker(),
                'fpd': ForeshadowingPayoffDetector(),
                'rie': RecursiveInquiryEngine()
            }
            logger.info("Growth Pass: Elegance modules (EAT/FPD/RIE-lite) initialized")
        else:
            logger.info("Growth Pass: Operating without elegance modules")
        self.growth_metrics = GrowthMetrics(
            total_iterations=0,
            successful_expansions=0,
            failed_expansions=0,
            insights_generated=0,
            budget_resets=0,
            saturation_events=0,
            average_expansion_quality=0.0,
            growth_velocity=0.0
        )

    def initialize_growth_session(self, session_type: str = "default") -> None:
        """Initialize a new recursive growth session"""
        logger.info(f"Initializing recursive growth session: {session_type}")

        # Configure ZC gates for different subsystems
        self.zc_gates = {
            "surface_expansion": ZCGate(RecursionBudget.SURFACE_EXPANSION, "surface_expansion"),
            "deep_analysis": ZCGate(RecursionBudget.DEEP_ANALYSIS, "deep_analysis"),
            "character_development": ZCGate(RecursionBudget.CHARACTER_DEVELOPMENT, "character_development"),
            "thematic_exploration": ZCGate(RecursionBudget.THEMATIC_EXPLORATION, "thematic_exploration"),
            "obligation_resolution": ZCGate(RecursionBudget.OBLIGATION_RESOLUTION, "obligation_resolution"),
        }

        # Reset recursion state
        self.recursion_state = RecursionState(
            current_phase=GrowthPhase.SEEDING,
            iteration_count=0,
            budget_remaining=sum(gate.budget_remaining for gate in self.zc_gates.values()),
            initial_budget=sum(gate.initial_budget for gate in self.zc_gates.values()),
            last_insight_time=datetime.now(),
            stagnation_counter=0,
            quality_trend=[],
            expansion_history=[],
            saturation_detected=False
        )

        logger.info(f"Growth session initialized with total budget: {self.recursion_state.initial_budget}")

    async def execute_growth_pass(self, constraint_genome, recursive_expander,
                                max_iterations: int = 20) -> List[str]:
        """
        Execute a complete recursive growth pass with saturation control.

        Args:
            constraint_genome: RIP constraint genome from expander
            recursive_expander: Recursive expander instance
            max_iterations: Maximum iterations before forced termination

        Returns:
            List of validated expansions
        """
        logger.info(f"Starting recursive growth pass (max {max_iterations} iterations)")

        # Light elegance hooks - payoff detection for growth optimization
        if 'fpd' in self.elegance_modules:
            self._check_payoff_opportunities(constraint_genome)

        expansions = []
        iteration = 0

        while iteration < max_iterations and not self.recursion_state.saturation_detected:
            # Update current phase
            self._update_growth_phase()

            # Check RIC integration for global saturation
            if self.ric_integration:
                if not self.ric_integration.can_iterate("recursive_growth_pass"):
                    logger.info("RIC global saturation detected, terminating growth pass")
                    break

            # Check local ZC gates
            active_gates = [gate for gate in self.zc_gates.values() if gate.can_continue()]
            if not active_gates:
                logger.info("All ZC gates saturated, terminating growth pass")
                self.recursion_state.saturation_detected = True
                break

            # Select appropriate gate for this iteration
            selected_gate = self._select_optimal_gate(active_gates)
            if not selected_gate:
                logger.warning("No optimal gate available, terminating growth pass")
                break

            # Execute expansion iteration
            iteration_result = await self._execute_iteration(
                constraint_genome, recursive_expander, selected_gate
            )

            if iteration_result:
                expansions.extend(iteration_result.expansions)
                self._process_iteration_result(iteration_result, selected_gate)
            else:
                self.growth_metrics.failed_expansions += 1
                selected_gate.consume_budget()  # Still consume budget for failed iteration

            iteration += 1
            self.recursion_state.iteration_count = iteration
            self.growth_metrics.total_iterations = iteration

            # Check for stagnation
            if self._detect_stagnation():
                logger.info("Growth stagnation detected, terminating pass")
                break

        # Finalize growth pass
        self._finalize_growth_pass()

        logger.info(f"Recursive growth pass complete: {len(expansions)} expansions, "
                   f"{iteration} iterations, final phase: {self.recursion_state.current_phase.value}")

        return expansions

    def _update_growth_phase(self) -> None:
        """Update current growth phase based on metrics"""
        metrics = self.growth_metrics
        state = self.recursion_state

        if state.iteration_count == 0:
            state.current_phase = GrowthPhase.SEEDING
        elif metrics.insights_generated == 0 and state.iteration_count > 5:
            state.current_phase = GrowthPhase.SATURATION
        elif state.saturation_detected:
            state.current_phase = GrowthPhase.TERMINATION
        elif len(state.quality_trend) >= 3 and all(q > 0.7 for q in state.quality_trend[-3:]):
            state.current_phase = GrowthPhase.INTEGRATION
        elif metrics.successful_expansions > 0:
            state.current_phase = GrowthPhase.ELABORATION
        else:
            state.current_phase = GrowthPhase.SEEDING

    def _select_optimal_gate(self, available_gates: List[ZCGate]) -> Optional[ZCGate]:
        """Select the optimal ZC gate for the current iteration"""
        if not available_gates:
            return None

        # Prioritize based on current phase
        phase = self.recursion_state.current_phase

        priority_map = {
            GrowthPhase.SEEDING: ["surface_expansion", "obligation_resolution"],
            GrowthPhase.ELABORATION: ["deep_analysis", "character_development"],
            GrowthPhase.INTEGRATION: ["thematic_exploration", "character_development"],
            GrowthPhase.SATURATION: ["obligation_resolution"],
            GrowthPhase.TERMINATION: []
        }

        preferred_types = priority_map.get(phase, [])

        # Try to find a preferred gate
        for gate_type in preferred_types:
            for gate in available_gates:
                if gate.subsystem_name == gate_type:
                    return gate

        # Fall back to gate with most remaining budget
        return max(available_gates, key=lambda g: g.budget_remaining)

    async def _execute_iteration(self, constraint_genome, recursive_expander,
                                selected_gate: ZCGate) -> Optional['IterationResult']:
        """Execute a single growth iteration"""
        logger.debug(f"Executing iteration with gate: {selected_gate.subsystem_name}")

        try:
            # Generate expansion candidates
            candidates = await recursive_expander._generate_expansion_candidates(
                constraint_genome, self.recursion_state.expansion_history
            )

            if not candidates:
                return None

            # Filter candidates based on current phase and gate type
            filtered_candidates = self._filter_candidates_for_gate(candidates, selected_gate)

            if not filtered_candidates:
                return None

            # Validate candidates through guard chain
            validated_expansions = []
            quality_scores = []

            for candidate in filtered_candidates:
                # Apply guard chain
                guard_result = recursive_expander.guard_chain_passes(candidate, constraint_genome)

                if guard_result.value == "pass":  # Assuming guard_result has a value attribute
                    # Apply drift detection if available
                    if self.drift_detector:
                        drift_result, vote = await self.drift_detector.scan_and_vote(candidate.content)
                        if vote in ["Continue", "Suggest"]:
                            validated_expansions.append(candidate.content)
                            quality_scores.append(self._calculate_expansion_quality(candidate, drift_result))
                    else:
                        validated_expansions.append(candidate.content)
                        quality_scores.append(self._calculate_expansion_quality(candidate))

            if validated_expansions:
                return IterationResult(
                    expansions=validated_expansions,
                    quality_scores=quality_scores,
                    gate_used=selected_gate.subsystem_name,
                    insight_generated=len(validated_expansions) > 0
                )

            return None

        except Exception as e:
            logger.error(f"Error in iteration execution: {e}")
            return None

    def _filter_candidates_for_gate(self, candidates: List, gate: ZCGate) -> List:
        """Filter candidates based on gate type and current phase"""
        # Simple filtering based on gate type
        # In practice, this would be more sophisticated

        if gate.subsystem_name == "surface_expansion":
            # Prefer simpler, shorter expansions
            return [c for c in candidates if len(c.content) < 200]
        elif gate.subsystem_name == "deep_analysis":
            # Prefer longer, more complex expansions
            return [c for c in candidates if len(c.content) >= 100]
        elif gate.subsystem_name == "character_development":
            # Prefer character-focused expansions
            return [c for c in candidates if any(char in c.content.lower()
                   for char in ["he", "she", "they", "character", "person"])]
        elif gate.subsystem_name == "thematic_exploration":
            # Prefer theme-related expansions
            return [c for c in candidates if any(theme in c.content.lower()
                   for theme in ["meaning", "significance", "represents", "symbolizes"])]
        elif gate.subsystem_name == "obligation_resolution":
            # Prefer obligation-addressing expansions
            return [c for c in candidates if c.obligations_addressed]

        return candidates

    def _calculate_expansion_quality(self, candidate, drift_result=None) -> float:
        """Calculate quality score for an expansion"""
        base_quality = 0.5  # Baseline quality

        # Content length factor (prefer moderate length)
        length_factor = min(1.0, len(candidate.content) / 150)
        if length_factor > 0.8:
            length_factor = 1.0 - (length_factor - 0.8) * 2  # Penalize very long content

        # Obligation resolution factor
        obligation_factor = 0.3 if candidate.obligations_addressed else 0.0

        # Drift health factor
        drift_factor = 0.0
        if drift_result:
            drift_factor = drift_result.overall_health_score * 0.3

        # Depth appropriateness factor
        depth_factor = min(0.2, candidate.proposed_depth * 0.05)

        total_quality = base_quality + length_factor * 0.3 + obligation_factor + drift_factor + depth_factor
        return min(1.0, total_quality)

    def _process_iteration_result(self, result: 'IterationResult', gate: ZCGate) -> None:
        """Process the results of an iteration"""
        self.growth_metrics.successful_expansions += len(result.expansions)

        # Update quality trend
        avg_quality = sum(result.quality_scores) / len(result.quality_scores) if result.quality_scores else 0.0
        self.recursion_state.quality_trend.append(avg_quality)

        # Keep only recent quality scores
        if len(self.recursion_state.quality_trend) > 10:
            self.recursion_state.quality_trend.pop(0)

        # Update expansion history
        self.recursion_state.expansion_history.extend(result.expansions)

        # Consume gate budget
        gate.consume_budget()

        # Check for insight and reset if quality is high
        if result.insight_generated and avg_quality > 0.7:
            gate.reset_on_insight(avg_quality)
            self.growth_metrics.insights_generated += 1
            self.growth_metrics.budget_resets += 1
            self.recursion_state.last_insight_time = datetime.now()
            self.recursion_state.stagnation_counter = 0
        else:
            self.recursion_state.stagnation_counter += 1

    def _detect_stagnation(self) -> bool:
        """Detect if growth has stagnated"""
        # Check stagnation counter
        if self.recursion_state.stagnation_counter >= 5:
            return True

        # Check quality trend
        if len(self.recursion_state.quality_trend) >= 5:
            recent_avg = sum(self.recursion_state.quality_trend[-5:]) / 5
            if recent_avg < 0.3:
                return True

        # Check time since last insight
        time_since_insight = datetime.now() - self.recursion_state.last_insight_time
        if time_since_insight > timedelta(minutes=10):  # Configurable threshold
            return True

        return False

    def _finalize_growth_pass(self) -> None:
        """Finalize the growth pass and update metrics"""
        # Update final metrics
        if self.growth_metrics.total_iterations > 0:
            self.growth_metrics.average_expansion_quality = (
                sum(self.recursion_state.quality_trend) / len(self.recursion_state.quality_trend)
                if self.recursion_state.quality_trend else 0.0
            )
            self.growth_metrics.growth_velocity = (
                self.growth_metrics.successful_expansions / self.growth_metrics.total_iterations
            )

        # Update saturation events
        if self.recursion_state.saturation_detected:
            self.growth_metrics.saturation_events += 1

        # Set final phase
        if self.growth_metrics.successful_expansions > 0:
            self.recursion_state.current_phase = GrowthPhase.TERMINATION
        else:
            self.recursion_state.current_phase = GrowthPhase.SATURATION

    def get_growth_statistics(self) -> Dict[str, any]:
        """Get comprehensive growth statistics"""
        total_budget = sum(gate.initial_budget for gate in self.zc_gates.values())
        remaining_budget = sum(gate.budget_remaining for gate in self.zc_gates.values())

        gate_stats = {}
        for name, gate in self.zc_gates.items():
            gate_stats[name] = {
                "budget_remaining": gate.budget_remaining,
                "saturation_level": gate.get_saturation_level(),
                "iterations": gate.iteration_count,
                "time_since_reset": gate.time_since_last_reset().total_seconds()
            }

        return {
            "current_phase": self.recursion_state.current_phase.value,
            "total_iterations": self.growth_metrics.total_iterations,
            "successful_expansions": self.growth_metrics.successful_expansions,
            "failed_expansions": self.growth_metrics.failed_expansions,
            "insights_generated": self.growth_metrics.insights_generated,
            "budget_resets": self.growth_metrics.budget_resets,
            "average_quality": self.growth_metrics.average_expansion_quality,
            "growth_velocity": self.growth_metrics.growth_velocity,
            "budget_utilization": (total_budget - remaining_budget) / total_budget if total_budget > 0 else 0,
            "saturation_detected": self.recursion_state.saturation_detected,
            "gate_statistics": gate_stats
        }


    # ========== ELEGANCE MODULES INTEGRATION (Light hooks) ==========

    def _check_payoff_opportunities(self, constraint_genome):
        """FPD Integration: Look for payoff opportunities during growth pass"""
        if 'fpd' not in self.elegance_modules:
            return

        fpd_detector = self.elegance_modules['fpd']

        # Check if any setups are ready for payoff based on growth phase
        if self.recursion_state.current_phase in [GrowthPhase.EXPANSION, GrowthPhase.REFINEMENT]:
            # Get orphaned setups that might need payoffs
            fpd_export = fpd_detector.export_for_rip_integration()
            orphaned_setups = fpd_export.get('orphaned_setups', [])

            if orphaned_setups and len(orphaned_setups) > 3:  # Many unfulfilled promises
                logger.debug(f"FPD: {len(orphaned_setups)} orphaned setups detected - consider payoff generation")

    def get_elegance_insights(self) -> Dict[str, Any]:
        """Get insights from elegance modules for growth optimization"""
        if not ELEGANCE_MODULES_AVAILABLE:
            return {'status': 'unavailable'}

        insights = {
            'status': 'active',
            'modules': {}
        }

        for module_name, module in self.elegance_modules.items():
            try:
                if hasattr(module, 'export_for_rip_integration'):
                    module_export = module.export_for_rip_integration()
                    insights['modules'][module_name] = {
                        'health': module.get_tracker_health(),
                        'rip_data': module_export
                    }
                else:
                    insights['modules'][module_name] = {
                        'health': module.get_tracker_health()
                    }
            except Exception as e:
                insights['modules'][module_name] = {'error': str(e)}

        return insights


@dataclass
class IterationResult:
    """Result of a single growth iteration"""
    expansions: List[str]
    quality_scores: List[float]
    gate_used: str
    insight_generated: bool


# Example usage and testing
if __name__ == "__main__":
    # Test the recursive growth pass
    growth_pass = RecursiveGrowthPass()

    # Initialize session
    growth_pass.initialize_growth_session("test_session")

    # Print initial statistics
    stats = growth_pass.get_growth_statistics()
    print("Initial Growth Statistics:")
    for key, value in stats.items():
        if key != "gate_statistics":
            print(f"  {key}: {value}")

    print("\nGate Statistics:")
    for gate_name, gate_stats in stats["gate_statistics"].items():
        print(f"  {gate_name}:")
        for stat_name, stat_value in gate_stats.items():
            print(f"    {stat_name}: {stat_value}")

    print(f"\nGrowth pass initialized with phase: {growth_pass.recursion_state.current_phase.value}")