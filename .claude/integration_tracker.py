#!/usr/bin/env python3
"""
🔧 Integration Tracking System
Verification protocols and session tracking for enhanced Claude capabilities
"""
import os
import json
import sys
from pathlib import Path
from datetime import datetime
import subprocess

class IntegrationTracker:
    def __init__(self):
        self.claude_dir = Path.cwd() / '.claude'
        self.tracking_file = self.claude_dir / 'integration_status.json'
        self.session_log = self.claude_dir / 'session_log.json'

    def check_system_status(self):
        """Check status of all enhanced capabilities"""
        status = {
            'timestamp': datetime.now().isoformat(),
            'memory_system': self._check_memory_system(),
            'performance_optimizations': self._check_performance_features(),
            'settings_configuration': self._check_settings(),
            'verification_complete': False
        }

        # Overall verification
        status['verification_complete'] = all([
            status['memory_system']['functional'],
            status['performance_optimizations']['enabled'],
            status['settings_configuration']['valid']
        ])

        self._save_status(status)
        return status

    def _check_memory_system(self):
        """Verify memory system components"""
        clrecall_exists = (self.claude_dir / 'clrecall.py').exists()
        claudecache_exists = (self.claude_dir / 'claudecache.py').exists()
        cache_dir_exists = (self.claude_dir / 'memory_cache').exists()

        return {
            'clrecall_script': clrecall_exists,
            'claudecache_script': claudecache_exists,
            'cache_directory': cache_dir_exists,
            'functional': clrecall_exists and claudecache_exists
        }

    def _check_performance_features(self):
        """Check performance optimization status"""
        settings_file = self.claude_dir / 'settings.local.json'
        if not settings_file.exists():
            return {'enabled': False, 'env_vars': {}}

        try:
            with open(settings_file, 'r') as f:
                settings = json.load(f)

            env_vars = settings.get('env', {})
            performance_vars = {
                'batch_operations': env_vars.get('CLAUDE_BATCH_OPERATIONS') == 'true',
                'parallel_execution': env_vars.get('CLAUDE_PARALLEL_EXECUTION') == 'true',
                'proactive_planning': env_vars.get('CLAUDE_PROACTIVE_PLANNING') == 'true',
                'todowrite_integration': env_vars.get('CLAUDE_TODOWRITE_INTEGRATION') == 'true'
            }

            return {
                'enabled': all(performance_vars.values()),
                'env_vars': performance_vars
            }
        except Exception:
            return {'enabled': False, 'env_vars': {}}

    def _check_settings(self):
        """Validate settings configuration"""
        settings_file = self.claude_dir / 'settings.local.json'
        if not settings_file.exists():
            return {'valid': False, 'issues': ['Settings file missing']}

        try:
            with open(settings_file, 'r') as f:
                settings = json.load(f)

            issues = []

            # Check permissions
            permissions = settings.get('permissions', {})
            if 'python .claude/clrecall.py' not in str(permissions.get('allow', [])):
                issues.append('Missing clrecall.py permission')
            if 'python .claude/claudecache.py' not in str(permissions.get('allow', [])):
                issues.append('Missing claudecache.py permission')

            # Check hooks
            hooks = settings.get('hooks', {})
            if 'SessionStart' not in hooks:
                issues.append('Missing SessionStart hook')

            return {
                'valid': len(issues) == 0,
                'issues': issues
            }
        except Exception as e:
            return {'valid': False, 'issues': [f'JSON parse error: {str(e)}']}

    def _save_status(self, status):
        """Save integration status"""
        with open(self.tracking_file, 'w') as f:
            json.dump(status, f, indent=2)

    def log_session_event(self, event_type, details=None):
        """Log session events for tracking"""
        if not self.session_log.exists():
            session_data = {'sessions': []}
        else:
            with open(self.session_log, 'r') as f:
                session_data = json.load(f)

        event = {
            'timestamp': datetime.now().isoformat(),
            'type': event_type,
            'details': details or {}
        }

        session_data['sessions'].append(event)

        # Keep only last 50 events
        if len(session_data['sessions']) > 50:
            session_data['sessions'] = session_data['sessions'][-50:]

        with open(self.session_log, 'w') as f:
            json.dump(session_data, f, indent=2)

    def display_status_report(self):
        """Display comprehensive status report"""
        status = self.check_system_status()

        print("🔧 INTEGRATION TRACKING SYSTEM")
        print("=" * 50)

        # Memory System Status
        memory = status['memory_system']
        print(f"\n🧠 MEMORY SYSTEM: {'✅ FUNCTIONAL' if memory['functional'] else '❌ ISSUES'}")
        print(f"  • clrecall.py: {'✅' if memory['clrecall_script'] else '❌'}")
        print(f"  • claudecache.py: {'✅' if memory['claudecache_script'] else '❌'}")
        print(f"  • Cache directory: {'✅' if memory['cache_directory'] else '❌'}")

        # Performance Optimizations
        perf = status['performance_optimizations']
        print(f"\n⚡ PERFORMANCE: {'✅ ENABLED' if perf['enabled'] else '❌ DISABLED'}")
        for feature, enabled in perf['env_vars'].items():
            print(f"  • {feature.replace('_', ' ').title()}: {'✅' if enabled else '❌'}")

        # Settings Configuration
        settings = status['settings_configuration']
        print(f"\n⚙️ SETTINGS: {'✅ VALID' if settings['valid'] else '❌ ISSUES'}")
        if settings['issues']:
            for issue in settings['issues']:
                print(f"  • ❌ {issue}")

        # Overall Status
        print(f"\n🎯 OVERALL STATUS: {'✅ VERIFIED' if status['verification_complete'] else '❌ NEEDS ATTENTION'}")

        if status['verification_complete']:
            print("\n📋 USAGE INSTRUCTIONS:")
            print("  1. Start sessions with: python .claude/clrecall.py")
            print("  2. Update memories: python .claude/claudecache.py")
            print("  3. Use TodoWrite proactively")
            print("  4. Batch tool calls for performance")

        print("\n" + "=" * 50)
        return status

if __name__ == "__main__":
    tracker = IntegrationTracker()

    if len(sys.argv) > 1:
        if sys.argv[1] == '--log':
            event_type = sys.argv[2] if len(sys.argv) > 2 else 'manual'
            tracker.log_session_event(event_type)
            print(f"📝 Logged event: {event_type}")
        elif sys.argv[1] == '--status':
            tracker.display_status_report()
    else:
        tracker.display_status_report()