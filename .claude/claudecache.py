#!/usr/bin/env python3
"""
🧠 Enhanced Memory System - Cache Management
Memory updates with analysis tracking and cross-session learning
"""
import os
import json
import sys
from pathlib import Path
from datetime import datetime
import hashlib

class ClaudeMemoryCache:
    def __init__(self):
        self.cache_dir = Path.cwd() / '.claude' / 'memory_cache'
        self.cache_dir.mkdir(parents=True, exist_ok=True)
        self.session_file = self.cache_dir / 'session_context.json'
        self.project_file = self.cache_dir / 'project_memory.json'
        self.analysis_file = self.cache_dir / 'analysis_history.json'

    def load_existing_data(self, file_path):
        """Load existing data from file"""
        if file_path.exists():
            with open(file_path, 'r', encoding='utf-8') as f:
                return json.load(f)
        return {}

    def save_data(self, file_path, data):
        """Save data to file with backup"""
        backup_path = file_path.with_suffix(f'.bak.{datetime.now().strftime("%Y%m%d_%H%M%S")}')
        if file_path.exists():
            file_path.rename(backup_path)

        with open(file_path, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=2, ensure_ascii=False)

    def update_session_context(self, updates):
        """Update session context with new information"""
        session_data = self.load_existing_data(self.session_file)
        session_data.update({
            'last_updated': datetime.now().isoformat(),
            'session_id': hashlib.md5(str(datetime.now()).encode()).hexdigest()[:8],
            **updates
        })
        self.save_data(self.session_file, session_data)
        return session_data

    def update_project_memory(self, updates):
        """Update project-specific memory"""
        project_data = self.load_existing_data(self.project_file)
        project_data.update({
            'last_updated': datetime.now().isoformat(),
            **updates
        })
        self.save_data(self.project_file, project_data)
        return project_data

    def log_analysis(self, analysis_type, details):
        """Log analysis for cross-session learning"""
        analysis_data = self.load_existing_data(self.analysis_file)
        if 'analyses' not in analysis_data:
            analysis_data['analyses'] = []

        analysis_entry = {
            'timestamp': datetime.now().isoformat(),
            'type': analysis_type,
            'details': details,
            'session_id': analysis_data.get('current_session', 'unknown')
        }

        analysis_data['analyses'].append(analysis_entry)
        analysis_data['last_analysis'] = datetime.now().isoformat()

        # Keep only last 100 analyses to manage size
        if len(analysis_data['analyses']) > 100:
            analysis_data['analyses'] = analysis_data['analyses'][-100:]

        self.save_data(self.analysis_file, analysis_data)
        return analysis_entry

    def interactive_update(self):
        """Interactive memory update session"""
        print("🧠 CLAUDE MEMORY CACHE UPDATE")
        print("=" * 40)

        # Session Context Updates
        print("\n📋 SESSION CONTEXT UPDATES:")
        session_updates = {}

        current_task = input("Current task/focus (or Enter to skip): ").strip()
        if current_task:
            session_updates['current_task'] = current_task

        key_findings = input("Key findings/learnings (or Enter to skip): ").strip()
        if key_findings:
            session_updates['key_findings'] = key_findings.split(';')

        # Project Memory Updates
        print("\n🏗️ PROJECT MEMORY UPDATES:")
        project_updates = {}

        project_type = input("Project type/technology (or Enter to skip): ").strip()
        if project_type:
            project_updates['project_type'] = project_type

        key_files = input("Important files identified (comma-separated, or Enter to skip): ").strip()
        if key_files:
            project_updates['key_files'] = [f.strip() for f in key_files.split(',')]

        patterns = input("Code patterns/conventions noted (or Enter to skip): ").strip()
        if patterns:
            project_updates['patterns'] = patterns.split(';')

        # Save updates
        if session_updates:
            self.update_session_context(session_updates)
            print(f"✅ Session context updated with {len(session_updates)} items")

        if project_updates:
            self.update_project_memory(project_updates)
            print(f"✅ Project memory updated with {len(project_updates)} items")

        # Log this update
        self.log_analysis('manual_update', {
            'session_items': len(session_updates),
            'project_items': len(project_updates)
        })

        print("\n🎯 Memory cache updated successfully!")

    def auto_detect_updates(self):
        """Auto-detect potential memory updates from current directory"""
        cwd = Path.cwd()
        updates = {
            'working_directory': str(cwd),
            'detected_at': datetime.now().isoformat()
        }

        # Detect project type
        if (cwd / 'Cargo.toml').exists():
            updates['project_type'] = 'rust'
        elif (cwd / 'package.json').exists():
            updates['project_type'] = 'nodejs'
        elif (cwd / 'requirements.txt').exists() or (cwd / 'pyproject.toml').exists():
            updates['project_type'] = 'python'

        # Detect key files
        key_files = []
        for pattern in ['*.rs', '*.py', '*.js', '*.ts', '*.json', '*.md']:
            key_files.extend([str(p.relative_to(cwd)) for p in cwd.glob(pattern)])

        if key_files:
            updates['detected_files'] = key_files[:20]  # Limit to 20 files

        return self.update_project_memory(updates)

if __name__ == "__main__":
    cache = ClaudeMemoryCache()

    if len(sys.argv) > 1 and sys.argv[1] == '--auto':
        cache.auto_detect_updates()
        print("🤖 Auto-detection complete")
    else:
        cache.interactive_update()