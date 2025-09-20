#!/usr/bin/env python3
"""
Shimmy-DS + LongWriter Integration

Direct integration with LongWriter's PyTorch-based long-form generation.
This creates a pipeline that uses Shimmy-DS for narrative intelligence
and LongWriter for extended generation.

Requirements:
- LongWriter repository cloned and set up
- Shimmy-DS running locally
- Python dependencies: torch, transformers, datasets, etc.

Usage:
    python longwriter_integration.py --prompt "Write a long story about..." --max-length 32000
"""

import os
import sys
import json
import asyncio
import aiohttp
import argparse
import logging
from typing import Dict, List, Any, Optional, Tuple
from dataclasses import dataclass
import torch
from transformers import AutoTokenizer, AutoModelForCausalLM
import warnings
warnings.filterwarnings("ignore")

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

@dataclass
class NarrativeInsights:
    """Narrative intelligence insights from Shimmy-DS"""
    health_score: float
    patterns: List[str]
    character_guidance: List[str]
    structure_suggestions: List[str]
    engagement_tips: List[str]

class LongWriterShimmyIntegration:
    """Integration between LongWriter and Shimmy-DS"""

    def __init__(self,
                 longwriter_model_path: str = "THUDM/LongWriter-llama3.1-8b",
                 shimmy_url: str = "http://127.0.0.1:11435",
                 device: str = "auto"):
        self.longwriter_model_path = longwriter_model_path
        self.shimmy_url = shimmy_url.rstrip('/')
        self.device = self._get_device(device)

        # LongWriter components
        self.tokenizer = None
        self.model = None
        self.session = None

        logger.info(f"Initializing integration with device: {self.device}")

    def _get_device(self, device: str) -> str:
        """Determine the best device to use"""
        if device == "auto":
            if torch.cuda.is_available():
                return "cuda"
            elif hasattr(torch.backends, 'mps') and torch.backends.mps.is_available():
                return "mps"
            else:
                return "cpu"
        return device

    async def __aenter__(self):
        """Async context manager entry"""
        self.session = aiohttp.ClientSession()
        await self.load_longwriter()
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        """Async context manager exit"""
        if self.session:
            await self.session.close()

    async def load_longwriter(self):
        """Load LongWriter model and tokenizer"""
        try:
            logger.info(f"Loading LongWriter model: {self.longwriter_model_path}")

            # Load tokenizer
            self.tokenizer = AutoTokenizer.from_pretrained(
                self.longwriter_model_path,
                trust_remote_code=True,
                use_fast=False
            )

            # Add padding token if missing
            if self.tokenizer.pad_token is None:
                self.tokenizer.pad_token = self.tokenizer.eos_token

            # Load model
            self.model = AutoModelForCausalLM.from_pretrained(
                self.longwriter_model_path,
                torch_dtype=torch.float16 if self.device == "cuda" else torch.float32,
                device_map="auto" if self.device == "cuda" else None,
                trust_remote_code=True,
                low_cpu_mem_usage=True
            )

            if self.device != "cuda":
                self.model = self.model.to(self.device)

            self.model.eval()
            logger.info("LongWriter model loaded successfully")

        except Exception as e:
            logger.error(f"Failed to load LongWriter: {e}")
            raise

    async def get_narrative_insights(self, text: str) -> NarrativeInsights:
        """Get narrative intelligence insights from Shimmy-DS"""
        try:
            # Get narrative analysis
            async with self.session.get(f"{self.shimmy_url}/narrative/analyze") as resp:
                if resp.status != 200:
                    logger.warning(f"Shimmy-DS analysis failed: {resp.status}")
                    return self._default_insights()

                data = await resp.json()

                # Extract insights
                health = data.get("narrative_health", {}).get("overall_score", 0.5)
                patterns = [p.get("description", "") for p in data.get("active_patterns", [])]

                # Get character guidance
                char_guidance = []
                for pattern in data.get("active_patterns", []):
                    if pattern.get("type") == "character_arc":
                        char = pattern.get("character", "")
                        arc = pattern.get("arc_type", "")
                        progress = pattern.get("progress", 0)
                        char_guidance.append(f"{char}: {arc} arc at {progress:.0%} completion")

                # Generate structure suggestions
                structure_suggestions = []
                constraints = data.get("constraints", {})
                if constraints.get("freedom_score", 1.0) < 0.5:
                    structure_suggestions.append("Story has limited narrative paths - consider opening new possibilities")

                # Generate engagement tips
                engagement_tips = []
                engagement = data.get("engagement_metrics", {})
                if engagement.get("curiosity_score", 0) < 0.7:
                    engagement_tips.append("Increase mystery and reader curiosity")
                if engagement.get("tension_level", 0) < 0.6:
                    engagement_tips.append("Build more dramatic tension")

                return NarrativeInsights(
                    health_score=health,
                    patterns=patterns,
                    character_guidance=char_guidance,
                    structure_suggestions=structure_suggestions,
                    engagement_tips=engagement_tips
                )

        except Exception as e:
            logger.error(f"Error getting narrative insights: {e}")
            return self._default_insights()

    def _default_insights(self) -> NarrativeInsights:
        """Default insights when Shimmy-DS is unavailable"""
        return NarrativeInsights(
            health_score=0.5,
            patterns=[],
            character_guidance=[],
            structure_suggestions=["Focus on clear story structure"],
            engagement_tips=["Maintain reader interest through pacing"]
        )

    def enhance_prompt_with_insights(self, prompt: str, insights: NarrativeInsights) -> str:
        """Enhance the prompt with narrative intelligence"""
        enhancements = []

        # Add health-based guidance
        if insights.health_score < 0.6:
            enhancements.append("Focus on narrative coherence and consistency.")
        elif insights.health_score > 0.8:
            enhancements.append("Continue the strong narrative momentum.")

        # Add pattern guidance
        for pattern in insights.patterns[:2]:  # Top 2 patterns
            if pattern:
                enhancements.append(f"Continue developing: {pattern}")

        # Add character guidance
        for guidance in insights.character_guidance[:2]:
            enhancements.append(f"Character development: {guidance}")

        # Add structure suggestions
        for suggestion in insights.structure_suggestions[:1]:
            enhancements.append(suggestion)

        # Add engagement tips
        for tip in insights.engagement_tips[:1]:
            enhancements.append(tip)

        # Combine with original prompt
        if enhancements:
            enhanced = f"{prompt}\n\n[Narrative Intelligence Guidelines: {' '.join(enhancements)}]"
        else:
            enhanced = prompt

        logger.info(f"Enhanced prompt with {len(enhancements)} narrative insights")
        return enhanced

    def generate_with_longwriter(self,
                                prompt: str,
                                max_length: int = 32000,
                                temperature: float = 0.7,
                                top_p: float = 0.9,
                                repetition_penalty: float = 1.1) -> str:
        """Generate long-form content with LongWriter"""
        try:
            logger.info(f"Generating {max_length} tokens with LongWriter")

            # Tokenize input
            inputs = self.tokenizer(
                prompt,
                return_tensors="pt",
                truncation=True,
                max_length=self.tokenizer.model_max_length - max_length
            ).to(self.device)

            # Generate
            with torch.no_grad():
                outputs = self.model.generate(
                    inputs.input_ids,
                    max_length=min(max_length + inputs.input_ids.shape[1], self.tokenizer.model_max_length),
                    temperature=temperature,
                    top_p=top_p,
                    repetition_penalty=repetition_penalty,
                    do_sample=True,
                    pad_token_id=self.tokenizer.pad_token_id,
                    eos_token_id=self.tokenizer.eos_token_id,
                    use_cache=True
                )

            # Decode generated text
            generated_text = self.tokenizer.decode(
                outputs[0][inputs.input_ids.shape[1]:],
                skip_special_tokens=True
            )

            logger.info(f"Generated {len(generated_text.split())} words")
            return generated_text.strip()

        except Exception as e:
            logger.error(f"Error generating with LongWriter: {e}")
            return ""

    async def post_analyze_with_shimmy(self, generated_text: str) -> Dict[str, Any]:
        """Analyze generated content with Shimmy-DS"""
        try:
            # Send for analysis
            payload = {
                "model": "analysis",
                "messages": [
                    {"role": "system", "content": "Analyze this long-form text for narrative quality, structure, and engagement."},
                    {"role": "user", "content": f"Please analyze this text:\n\n{generated_text}"}
                ],
                "max_tokens": 500
            }

            async with self.session.post(
                f"{self.shimmy_url}/v1/chat/completions",
                json=payload
            ) as resp:
                if resp.status == 200:
                    data = await resp.json()
                    analysis = data.get("choices", [{}])[0].get("message", {}).get("content", "")

                    # Get updated insights
                    final_insights = await self.get_narrative_insights(generated_text)

                    return {
                        "analysis": analysis,
                        "final_health_score": final_insights.health_score,
                        "identified_patterns": final_insights.patterns,
                        "character_analysis": final_insights.character_guidance,
                        "suggestions": final_insights.structure_suggestions + final_insights.engagement_tips
                    }
                else:
                    logger.warning(f"Post-analysis failed: {resp.status}")
                    return {"analysis": "Analysis unavailable", "final_health_score": 0.5}

        except Exception as e:
            logger.error(f"Error in post-analysis: {e}")
            return {"analysis": f"Analysis error: {e}", "final_health_score": 0.5}

    async def generate_intelligent_longform(self,
                                          prompt: str,
                                          max_length: int = 32000,
                                          temperature: float = 0.7) -> Dict[str, Any]:
        """Complete workflow: analyze → enhance → generate → post-analyze"""

        logger.info("Starting intelligent long-form generation")

        # Step 1: Get initial narrative insights
        logger.info("Getting initial narrative insights from Shimmy-DS")
        initial_insights = await self.get_narrative_insights(prompt)

        # Step 2: Enhance prompt with insights
        logger.info("Enhancing prompt with narrative intelligence")
        enhanced_prompt = self.enhance_prompt_with_insights(prompt, initial_insights)

        # Step 3: Generate with LongWriter
        logger.info("Generating long-form content with LongWriter")
        generated_text = self.generate_with_longwriter(
            enhanced_prompt,
            max_length=max_length,
            temperature=temperature
        )

        # Step 4: Post-analyze with Shimmy-DS
        logger.info("Post-analyzing generated content")
        post_analysis = await self.post_analyze_with_shimmy(generated_text)

        return {
            "original_prompt": prompt,
            "enhanced_prompt": enhanced_prompt,
            "generated_text": generated_text,
            "initial_insights": {
                "health_score": initial_insights.health_score,
                "patterns": initial_insights.patterns,
                "character_guidance": initial_insights.character_guidance,
                "suggestions": initial_insights.structure_suggestions + initial_insights.engagement_tips
            },
            "post_analysis": post_analysis,
            "metrics": {
                "word_count": len(generated_text.split()),
                "character_count": len(generated_text),
                "enhancement_count": len(enhanced_prompt.split()) - len(prompt.split())
            }
        }

async def main():
    parser = argparse.ArgumentParser(description="LongWriter + Shimmy-DS Integration")
    parser.add_argument("--prompt", required=True, help="Prompt for long-form generation")
    parser.add_argument("--max-length", type=int, default=16000, help="Maximum tokens to generate")
    parser.add_argument("--temperature", type=float, default=0.7, help="Generation temperature")
    parser.add_argument("--model", default="THUDM/LongWriter-llama3.1-8b", help="LongWriter model path")
    parser.add_argument("--shimmy-url", default="http://127.0.0.1:11435", help="Shimmy-DS URL")
    parser.add_argument("--device", default="auto", help="Device to use (auto/cuda/cpu/mps)")
    parser.add_argument("--output", help="Output file for results")

    args = parser.parse_args()

    try:
        async with LongWriterShimmyIntegration(
            longwriter_model_path=args.model,
            shimmy_url=args.shimmy_url,
            device=args.device
        ) as integration:

            result = await integration.generate_intelligent_longform(
                prompt=args.prompt,
                max_length=args.max_length,
                temperature=args.temperature
            )

            # Output results
            if args.output:
                with open(args.output, 'w', encoding='utf-8') as f:
                    json.dump(result, f, indent=2, ensure_ascii=False)
                print(f"Results saved to {args.output}")
                print(f"Generated {result['metrics']['word_count']} words")
            else:
                print(json.dumps(result, indent=2, ensure_ascii=False))

    except Exception as e:
        logger.error(f"Integration failed: {e}")
        sys.exit(1)

if __name__ == "__main__":
    asyncio.run(main())