#!/usr/bin/env python3
"""
Creative Writing Test for Shimmy-DS
Tests narrative intelligence and creative capabilities
"""

import json
import requests
import time
import sys

# Test configuration
SHIMMY_URL = "http://localhost:11435"
TEST_PROMPTS = [
    {
        "name": "Character Development",
        "prompt": "Write a short story about a mysterious librarian who discovers books that write themselves. Focus on character development and show the librarian's emotional journey.",
        "expected_elements": ["character development", "emotional journey", "mystery", "books"]
    },
    {
        "name": "World Building",
        "prompt": "Describe a floating city powered by crystallized music. Include details about how the society functions and what daily life is like.",
        "expected_elements": ["floating city", "crystallized music", "society", "daily life"]
    },
    {
        "name": "Dialogue and Voice",
        "prompt": "Create a conversation between a time traveler and their past self. Make each voice distinct and show their different perspectives.",
        "expected_elements": ["dialogue", "time travel", "distinct voices", "different perspectives"]
    }
]

def test_shimmy_connection():
    """Test if Shimmy server is running"""
    try:
        response = requests.get(f"{SHIMMY_URL}/health", timeout=5)
        return response.status_code == 200
    except:
        return False

def test_creative_writing(prompt_data):
    """Test creative writing with a specific prompt"""
    print(f"\nTesting: {prompt_data['name']}")
    print(f"Prompt: {prompt_data['prompt'][:100]}...")

    payload = {
        "model": "creative-writing-lora",
        "messages": [
            {
                "role": "system",
                "content": "You are a creative writing assistant specializing in narrative intelligence. Focus on character development, world-building, and engaging storytelling."
            },
            {
                "role": "user",
                "content": prompt_data['prompt']
            }
        ],
        "max_tokens": 500,
        "temperature": 0.8,
        "stream": False
    }

    try:
        start_time = time.time()
        response = requests.post(f"{SHIMMY_URL}/v1/chat/completions",
                               json=payload,
                               timeout=30)
        end_time = time.time()

        if response.status_code == 200:
            result = response.json()
            content = result['choices'][0]['message']['content']

            print(f"Generated ({end_time - start_time:.2f}s):")
            print(f"Content: {content[:200]}...")

            # Check for expected elements
            found_elements = []
            for element in prompt_data['expected_elements']:
                if any(word in content.lower() for word in element.lower().split()):
                    found_elements.append(element)

            print(f"Found elements: {found_elements}")
            return True, content, end_time - start_time
        else:
            print(f"Error: HTTP {response.status_code}")
            print(f"Response: {response.text}")
            return False, None, 0

    except Exception as e:
        print(f"Exception: {e}")
        return False, None, 0

def main():
    print("Shimmy-DS Creative Writing Test")
    print("=" * 50)

    # Check if server is running
    if not test_shimmy_connection():
        print("Cannot connect to Shimmy server at", SHIMMY_URL)
        print("Please start the server with: ./target/release/shimmy.exe serve")
        sys.exit(1)

    print("Connected to Shimmy server")

    # Run creative writing tests
    results = []
    total_time = 0

    for prompt_data in TEST_PROMPTS:
        success, content, duration = test_creative_writing(prompt_data)
        results.append({
            'name': prompt_data['name'],
            'success': success,
            'duration': duration,
            'content': content
        })
        total_time += duration
        time.sleep(1)  # Brief pause between tests

    # Summary
    print("\n" + "=" * 50)
    print("Test Summary")
    print("=" * 50)

    successful_tests = sum(1 for r in results if r['success'])
    print(f"Successful tests: {successful_tests}/{len(results)}")
    print(f"Total time: {total_time:.2f}s")
    print(f"Average time per test: {total_time/len(results):.2f}s")

    if successful_tests == len(results):
        print("All creative writing tests passed!")
        return 0
    else:
        print("Some tests failed")
        return 1

if __name__ == "__main__":
    sys.exit(main())