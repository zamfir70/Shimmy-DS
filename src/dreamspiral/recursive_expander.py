"""
🧬 Recursive Expander - RIP + RIC Unified Protocol Stack
========================================================

Implements the Constraint Genome and Guard Chain for recursive prose expansion.
Binds content integrity (RIP) with process integrity (RIC) through ligand-based
obligation anchoring and multi-layer validation.

Philosophy: Only elaborate meaning already seeded. Every expansion must bind
to an extracted ligand (anchor point) and pass the 4-layer guard chain.
"""

import re
import json
import logging
from typing import Dict, List, Optional, Tuple, NamedTuple
from dataclasses import dataclass
from enum import Enum
import asyncio
from datetime import datetime

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class LigandType(Enum):
    """Types of narrative ligands (anchor points for expansion)"""
    CHARACTER_ACTION = "character_action"
    EMOTIONAL_STATE = "emotional_state"
    SETTING_DETAIL = "setting_detail"
    CONFLICT_TENSION = "conflict_tension"
    DIALOGUE_SUBTEXT = "dialogue_subtext"
    THEME_RESONANCE = "theme_resonance"
    OBLIGATION_FULFILLMENT = "obligation_fulfillment"


@dataclass
class Ligand:
    """Narrative ligand - an anchor point that can be elaborated"""
    id: str
    type: LigandType
    content: str
    scope_constraints: List[str]  # Entity scope limitations
    emotional_vector: str  # Required emotional alignment
    obligation_anchor: Optional[str]  # Which obligation this resolves
    depth_level: int  # Surface (1) to Deep (5) elaboration level

    def __post_init__(self):
        """Validate ligand integrity"""
        if not self.content.strip():
            raise ValueError("Ligand content cannot be empty")
        if self.depth_level < 1 or self.depth_level > 5:
            raise ValueError("Depth level must be between 1-5")


@dataclass
class ConstraintGenome:
    """Complete constraint set parsed from seed and beat"""
    seed_text: str
    beat_text: str
    obligations: List[str]
    characters: List[str]
    tone_vector: str
    scope_entities: List[str]
    ligands: List[Ligand]
    extraction_timestamp: datetime

    def get_ligands_by_type(self, ligand_type: LigandType) -> List[Ligand]:
        """Get all ligands of a specific type"""
        return [l for l in self.ligands if l.type == ligand_type]

    def get_obligation_ligands(self) -> List[Ligand]:
        """Get ligands that can fulfill obligations"""
        return [l for l in self.ligands if l.obligation_anchor is not None]


class GuardChainResult(Enum):
    """Results from guard chain validation"""
    PASS = "pass"
    FAIL_ENTITY_SCOPE = "fail_entity_scope"
    FAIL_PROOF_ANCHOR = "fail_proof_anchor"
    FAIL_EMOTIONAL_VECTOR = "fail_emotional_vector"
    FAIL_SURFACE_DEPTH = "fail_surface_depth"


@dataclass
class ExpansionCandidate:
    """Candidate expansion for validation"""
    content: str
    target_ligand: Ligand
    proposed_depth: int
    emotional_tone: str
    entities_introduced: List[str]
    obligations_addressed: List[str]
    generation_timestamp: datetime


class RecursiveExpander:
    """
    Core recursive expansion engine with RIP constraint genome and guard chain.
    Implements the unified RIP+RIC protocol for fail-closed, sovereignty-preserving
    narrative expansion.
    """

    def __init__(self, ric_integration=None):
        """Initialize with optional RIC integration"""
        self.ric_integration = ric_integration
        self.expansion_history: List[Tuple[ExpansionCandidate, GuardChainResult]] = []
        self.ligand_extraction_patterns = self._init_extraction_patterns()

    def _init_extraction_patterns(self) -> Dict[LigandType, List[str]]:
        """Initialize regex patterns for ligand extraction"""
        return {
            LigandType.CHARACTER_ACTION: [
                r'(\w+)\s+(walked|ran|looked|said|thought|felt)',
                r'(\w+)\'s\s+(eyes|hands|voice|expression)',
                r'(\w+)\s+(hesitated|paused|smiled|frowned)',
            ],
            LigandType.EMOTIONAL_STATE: [
                r'(anxiety|fear|joy|sadness|anger|hope|despair)',
                r'(felt|feeling|emotion|mood)\s+(\w+)',
                r'(tension|pressure|relief|comfort)',
            ],
            LigandType.SETTING_DETAIL: [
                r'(the|a)\s+(room|house|street|forest|mountain)',
                r'(air|atmosphere|environment)\s+(was|felt|seemed)',
                r'(light|shadow|darkness|brightness)',
            ],
            LigandType.CONFLICT_TENSION: [
                r'(conflict|tension|disagreement|argument)',
                r'(against|opposed|conflicted|struggled)',
                r'(problem|issue|challenge|obstacle)',
            ],
            LigandType.DIALOGUE_SUBTEXT: [
                r'"([^"]+)"\s+(he|she|they)\s+(said|whispered|shouted)',
                r'(unspoken|implied|suggested|hinted)',
                r'(between the lines|subtext|meaning)',
            ],
            LigandType.THEME_RESONANCE: [
                r'(theme|meaning|significance|purpose)',
                r'(represents|symbolizes|embodies|reflects)',
                r'(deeper|profound|surface|shallow)',
            ]
        }

    def extract_beat_ligands(self, seed: str, beat: str) -> ConstraintGenome:
        """
        Extract ligands (anchor points) from seed and beat text.
        Creates the constraint genome that governs all expansions.
        """
        logger.info("Extracting beat ligands from seed and beat")

        # Parse basic constraints
        obligations = self._extract_obligations(seed, beat)
        characters = self._extract_characters(seed, beat)
        tone_vector = self._extract_tone_vector(seed, beat)
        scope_entities = self._extract_scope_entities(seed, beat)

        # Extract ligands using pattern matching
        ligands = []
        combined_text = f"{seed} {beat}"

        for ligand_type, patterns in self.ligand_extraction_patterns.items():
            type_ligands = self._extract_ligands_by_type(
                combined_text, ligand_type, patterns, scope_entities, tone_vector
            )
            ligands.extend(type_ligands)

        # Add obligation-specific ligands
        obligation_ligands = self._create_obligation_ligands(obligations, scope_entities, tone_vector)
        ligands.extend(obligation_ligands)

        genome = ConstraintGenome(
            seed_text=seed,
            beat_text=beat,
            obligations=obligations,
            characters=characters,
            tone_vector=tone_vector,
            scope_entities=scope_entities,
            ligands=ligands,
            extraction_timestamp=datetime.now()
        )

        logger.info(f"Extracted {len(ligands)} ligands across {len(set(l.type for l in ligands))} types")
        return genome

    def _extract_obligations(self, seed: str, beat: str) -> List[str]:
        """Extract narrative obligations from seed and beat"""
        obligations = []

        # Look for explicit obligation markers
        obligation_patterns = [
            r'must\s+(\w+(?:\s+\w+)*)',
            r'needs?\s+to\s+(\w+(?:\s+\w+)*)',
            r'has\s+to\s+(\w+(?:\s+\w+)*)',
            r'should\s+(\w+(?:\s+\w+)*)',
            r'will\s+(\w+(?:\s+\w+)*)',
        ]

        combined_text = f"{seed} {beat}"
        for pattern in obligation_patterns:
            matches = re.findall(pattern, combined_text, re.IGNORECASE)
            obligations.extend(matches)

        # Default obligation if none found
        if not obligations:
            obligations = ["advance the narrative"]

        return list(set(obligations))  # Remove duplicates

    def _extract_characters(self, seed: str, beat: str) -> List[str]:
        """Extract character names from seed and beat"""
        # Simple capitalized word extraction (could be enhanced with NER)
        combined_text = f"{seed} {beat}"
        potential_names = re.findall(r'\b[A-Z][a-z]+\b', combined_text)

        # Filter out common non-name words
        common_words = {'The', 'This', 'That', 'And', 'But', 'Or', 'In', 'On', 'At', 'To', 'For'}
        characters = [name for name in potential_names if name not in common_words]

        return list(set(characters))

    def _extract_tone_vector(self, seed: str, beat: str) -> str:
        """Extract emotional tone from seed and beat"""
        # Simple sentiment analysis based on keyword presence
        positive_words = ['happy', 'joy', 'smile', 'bright', 'hope', 'love', 'peace']
        negative_words = ['sad', 'dark', 'fear', 'anger', 'death', 'pain', 'despair']
        neutral_words = ['calm', 'quiet', 'steady', 'normal', 'regular']

        combined_text = f"{seed} {beat}".lower()

        pos_count = sum(1 for word in positive_words if word in combined_text)
        neg_count = sum(1 for word in negative_words if word in combined_text)
        neut_count = sum(1 for word in neutral_words if word in combined_text)

        if pos_count > neg_count and pos_count > neut_count:
            return "positive"
        elif neg_count > pos_count and neg_count > neut_count:
            return "negative"
        else:
            return "neutral"

    def _extract_scope_entities(self, seed: str, beat: str) -> List[str]:
        """Extract entities that define the scope boundaries"""
        entities = []

        # Extract locations
        location_patterns = [
            r'\b(house|home|room|kitchen|bedroom|office|street|park|forest|mountain|city|town|village)\b',
            r'\b(in|at|on|near|by)\s+(?:the\s+)?(\w+)',
        ]

        combined_text = f"{seed} {beat}".lower()
        for pattern in location_patterns:
            matches = re.findall(pattern, combined_text)
            if isinstance(matches[0], tuple) if matches else False:
                entities.extend([match[1] if len(match) > 1 else match[0] for match in matches])
            else:
                entities.extend(matches)

        # Add characters as entities
        entities.extend(self._extract_characters(seed, beat))

        return list(set(entities))

    def _extract_ligands_by_type(self, text: str, ligand_type: LigandType,
                                patterns: List[str], scope_entities: List[str],
                                tone_vector: str) -> List[Ligand]:
        """Extract ligands of a specific type using patterns"""
        ligands = []

        for i, pattern in enumerate(patterns):
            matches = re.finditer(pattern, text, re.IGNORECASE)
            for j, match in enumerate(matches):
                ligand_id = f"{ligand_type.value}_{i}_{j}"

                ligand = Ligand(
                    id=ligand_id,
                    type=ligand_type,
                    content=match.group(0),
                    scope_constraints=scope_entities,
                    emotional_vector=tone_vector,
                    obligation_anchor=None,  # Will be set for specific ligands
                    depth_level=1  # Start at surface level
                )
                ligands.append(ligand)

        return ligands

    def _create_obligation_ligands(self, obligations: List[str], scope_entities: List[str],
                                 tone_vector: str) -> List[Ligand]:
        """Create ligands specifically for obligation fulfillment"""
        ligands = []

        for i, obligation in enumerate(obligations):
            ligand = Ligand(
                id=f"obligation_{i}",
                type=LigandType.OBLIGATION_FULFILLMENT,
                content=f"fulfill: {obligation}",
                scope_constraints=scope_entities,
                emotional_vector=tone_vector,
                obligation_anchor=obligation,
                depth_level=2  # Obligations are typically deeper
            )
            ligands.append(ligand)

        return ligands

    def guard_chain_passes(self, expansion: ExpansionCandidate,
                          constraint_genome: ConstraintGenome) -> GuardChainResult:
        """
        Apply the 4-layer guard chain to validate expansion candidates.

        Guards:
        1. Entity Scope - no unauthorized people/places
        2. Proof Anchor - must resolve beat obligation
        3. Emotional Vector - must align to seed/beat tone
        4. Surface-Depth Congruence - metaphors must clarify, not obscure
        """

        # Guard 1: Entity Scope Check
        scope_result = self._check_entity_scope(expansion, constraint_genome)
        if scope_result != GuardChainResult.PASS:
            logger.warning(f"Entity scope guard failed for expansion: {expansion.content[:50]}...")
            return scope_result

        # Guard 2: Proof Anchor Check
        anchor_result = self._check_proof_anchor(expansion, constraint_genome)
        if anchor_result != GuardChainResult.PASS:
            logger.warning(f"Proof anchor guard failed for expansion: {expansion.content[:50]}...")
            return anchor_result

        # Guard 3: Emotional Vector Check
        emotion_result = self._check_emotional_vector(expansion, constraint_genome)
        if emotion_result != GuardChainResult.PASS:
            logger.warning(f"Emotional vector guard failed for expansion: {expansion.content[:50]}...")
            return emotion_result

        # Guard 4: Surface-Depth Congruence Check
        depth_result = self._check_surface_depth_congruence(expansion, constraint_genome)
        if depth_result != GuardChainResult.PASS:
            logger.warning(f"Surface-depth congruence guard failed for expansion: {expansion.content[:50]}...")
            return depth_result

        logger.info(f"All guards passed for expansion: {expansion.content[:50]}...")
        return GuardChainResult.PASS

    def _check_entity_scope(self, expansion: ExpansionCandidate,
                           constraint_genome: ConstraintGenome) -> GuardChainResult:
        """Guard 1: Check that expansion doesn't introduce unauthorized entities"""

        # Extract entities from expansion
        expansion_entities = self._extract_entities_from_text(expansion.content)

        # Check if any new entities are introduced that aren't in scope
        authorized_entities = set(constraint_genome.scope_entities)
        unauthorized_entities = []

        for entity in expansion_entities:
            if entity.lower() not in [e.lower() for e in authorized_entities]:
                # Allow some common entities that don't break scope
                if not self._is_allowed_generic_entity(entity):
                    unauthorized_entities.append(entity)

        if unauthorized_entities:
            logger.debug(f"Unauthorized entities detected: {unauthorized_entities}")
            return GuardChainResult.FAIL_ENTITY_SCOPE

        return GuardChainResult.PASS

    def _check_proof_anchor(self, expansion: ExpansionCandidate,
                           constraint_genome: ConstraintGenome) -> GuardChainResult:
        """Guard 2: Check that expansion resolves or advances beat obligations"""

        # Must bind to a ligand
        if not expansion.target_ligand:
            return GuardChainResult.FAIL_PROOF_ANCHOR

        # Check if expansion addresses obligations
        obligations_addressed = expansion.obligations_addressed
        available_obligations = constraint_genome.obligations

        # Must address at least one obligation or be bound to an obligation ligand
        if (not obligations_addressed and
            expansion.target_ligand.obligation_anchor is None):
            return GuardChainResult.FAIL_PROOF_ANCHOR

        # Check that addressed obligations are valid
        for obligation in obligations_addressed:
            if obligation not in available_obligations:
                return GuardChainResult.FAIL_PROOF_ANCHOR

        return GuardChainResult.PASS

    def _check_emotional_vector(self, expansion: ExpansionCandidate,
                               constraint_genome: ConstraintGenome) -> GuardChainResult:
        """Guard 3: Check emotional alignment with seed/beat tone"""

        required_tone = constraint_genome.tone_vector
        expansion_tone = expansion.emotional_tone

        # Allow neutral expansions with any tone
        if expansion_tone == "neutral":
            return GuardChainResult.PASS

        # Check for direct alignment
        if expansion_tone == required_tone:
            return GuardChainResult.PASS

        # Allow complementary emotional progressions
        if self._is_complementary_emotion(required_tone, expansion_tone):
            return GuardChainResult.PASS

        logger.debug(f"Emotional vector mismatch: required={required_tone}, got={expansion_tone}")
        return GuardChainResult.FAIL_EMOTIONAL_VECTOR

    def _check_surface_depth_congruence(self, expansion: ExpansionCandidate,
                                       constraint_genome: ConstraintGenome) -> GuardChainResult:
        """Guard 4: Check that metaphors clarify rather than obscure"""

        target_depth = expansion.target_ligand.depth_level
        proposed_depth = expansion.proposed_depth

        # Don't allow jumps of more than 2 levels
        if abs(proposed_depth - target_depth) > 2:
            return GuardChainResult.FAIL_SURFACE_DEPTH

        # Check for obscuring metaphors (simple heuristic)
        if self._contains_obscuring_metaphors(expansion.content):
            return GuardChainResult.FAIL_SURFACE_DEPTH

        return GuardChainResult.PASS

    def _extract_entities_from_text(self, text: str) -> List[str]:
        """Extract entity names from text"""
        # Simple capitalized word extraction
        entities = re.findall(r'\b[A-Z][a-z]+\b', text)
        return entities

    def _is_allowed_generic_entity(self, entity: str) -> bool:
        """Check if entity is a generic/common entity that doesn't break scope"""
        generic_entities = {
            'He', 'She', 'They', 'It', 'This', 'That', 'The', 'A', 'An',
            'Someone', 'Something', 'Everyone', 'Everything', 'Nobody', 'Nothing'
        }
        return entity in generic_entities

    def _is_complementary_emotion(self, required: str, actual: str) -> bool:
        """Check if emotions are complementary in narrative progression"""
        complementary_pairs = {
            ('negative', 'neutral'),
            ('neutral', 'positive'),
            ('positive', 'neutral'),
            ('neutral', 'negative'),
        }
        return (required, actual) in complementary_pairs

    def _contains_obscuring_metaphors(self, text: str) -> bool:
        """Check for metaphors that obscure rather than clarify"""
        # Simple heuristic: look for overly abstract or disconnected metaphors
        obscuring_patterns = [
            r'like\s+a\s+(\w+)\s+in\s+a\s+(\w+)',  # "like a fish in a library"
            r'as\s+if\s+(\w+)\s+were\s+(\w+)',     # "as if emotions were mathematics"
            r'metaphorically\s+speaking',
            r'in\s+a\s+sense',
            r'so\s+to\s+speak',
        ]

        for pattern in obscuring_patterns:
            if re.search(pattern, text, re.IGNORECASE):
                # Additional check: are the metaphor components related to the context?
                # This is a simplified check - could be enhanced with semantic analysis
                if not self._metaphor_components_related(text):
                    return True

        return False

    def _metaphor_components_related(self, text: str) -> bool:
        """Check if metaphor components are contextually related"""
        # Simplified check - in practice, this would use semantic similarity
        return True  # Placeholder - assume related for now

    async def expand_recursively(self, constraint_genome: ConstraintGenome,
                               max_iterations: int = 10) -> List[str]:
        """
        Perform recursive expansion with RIP+RIC protocol integration.
        Returns list of validated expansions.
        """
        logger.info(f"Starting recursive expansion with max {max_iterations} iterations")

        expansions = []
        iteration = 0

        while iteration < max_iterations:
            # Check RIC saturation if integrated
            if self.ric_integration:
                if not self.ric_integration.can_iterate("recursive_expander"):
                    logger.info("RIC saturation reached, stopping expansion")
                    break

            # Generate expansion candidates
            candidates = await self._generate_expansion_candidates(constraint_genome, expansions)

            if not candidates:
                logger.info("No more expansion candidates available")
                break

            # Validate each candidate through guard chain
            validated_expansions = []
            for candidate in candidates:
                guard_result = self.guard_chain_passes(candidate, constraint_genome)
                self.expansion_history.append((candidate, guard_result))

                if guard_result == GuardChainResult.PASS:
                    validated_expansions.append(candidate.content)

                    # Notify RIC of successful insight if integrated
                    if self.ric_integration:
                        self.ric_integration.reset_subsystem_on_insight("recursive_expander")

            if not validated_expansions:
                logger.info("No candidates passed guard chain validation")
                break

            expansions.extend(validated_expansions)
            iteration += 1

            logger.info(f"Iteration {iteration}: Added {len(validated_expansions)} expansions")

        logger.info(f"Recursive expansion complete: {len(expansions)} total expansions")
        return expansions

    async def _generate_expansion_candidates(self, constraint_genome: ConstraintGenome,
                                           existing_expansions: List[str]) -> List[ExpansionCandidate]:
        """Generate candidate expansions based on available ligands"""
        candidates = []

        # For each ligand, generate potential expansions
        for ligand in constraint_genome.ligands:
            candidate = await self._generate_candidate_for_ligand(ligand, constraint_genome, existing_expansions)
            if candidate:
                candidates.append(candidate)

        return candidates

    async def _generate_candidate_for_ligand(self, ligand: Ligand,
                                           constraint_genome: ConstraintGenome,
                                           existing_expansions: List[str]) -> Optional[ExpansionCandidate]:
        """Generate expansion candidate for a specific ligand"""

        # Simple expansion generation (in practice, this would use an LLM)
        base_content = ligand.content

        if ligand.type == LigandType.CHARACTER_ACTION:
            expansion_content = f"The action of {base_content} revealed deeper motivations."
        elif ligand.type == LigandType.EMOTIONAL_STATE:
            expansion_content = f"This {base_content} resonated through the scene, affecting everyone present."
        elif ligand.type == LigandType.SETTING_DETAIL:
            expansion_content = f"The {base_content} held significance beyond its immediate appearance."
        elif ligand.type == LigandType.OBLIGATION_FULFILLMENT:
            expansion_content = f"To {ligand.obligation_anchor}, the character took deliberate action."
        else:
            expansion_content = f"The element {base_content} demanded further exploration."

        # Extract entities and obligations addressed
        entities_introduced = self._extract_entities_from_text(expansion_content)
        obligations_addressed = [ligand.obligation_anchor] if ligand.obligation_anchor else []

        candidate = ExpansionCandidate(
            content=expansion_content,
            target_ligand=ligand,
            proposed_depth=ligand.depth_level + 1,
            emotional_tone=constraint_genome.tone_vector,
            entities_introduced=entities_introduced,
            obligations_addressed=obligations_addressed,
            generation_timestamp=datetime.now()
        )

        return candidate

    def get_guard_chain_statistics(self) -> Dict[str, int]:
        """Get statistics on guard chain performance"""
        stats = {
            "total_candidates": len(self.expansion_history),
            "passed": 0,
            "failed_entity_scope": 0,
            "failed_proof_anchor": 0,
            "failed_emotional_vector": 0,
            "failed_surface_depth": 0,
        }

        for _, result in self.expansion_history:
            if result == GuardChainResult.PASS:
                stats["passed"] += 1
            elif result == GuardChainResult.FAIL_ENTITY_SCOPE:
                stats["failed_entity_scope"] += 1
            elif result == GuardChainResult.FAIL_PROOF_ANCHOR:
                stats["failed_proof_anchor"] += 1
            elif result == GuardChainResult.FAIL_EMOTIONAL_VECTOR:
                stats["failed_emotional_vector"] += 1
            elif result == GuardChainResult.FAIL_SURFACE_DEPTH:
                stats["failed_surface_depth"] += 1

        return stats


# Example usage and testing
if __name__ == "__main__":
    # Test the recursive expander
    expander = RecursiveExpander()

    seed = "Maria stood in the empty kitchen, holding her grandmother's letter."
    beat = "She must decide whether to sell the house or honor her grandmother's wishes."

    # Extract constraint genome
    genome = expander.extract_beat_ligands(seed, beat)

    print(f"Extracted {len(genome.ligands)} ligands")
    print(f"Obligations: {genome.obligations}")
    print(f"Characters: {genome.characters}")
    print(f"Tone: {genome.tone_vector}")

    # Test expansion
    async def test_expansion():
        expansions = await expander.expand_recursively(genome, max_iterations=3)
        print(f"\nGenerated {len(expansions)} expansions:")
        for i, expansion in enumerate(expansions, 1):
            print(f"{i}. {expansion}")

        stats = expander.get_guard_chain_statistics()
        print(f"\nGuard Chain Statistics: {stats}")

    # Run the async test
    asyncio.run(test_expansion())