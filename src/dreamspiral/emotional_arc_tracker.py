"""
EAT (Emotional Arc Tracker) - Minimal Elegance Module

Per-character, per-beat emotion tracking with light RIP+RIC integration.
Goal: Increase creative power without complexity.

Core Focus:
- Track emotional points across narrative beats
- Maintain character-specific emotional trajectories
- Provide light hooks for MirrorPass continuity
- 100% compatible with existing RIP+RIC pipeline
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple, Any
from enum import Enum
import json
import re


class EmotionalIntensity(Enum):
    SUBTLE = 0.2
    MODERATE = 0.5
    STRONG = 0.8
    OVERWHELMING = 1.0


class EmotionalValence(Enum):
    NEGATIVE = -1.0
    NEUTRAL = 0.0
    POSITIVE = 1.0
    CONFLICTED = 0.5  # Mixed emotions


@dataclass
class EmotionalPoint:
    """Single emotional state measurement at a specific narrative moment"""
    emotion_type: str  # joy, fear, anger, sadness, surprise, etc.
    intensity: EmotionalIntensity
    valence: EmotionalValence
    beat_id: str  # Which narrative beat this occurs in
    context: str  # Brief description of the emotional trigger
    timestamp: int = 0  # Beat sequence number

    def to_dict(self) -> Dict[str, Any]:
        return {
            'emotion_type': self.emotion_type,
            'intensity': self.intensity.value,
            'valence': self.valence.value,
            'beat_id': self.beat_id,
            'context': self.context,
            'timestamp': self.timestamp
        }


@dataclass
class EmotionalBeatState:
    """Complete emotional state for a character during a specific beat"""
    character_name: str
    beat_id: str
    primary_emotion: EmotionalPoint
    secondary_emotions: List[EmotionalPoint] = field(default_factory=list)
    emotional_shift: Optional[Tuple[str, str]] = None  # (from_emotion, to_emotion)
    arc_momentum: float = 0.0  # -1.0 (declining) to 1.0 (rising)
    continuity_flags: List[str] = field(default_factory=list)  # RIP+RIC integration points

    def calculate_complexity_score(self) -> float:
        """Simple complexity metric for MirrorPass integration"""
        base_score = self.primary_emotion.intensity.value
        secondary_weight = len(self.secondary_emotions) * 0.1
        shift_weight = 0.2 if self.emotional_shift else 0.0
        return min(1.0, base_score + secondary_weight + shift_weight)

    def to_dict(self) -> Dict[str, Any]:
        return {
            'character_name': self.character_name,
            'beat_id': self.beat_id,
            'primary_emotion': self.primary_emotion.to_dict(),
            'secondary_emotions': [e.to_dict() for e in self.secondary_emotions],
            'emotional_shift': self.emotional_shift,
            'arc_momentum': self.arc_momentum,
            'continuity_flags': self.continuity_flags,
            'complexity_score': self.calculate_complexity_score()
        }


class EmotionalArcTracker:
    """
    Light emotional tracking system that plugs into existing RIP+RIC pipeline.

    Design Philosophy:
    - Minimal surface area - only track what matters for continuity
    - Character-focused - each character has independent emotional trajectory
    - Beat-aligned - emotions tied to narrative structure
    - RIP+RIC compatible - provides hooks for constraint validation
    """

    def __init__(self):
        self.character_arcs: Dict[str, List[EmotionalBeatState]] = {}
        self.beat_registry: Dict[str, List[str]] = {}  # beat_id -> [character_names]
        self.emotional_patterns: Dict[str, List[str]] = {}  # pattern_name -> [triggers]

        # Light pattern library for common emotional arcs
        self._initialize_pattern_library()

    def _initialize_pattern_library(self):
        """Initialize common emotional arc patterns for quick reference"""
        self.emotional_patterns = {
            'hero_journey': ['doubt', 'determination', 'fear', 'courage', 'triumph'],
            'tragic_fall': ['pride', 'overconfidence', 'realization', 'despair', 'acceptance'],
            'redemption': ['guilt', 'denial', 'acknowledgment', 'effort', 'resolution'],
            'romance': ['attraction', 'uncertainty', 'connection', 'conflict', 'union'],
            'mystery': ['curiosity', 'confusion', 'revelation', 'understanding', 'closure'],
            'horror': ['unease', 'fear', 'terror', 'desperation', 'resolution_or_doom']
        }

    def track_emotional_point(self, character_name: str, beat_id: str,
                            emotion_type: str, intensity: EmotionalIntensity,
                            valence: EmotionalValence, context: str) -> EmotionalPoint:
        """Create and register a new emotional point"""
        point = EmotionalPoint(
            emotion_type=emotion_type,
            intensity=intensity,
            valence=valence,
            beat_id=beat_id,
            context=context,
            timestamp=self._get_next_timestamp(character_name)
        )

        # Update beat registry
        if beat_id not in self.beat_registry:
            self.beat_registry[beat_id] = []
        if character_name not in self.beat_registry[beat_id]:
            self.beat_registry[beat_id].append(character_name)

        return point

    def create_beat_state(self, character_name: str, beat_id: str,
                         primary_emotion: EmotionalPoint,
                         secondary_emotions: Optional[List[EmotionalPoint]] = None,
                         emotional_shift: Optional[Tuple[str, str]] = None) -> EmotionalBeatState:
        """Create complete emotional state for a character in a beat"""
        if character_name not in self.character_arcs:
            self.character_arcs[character_name] = []

        # Calculate arc momentum based on previous states
        arc_momentum = self._calculate_arc_momentum(character_name, primary_emotion)

        # Generate continuity flags for RIP+RIC integration
        continuity_flags = self._generate_continuity_flags(character_name, primary_emotion)

        beat_state = EmotionalBeatState(
            character_name=character_name,
            beat_id=beat_id,
            primary_emotion=primary_emotion,
            secondary_emotions=secondary_emotions or [],
            emotional_shift=emotional_shift,
            arc_momentum=arc_momentum,
            continuity_flags=continuity_flags
        )

        # Add to character's arc
        self.character_arcs[character_name].append(beat_state)

        return beat_state

    def _calculate_arc_momentum(self, character_name: str, current_emotion: EmotionalPoint) -> float:
        """Calculate emotional momentum based on character's emotional history"""
        if character_name not in self.character_arcs or len(self.character_arcs[character_name]) == 0:
            return 0.0

        previous_states = self.character_arcs[character_name]
        if len(previous_states) < 2:
            return 0.0

        # Simple momentum: compare current intensity with previous
        prev_intensity = previous_states[-1].primary_emotion.intensity.value
        curr_intensity = current_emotion.intensity.value

        momentum = (curr_intensity - prev_intensity) / 2.0  # Normalize to -1.0 to 1.0 range
        return max(-1.0, min(1.0, momentum))

    def _generate_continuity_flags(self, character_name: str, emotion: EmotionalPoint) -> List[str]:
        """Generate flags for RIP+RIC constraint validation"""
        flags = []

        # Flag sudden emotional shifts for RIP validation
        if character_name in self.character_arcs and len(self.character_arcs[character_name]) > 0:
            last_emotion = self.character_arcs[character_name][-1].primary_emotion

            # Large intensity change
            intensity_delta = abs(emotion.intensity.value - last_emotion.intensity.value)
            if intensity_delta > 0.5:
                flags.append('SUDDEN_INTENSITY_SHIFT')

            # Valence flip
            if (emotion.valence.value > 0) != (last_emotion.valence.value > 0):
                flags.append('VALENCE_FLIP')

            # Emotion type change
            if emotion.emotion_type != last_emotion.emotion_type:
                flags.append('EMOTION_TYPE_CHANGE')

        # Flag extreme emotional states
        if emotion.intensity == EmotionalIntensity.OVERWHELMING:
            flags.append('EXTREME_INTENSITY')

        if emotion.valence == EmotionalValence.CONFLICTED:
            flags.append('CONFLICTED_STATE')

        return flags

    def _get_next_timestamp(self, character_name: str) -> int:
        """Get next timestamp for character's emotional timeline"""
        if character_name not in self.character_arcs:
            return 0
        return len(self.character_arcs[character_name])

    def analyze_character_arc(self, character_name: str) -> Dict[str, Any]:
        """Analyze complete emotional arc for a character"""
        if character_name not in self.character_arcs:
            return {'error': f'No emotional data for character: {character_name}'}

        states = self.character_arcs[character_name]
        if not states:
            return {'error': f'No emotional states recorded for: {character_name}'}

        # Calculate arc statistics
        intensities = [state.primary_emotion.intensity.value for state in states]
        valences = [state.primary_emotion.valence.value for state in states]

        analysis = {
            'character_name': character_name,
            'total_beats': len(states),
            'emotional_range': {
                'min_intensity': min(intensities),
                'max_intensity': max(intensities),
                'avg_intensity': sum(intensities) / len(intensities)
            },
            'valence_pattern': {
                'avg_valence': sum(valences) / len(valences),
                'valence_shifts': sum(1 for i in range(1, len(valences))
                                   if (valences[i] > 0) != (valences[i-1] > 0))
            },
            'arc_momentum': states[-1].arc_momentum,
            'continuity_issues': sum(len(state.continuity_flags) for state in states),
            'dominant_emotions': self._get_dominant_emotions(states),
            'pattern_match': self._match_emotional_patterns(states)
        }

        return analysis

    def _get_dominant_emotions(self, states: List[EmotionalBeatState]) -> Dict[str, int]:
        """Get frequency count of emotions across all states"""
        emotion_counts = {}
        for state in states:
            emotion = state.primary_emotion.emotion_type
            emotion_counts[emotion] = emotion_counts.get(emotion, 0) + 1
        return dict(sorted(emotion_counts.items(), key=lambda x: x[1], reverse=True))

    def _match_emotional_patterns(self, states: List[EmotionalBeatState]) -> Optional[str]:
        """Match character's emotional sequence to known patterns"""
        if len(states) < 3:
            return None

        emotions = [state.primary_emotion.emotion_type for state in states]

        # Simple pattern matching - check if character emotions follow known arcs
        for pattern_name, pattern_emotions in self.emotional_patterns.items():
            if len(emotions) >= len(pattern_emotions):
                # Check if character's emotions contain the pattern sequence
                matches = 0
                for i, pattern_emotion in enumerate(pattern_emotions):
                    if i < len(emotions) and emotions[i] == pattern_emotion:
                        matches += 1

                # If majority of pattern matches, consider it a match
                if matches >= len(pattern_emotions) * 0.6:
                    return pattern_name

        return None

    def get_beat_emotional_summary(self, beat_id: str) -> Dict[str, Any]:
        """Get emotional summary for all characters in a specific beat"""
        if beat_id not in self.beat_registry:
            return {'error': f'No emotional data for beat: {beat_id}'}

        characters_in_beat = self.beat_registry[beat_id]
        beat_summary = {
            'beat_id': beat_id,
            'characters': {},
            'overall_intensity': 0.0,
            'emotional_conflicts': [],
            'continuity_flags': []
        }

        total_intensity = 0.0
        character_count = 0

        for character in characters_in_beat:
            if character in self.character_arcs:
                # Find the beat state for this character
                character_states = [state for state in self.character_arcs[character]
                                  if state.beat_id == beat_id]

                if character_states:
                    state = character_states[0]  # Should only be one per beat
                    beat_summary['characters'][character] = state.to_dict()
                    total_intensity += state.primary_emotion.intensity.value
                    character_count += 1
                    beat_summary['continuity_flags'].extend(state.continuity_flags)

        if character_count > 0:
            beat_summary['overall_intensity'] = total_intensity / character_count

        # Detect emotional conflicts between characters
        beat_summary['emotional_conflicts'] = self._detect_emotional_conflicts(beat_id)

        return beat_summary

    def _detect_emotional_conflicts(self, beat_id: str) -> List[Dict[str, str]]:
        """Detect emotional conflicts between characters in the same beat"""
        conflicts = []

        if beat_id not in self.beat_registry:
            return conflicts

        characters = self.beat_registry[beat_id]
        if len(characters) < 2:
            return conflicts

        # Get emotional states for all characters in this beat
        character_emotions = {}
        for character in characters:
            if character in self.character_arcs:
                states = [state for state in self.character_arcs[character]
                         if state.beat_id == beat_id]
                if states:
                    character_emotions[character] = states[0].primary_emotion

        # Check for valence conflicts (opposite emotional valences)
        for i, char1 in enumerate(characters):
            for char2 in characters[i+1:]:
                if char1 in character_emotions and char2 in character_emotions:
                    emotion1 = character_emotions[char1]
                    emotion2 = character_emotions[char2]

                    # Opposite valences with high intensity suggest conflict
                    if (emotion1.valence.value > 0) != (emotion2.valence.value > 0):
                        if emotion1.intensity.value >= 0.5 or emotion2.intensity.value >= 0.5:
                            conflicts.append({
                                'type': 'valence_conflict',
                                'character1': char1,
                                'character2': char2,
                                'emotion1': emotion1.emotion_type,
                                'emotion2': emotion2.emotion_type
                            })

        return conflicts

    def export_for_rip_integration(self) -> Dict[str, Any]:
        """Export emotional data in format suitable for RIP constraint validation"""
        export_data = {
            'tracker_type': 'EAT',
            'character_summaries': {},
            'beat_summaries': {},
            'continuity_flags': [],
            'pattern_violations': []
        }

        # Character summaries
        for character_name in self.character_arcs:
            analysis = self.analyze_character_arc(character_name)
            export_data['character_summaries'][character_name] = analysis

        # Beat summaries
        for beat_id in self.beat_registry:
            summary = self.get_beat_emotional_summary(beat_id)
            export_data['beat_summaries'][beat_id] = summary

        # Collect all continuity flags
        all_flags = []
        for character_states in self.character_arcs.values():
            for state in character_states:
                all_flags.extend(state.continuity_flags)
        export_data['continuity_flags'] = list(set(all_flags))

        # Detect pattern violations for RIP
        export_data['pattern_violations'] = self._detect_pattern_violations()

        return export_data

    def _detect_pattern_violations(self) -> List[Dict[str, Any]]:
        """Detect potential emotional pattern violations for RIP validation"""
        violations = []

        for character_name, states in self.character_arcs.items():
            if len(states) < 2:
                continue

            # Check for unrealistic emotional jumps
            for i in range(1, len(states)):
                prev_state = states[i-1]
                curr_state = states[i]

                intensity_jump = abs(curr_state.primary_emotion.intensity.value -
                                   prev_state.primary_emotion.intensity.value)

                # Flag massive intensity changes without emotional shift indication
                if intensity_jump > 0.7 and not curr_state.emotional_shift:
                    violations.append({
                        'type': 'sudden_intensity_change',
                        'character': character_name,
                        'beat_from': prev_state.beat_id,
                        'beat_to': curr_state.beat_id,
                        'intensity_delta': intensity_jump,
                        'severity': 'high' if intensity_jump > 0.8 else 'medium'
                    })

        return violations

    def reset_character_arc(self, character_name: str):
        """Reset emotional arc for a specific character"""
        if character_name in self.character_arcs:
            del self.character_arcs[character_name]

        # Clean up beat registry
        for beat_id in list(self.beat_registry.keys()):
            if character_name in self.beat_registry[beat_id]:
                self.beat_registry[beat_id].remove(character_name)
                if not self.beat_registry[beat_id]:
                    del self.beat_registry[beat_id]

    def get_tracker_health(self) -> Dict[str, Any]:
        """Get overall health metrics for the emotional tracking system"""
        total_characters = len(self.character_arcs)
        total_beats = len(self.beat_registry)
        total_states = sum(len(states) for states in self.character_arcs.values())
        total_flags = sum(len(state.continuity_flags)
                         for states in self.character_arcs.values()
                         for state in states)

        return {
            'tracker_type': 'EAT',
            'total_characters': total_characters,
            'total_beats': total_beats,
            'total_emotional_states': total_states,
            'total_continuity_flags': total_flags,
            'avg_states_per_character': total_states / max(1, total_characters),
            'avg_characters_per_beat': total_characters / max(1, total_beats) if total_beats > 0 else 0,
            'health_score': min(1.0, (total_states * 0.1) - (total_flags * 0.05))
        }