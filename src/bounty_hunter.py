#!/usr/bin/env python3
"""
GitHub Bounty Hunter Engine
Automatically scans GitHub for bounty issues and generates fix proposals.
"""

import json
import subprocess
import sys
import urllib.parse
import urllib.request
import re
from typing import Optional, List, Dict


def extract_num(text: str) -> Optional[float]:
    """Extract the first number from text."""
    if not text:
        return None
    match = re.search(r'\d+(?:\.\d+)?', text)
    return float(match.group()) if match else None


def parse_bounty_amount(title: str, body: str) -> Optional[str]:
    """
    Parse bounty amount from issue title or body.
    Looks for patterns like $50, $100, etc.
    """
    # Combined text to search
    text = f"{title} {body}"

    # Look for dollar amounts
    dollar_matches = re.findall(r'\$(\d+(?:\.\d+)?)', text)
    if dollar_matches:
        # Return the first dollar amount found
        return f"${dollar_matches[0]}"

    # Look for other currency indicators
    usd_matches = re.findall(r'(\d+(?:\.\d+)?)\s*USD', text, re.IGNORECASE)
    if usd_matches:
        return f"${usd_matches[0]}"

    return None


def search_github_bounties(keyword="bounty", min_amount=50):
    """
    Search GitHub for bounty issues.

    Args:
        keyword: Search keyword (default: "bounty")
        min_amount: Minimum bounty amount in USD (default: 50)
    """
    query = f"{keyword} is:issue is:open"
    url = f"https://api.github.com/search/issues?q={urllib.parse.quote(query)}&sort=created&order=desc"

    try:
        req = urllib.request.Request(
            url,
            headers={"User-Agent": "Psyverse-Hunter/1.0"}
        )
        with urllib.request.urlopen(req) as resp:
            data = json.loads(resp.read().decode())

        found_bounties = []
        for item in data.get("items", []):
            amount = parse_bounty_amount(item["title"], item.get("body", ""))
            if amount and extract_num(amount) >= min_amount:
                found_bounties.append({
                    "title": item["title"],
                    "url": item["html_url"],
                    "amount": amount,
                    "repository": item["repository_url"].split("/")[-1] if item.get("repository_url") else "unknown"
                })
                print(f"🎯 发现悬赏 [{amount}]: {item['title']}")
                print(f"🔗 Issue: {item['html_url']}")
                print(f"📂 仓库: {item['repository_url'].split('/')[-1] if item.get('repository_url') else 'unknown'}")
                print("-" * 50)

        return found_bounties

    except Exception as e:
        print(f"❌ 搜索GitHub时出错: {e}")
        return []


def generate_pr_proposal(issue_url: str, repo_owner: str, repo_name: str) -> Dict:
    """
    Generate a PR proposal for a bounty issue.
    This is a simplified version - in practice, this would involve:
    1. Cloning the repository
    2. Analyzing the issue
    3. Creating a fix
    4. Generating a PR

    For now, we return a template proposal.
    """
    return {
        "issue_url": issue_url,
        "repo_owner": repo_owner,
        "repo_name": repo_name,
        "proposed_changes": [
            "Analyze issue description",
            "Locate relevant code",
            "Create minimal reproduction test",
            "Implement fix",
            "Run existing tests",
            "Submit PR"
        ],
        "estimated_time": "30-60 minutes",
        "difficulty": "medium"
    }


def main():
    """Main entry point for the bounty hunter."""
    print("🚀 启动Psyverse GitHub悬赏猎人...")
    print("=" * 50)

    # Search for bounties
    bounties = search_github_bounties(min_amount=50)

    if not bounties:
        print("🔍 未找到符合条件的悬赏")
        return

    print(f"\n✅ 找到 {len(bounties)} 个符合条件的悬赏")

    # Example: Generate proposals for first few bounties
    for i, bounty in enumerate(bounties[:3]):  # Limit to first 3
        print(f"\n📝 为悬赏 #{i+1} 生成PR提案:")
        print(f"   标题: {bounty['title']}")
        print(f"   金额: {bounty['amount']}")

        # Extract repo info from URL (simplified)
        # In practice, we'd parse this properly from the issue
        repo_full_name = "example/repo"  # Placeholder
        proposal = generate_pr_proposal(bounty["url"], "example", "repo")

        print(f"   预估时间: {proposal['estimated_time']}")
        print(f"   难度: {proposal['difficulty']}")


if __name__ == "__main__":
    main()