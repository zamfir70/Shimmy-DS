#!/usr/bin/env python3
"""
Shimmy-DS + LongWriter Integration Bridge

This script creates a bridge between Shimmy-DS's recursive narrative intelligence
and LongWriter's long-form generation capabilities.

Usage:
    python longwriter_bridge.py --shimmy-url http://127.0.0.1:11435 --longwriter-url http://127.0.0.1:8000
"""

import asyncio
import aiohttp
import argparse
import json
import logging
from typing import Dict, List, Any, Optional
from dataclasses import dataclass
from datetime import datetime

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

@dataclass
class NarrativeContext:
    """Narrative context from Shimmy-DS analysis"""
    health_score: float
    active_patterns: List[Dict[str, Any]]
    constraints: Dict[str, Any]
    character_states: List[Dict[str, Any]]
    engagement_metrics: Dict[str, float]

@dataclass
class LongWriterRequest:
    """LongWriter generation request"""
    prompt: str
    max_tokens: int = 8192
    temperature: float = 0.7
    top_p: float = 0.9
    repetition_penalty: float = 1.1

class ShimmyDSLongWriterBridge:
    """Bridge between Shimmy-DS and LongWriter"""

    def __init__(self, shimmy_url: str, longwriter_url: str):
        self.shimmy_url = shimmy_url.rstrip('/')
        self.longwriter_url = longwriter_url.rstrip('/')
        self.session: Optional[aiohttp.ClientSession] = None

    async def __aenter__(self):
        self.session = aiohttp.ClientSession()
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        if self.session:
            await self.session.close()

    async def analyze_narrative_context(self, text: str) -> NarrativeContext:
        """Get narrative analysis from Shimmy-DS"""
        try:
            # Get narrative analysis
            async with self.session.get(f"{self.shimmy_url}/narrative/analyze") as resp:
                if resp.status == 200:
                    data = await resp.json()
                    return NarrativeContext(
                        health_score=data.get("narrative_health", {}).get("overall_score", 0.5),
                        active_patterns=data.get("active_patterns", []),
                        constraints=data.get("constraints", {}),
                        character_states=data.get("character_states", []),
                        engagement_metrics=data.get("engagement_metrics", {})
                    )
                else:
                    logger.warning(f"Shimmy-DS analysis failed: {resp.status}")
                    return self._default_context()
        except Exception as e:
            logger.error(f"Error getting narrative context: {e}")
            return self._default_context()

    def _default_context(self) -> NarrativeContext:
        """Default narrative context when analysis fails"""
        return NarrativeContext(
            health_score=0.5,
            active_patterns=[],
            constraints={},
            character_states=[],
            engagement_metrics={}
        )

    def enhance_prompt_with_context(self, original_prompt: str, context: NarrativeContext) -> str:
        """Enhance prompt with narrative intelligence"""
        enhancements = []

        # Add narrative health guidance
        if context.health_score < 0.6:
            enhancements.append("Focus on narrative coherence and character consistency.")
        elif context.health_score > 0.8:
            enhancements.append("Continue the strong narrative momentum.")

        # Add pattern continuation
        for pattern in context.active_patterns[:3]:  # Top 3 patterns
            if pattern.get("type") == "CAPR_loop":
                enhancements.append(f"Continue the narrative pattern: {pattern.get('description', '')}")
            elif pattern.get("type") == "character_arc":
                char = pattern.get("character", "")
                arc = pattern.get("arc_type", "")
                enhancements.append(f"Develop {char}'s {arc} arc further.")

        # Add constraint awareness
        if context.constraints.get("pressure_points"):
            points = context.constraints["pressure_points"]
            enhancements.append(f"Pay attention to: {', '.join(points)}")

        # Add engagement optimization
        engagement = context.engagement_metrics
        if engagement.get("curiosity_score", 0) < 0.7:
            enhancements.append("Increase reader curiosity and intrigue.")
        if engagement.get("tension_level", 0) < 0.6:
            enhancements.append("Build narrative tension.")

        # Combine original prompt with enhancements
        if enhancements:
            enhanced = f"{original_prompt}\n\n[Narrative Intelligence Guidance: {' '.join(enhancements)}]"
        else:
            enhanced = original_prompt

        logger.info(f"Enhanced prompt with {len(enhancements)} narrative insights")
        return enhanced

    async def generate_with_longwriter(self, request: LongWriterRequest) -> str:
        """Generate long-form content with LongWriter"""
        try:
            payload = {
                "prompt": request.prompt,
                "max_tokens": request.max_tokens,
                "temperature": request.temperature,
                "top_p": request.top_p,
                "repetition_penalty": request.repetition_penalty
            }

            async with self.session.post(
                f"{self.longwriter_url}/v1/completions",
                json=payload,
                headers={"Content-Type": "application/json"}
            ) as resp:
                if resp.status == 200:
                    data = await resp.json()
                    return data.get("choices", [{}])[0].get("text", "")
                else:
                    logger.error(f"LongWriter generation failed: {resp.status}")
                    return ""
        except Exception as e:
            logger.error(f"Error generating with LongWriter: {e}")
            return ""

    async def post_process_with_shimmy(self, generated_text: str) -> Dict[str, Any]:
        """Post-process generated content with Shimmy-DS"""
        try:
            # Send generated text for analysis
            payload = {
                "model": "analysis",
                "messages": [
                    {"role": "system", "content": "Analyze this generated text for narrative quality."},
                    {"role": "user", "content": generated_text}
                ],
                "max_tokens": 100
            }

            async with self.session.post(
                f"{self.shimmy_url}/v1/chat/completions",
                json=payload
            ) as resp:
                if resp.status == 200:
                    data = await resp.json()
                    analysis = data.get("choices", [{}])[0].get("message", {}).get("content", "")

                    # Get updated narrative analysis
                    context = await self.analyze_narrative_context(generated_text)

                    return {
                        "analysis": analysis,
                        "narrative_health": context.health_score,
                        "suggestions": self._generate_suggestions(context)
                    }
                else:
                    logger.warning(f"Post-processing failed: {resp.status}")
                    return {"analysis": "Analysis unavailable", "narrative_health": 0.5, "suggestions": []}
        except Exception as e:
            logger.error(f"Error in post-processing: {e}")
            return {"analysis": "Analysis error", "narrative_health": 0.5, "suggestions": []}

    def _generate_suggestions(self, context: NarrativeContext) -> List[str]:
        """Generate improvement suggestions based on context"""
        suggestions = []

        if context.health_score < 0.6:
            suggestions.append("Consider revising for better narrative coherence")

        if not context.active_patterns:
            suggestions.append("Consider adding more narrative patterns or themes")

        engagement = context.engagement_metrics
        if engagement.get("tension_level", 0) < 0.5:
            suggestions.append("Increase narrative tension and conflict")

        return suggestions

    async def generate_long_narrative(
        self,
        prompt: str,
        max_tokens: int = 8192,
        temperature: float = 0.7
    ) -> Dict[str, Any]:
        """Complete workflow: analyze → enhance → generate → post-process"""

        logger.info("Starting long narrative generation workflow")

        # Step 1: Analyze current narrative context
        logger.info("Step 1: Analyzing narrative context")
        context = await self.analyze_narrative_context(prompt)

        # Step 2: Enhance prompt with narrative intelligence
        logger.info("Step 2: Enhancing prompt with narrative intelligence")
        enhanced_prompt = self.enhance_prompt_with_context(prompt, context)

        # Step 3: Generate with LongWriter
        logger.info("Step 3: Generating long-form content with LongWriter")
        request = LongWriterRequest(
            prompt=enhanced_prompt,
            max_tokens=max_tokens,
            temperature=temperature
        )
        generated_text = await self.generate_with_longwriter(request)

        # Step 4: Post-process with Shimmy-DS
        logger.info("Step 4: Post-processing with Shimmy-DS")
        post_analysis = await self.post_process_with_shimmy(generated_text)

        return {
            "original_prompt": prompt,
            "enhanced_prompt": enhanced_prompt,
            "generated_text": generated_text,
            "initial_context": context.__dict__,
            "post_analysis": post_analysis,
            "workflow_completed": datetime.now().isoformat(),
            "total_tokens": len(generated_text.split()) if generated_text else 0
        }

async def main():
    parser = argparse.ArgumentParser(description="Shimmy-DS + LongWriter Integration Bridge")
    parser.add_argument("--shimmy-url", default="http://127.0.0.1:11435", help="Shimmy-DS server URL")
    parser.add_argument("--longwriter-url", default="http://127.0.0.1:8000", help="LongWriter server URL")
    parser.add_argument("--prompt", required=True, help="Initial prompt for generation")
    parser.add_argument("--max-tokens", type=int, default=8192, help="Maximum tokens to generate")
    parser.add_argument("--temperature", type=float, default=0.7, help="Generation temperature")
    parser.add_argument("--output", help="Output file for results")

    args = parser.parse_args()

    async with ShimmyDSLongWriterBridge(args.shimmy_url, args.longwriter_url) as bridge:
        result = await bridge.generate_long_narrative(
            prompt=args.prompt,
            max_tokens=args.max_tokens,
            temperature=args.temperature
        )

        # Output results
        output_data = json.dumps(result, indent=2, ensure_ascii=False)

        if args.output:
            with open(args.output, 'w', encoding='utf-8') as f:
                f.write(output_data)
            print(f"Results saved to {args.output}")
        else:
            print(output_data)

if __name__ == "__main__":
    asyncio.run(main())