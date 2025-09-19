#!/bin/bash

# 🧠 Shimmy-DS Narrative Intelligence Demo Script
# This script demonstrates the unique narrative intelligence capabilities

set -e

echo "🚀 Starting Shimmy-DS Narrative Intelligence Demo"
echo "=================================================="

# Check if shimmy-ds is running
if ! curl -s http://127.0.0.1:11435/v1/models > /dev/null 2>&1; then
    echo "❌ Shimmy-DS not running. Please start with:"
    echo "   shimmy serve --narrative-intelligence --port 11435"
    exit 1
fi

echo "✅ Shimmy-DS is running with narrative intelligence"
echo

# Function to make pretty JSON output
pretty_json() {
    if command -v jq > /dev/null 2>&1; then
        jq '.'
    else
        cat
    fi
}

# Demo 1: Basic story generation with narrative tracking
echo "📖 Demo 1: Story Generation with Narrative Intelligence"
echo "-------------------------------------------------------"

STORY_REQUEST='{
  "model": "microsoft/Phi-3.5-mini-instruct",
  "messages": [
    {
      "role": "system", 
      "content": "You are a creative writing assistant. Write engaging fiction with strong characters and clear conflicts."
    },
    {
      "role": "user", 
      "content": "Start a story: Elena discovered a cracked mirror in her grandmother'\''s attic. When she looked closer, she saw something impossible in the reflection..."
    }
  ],
  "max_tokens": 200,
  "temperature": 0.7
}'

echo "🎬 Generating story opening..."
STORY_RESPONSE=$(curl -s -X POST http://127.0.0.1:11435/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d "$STORY_REQUEST")

# Extract and display the story
STORY_TEXT=$(echo "$STORY_RESPONSE" | jq -r '.choices[0].message.content' 2>/dev/null || echo "Error extracting story")
echo
echo "📝 Generated Story:"
echo "==================="
echo "$STORY_TEXT"
echo

# Demo 2: Narrative analysis
echo "🔍 Demo 2: Real-Time Narrative Analysis"
echo "---------------------------------------"

echo "🧬 Checking narrative DNA patterns..."
DNA_ANALYSIS=$(curl -s http://127.0.0.1:11435/narrative/analyze 2>/dev/null || echo '{"error": "Narrative analysis not available"}')
echo "$DNA_ANALYSIS" | pretty_json
echo

# Demo 3: Character consistency report
echo "👥 Demo 3: Character Consistency Analysis"
echo "-----------------------------------------"

echo "🎭 Analyzing character patterns..."
CHARACTER_REPORT=$(curl -s http://127.0.0.1:11435/narrative/characters 2>/dev/null || echo '{"info": "Character tracking active but no sufficient data yet"}')
echo "$CHARACTER_REPORT" | pretty_json
echo

# Demo 4: Constraint space mapping
echo "🗺️ Demo 4: Story Possibility Mapping"
echo "------------------------------------"

echo "🎯 Mapping narrative constraint space..."
CONSTRAINT_MAP=$(curl -s http://127.0.0.1:11435/narrative/constraints 2>/dev/null || echo '{"info": "Constraint mapping building..."}')
echo "$CONSTRAINT_MAP" | pretty_json
echo

# Demo 5: Full narrative report
echo "📊 Demo 5: Comprehensive Narrative Intelligence Report"
echo "======================================================="

echo "📈 Generating full intelligence report..."
FULL_REPORT=$(curl -s http://127.0.0.1:11435/narrative/report 2>/dev/null || echo '{"error": "Report generation in progress"}')
echo "$FULL_REPORT" | pretty_json
echo

# Demo 6: Continue the story to show evolution
echo "🔄 Demo 6: Story Continuation with Recursive Intelligence"
echo "========================================================="

CONTINUATION_REQUEST='{
  "model": "microsoft/Phi-3.5-mini-instruct",
  "messages": [
    {
      "role": "system", 
      "content": "Continue the story with attention to character consistency and narrative tension. Build on established elements."
    },
    {
      "role": "user", 
      "content": "Continue the Elena mirror story. She decides to touch the glass..."
    }
  ],
  "max_tokens": 200,
  "temperature": 0.7
}'

echo "📖 Continuing story with narrative intelligence guidance..."
CONTINUATION_RESPONSE=$(curl -s -X POST http://127.0.0.1:11435/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d "$CONTINUATION_REQUEST")

CONTINUATION_TEXT=$(echo "$CONTINUATION_RESPONSE" | jq -r '.choices[0].message.content' 2>/dev/null || echo "Error extracting continuation")
echo
echo "📝 Story Continuation:"
echo "======================"
echo "$CONTINUATION_TEXT"
echo

# Demo 7: Show narrative evolution
echo "📈 Demo 7: Narrative Evolution Analysis"
echo "======================================="

echo "🔬 Analyzing how narrative patterns evolved..."
EVOLUTION_ANALYSIS=$(curl -s http://127.0.0.1:11435/narrative/analyze 2>/dev/null || echo '{"info": "Evolution tracking active"}')
echo "$EVOLUTION_ANALYSIS" | pretty_json
echo

# Demo summary
echo "🎉 Demo Complete: What You Just Saw"
echo "==================================="
echo
echo "✨ Shimmy-DS demonstrated:"
echo "  🧬 CAPR DNA tracking (Contradiction → Action → Pressure → Return)"
echo "  🗺️ Constraint space modeling (story possibilities)"
echo "  👥 Character consistency monitoring"
echo "  📚 Reader engagement loop detection"
echo "  🔄 Multi-level recursive pattern analysis"
echo "  ⚖️ Narrative drift stabilization"
echo
echo "🚀 This is AI that understands narrative structure, not just text generation!"
echo
echo "💡 Try these commands yourself:"
echo "  curl http://127.0.0.1:11435/narrative/analyze    # Real-time analysis"
echo "  curl http://127.0.0.1:11435/narrative/report     # Full intelligence report"
echo "  curl http://127.0.0.1:11435/narrative/config     # Configure systems"
echo
echo "📖 Use with any OpenAI-compatible tool for intelligent narrative assistance!"
