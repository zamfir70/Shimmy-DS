"""
🦠 Drift Detector - Pathogen Fingerprint Library
==============================================

Implements pathogen detection for the RIP+RIC unified protocol.
Scans expansions for narrative drift pathogens and acts as a voting
subsystem in the RIC arbitration engine.

Pathogen Library:
- Ancestral Drift: Unintended backstory introduction
- Sentimentality Bloom: Over-emotional elaboration
- Hallucinated Precision: False specific details
- Thematic Mutation: Theme divergence
- Time-Jump Parasite: Temporal inconsistencies
"""

import re
import json
import logging
from typing import Dict, List, Optional, Tuple, NamedTuple, Set
from dataclasses import dataclass
from enum import Enum
from datetime import datetime, timedelta
import asyncio

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class PathogenType(Enum):
    """Types of narrative drift pathogens"""
    ANCESTRAL_DRIFT = "ancestral_drift"
    SENTIMENTALITY_BLOOM = "sentimentality_bloom"
    HALLUCINATED_PRECISION = "hallucinated_precision"
    THEMATIC_MUTATION = "thematic_mutation"
    TIME_JUMP_PARASITE = "time_jump_parasite"
    SCOPE_CREEP = "scope_creep"
    CHARACTER_CONTAMINATION = "character_contamination"
    TONAL_INFECTION = "tonal_infection"


class PathogenSeverity(Enum):
    """Severity levels for detected pathogens"""
    LOW = 1
    MEDIUM = 2
    HIGH = 3
    CRITICAL = 4


@dataclass
class PathogenFingerprint:
    """Fingerprint definition for a specific pathogen"""
    pathogen_type: PathogenType
    name: str
    description: str
    detection_patterns: List[str]  # Regex patterns
    semantic_markers: List[str]    # Semantic indicators
    severity_weights: Dict[str, float]  # Pattern -> severity weight
    threshold: float  # Detection threshold
    mutation_rate: float  # How quickly this pathogen adapts

    def calculate_severity(self, matches: Dict[str, int]) -> PathogenSeverity:
        """Calculate severity based on pattern matches"""
        score = 0.0
        for pattern, count in matches.items():
            weight = self.severity_weights.get(pattern, 1.0)
            score += count * weight

        if score >= self.threshold * 3:
            return PathogenSeverity.CRITICAL
        elif score >= self.threshold * 2:
            return PathogenSeverity.HIGH
        elif score >= self.threshold:
            return PathogenSeverity.MEDIUM
        else:
            return PathogenSeverity.LOW


@dataclass
class PathogenDetection:
    """Result of pathogen detection"""
    pathogen_type: PathogenType
    severity: PathogenSeverity
    confidence: float
    matched_patterns: List[str]
    infected_regions: List[Tuple[int, int]]  # Start, end positions
    description: str
    remediation_suggestions: List[str]
    detection_timestamp: datetime


@dataclass
class DriftScanResult:
    """Complete drift scan result"""
    text_scanned: str
    scan_timestamp: datetime
    pathogens_detected: List[PathogenDetection]
    overall_health_score: float  # 0.0 (infected) to 1.0 (clean)
    ric_vote_recommendation: str  # Continue, Suggest, Block, Stalled


class DriftDetector:
    """
    Pathogen detection engine for narrative drift.
    Integrates with RIC arbitration as a voting subsystem.
    """

    def __init__(self, ric_integration=None):
        """Initialize with optional RIC integration"""
        self.ric_integration = ric_integration
        self.pathogen_library = self._initialize_pathogen_library()
        self.detection_history: List[DriftScanResult] = []
        self.mutation_tracker: Dict[PathogenType, float] = {}

    def _initialize_pathogen_library(self) -> Dict[PathogenType, PathogenFingerprint]:
        """Initialize the pathogen fingerprint library"""
        library = {}

        # Ancestral Drift - Unintended backstory introduction
        library[PathogenType.ANCESTRAL_DRIFT] = PathogenFingerprint(
            pathogen_type=PathogenType.ANCESTRAL_DRIFT,
            name="Ancestral Drift",
            description="Unintended introduction of character backstory or historical details",
            detection_patterns=[
                r'years? ago',
                r'when (?:he|she|they) was (?:young|younger|a child)',
                r'(?:mother|father|parent)(?:s)? (?:always|used to|would)',
                r'remember(?:ed|ing)? when',
                r'(?:childhood|youth|past) (?:memories?|experiences?)',
                r'(?:grandmother|grandfather) (?:told|said|taught)',
                r'family tradition',
                r'inherited from',
                r'(?:born|raised) in',
                r'(?:learned|discovered) as a (?:child|youth)',
            ],
            semantic_markers=[
                "backstory", "history", "past", "childhood", "ancestry",
                "heritage", "legacy", "tradition", "memory", "origin"
            ],
            severity_weights={
                r'years? ago': 2.0,
                r'when (?:he|she|they) was (?:young|younger|a child)': 3.0,
                r'(?:grandmother|grandfather) (?:told|said|taught)': 2.5,
                r'family tradition': 3.5,
                r'inherited from': 3.0,
            },
            threshold=2.0,
            mutation_rate=0.1
        )

        # Sentimentality Bloom - Over-emotional elaboration
        library[PathogenType.SENTIMENTALITY_BLOOM] = PathogenFingerprint(
            pathogen_type=PathogenType.SENTIMENTALITY_BLOOM,
            name="Sentimentality Bloom",
            description="Excessive emotional elaboration that overwhelms narrative purpose",
            detection_patterns=[
                r'(?:deeply|profoundly|overwhelmingly) (?:moved|touched|affected)',
                r'tears? (?:streaming|flowing|cascading)',
                r'heart (?:breaking|shattering|aching|swelling)',
                r'(?:beautiful|precious|sacred) (?:moment|memory|feeling)',
                r'(?:pure|innocent|perfect) (?:love|joy|happiness)',
                r'(?:magical|enchanting|mystical) (?:connection|bond)',
                r'soul(?:-?deep|(?:\s+)touching)',
                r'(?:trembling|quivering) with (?:emotion|feeling)',
                r'(?:overwhelmed|consumed) by (?:love|grief|joy)',
                r'(?:divine|spiritual|transcendent) (?:moment|experience)',
            ],
            semantic_markers=[
                "emotional", "sentimental", "touching", "heartwarming",
                "precious", "sacred", "pure", "innocent", "magical"
            ],
            severity_weights={
                r'(?:deeply|profoundly|overwhelmingly) (?:moved|touched|affected)': 2.5,
                r'heart (?:breaking|shattering|aching|swelling)': 3.0,
                r'(?:pure|innocent|perfect) (?:love|joy|happiness)': 3.5,
                r'soul(?:-?deep|(?:\s+)touching)': 4.0,
                r'(?:divine|spiritual|transcendent) (?:moment|experience)': 4.5,
            },
            threshold=3.0,
            mutation_rate=0.15
        )

        # Hallucinated Precision - False specific details
        library[PathogenType.HALLUCINATED_PRECISION] = PathogenFingerprint(
            pathogen_type=PathogenType.HALLUCINATED_PRECISION,
            name="Hallucinated Precision",
            description="Introduction of overly specific details not grounded in source material",
            detection_patterns=[
                r'\d+\.?\d*\s*(?:inches?|feet|yards?|miles?|meters?|cm|mm)',
                r'(?:exactly|precisely) \d+',
                r'\d+:\d+\s*(?:AM|PM|am|pm)',
                r'(?:Model|Type|Brand)\s+[A-Z]\d+',
                r'\$\d+\.?\d*',
                r'(?:License|Serial|ID)\s*(?:Number|#)?\s*[A-Z0-9]+',
                r'(?:born|died|happened) (?:on|in) (?:January|February|March|April|May|June|July|August|September|October|November|December)\s+\d+',
                r'(?:weighed|measured|cost|lasted) (?:exactly|precisely)',
                r'(?:\d+\s*(?:degrees?|°))',
                r'(?:ISBN|SSN|Phone)\s*:?\s*\d+',
            ],
            semantic_markers=[
                "exactly", "precisely", "specifically", "measured",
                "calculated", "documented", "recorded", "verified"
            ],
            severity_weights={
                r'(?:exactly|precisely) \d+': 3.0,
                r'(?:Model|Type|Brand)\s+[A-Z]\d+': 4.0,
                r'(?:License|Serial|ID)\s*(?:Number|#)?\s*[A-Z0-9]+': 4.5,
                r'(?:born|died|happened) (?:on|in) (?:January|February|March|April|May|June|July|August|September|October|November|December)\s+\d+': 3.5,
            },
            threshold=2.5,
            mutation_rate=0.2
        )

        # Thematic Mutation - Theme divergence
        library[PathogenType.THEMATIC_MUTATION] = PathogenFingerprint(
            pathogen_type=PathogenType.THEMATIC_MUTATION,
            name="Thematic Mutation",
            description="Divergence from established thematic direction",
            detection_patterns=[
                r'(?:suddenly|unexpectedly|out of nowhere)',
                r'(?:completely|totally|entirely) different',
                r'(?:unrelated|disconnected|separate) (?:topic|subject|matter)',
                r'(?:shifted|changed|turned) (?:focus|attention|direction)',
                r'(?:meanwhile|elsewhere|somewhere else)',
                r'(?:another|different|separate) (?:story|narrative|plot)',
                r'(?:tangent|digression|aside)',
                r'(?:unconnected|isolated) (?:incident|event|occurrence)',
                r'(?:random|arbitrary|chance) (?:event|occurrence|happening)',
                r'(?:irrelevant|unimportant|peripheral) (?:detail|information)',
            ],
            semantic_markers=[
                "divergent", "unrelated", "tangential", "disconnected",
                "separate", "different", "unconnected", "random", "arbitrary"
            ],
            severity_weights={
                r'(?:suddenly|unexpectedly|out of nowhere)': 2.0,
                r'(?:completely|totally|entirely) different': 3.5,
                r'(?:unrelated|disconnected|separate) (?:topic|subject|matter)': 4.0,
                r'(?:another|different|separate) (?:story|narrative|plot)': 4.5,
            },
            threshold=2.5,
            mutation_rate=0.12
        )

        # Time-Jump Parasite - Temporal inconsistencies
        library[PathogenType.TIME_JUMP_PARASITE] = PathogenFingerprint(
            pathogen_type=PathogenType.TIME_JUMP_PARASITE,
            name="Time-Jump Parasite",
            description="Temporal inconsistencies and unexpected time shifts",
            detection_patterns=[
                r'(?:later|earlier|before|after),?\s*(?:that same day|that evening|that morning)',
                r'(?:minutes?|hours?|days?|weeks?|months?|years?) (?:later|earlier|ago|before)',
                r'(?:suddenly|instantly|immediately),?\s*(?:time|everything) (?:shifted|changed|jumped)',
                r'(?:flashback|flash forward|time skip)',
                r'(?:when|while|as) (?:time|clock) (?:moved|passed|went)',
                r'(?:past|present|future) (?:tense|time|moment)',
                r'(?:chronology|timeline|sequence) (?:shifted|changed|broke)',
                r'(?:temporal|time) (?:shift|jump|break|loop)',
                r'(?:anachronism|time paradox)',
                r'(?:now|then),?\s*(?:back|forward) (?:in time|to)',
            ],
            semantic_markers=[
                "temporal", "chronological", "timeline", "sequence",
                "flashback", "future", "past", "time", "when", "anachronism"
            ],
            severity_weights={
                r'(?:flashback|flash forward|time skip)': 3.5,
                r'(?:temporal|time) (?:shift|jump|break|loop)': 4.0,
                r'(?:anachronism|time paradox)': 4.5,
                r'(?:chronology|timeline|sequence) (?:shifted|changed|broke)': 4.0,
            },
            threshold=2.0,
            mutation_rate=0.08
        )

        # Scope Creep - Expansion beyond narrative boundaries
        library[PathogenType.SCOPE_CREEP] = PathogenFingerprint(
            pathogen_type=PathogenType.SCOPE_CREEP,
            name="Scope Creep",
            description="Expansion beyond established narrative boundaries",
            detection_patterns=[
                r'(?:entire|whole|complete) (?:world|universe|reality)',
                r'(?:global|worldwide|universal) (?:impact|effect|consequence)',
                r'(?:government|authorities|officials) (?:involved|concerned|interested)',
                r'(?:news|media|press) (?:coverage|attention|reports)',
                r'(?:scientific|medical|legal) (?:community|establishment|experts)',
                r'(?:international|national) (?:implications|consequences|ramifications)',
                r'(?:cosmic|galactic|planetary) (?:significance|importance|scale)',
                r'(?:historical|epoch|era)(?:-(?:making|defining|changing))?',
                r'(?:all of humanity|human race|mankind|civilization)',
                r'(?:paradigm|worldview) (?:shift|change|transformation)',
            ],
            semantic_markers=[
                "global", "universal", "cosmic", "historical", "paradigm",
                "civilization", "humanity", "worldwide", "epoch", "era"
            ],
            severity_weights={
                r'(?:entire|whole|complete) (?:world|universe|reality)': 4.0,
                r'(?:cosmic|galactic|planetary) (?:significance|importance|scale)': 4.5,
                r'(?:all of humanity|human race|mankind|civilization)': 4.0,
                r'(?:paradigm|worldview) (?:shift|change|transformation)': 3.5,
            },
            threshold=3.0,
            mutation_rate=0.1
        )

        return library

    async def scan_for_drift(self, text: str, context: Optional[Dict] = None) -> DriftScanResult:
        """
        Scan text for narrative drift pathogens.
        Returns complete drift analysis with RIC vote recommendation.
        """
        logger.info(f"Scanning text for drift pathogens ({len(text)} characters)")

        detections = []
        overall_score = 1.0  # Start with perfect health

        # Scan for each pathogen type
        for pathogen_type, fingerprint in self.pathogen_library.items():
            detection = await self._scan_for_pathogen(text, fingerprint)
            if detection:
                detections.append(detection)
                # Reduce overall health based on severity
                severity_impact = {
                    PathogenSeverity.LOW: 0.1,
                    PathogenSeverity.MEDIUM: 0.25,
                    PathogenSeverity.HIGH: 0.4,
                    PathogenSeverity.CRITICAL: 0.7
                }
                overall_score -= severity_impact.get(detection.severity, 0.1)

        # Ensure score doesn't go below 0
        overall_score = max(0.0, overall_score)

        # Generate RIC vote recommendation
        vote = self._generate_ric_vote(detections, overall_score)

        result = DriftScanResult(
            text_scanned=text,
            scan_timestamp=datetime.now(),
            pathogens_detected=detections,
            overall_health_score=overall_score,
            ric_vote_recommendation=vote
        )

        self.detection_history.append(result)
        logger.info(f"Drift scan complete: {len(detections)} pathogens detected, health score: {overall_score:.2f}")

        return result

    async def _scan_for_pathogen(self, text: str, fingerprint: PathogenFingerprint) -> Optional[PathogenDetection]:
        """Scan text for a specific pathogen"""
        matches = {}
        infected_regions = []

        # Check detection patterns
        for pattern in fingerprint.detection_patterns:
            pattern_matches = list(re.finditer(pattern, text, re.IGNORECASE))
            if pattern_matches:
                matches[pattern] = len(pattern_matches)
                for match in pattern_matches:
                    infected_regions.append((match.start(), match.end()))

        # Check semantic markers
        semantic_matches = 0
        for marker in fingerprint.semantic_markers:
            marker_pattern = r'\b' + re.escape(marker) + r'\b'
            marker_matches = list(re.finditer(marker_pattern, text, re.IGNORECASE))
            semantic_matches += len(marker_matches)
            for match in marker_matches:
                infected_regions.append((match.start(), match.end()))

        if semantic_matches > 0:
            matches[f"semantic_markers_{fingerprint.pathogen_type.value}"] = semantic_matches

        # Calculate severity
        severity = fingerprint.calculate_severity(matches)

        # Only report if above minimum threshold
        if not matches or severity == PathogenSeverity.LOW:
            return None

        # Calculate confidence based on pattern density
        text_length = len(text)
        pattern_density = sum(matches.values()) / max(text_length / 100, 1)  # Patterns per 100 chars
        confidence = min(0.95, pattern_density * 0.3 + 0.1)

        # Generate remediation suggestions
        remediation = self._generate_remediation_suggestions(fingerprint.pathogen_type, matches)

        detection = PathogenDetection(
            pathogen_type=fingerprint.pathogen_type,
            severity=severity,
            confidence=confidence,
            matched_patterns=list(matches.keys()),
            infected_regions=sorted(infected_regions),
            description=f"{fingerprint.name}: {fingerprint.description}",
            remediation_suggestions=remediation,
            detection_timestamp=datetime.now()
        )

        return detection

    def _generate_ric_vote(self, detections: List[PathogenDetection], health_score: float) -> str:
        """Generate RIC vote recommendation based on detections"""

        # Count critical and high severity detections
        critical_count = sum(1 for d in detections if d.severity == PathogenSeverity.CRITICAL)
        high_count = sum(1 for d in detections if d.severity == PathogenSeverity.HIGH)

        # Vote logic
        if critical_count >= 2 or health_score < 0.3:
            return "Block"  # Multiple critical issues or very low health
        elif critical_count >= 1 or high_count >= 2 or health_score < 0.5:
            return "Suggest"  # Some serious issues detected
        elif len(detections) == 0 and health_score > 0.9:
            return "Continue"  # Clean text
        elif health_score < 0.7:
            return "Suggest"  # Moderate health concerns
        else:
            return "Continue"  # Generally healthy

    def _generate_remediation_suggestions(self, pathogen_type: PathogenType,
                                        matches: Dict[str, int]) -> List[str]:
        """Generate suggestions for remediating detected pathogens"""
        suggestions = []

        if pathogen_type == PathogenType.ANCESTRAL_DRIFT:
            suggestions = [
                "Focus on present moment actions rather than backstory",
                "Ground character motivations in current scene context",
                "Remove unnecessary historical references",
                "Keep family/past references minimal and relevant"
            ]
        elif pathogen_type == PathogenType.SENTIMENTALITY_BLOOM:
            suggestions = [
                "Reduce emotional intensity descriptors",
                "Show emotion through action rather than description",
                "Avoid superlative emotional language",
                "Ground feelings in specific, concrete details"
            ]
        elif pathogen_type == PathogenType.HALLUCINATED_PRECISION:
            suggestions = [
                "Remove specific measurements not established in source",
                "Replace exact figures with approximate descriptions",
                "Eliminate fabricated identification numbers or codes",
                "Focus on relative rather than absolute descriptions"
            ]
        elif pathogen_type == PathogenType.THEMATIC_MUTATION:
            suggestions = [
                "Ensure all content relates to established themes",
                "Remove tangential or unrelated story elements",
                "Maintain focus on current narrative thread",
                "Connect new elements to existing thematic framework"
            ]
        elif pathogen_type == PathogenType.TIME_JUMP_PARASITE:
            suggestions = [
                "Maintain consistent temporal perspective",
                "Remove unexpected time shifts",
                "Clarify chronological relationships",
                "Stay within established timeline"
            ]
        elif pathogen_type == PathogenType.SCOPE_CREEP:
            suggestions = [
                "Limit scope to immediate narrative context",
                "Remove global or universal implications",
                "Focus on character-level rather than world-level impacts",
                "Keep consequences proportional to story scale"
            ]

        return suggestions

    def get_pathogen_statistics(self) -> Dict[str, any]:
        """Get statistics on pathogen detection history"""
        if not self.detection_history:
            return {"total_scans": 0}

        total_scans = len(self.detection_history)
        total_detections = sum(len(scan.pathogens_detected) for scan in self.detection_history)

        pathogen_counts = {}
        severity_counts = {"LOW": 0, "MEDIUM": 0, "HIGH": 0, "CRITICAL": 0}

        for scan in self.detection_history:
            for detection in scan.pathogens_detected:
                pathogen_type = detection.pathogen_type.value
                pathogen_counts[pathogen_type] = pathogen_counts.get(pathogen_type, 0) + 1
                severity_counts[detection.severity.name] += 1

        avg_health_score = sum(scan.overall_health_score for scan in self.detection_history) / total_scans

        return {
            "total_scans": total_scans,
            "total_detections": total_detections,
            "average_health_score": avg_health_score,
            "pathogen_frequency": pathogen_counts,
            "severity_distribution": severity_counts,
            "most_common_pathogen": max(pathogen_counts.items(), key=lambda x: x[1])[0] if pathogen_counts else None
        }

    def update_pathogen_library(self, mutations: Dict[PathogenType, Dict]) -> None:
        """Update pathogen library with mutations (adaptive learning)"""
        for pathogen_type, mutation_data in mutations.items():
            if pathogen_type in self.pathogen_library:
                fingerprint = self.pathogen_library[pathogen_type]

                # Update patterns if provided
                if "new_patterns" in mutation_data:
                    fingerprint.detection_patterns.extend(mutation_data["new_patterns"])

                # Update threshold if provided
                if "threshold_adjustment" in mutation_data:
                    adjustment = mutation_data["threshold_adjustment"]
                    fingerprint.threshold = max(0.1, fingerprint.threshold + adjustment)

                # Track mutation
                self.mutation_tracker[pathogen_type] = self.mutation_tracker.get(pathogen_type, 0) + 1

                logger.info(f"Updated {pathogen_type.value} fingerprint with mutations")

    async def scan_and_vote(self, text: str, context: Optional[Dict] = None) -> Tuple[DriftScanResult, str]:
        """
        Convenience method for RIC integration.
        Scans text and returns both detailed results and simple vote.
        """
        scan_result = await self.scan_for_drift(text, context)
        vote = scan_result.ric_vote_recommendation

        # Notify RIC integration if available
        if self.ric_integration:
            if vote == "Continue" and scan_result.overall_health_score > 0.8:
                # High quality expansion - reset budget
                self.ric_integration.reset_subsystem_on_insight("drift_detector")

        return scan_result, vote


# Example usage and testing
if __name__ == "__main__":
    # Test the drift detector
    detector = DriftDetector()

    # Test cases for different pathogens
    test_cases = [
        # Clean text
        "Maria walked to the window and looked outside. The morning light revealed a garden.",

        # Ancestral drift
        "Maria remembered when her grandmother used to tell her stories about the old country, years ago when she was young.",

        # Sentimentality bloom
        "Maria felt deeply moved by the overwhelmingly beautiful moment, her heart breaking with pure, innocent love that touched her soul.",

        # Hallucinated precision
        "The letter was exactly 8.5 inches long, written at 3:47 PM on model A-47 paper, costing precisely $12.99.",

        # Thematic mutation
        "Suddenly, completely different and unrelated events began happening. Meanwhile, elsewhere, another story emerged.",

        # Time jump parasite
        "Years later, in a flashback, the temporal shift caused a chronological break in the timeline.",
    ]

    async def test_detection():
        for i, test_text in enumerate(test_cases, 1):
            print(f"\n=== Test Case {i} ===")
            print(f"Text: {test_text}")

            scan_result = await detector.scan_for_drift(test_text)
            print(f"Health Score: {scan_result.overall_health_score:.2f}")
            print(f"RIC Vote: {scan_result.ric_vote_recommendation}")

            if scan_result.pathogens_detected:
                print("Pathogens Detected:")
                for detection in scan_result.pathogens_detected:
                    print(f"  - {detection.pathogen_type.value}: {detection.severity.name} "
                          f"(confidence: {detection.confidence:.2f})")
                    if detection.remediation_suggestions:
                        print(f"    Suggestions: {detection.remediation_suggestions[0]}")
            else:
                print("No pathogens detected")

        # Print overall statistics
        stats = detector.get_pathogen_statistics()
        print(f"\n=== Detection Statistics ===")
        print(f"Total scans: {stats['total_scans']}")
        print(f"Total detections: {stats['total_detections']}")
        print(f"Average health score: {stats['average_health_score']:.2f}")
        print(f"Pathogen frequency: {stats['pathogen_frequency']}")

    # Run the async test
    asyncio.run(test_detection())