#!/usr/bin/env python3
"""
Tests for the Psyverse Cash Matrix components
"""

import unittest
import tempfile
import os
import json
from unittest.mock import patch, mock_open, MagicMock

# Import the modules we want to test
import sys
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'src'))

from bounty_hunter import extract_num, parse_bounty_amount, search_github_bounties
from traffic_cash_injector import verify_medium_video_compliance, get_video_info


class TestBountyHunter(unittest.TestCase):

    def test_extract_num(self):
        """Test number extraction from text"""
        self.assertEqual(extract_num("奖励 $100"), 100.0)
        self.assertEqual(extract_num("价格为 99.5 元"), 99.5)
        self.assertEqual(extract_num("没有数字"), None)
        self.assertEqual(extract_num(""), None)

    def test_parse_bounty_amount(self):
        """Test bounty amount parsing"""
        # Test dollar amounts
        self.assertEqual(parse_bounty_amount("Issue: Fix bug for $50", ""), "$50")
        self.assertEqual(parse_bounty_amount("Help wanted: $100 bounty", ""), "$100")
        self.assertEqual(parse_bountyAmount("Reward: 75 USD", ""), "$75")
        self.assertEqual(parse_bounty_amount("No bounty here", ""), None)

    @patch('bounty_hunter.urllib.request.urlopen')
    def test_search_github_bounties(self, mock_urlopen):
        """Test GitHub bounty search (mocked)"""
        # Mock response data
        mock_response = MagicMock()
        mock_response.read.return_value.decode.return_value = json.dumps({
            "items": [
                {
                    "title": "Fix login bug for $75",
                    "body": "Please fix this issue",
                    "html_url": "https://github.com/example/repo/issues/1",
                    "repository_url": "https://github.com/example/repo"
                },
                {
                    "title": "Update documentation",
                    "body": "No bounty here",
                    "html_url": "https://github.com/example/repo/issues/2",
                    "repository_url": "https://github.com/example/repo"
                }
            ]
        })
        mock_urlopen.return_value.__enter__.return_value = mock_response

        # Test search
        results = search_github_bounties(min_amount=50)

        # Should find one bounty
        self.assertEqual(len(results), 1)
        self.assertEqual(results[0]["amount"], "$75")
        self.assertEqual(results[0]["title"], "Fix login bug for $75")


class TestTrafficCashInjector(unittest.TestCase):

    @patch('traffic_cash_injector.subprocess.run')
    @patch('traffic_cash_injector.os.path.exists')
    def test_verify_medium_video_compliance_valid(self, mock_exists, mock_run):
        """Test video compliance verification with valid video"""
        mock_exists.return_value = True

        # Mock ffprobe output for a valid 16:9 video > 60s
        mock_result = MagicMock()
        mock_result.stdout = json.dumps({
            "format": {
                "duration": "75.5"
            },
            "streams": [
                {
                    "codec_type": "video",
                    "width": 1920,
                    "height": 1080
                }
            ]
        })
        mock_run.return_value = mock_result

        result = verify_medium_video_compliance("/fake/path/video.mp4")
        self.assertTrue(result)

    @patch('traffic_cash_injector.subprocess.run')
    @patch('traffic_cash_injector.os.path.exists')
    def test_verify_medium_video_compliance_invalid_ratio(self, mock_exists, mock_run):
        """Test video compliance verification with wrong aspect ratio"""
        mock_exists.return_value = True

        # Mock ffprobe output for a 4:3 video
        mock_result = MagicMock()
        mock_result.stdout = json.dumps({
            "format": {
                "duration": "75.5"
            },
            "streams": [
                {
                    "codec_type": "video",
                    "width": 1440,
                    "height": 1080
                }
            ]
        })
        mock_run.return_value = mock_result

        result = verify_medium_video_compliance("/fake/path/video.mp4")
        self.assertFalse(result)

    @patch('traffic_cash_injector.subprocess.run')
    @patch('traffic_cash_injector.os.path.exists')
    def test_verify_medium_video_compliance_too_short(self, mock_exists, mock_run):
        """Test video compliance verification with video too short"""
        mock_exists.return_value = True

        # Mock ffprobe output for a 30 second video
        mock_result = MagicMock()
        mock_result.stdout = json.dumps({
            "format": {
                "duration": "30.0"
            },
            "streams": [
                {
                    "codec_type": "video",
                    "width": 1920,
                    "height": 1080
                }
            ]
        })
        mock_run.return_value = mock_result

        result = verify_medium_video_compliance("/fake/path/video.mp4")
        self.assertFalse(result)

    @patch('traffic_cash_injector.os.path.exists')
    def test_verify_medium_video_compliance_not_found(self, mock_exists):
        """Test video compliance verification with non-existent file"""
        mock_exists.return_value = False

        result = verify_medium_video_compliance("/fake/path/nonexistent.mp4")
        self.assertFalse(result)


if __name__ == '__main__':
    unittest.main()