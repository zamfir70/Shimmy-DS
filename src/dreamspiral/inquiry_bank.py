"""
RIE-lite (Recursive Inquiry Engine) - Minimal Elegance Module

Seed/beat-bound question bank with light RIP+RIC integration.
Goal: Increase creative power without complexity.

Core Focus:
- Generate contextual creative questions for narrative expansion
- Maintain question templates tied to narrative structure
- Provide recursive inquiry loops for deeper exploration
- 100% compatible with existing RIP+RIC pipeline
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple, Any, Callable
from enum import Enum
import random
import re


class InquiryType(Enum):
    CHARACTER_DEPTH = "character_depth"        # What drives this character?
    WORLD_BUILDING = "world_building"          # How does this world work?
    PLOT_DEVELOPMENT = "plot_development"      # What happens next?
    EMOTIONAL_EXPLORATION = "emotional_exploration"  # What are they feeling?
    RELATIONSHIP_DYNAMICS = "relationship_dynamics"  # How do characters relate?
    THEMATIC_RESONANCE = "thematic_resonance"  # What does this mean?
    SENSORY_DETAIL = "sensory_detail"          # What do we see/hear/feel?
    CONTRADICTION_PROBE = "contradiction_probe"  # What doesn't fit?


class InquiryDepth(Enum):
    SURFACE = 1      # Quick questions for immediate expansion
    MEDIUM = 2       # Deeper questions requiring thought
    DEEP = 3         # Complex questions that reshape understanding
    RECURSIVE = 4    # Questions that generate more questions


@dataclass
class InquiryTemplate:
    """Template for generating contextual questions"""
    template_id: str
    inquiry_type: InquiryType
    depth: InquiryDepth
    question_pattern: str  # Pattern with {placeholders}
    context_requirements: List[str]  # What context is needed
    followup_triggers: List[str] = field(default_factory=list)  # When to ask followups
    rip_anchors: List[str] = field(default_factory=list)  # RIP integration points

    def generate_question(self, context: Dict[str, Any]) -> Optional[str]:
        """Generate actual question from template using context"""
        try:
            return self.question_pattern.format(**context)
        except KeyError:
            return None  # Missing required context

    def to_dict(self) -> Dict[str, Any]:
        return {
            'template_id': self.template_id,
            'inquiry_type': self.inquiry_type.value,
            'depth': self.depth.value,
            'question_pattern': self.question_pattern,
            'context_requirements': self.context_requirements,
            'followup_triggers': self.followup_triggers,
            'rip_anchors': self.rip_anchors
        }


@dataclass
class ActiveInquiry:
    """An active question generated for a specific narrative context"""
    inquiry_id: str
    beat_id: str
    seed_id: str  # Which narrative seed this relates to
    question: str
    inquiry_type: InquiryType
    depth: InquiryDepth
    context_snapshot: Dict[str, Any]
    generated_answers: List[str] = field(default_factory=list)
    followup_questions: List[str] = field(default_factory=list)
    exploration_paths: List[str] = field(default_factory=list)  # Potential directions
    rip_constraints: List[str] = field(default_factory=list)  # RIP validation hooks

    def add_answer(self, answer: str):
        """Add an answer and potentially trigger followups"""
        self.generated_answers.append(answer)
        # Analyze answer for new exploration paths
        self._analyze_answer_for_paths(answer)

    def _analyze_answer_for_paths(self, answer: str):
        """Analyze answer to discover new exploration paths"""
        # Simple keyword analysis for new concepts
        answer_words = re.findall(r'\b[a-zA-Z]{4,}\b', answer.lower())

        # Look for new characters mentioned
        potential_chars = re.findall(r'\b[A-Z][a-z]+\b', answer)
        for char in potential_chars:
            if char not in self.context_snapshot.get('characters', []):
                self.exploration_paths.append(f"EXPLORE_CHARACTER:{char}")

        # Look for emotional language
        emotional_words = ['love', 'hate', 'fear', 'hope', 'anger', 'joy', 'sadness', 'surprise']
        for emotion in emotional_words:
            if emotion in answer_words:
                self.exploration_paths.append(f"EXPLORE_EMOTION:{emotion}")

        # Look for conflict indicators
        conflict_words = ['against', 'fight', 'battle', 'oppose', 'conflict', 'struggle']
        for conflict in conflict_words:
            if conflict in answer_words:
                self.exploration_paths.append(f"EXPLORE_CONFLICT:{conflict}")

    def to_dict(self) -> Dict[str, Any]:
        return {
            'inquiry_id': self.inquiry_id,
            'beat_id': self.beat_id,
            'seed_id': self.seed_id,
            'question': self.question,
            'inquiry_type': self.inquiry_type.value,
            'depth': self.depth.value,
            'context_snapshot': self.context_snapshot,
            'generated_answers': self.generated_answers,
            'followup_questions': self.followup_questions,
            'exploration_paths': self.exploration_paths,
            'rip_constraints': self.rip_constraints
        }


class RecursiveInquiryEngine:
    """
    Light inquiry system that generates contextual questions for narrative expansion.

    Design Philosophy:
    - Generate creative questions tied to narrative context
    - Minimal surface area - focus on actionable questions
    - Seed/beat alignment - questions bound to narrative structure
    - RIP+RIC compatible - provides constraint hooks
    """

    def __init__(self):
        self.templates: Dict[str, InquiryTemplate] = {}
        self.active_inquiries: Dict[str, ActiveInquiry] = {}
        self.seed_question_map: Dict[str, List[str]] = {}  # seed_id -> [inquiry_ids]
        self.beat_question_map: Dict[str, List[str]] = {}  # beat_id -> [inquiry_ids]
        self.inquiry_counter = 0

        # Initialize template library
        self._initialize_template_library()

    def _initialize_template_library(self):
        """Initialize common question templates"""
        templates = [
            # Character depth templates
            InquiryTemplate(
                template_id="char_motivation",
                inquiry_type=InquiryType.CHARACTER_DEPTH,
                depth=InquiryDepth.MEDIUM,
                question_pattern="What truly motivates {character} to {action}?",
                context_requirements=["character", "action"],
                followup_triggers=["backstory", "fear", "desire"],
                rip_anchors=["CHARACTER_CONSISTENCY", "MOTIVATION_TRACKING"]
            ),
            InquiryTemplate(
                template_id="char_secret",
                inquiry_type=InquiryType.CHARACTER_DEPTH,
                depth=InquiryDepth.DEEP,
                question_pattern="What secret is {character} hiding that would change everything?",
                context_requirements=["character"],
                followup_triggers=["reveal", "consequence", "discovery"],
                rip_anchors=["CHARACTER_SECRETS", "PLOT_IMPLICATIONS"]
            ),
            InquiryTemplate(
                template_id="char_contradiction",
                inquiry_type=InquiryType.CONTRADICTION_PROBE,
                depth=InquiryDepth.RECURSIVE,
                question_pattern="How does {character}'s {trait} contradict their {action}?",
                context_requirements=["character", "trait", "action"],
                followup_triggers=["internal_conflict", "growth", "resolution"],
                rip_anchors=["CHARACTER_CONSISTENCY", "CONTRADICTION_RESOLUTION"]
            ),

            # World building templates
            InquiryTemplate(
                template_id="world_rule",
                inquiry_type=InquiryType.WORLD_BUILDING,
                depth=InquiryDepth.MEDIUM,
                question_pattern="How does {element} work in this world?",
                context_requirements=["element"],
                followup_triggers=["limitation", "cost", "consequence"],
                rip_anchors=["WORLD_CONSISTENCY", "RULE_TRACKING"]
            ),
            InquiryTemplate(
                template_id="world_history",
                inquiry_type=InquiryType.WORLD_BUILDING,
                depth=InquiryDepth.DEEP,
                question_pattern="What happened here before {event} that still affects {location}?",
                context_requirements=["event", "location"],
                followup_triggers=["legacy", "hidden_truth", "connection"],
                rip_anchors=["WORLD_HISTORY", "LOCATION_SIGNIFICANCE"]
            ),

            # Plot development templates
            InquiryTemplate(
                template_id="plot_obstacle",
                inquiry_type=InquiryType.PLOT_DEVELOPMENT,
                depth=InquiryDepth.SURFACE,
                question_pattern="What obstacle prevents {character} from {goal}?",
                context_requirements=["character", "goal"],
                followup_triggers=["complication", "escalation", "help"],
                rip_anchors=["PLOT_PROGRESSION", "OBSTACLE_TRACKING"]
            ),
            InquiryTemplate(
                template_id="plot_stakes",
                inquiry_type=InquiryType.PLOT_DEVELOPMENT,
                depth=InquiryDepth.MEDIUM,
                question_pattern="What happens if {character} fails at {task}?",
                context_requirements=["character", "task"],
                followup_triggers=["consequence", "alternative", "rescue"],
                rip_anchors=["STAKES_TRACKING", "CONSEQUENCE_CHAIN"]
            ),

            # Emotional exploration templates
            InquiryTemplate(
                template_id="emotion_root",
                inquiry_type=InquiryType.EMOTIONAL_EXPLORATION,
                depth=InquiryDepth.DEEP,
                question_pattern="What childhood experience makes {character} {emotion} about {situation}?",
                context_requirements=["character", "emotion", "situation"],
                followup_triggers=["memory", "trigger", "healing"],
                rip_anchors=["EMOTIONAL_CONSISTENCY", "BACKSTORY_CONNECTION"]
            ),
            InquiryTemplate(
                template_id="emotion_conflict",
                inquiry_type=InquiryType.EMOTIONAL_EXPLORATION,
                depth=InquiryDepth.RECURSIVE,
                question_pattern="How does {character} feel about feeling {emotion}?",
                context_requirements=["character", "emotion"],
                followup_triggers=["meta_emotion", "shame", "acceptance"],
                rip_anchors=["EMOTIONAL_LAYERS", "INTERNAL_CONFLICT"]
            ),

            # Relationship dynamics templates
            InquiryTemplate(
                template_id="relationship_tension",
                inquiry_type=InquiryType.RELATIONSHIP_DYNAMICS,
                depth=InquiryDepth.MEDIUM,
                question_pattern="What unspoken tension exists between {character1} and {character2}?",
                context_requirements=["character1", "character2"],
                followup_triggers=["confrontation", "resolution", "escalation"],
                rip_anchors=["RELATIONSHIP_TRACKING", "TENSION_MANAGEMENT"]
            ),
            InquiryTemplate(
                template_id="relationship_history",
                inquiry_type=InquiryType.RELATIONSHIP_DYNAMICS,
                depth=InquiryDepth.DEEP,
                question_pattern="What shared experience bonded {character1} and {character2}?",
                context_requirements=["character1", "character2"],
                followup_triggers=["shared_secret", "loyalty", "betrayal"],
                rip_anchors=["RELATIONSHIP_HISTORY", "BOND_STRENGTH"]
            ),

            # Thematic resonance templates
            InquiryTemplate(
                template_id="theme_symbol",
                inquiry_type=InquiryType.THEMATIC_RESONANCE,
                depth=InquiryDepth.DEEP,
                question_pattern="What does {object} symbolize for {character}?",
                context_requirements=["object", "character"],
                followup_triggers=["meaning", "change", "loss"],
                rip_anchors=["SYMBOLIC_TRACKING", "THEMATIC_CONSISTENCY"]
            ),
            InquiryTemplate(
                template_id="theme_lesson",
                inquiry_type=InquiryType.THEMATIC_RESONANCE,
                depth=InquiryDepth.RECURSIVE,
                question_pattern="What is this story teaching {character} about {theme}?",
                context_requirements=["character", "theme"],
                followup_triggers=["realization", "growth", "application"],
                rip_anchors=["THEMATIC_ARC", "CHARACTER_GROWTH"]
            ),

            # Sensory detail templates
            InquiryTemplate(
                template_id="sensory_atmosphere",
                inquiry_type=InquiryType.SENSORY_DETAIL,
                depth=InquiryDepth.SURFACE,
                question_pattern="What does {location} smell like when {character} is {emotion}?",
                context_requirements=["location", "character", "emotion"],
                followup_triggers=["memory", "association", "comfort"],
                rip_anchors=["SENSORY_CONSISTENCY", "ATMOSPHERE_TRACKING"]
            ),
            InquiryTemplate(
                template_id="sensory_memory",
                inquiry_type=InquiryType.SENSORY_DETAIL,
                depth=InquiryDepth.MEDIUM,
                question_pattern="What sound takes {character} back to {memory}?",
                context_requirements=["character", "memory"],
                followup_triggers=["flashback", "emotion", "connection"],
                rip_anchors=["SENSORY_MEMORY", "BACKSTORY_TRIGGER"]
            )
        ]

        for template in templates:
            self.templates[template.template_id] = template

    def generate_inquiry(self, seed_id: str, beat_id: str, context: Dict[str, Any],
                        inquiry_type: Optional[InquiryType] = None,
                        depth: Optional[InquiryDepth] = None) -> Optional[ActiveInquiry]:
        """Generate a contextual inquiry for the given narrative context"""

        # Filter templates by type and depth if specified
        eligible_templates = list(self.templates.values())

        if inquiry_type:
            eligible_templates = [t for t in eligible_templates if t.inquiry_type == inquiry_type]

        if depth:
            eligible_templates = [t for t in eligible_templates if t.depth == depth]

        # Find templates that can be satisfied by available context
        usable_templates = []
        for template in eligible_templates:
            if all(req in context for req in template.context_requirements):
                usable_templates.append(template)

        if not usable_templates:
            return None

        # Select template (could be random or based on some preference)
        selected_template = random.choice(usable_templates)

        # Generate question
        question = selected_template.generate_question(context)
        if not question:
            return None

        # Create active inquiry
        inquiry_id = f"inquiry_{self.inquiry_counter}"
        self.inquiry_counter += 1

        # Generate RIP constraints based on template and context
        rip_constraints = self._generate_rip_constraints(selected_template, context)

        inquiry = ActiveInquiry(
            inquiry_id=inquiry_id,
            beat_id=beat_id,
            seed_id=seed_id,
            question=question,
            inquiry_type=selected_template.inquiry_type,
            depth=selected_template.depth,
            context_snapshot=context.copy(),
            rip_constraints=rip_constraints
        )

        # Register inquiry
        self.active_inquiries[inquiry_id] = inquiry

        if seed_id not in self.seed_question_map:
            self.seed_question_map[seed_id] = []
        self.seed_question_map[seed_id].append(inquiry_id)

        if beat_id not in self.beat_question_map:
            self.beat_question_map[beat_id] = []
        self.beat_question_map[beat_id].append(inquiry_id)

        return inquiry

    def generate_followup_inquiry(self, parent_inquiry_id: str, answer: str) -> List[ActiveInquiry]:
        """Generate followup inquiries based on an answer to a previous question"""
        if parent_inquiry_id not in self.active_inquiries:
            return []

        parent = self.active_inquiries[parent_inquiry_id]
        parent.add_answer(answer)

        followups = []

        # Generate followups based on exploration paths discovered in answer
        for path in parent.exploration_paths[-3:]:  # Limit to recent paths
            followup_context = parent.context_snapshot.copy()

            # Parse exploration path
            if ':' in path:
                path_type, path_value = path.split(':', 1)

                if path_type == 'EXPLORE_CHARACTER':
                    followup_context['character'] = path_value
                    followup_context['new_character'] = True
                elif path_type == 'EXPLORE_EMOTION':
                    followup_context['emotion'] = path_value
                elif path_type == 'EXPLORE_CONFLICT':
                    followup_context['conflict'] = path_value

            # Generate inquiry for this path
            followup = self.generate_inquiry(
                seed_id=parent.seed_id,
                beat_id=parent.beat_id,
                context=followup_context,
                depth=InquiryDepth.MEDIUM  # Followups are usually medium depth
            )

            if followup:
                followup.inquiry_id = f"{parent_inquiry_id}_followup_{len(followups)}"
                self.active_inquiries[followup.inquiry_id] = followup
                parent.followup_questions.append(followup.question)
                followups.append(followup)

        return followups

    def _generate_rip_constraints(self, template: InquiryTemplate, context: Dict[str, Any]) -> List[str]:
        """Generate RIP constraints for inquiry validation"""
        constraints = template.rip_anchors.copy()

        # Add context-based constraints
        if 'character' in context:
            constraints.append(f"CHARACTER_BOUND:{context['character']}")

        if 'location' in context:
            constraints.append(f"LOCATION_BOUND:{context['location']}")

        # Add depth-based constraints
        if template.depth == InquiryDepth.RECURSIVE:
            constraints.append("RECURSIVE_SAFETY_CHECK")

        if template.depth == InquiryDepth.DEEP:
            constraints.append("DEEP_EXPLORATION_VALIDATION")

        return constraints

    def get_seed_inquiries(self, seed_id: str) -> List[ActiveInquiry]:
        """Get all inquiries for a specific seed"""
        if seed_id not in self.seed_question_map:
            return []

        inquiry_ids = self.seed_question_map[seed_id]
        return [self.active_inquiries[inquiry_id] for inquiry_id in inquiry_ids
                if inquiry_id in self.active_inquiries]

    def get_beat_inquiries(self, beat_id: str) -> List[ActiveInquiry]:
        """Get all inquiries for a specific beat"""
        if beat_id not in self.beat_question_map:
            return []

        inquiry_ids = self.beat_question_map[beat_id]
        return [self.active_inquiries[inquiry_id] for inquiry_id in inquiry_ids
                if inquiry_id in self.active_inquiries]

    def suggest_inquiry_type(self, context: Dict[str, Any]) -> InquiryType:
        """Suggest most appropriate inquiry type based on context"""
        context_keys = set(context.keys())

        # Heuristic suggestion based on available context
        if 'character' in context and 'emotion' in context:
            return InquiryType.EMOTIONAL_EXPLORATION

        if 'character1' in context and 'character2' in context:
            return InquiryType.RELATIONSHIP_DYNAMICS

        if 'world' in context or 'location' in context:
            return InquiryType.WORLD_BUILDING

        if 'contradiction' in context or 'inconsistency' in context:
            return InquiryType.CONTRADICTION_PROBE

        if len(context.get('characters', [])) > 1:
            return InquiryType.RELATIONSHIP_DYNAMICS

        if 'character' in context:
            return InquiryType.CHARACTER_DEPTH

        return InquiryType.PLOT_DEVELOPMENT  # Default

    def analyze_inquiry_patterns(self) -> Dict[str, Any]:
        """Analyze patterns in active inquiries"""
        analysis = {
            'total_inquiries': len(self.active_inquiries),
            'type_distribution': {},
            'depth_distribution': {},
            'answered_ratio': 0.0,
            'followup_ratio': 0.0,
            'recursive_chains': 0,
            'exploration_paths': []
        }

        if not self.active_inquiries:
            return analysis

        # Type and depth distribution
        for inquiry in self.active_inquiries.values():
            inquiry_type = inquiry.inquiry_type.value
            depth = inquiry.depth.value

            analysis['type_distribution'][inquiry_type] = analysis['type_distribution'].get(inquiry_type, 0) + 1
            analysis['depth_distribution'][depth] = analysis['depth_distribution'].get(depth, 0) + 1

        # Answer metrics
        answered_count = sum(1 for inquiry in self.active_inquiries.values() if inquiry.generated_answers)
        analysis['answered_ratio'] = answered_count / len(self.active_inquiries)

        # Followup metrics
        followup_count = sum(1 for inquiry in self.active_inquiries.values() if inquiry.followup_questions)
        analysis['followup_ratio'] = followup_count / len(self.active_inquiries)

        # Recursive chains (inquiries that generated followups)
        analysis['recursive_chains'] = followup_count

        # Collect all exploration paths
        all_paths = []
        for inquiry in self.active_inquiries.values():
            all_paths.extend(inquiry.exploration_paths)
        analysis['exploration_paths'] = list(set(all_paths))

        return analysis

    def export_for_rip_integration(self) -> Dict[str, Any]:
        """Export inquiry data for RIP constraint validation"""
        return {
            'tracker_type': 'RIE-lite',
            'inquiry_analysis': self.analyze_inquiry_patterns(),
            'unanswered_inquiries': [
                inquiry.to_dict() for inquiry in self.active_inquiries.values()
                if not inquiry.generated_answers
            ],
            'recursive_chains': [
                inquiry.to_dict() for inquiry in self.active_inquiries.values()
                if inquiry.followup_questions
            ],
            'rip_constraint_violations': self._detect_rip_constraint_violations()
        }

    def _detect_rip_constraint_violations(self) -> List[Dict[str, Any]]:
        """Detect violations that should trigger RIP constraint validation"""
        violations = []

        # Check for too many unanswered inquiries
        unanswered = sum(1 for inquiry in self.active_inquiries.values() if not inquiry.generated_answers)
        if unanswered > len(self.active_inquiries) * 0.7:
            violations.append({
                'type': 'excessive_unanswered_inquiries',
                'severity': 'medium',
                'count': unanswered,
                'recommendation': 'Address pending questions before generating new ones'
            })

        # Check for recursive depth issues
        deep_recursive = sum(1 for inquiry in self.active_inquiries.values()
                           if inquiry.depth == InquiryDepth.RECURSIVE and len(inquiry.followup_questions) > 3)
        if deep_recursive > 0:
            violations.append({
                'type': 'excessive_recursive_depth',
                'severity': 'high',
                'count': deep_recursive,
                'recommendation': 'Limit recursive inquiry chains to prevent analysis paralysis'
            })

        return violations

    def get_tracker_health(self) -> Dict[str, Any]:
        """Get overall health metrics for the inquiry system"""
        analysis = self.analyze_inquiry_patterns()

        return {
            'tracker_type': 'RIE-lite',
            'total_inquiries': analysis['total_inquiries'],
            'answered_ratio': analysis['answered_ratio'],
            'followup_ratio': analysis['followup_ratio'],
            'recursive_chains': analysis['recursive_chains'],
            'exploration_paths_count': len(analysis['exploration_paths']),
            'template_coverage': len([t for t in self.templates.values() if any(
                inquiry.inquiry_type == t.inquiry_type for inquiry in self.active_inquiries.values()
            )]) / len(self.templates),
            'health_score': min(1.0, analysis['answered_ratio'] + (analysis['followup_ratio'] * 0.5))
        }

    def reset_seed_inquiries(self, seed_id: str):
        """Reset all inquiries for a specific seed"""
        if seed_id in self.seed_question_map:
            inquiry_ids = self.seed_question_map[seed_id].copy()
            for inquiry_id in inquiry_ids:
                if inquiry_id in self.active_inquiries:
                    del self.active_inquiries[inquiry_id]
            del self.seed_question_map[seed_id]

        # Clean up beat mappings
        for beat_id in list(self.beat_question_map.keys()):
            self.beat_question_map[beat_id] = [
                inquiry_id for inquiry_id in self.beat_question_map[beat_id]
                if inquiry_id in self.active_inquiries
            ]
            if not self.beat_question_map[beat_id]:
                del self.beat_question_map[beat_id]