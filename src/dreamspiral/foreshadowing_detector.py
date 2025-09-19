"""
FPD (Foreshadowing & Payoff Detector) - Minimal Elegance Module

Setup→payoff lattice tracking with light RIP+RIC integration.
Goal: Increase creative power without complexity.

Core Focus:
- Track setup→payoff relationships across narrative beats
- Maintain narrative promise inventory
- Provide hooks for constraint validation
- 100% compatible with existing RIP+RIC pipeline
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple, Set, Any
from enum import Enum
import re
import json


class SetupType(Enum):
    CHARACTER_PROMISE = "character_promise"    # Character says they'll do something
    MYSTERY_ELEMENT = "mystery_element"        # Questions raised that need answers
    THREAT_ESTABLISHMENT = "threat_establishment"  # Dangers introduced
    RELATIONSHIP_SETUP = "relationship_setup"  # Character dynamics established
    WORLD_RULE = "world_rule"                 # How the world works
    SYMBOLIC_ELEMENT = "symbolic_element"      # Objects, images with meaning
    EMOTIONAL_SETUP = "emotional_setup"       # Emotional states to resolve


class PayoffType(Enum):
    DIRECT_FULFILLMENT = "direct_fulfillment"     # Promise kept exactly
    IRONIC_SUBVERSION = "ironic_subversion"       # Promise kept but twisted
    CLEVER_MISDIRECTION = "clever_misdirection"   # Expectation redirected
    PARTIAL_RESOLUTION = "partial_resolution"     # Some but not all resolved
    ESCALATED_PAYOFF = "escalated_payoff"         # Bigger than promised
    FAILED_PROMISE = "failed_promise"             # Promise explicitly broken


class PayoffTiming(Enum):
    IMMEDIATE = 1      # Same beat or next beat
    SHORT_TERM = 5     # Within 5 beats
    MEDIUM_TERM = 15   # Within 15 beats
    LONG_TERM = 50     # Within 50 beats
    EPIC_ARC = 999     # Story-spanning payoff


@dataclass
class ForeshadowingSetup:
    """A narrative setup that creates expectation for future payoff"""
    setup_id: str
    beat_id: str
    setup_type: SetupType
    content: str  # The actual setup text/description
    characters_involved: List[str]
    emotional_weight: float  # 0.0 to 1.0 - how much emotional investment
    expected_timing: PayoffTiming
    setup_context: Dict[str, Any] = field(default_factory=dict)
    rip_anchors: List[str] = field(default_factory=list)  # RIP constraint connections

    def to_dict(self) -> Dict[str, Any]:
        return {
            'setup_id': self.setup_id,
            'beat_id': self.beat_id,
            'setup_type': self.setup_type.value,
            'content': self.content,
            'characters_involved': self.characters_involved,
            'emotional_weight': self.emotional_weight,
            'expected_timing': self.expected_timing.value,
            'setup_context': self.setup_context,
            'rip_anchors': self.rip_anchors
        }


@dataclass
class NarrativePayoff:
    """Resolution/payoff of a previous setup"""
    payoff_id: str
    beat_id: str
    setup_id: str  # Links back to the setup
    payoff_type: PayoffType
    content: str  # The actual payoff text/description
    satisfaction_score: float  # 0.0 to 1.0 - how satisfying the payoff feels
    timing_accuracy: float  # How close to expected timing
    characters_involved: List[str]
    payoff_context: Dict[str, Any] = field(default_factory=dict)
    continuity_flags: List[str] = field(default_factory=list)  # RIP+RIC integration

    def to_dict(self) -> Dict[str, Any]:
        return {
            'payoff_id': self.payoff_id,
            'beat_id': self.beat_id,
            'setup_id': self.setup_id,
            'payoff_type': self.payoff_type.value,
            'content': self.content,
            'satisfaction_score': self.satisfaction_score,
            'timing_accuracy': self.timing_accuracy,
            'characters_involved': self.characters_involved,
            'payoff_context': self.payoff_context,
            'continuity_flags': self.continuity_flags
        }


@dataclass
class SetupPayoffLattice:
    """Complete mapping of setup→payoff relationships"""
    setups: Dict[str, ForeshadowingSetup] = field(default_factory=dict)
    payoffs: Dict[str, NarrativePayoff] = field(default_factory=dict)
    lattice_connections: Dict[str, List[str]] = field(default_factory=dict)  # setup_id -> [payoff_ids]
    orphaned_setups: Set[str] = field(default_factory=set)  # Setups without payoffs
    orphaned_payoffs: Set[str] = field(default_factory=set)  # Payoffs without setups
    beat_sequence: List[str] = field(default_factory=list)  # Ordered list of beat_ids

    def add_connection(self, setup_id: str, payoff_id: str):
        """Add connection between setup and payoff"""
        if setup_id not in self.lattice_connections:
            self.lattice_connections[setup_id] = []
        if payoff_id not in self.lattice_connections[setup_id]:
            self.lattice_connections[setup_id].append(payoff_id)

        # Remove from orphaned sets
        self.orphaned_setups.discard(setup_id)
        self.orphaned_payoffs.discard(payoff_id)

    def get_connection_health(self) -> float:
        """Calculate overall health of setup→payoff connections"""
        total_setups = len(self.setups)
        total_payoffs = len(self.payoffs)

        if total_setups == 0 and total_payoffs == 0:
            return 1.0

        connected_setups = len(self.lattice_connections)
        orphaned_count = len(self.orphaned_setups) + len(self.orphaned_payoffs)

        if total_setups == 0:
            return 0.5  # Only payoffs, no setups

        connection_ratio = connected_setups / total_setups
        orphaned_penalty = orphaned_count / (total_setups + total_payoffs)

        return max(0.0, connection_ratio - orphaned_penalty)


class ForeshadowingPayoffDetector:
    """
    Light foreshadowing tracking system that plugs into existing RIP+RIC pipeline.

    Design Philosophy:
    - Track narrative promises and their fulfillment
    - Minimal surface area - focus on what matters for satisfaction
    - Beat-aligned - connected to narrative structure
    - RIP+RIC compatible - provides constraint validation hooks
    """

    def __init__(self):
        self.lattice = SetupPayoffLattice()
        self.beat_counter = 0
        self.promise_patterns = self._initialize_promise_patterns()
        self.payoff_patterns = self._initialize_payoff_patterns()

    def _initialize_promise_patterns(self) -> Dict[str, List[str]]:
        """Initialize regex patterns for detecting common setup types"""
        return {
            'character_promise': [
                r'(I will|I\'ll|I promise|I swear).*',
                r'(when I|if I|after I).*',
                r'(someday|tomorrow|next time).*'
            ],
            'mystery_element': [
                r'(what is|who is|why did|how did|where is).*\?',
                r'(strange|mysterious|unknown|hidden|secret).*',
                r'(but why|but how|but what|but who).*'
            ],
            'threat_establishment': [
                r'(danger|threat|warning|beware).*',
                r'(if you don\'t|unless you|you must).*',
                r'(coming for|after you|find you).*'
            ],
            'relationship_setup': [
                r'(I love|I hate|I trust|I fear).*',
                r'(we need to talk|we should).*',
                r'(between us|our relationship).*'
            ]
        }

    def _initialize_payoff_patterns(self) -> Dict[str, List[str]]:
        """Initialize patterns for detecting payoff types"""
        return {
            'direct_fulfillment': [
                r'(as promised|kept.*word|fulfilled).*',
                r'(just as.*said|exactly as).*'
            ],
            'ironic_subversion': [
                r'(but not|however|ironically).*',
                r'(twisted|unexpected|not what).*'
            ],
            'clever_misdirection': [
                r'(actually|in fact|really).*',
                r'(turned out|revealed|discovered).*'
            ]
        }

    def detect_setup(self, beat_id: str, content: str, characters: List[str],
                    emotional_weight: Optional[float] = None) -> Optional[ForeshadowingSetup]:
        """Automatically detect potential setups in content"""
        # Analyze content for setup patterns
        detected_type = self._classify_setup_type(content)

        if not detected_type:
            return None

        # Generate setup ID
        setup_id = f"setup_{self.beat_counter}_{len(self.lattice.setups)}"
        self.beat_counter += 1

        # Estimate emotional weight if not provided
        if emotional_weight is None:
            emotional_weight = self._estimate_emotional_weight(content, detected_type)

        # Estimate expected timing based on setup type
        expected_timing = self._estimate_payoff_timing(detected_type, content)

        # Generate RIP anchors for constraint validation
        rip_anchors = self._generate_rip_anchors(content, characters, detected_type)

        setup = ForeshadowingSetup(
            setup_id=setup_id,
            beat_id=beat_id,
            setup_type=detected_type,
            content=content,
            characters_involved=characters,
            emotional_weight=emotional_weight,
            expected_timing=expected_timing,
            setup_context={'auto_detected': True, 'confidence': self._calculate_detection_confidence(content, detected_type)},
            rip_anchors=rip_anchors
        )

        # Add to lattice
        self.lattice.setups[setup_id] = setup
        self.lattice.orphaned_setups.add(setup_id)

        # Update beat sequence
        if beat_id not in self.lattice.beat_sequence:
            self.lattice.beat_sequence.append(beat_id)
            self.lattice.beat_sequence.sort()  # Keep ordered

        return setup

    def register_manual_setup(self, beat_id: str, setup_type: SetupType, content: str,
                            characters: List[str], emotional_weight: float,
                            expected_timing: PayoffTiming,
                            setup_context: Optional[Dict[str, Any]] = None) -> ForeshadowingSetup:
        """Manually register a setup for more precise control"""
        setup_id = f"manual_setup_{len(self.lattice.setups)}"

        rip_anchors = self._generate_rip_anchors(content, characters, setup_type)

        setup = ForeshadowingSetup(
            setup_id=setup_id,
            beat_id=beat_id,
            setup_type=setup_type,
            content=content,
            characters_involved=characters,
            emotional_weight=emotional_weight,
            expected_timing=expected_timing,
            setup_context=setup_context or {'manual': True},
            rip_anchors=rip_anchors
        )

        self.lattice.setups[setup_id] = setup
        self.lattice.orphaned_setups.add(setup_id)

        if beat_id not in self.lattice.beat_sequence:
            self.lattice.beat_sequence.append(beat_id)
            self.lattice.beat_sequence.sort()

        return setup

    def detect_payoff(self, beat_id: str, content: str, characters: List[str]) -> List[NarrativePayoff]:
        """Automatically detect payoffs and match them to setups"""
        payoffs = []

        # Try to match content to existing setups
        potential_matches = self._find_matching_setups(content, characters, beat_id)

        for setup_id, match_confidence in potential_matches:
            setup = self.lattice.setups[setup_id]

            # Classify payoff type
            payoff_type = self._classify_payoff_type(content, setup)

            # Calculate satisfaction and timing scores
            satisfaction_score = self._calculate_satisfaction_score(setup, content, payoff_type)
            timing_accuracy = self._calculate_timing_accuracy(setup, beat_id)

            # Generate continuity flags for RIP+RIC
            continuity_flags = self._generate_payoff_continuity_flags(setup, content, timing_accuracy)

            payoff_id = f"payoff_{len(self.lattice.payoffs)}"

            payoff = NarrativePayoff(
                payoff_id=payoff_id,
                beat_id=beat_id,
                setup_id=setup_id,
                payoff_type=payoff_type,
                content=content,
                satisfaction_score=satisfaction_score,
                timing_accuracy=timing_accuracy,
                characters_involved=characters,
                payoff_context={'auto_detected': True, 'match_confidence': match_confidence},
                continuity_flags=continuity_flags
            )

            self.lattice.payoffs[payoff_id] = payoff
            self.lattice.add_connection(setup_id, payoff_id)
            payoffs.append(payoff)

        return payoffs

    def register_manual_payoff(self, beat_id: str, setup_id: str, payoff_type: PayoffType,
                             content: str, characters: List[str],
                             satisfaction_score: float) -> NarrativePayoff:
        """Manually register a payoff for precise control"""
        if setup_id not in self.lattice.setups:
            raise ValueError(f"Setup {setup_id} not found")

        setup = self.lattice.setups[setup_id]
        timing_accuracy = self._calculate_timing_accuracy(setup, beat_id)
        continuity_flags = self._generate_payoff_continuity_flags(setup, content, timing_accuracy)

        payoff_id = f"manual_payoff_{len(self.lattice.payoffs)}"

        payoff = NarrativePayoff(
            payoff_id=payoff_id,
            beat_id=beat_id,
            setup_id=setup_id,
            payoff_type=payoff_type,
            content=content,
            satisfaction_score=satisfaction_score,
            timing_accuracy=timing_accuracy,
            characters_involved=characters,
            payoff_context={'manual': True},
            continuity_flags=continuity_flags
        )

        self.lattice.payoffs[payoff_id] = payoff
        self.lattice.add_connection(setup_id, payoff_id)

        return payoff

    def _classify_setup_type(self, content: str) -> Optional[SetupType]:
        """Classify content into setup type using pattern matching"""
        content_lower = content.lower()

        for setup_type_name, patterns in self.promise_patterns.items():
            for pattern in patterns:
                if re.search(pattern, content_lower):
                    return SetupType(setup_type_name)

        return None

    def _estimate_emotional_weight(self, content: str, setup_type: SetupType) -> float:
        """Estimate emotional weight based on content analysis"""
        # Simple heuristic based on emotional keywords and setup type
        emotional_keywords = {
            'high': ['love', 'death', 'kill', 'save', 'destroy', 'forever', 'never'],
            'medium': ['promise', 'swear', 'important', 'must', 'need', 'will'],
            'low': ['maybe', 'might', 'could', 'sometimes', 'perhaps']
        }

        content_lower = content.lower()
        weight = 0.3  # Default

        for keyword in emotional_keywords['high']:
            if keyword in content_lower:
                weight += 0.3

        for keyword in emotional_keywords['medium']:
            if keyword in content_lower:
                weight += 0.2

        for keyword in emotional_keywords['low']:
            if keyword in content_lower:
                weight -= 0.1

        # Adjust based on setup type
        type_weights = {
            SetupType.CHARACTER_PROMISE: 0.2,
            SetupType.THREAT_ESTABLISHMENT: 0.3,
            SetupType.MYSTERY_ELEMENT: 0.1,
            SetupType.RELATIONSHIP_SETUP: 0.2,
            SetupType.WORLD_RULE: 0.0,
            SetupType.SYMBOLIC_ELEMENT: 0.1,
            SetupType.EMOTIONAL_SETUP: 0.3
        }

        weight += type_weights.get(setup_type, 0.0)
        return min(1.0, max(0.0, weight))

    def _estimate_payoff_timing(self, setup_type: SetupType, content: str) -> PayoffTiming:
        """Estimate expected payoff timing based on setup characteristics"""
        content_lower = content.lower()

        # Immediate timing indicators
        if any(word in content_lower for word in ['now', 'immediately', 'right now', 'this instant']):
            return PayoffTiming.IMMEDIATE

        # Short term indicators
        if any(word in content_lower for word in ['soon', 'shortly', 'next', 'tomorrow']):
            return PayoffTiming.SHORT_TERM

        # Long term indicators
        if any(word in content_lower for word in ['someday', 'eventually', 'one day', 'in the end']):
            return PayoffTiming.LONG_TERM

        # Epic arc indicators
        if any(word in content_lower for word in ['destiny', 'fate', 'forever', 'always']):
            return PayoffTiming.EPIC_ARC

        # Default based on setup type
        type_timings = {
            SetupType.CHARACTER_PROMISE: PayoffTiming.MEDIUM_TERM,
            SetupType.MYSTERY_ELEMENT: PayoffTiming.SHORT_TERM,
            SetupType.THREAT_ESTABLISHMENT: PayoffTiming.SHORT_TERM,
            SetupType.RELATIONSHIP_SETUP: PayoffTiming.MEDIUM_TERM,
            SetupType.WORLD_RULE: PayoffTiming.LONG_TERM,
            SetupType.SYMBOLIC_ELEMENT: PayoffTiming.LONG_TERM,
            SetupType.EMOTIONAL_SETUP: PayoffTiming.MEDIUM_TERM
        }

        return type_timings.get(setup_type, PayoffTiming.MEDIUM_TERM)

    def _generate_rip_anchors(self, content: str, characters: List[str], setup_type: SetupType) -> List[str]:
        """Generate RIP constraint anchors for validation"""
        anchors = []

        # Character-based anchors
        for character in characters:
            anchors.append(f"CHARACTER:{character}")

        # Setup type anchor
        anchors.append(f"SETUP_TYPE:{setup_type.value}")

        # Content-based anchors (extract key concepts)
        content_words = re.findall(r'\b\w+\b', content.lower())
        important_words = [word for word in content_words
                          if len(word) > 4 and word not in ['will', 'that', 'this', 'with', 'from']]

        for word in important_words[:3]:  # Limit to top 3
            anchors.append(f"CONCEPT:{word}")

        return anchors

    def _calculate_detection_confidence(self, content: str, setup_type: SetupType) -> float:
        """Calculate confidence score for automatic detection"""
        confidence = 0.5  # Base confidence

        # Check pattern strength
        content_lower = content.lower()
        patterns = self.promise_patterns.get(setup_type.value, [])

        strong_matches = 0
        for pattern in patterns:
            if re.search(pattern, content_lower):
                strong_matches += 1

        confidence += (strong_matches / max(1, len(patterns))) * 0.3

        # Adjust for content clarity
        if len(content.split()) > 10:  # Longer content generally more specific
            confidence += 0.1

        if '?' in content:  # Questions often indicate mysteries
            if setup_type == SetupType.MYSTERY_ELEMENT:
                confidence += 0.2

        return min(1.0, confidence)

    def _find_matching_setups(self, content: str, characters: List[str], beat_id: str) -> List[Tuple[str, float]]:
        """Find setups that could be paid off by this content"""
        matches = []

        for setup_id, setup in self.lattice.setups.items():
            if setup_id in self.lattice.orphaned_setups:  # Only match orphaned setups
                match_score = self._calculate_setup_match_score(setup, content, characters, beat_id)
                if match_score > 0.3:  # Threshold for considering a match
                    matches.append((setup_id, match_score))

        # Sort by match score (highest first)
        matches.sort(key=lambda x: x[1], reverse=True)
        return matches

    def _calculate_setup_match_score(self, setup: ForeshadowingSetup, content: str,
                                   characters: List[str], beat_id: str) -> float:
        """Calculate how well content matches as payoff for setup"""
        score = 0.0

        # Character overlap
        character_overlap = len(set(setup.characters_involved) & set(characters))
        if setup.characters_involved:
            score += (character_overlap / len(setup.characters_involved)) * 0.3

        # Content similarity (simple keyword matching)
        setup_words = set(re.findall(r'\b\w+\b', setup.content.lower()))
        content_words = set(re.findall(r'\b\w+\b', content.lower()))
        word_overlap = len(setup_words & content_words)
        if setup_words:
            score += (word_overlap / len(setup_words)) * 0.3

        # Timing appropriateness
        setup_beat_index = self.lattice.beat_sequence.index(setup.beat_id) if setup.beat_id in self.lattice.beat_sequence else 0
        current_beat_index = self.lattice.beat_sequence.index(beat_id) if beat_id in self.lattice.beat_sequence else len(self.lattice.beat_sequence)

        beats_elapsed = current_beat_index - setup_beat_index
        expected_beats = setup.expected_timing.value

        if beats_elapsed <= expected_beats:
            timing_score = 1.0 - (beats_elapsed / expected_beats) * 0.5
            score += timing_score * 0.4

        return min(1.0, score)

    def _classify_payoff_type(self, content: str, setup: ForeshadowingSetup) -> PayoffType:
        """Classify the type of payoff based on content analysis"""
        content_lower = content.lower()

        # Check for payoff type patterns
        for payoff_type_name, patterns in self.payoff_patterns.items():
            for pattern in patterns:
                if re.search(pattern, content_lower):
                    return PayoffType(payoff_type_name)

        # Heuristic classification based on content relationship to setup
        setup_words = set(re.findall(r'\b\w+\b', setup.content.lower()))
        content_words = set(re.findall(r'\b\w+\b', content_lower))

        # High overlap suggests direct fulfillment
        overlap_ratio = len(setup_words & content_words) / max(1, len(setup_words))
        if overlap_ratio > 0.6:
            return PayoffType.DIRECT_FULFILLMENT

        # Negation words suggest subversion
        negation_words = ['not', 'never', 'but', 'however', 'instead', 'although']
        if any(word in content_lower for word in negation_words):
            return PayoffType.IRONIC_SUBVERSION

        # Default to partial resolution
        return PayoffType.PARTIAL_RESOLUTION

    def _calculate_satisfaction_score(self, setup: ForeshadowingSetup, content: str, payoff_type: PayoffType) -> float:
        """Calculate how satisfying this payoff is for the setup"""
        base_score = 0.5

        # Payoff type affects satisfaction
        type_scores = {
            PayoffType.DIRECT_FULFILLMENT: 0.9,
            PayoffType.CLEVER_MISDIRECTION: 0.8,
            PayoffType.IRONIC_SUBVERSION: 0.7,
            PayoffType.ESCALATED_PAYOFF: 0.9,
            PayoffType.PARTIAL_RESOLUTION: 0.6,
            PayoffType.FAILED_PROMISE: 0.3
        }

        base_score = type_scores.get(payoff_type, 0.5)

        # Adjust for emotional weight
        emotional_bonus = setup.emotional_weight * 0.2
        base_score += emotional_bonus

        # Content quality (length and detail as proxy)
        content_bonus = min(0.2, len(content.split()) / 50)
        base_score += content_bonus

        return min(1.0, max(0.0, base_score))

    def _calculate_timing_accuracy(self, setup: ForeshadowingSetup, payoff_beat_id: str) -> float:
        """Calculate how accurate the timing was compared to expectation"""
        if setup.beat_id not in self.lattice.beat_sequence or payoff_beat_id not in self.lattice.beat_sequence:
            return 0.5  # Unknown timing

        setup_index = self.lattice.beat_sequence.index(setup.beat_id)
        payoff_index = self.lattice.beat_sequence.index(payoff_beat_id)

        actual_beats = payoff_index - setup_index
        expected_beats = setup.expected_timing.value

        if actual_beats <= 0:
            return 0.0  # Payoff before setup is impossible

        # Calculate accuracy - closer to expected timing = higher score
        timing_ratio = actual_beats / expected_beats
        if timing_ratio <= 1.0:
            return timing_ratio  # Early is good
        else:
            return max(0.0, 1.0 - (timing_ratio - 1.0) * 0.5)  # Late is less good

    def _generate_payoff_continuity_flags(self, setup: ForeshadowingSetup, content: str, timing_accuracy: float) -> List[str]:
        """Generate continuity flags for RIP+RIC integration"""
        flags = []

        # Timing flags
        if timing_accuracy < 0.3:
            flags.append('POOR_TIMING')
        elif timing_accuracy > 0.9:
            flags.append('PERFECT_TIMING')

        # Character consistency flags
        setup_chars = set(setup.characters_involved)
        content_chars = set(re.findall(r'\b[A-Z][a-z]+\b', content))  # Simple name detection

        if not (setup_chars & content_chars):
            flags.append('CHARACTER_MISMATCH')

        # Emotional weight flags
        if setup.emotional_weight > 0.8:
            flags.append('HIGH_STAKES_PAYOFF')

        # Content quality flags
        if len(content.split()) < 5:
            flags.append('MINIMAL_PAYOFF')

        return flags

    def analyze_lattice_health(self) -> Dict[str, Any]:
        """Analyze overall health of the setup→payoff lattice"""
        total_setups = len(self.lattice.setups)
        total_payoffs = len(self.lattice.payoffs)
        total_connections = len(self.lattice.lattice_connections)

        analysis = {
            'total_setups': total_setups,
            'total_payoffs': total_payoffs,
            'total_connections': total_connections,
            'orphaned_setups': len(self.lattice.orphaned_setups),
            'orphaned_payoffs': len(self.lattice.orphaned_payoffs),
            'connection_health': self.lattice.get_connection_health(),
            'avg_satisfaction': 0.0,
            'avg_timing_accuracy': 0.0,
            'setup_type_distribution': {},
            'payoff_type_distribution': {},
            'continuity_issues': []
        }

        if total_payoffs > 0:
            satisfaction_scores = [payoff.satisfaction_score for payoff in self.lattice.payoffs.values()]
            timing_scores = [payoff.timing_accuracy for payoff in self.lattice.payoffs.values()]

            analysis['avg_satisfaction'] = sum(satisfaction_scores) / len(satisfaction_scores)
            analysis['avg_timing_accuracy'] = sum(timing_scores) / len(timing_scores)

        # Setup type distribution
        setup_types = [setup.setup_type.value for setup in self.lattice.setups.values()]
        for setup_type in setup_types:
            analysis['setup_type_distribution'][setup_type] = analysis['setup_type_distribution'].get(setup_type, 0) + 1

        # Payoff type distribution
        payoff_types = [payoff.payoff_type.value for payoff in self.lattice.payoffs.values()]
        for payoff_type in payoff_types:
            analysis['payoff_type_distribution'][payoff_type] = analysis['payoff_type_distribution'].get(payoff_type, 0) + 1

        # Collect continuity issues
        all_flags = []
        for payoff in self.lattice.payoffs.values():
            all_flags.extend(payoff.continuity_flags)
        analysis['continuity_issues'] = list(set(all_flags))

        return analysis

    def export_for_rip_integration(self) -> Dict[str, Any]:
        """Export lattice data for RIP constraint validation"""
        return {
            'tracker_type': 'FPD',
            'lattice_health': self.analyze_lattice_health(),
            'orphaned_setups': [self.lattice.setups[setup_id].to_dict() for setup_id in self.lattice.orphaned_setups],
            'unsatisfying_payoffs': [
                payoff.to_dict() for payoff in self.lattice.payoffs.values()
                if payoff.satisfaction_score < 0.4
            ],
            'timing_violations': [
                payoff.to_dict() for payoff in self.lattice.payoffs.values()
                if payoff.timing_accuracy < 0.3
            ],
            'rip_constraint_violations': self._detect_rip_constraint_violations()
        }

    def _detect_rip_constraint_violations(self) -> List[Dict[str, Any]]:
        """Detect violations that should trigger RIP constraint validation"""
        violations = []

        # Check for too many orphaned setups
        orphan_ratio = len(self.lattice.orphaned_setups) / max(1, len(self.lattice.setups))
        if orphan_ratio > 0.3:
            violations.append({
                'type': 'excessive_orphaned_setups',
                'severity': 'medium',
                'ratio': orphan_ratio,
                'recommendation': 'Add payoffs for existing setups before creating new ones'
            })

        # Check for unsatisfying payoffs
        unsatisfying_count = sum(1 for payoff in self.lattice.payoffs.values() if payoff.satisfaction_score < 0.4)
        if unsatisfying_count > 0:
            violations.append({
                'type': 'unsatisfying_payoffs',
                'severity': 'high',
                'count': unsatisfying_count,
                'recommendation': 'Improve payoff quality or reconsider setup promises'
            })

        return violations

    def get_tracker_health(self) -> Dict[str, Any]:
        """Get overall health metrics for the foreshadowing system"""
        lattice_analysis = self.analyze_lattice_health()

        return {
            'tracker_type': 'FPD',
            'connection_health': lattice_analysis['connection_health'],
            'satisfaction_score': lattice_analysis['avg_satisfaction'],
            'timing_accuracy': lattice_analysis['avg_timing_accuracy'],
            'orphan_ratio': (lattice_analysis['orphaned_setups'] + lattice_analysis['orphaned_payoffs']) / max(1, lattice_analysis['total_setups'] + lattice_analysis['total_payoffs']),
            'total_narrative_promises': lattice_analysis['total_setups'],
            'total_fulfilled_promises': lattice_analysis['total_connections'],
            'health_score': min(1.0, (lattice_analysis['connection_health'] + lattice_analysis['avg_satisfaction']) / 2)
        }