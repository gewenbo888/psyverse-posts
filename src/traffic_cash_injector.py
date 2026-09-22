#!/usr/bin/env python3
"""
Traffic Cash Injector
Verifies medium video compliance for platforms like YouTube, B站, etc.
"""

import json
import subprocess
import sys
import os
from typing import Dict, List, Optional


def verify_medium_video_compliance(video_path: str) -> bool:
    """
    严格核验中视频伙伴计划：16:9横屏 (1920x1080) 且时长 > 60秒

    Args:
        video_path: Path to the video file to verify

    Returns:
        bool: True if video meets medium video requirements, False otherwise
    """
    # Check if file exists
    if not os.path.exists(video_path):
        print(f"❌ 视频文件不存在: {video_path}")
        return False

    try:
        # Use ffprobe to get video information
        cmd = [
            "ffprobe",
            "-v", "quiet",
            "-print_format", "json",
            "-show_format",
            "-show_streams",
            video_path
        ]

        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        probe = json.loads(result.stdout)

        # Get duration
        duration = float(probe["format"]["duration"])

        # Find video stream
        v_stream = None
        for stream in probe["streams"]:
            if stream["codec_type"] == "video":
                v_stream = stream
                break

        if not v_stream:
            print(f"❌ 未找到视频流: {video_path}")
            return False

        # Get width and height
        w = int(v_stream["width"])
        h = int(v_stream["height"])

        # Check if it's 16:9 landscape (allowing small tolerance)
        aspect_ratio = w / h
        is_landscape_16_9 = abs(aspect_ratio - 16 / 9) < 0.05

        # Check if duration is qualified (> 60 seconds)
        is_qualified_duration = duration >= 60.0

        # Log results for debugging
        print(f"📹 视频信息: {video_path}")
        print(f"   分辨率: {w}x{h}")
        print(f"   时长: {duration:.2f}秒")
        print(f"   是否16:9横屏: {is_landscape_16_9} (比例: {aspect_ratio:.2f})")
        print(f"   是否满足时长要求: {is_qualified_duration} (> 60秒)")

        return is_landscape_16_9 and is_qualified_duration

    except subprocess.CalledProcessError as e:
        print(f"❌ 调用ffprobe失败: {e}")
        if e.stderr:
            print(f"   错误输出: {e.stderr}")
        return False
    except json.JSONDecodeError as e:
        print(f"❌ 解析ffprobe输出失败: {e}")
        return False
    except Exception as e:
        print(f"❌ 视频合规性检查时发生未知错误: {e}")
        return False


def get_video_info(video_path: str) -> Optional[Dict]:
    """
    Get detailed information about a video file.

    Args:
        video_path: Path to the video file

    Returns:
        Dict containing video information or None if failed
    """
    if not os.path.exists(video_path):
        return None

    try:
        cmd = [
            "ffprobe",
            "-v", "quiet",
            "-print_format", "json",
            "-show_format",
            "-show_streams",
            video_path
        ]

        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        return json.loads(result.stdout)
    except Exception:
        return None


def batch_verify_videos(video_directory: str, extensions: List[str] = None) -> List[Dict]:
    """
    Batch verify all videos in a directory.

    Args:
        video_directory: Directory containing video files
        extensions: List of file extensions to check (default: common video formats)

    Returns:
        List of dictionaries containing verification results
    """
    if extensions is None:
        extensions = ['.mp4', '.avi', '.mov', '.mkv', '.webm', '.flv']

    if not os.path.exists(video_directory):
        print(f"❌ 目录不存在: {video_directory}")
        return []

    results = []

    for filename in os.listdir(video_directory):
        if any(filename.lower().endswith(ext) for ext in extensions):
            video_path = os.path.join(video_directory, filename)
            is_compliant = verify_medium_video_compliance(video_path)
            video_info = get_video_info(video_path)

            result = {
                "filename": filename,
                "path": video_path,
                "compliant": is_compliant,
                "info": video_info
            }
            results.append(result)

            status = "✅ 合规" if is_compliant else "❌ 不合规"
            print(f"{status}: {filename}")

    return results


def main():
    """Main entry point for the traffic cash injector."""
    print("🚀 启动Psyverse流量现金注入器...")
    print("=" * 50)

    if len(sys.argv) < 2:
        print("使用方法:")
        print("  python traffic_cash_injector.py <video_path>          # 检查单个视频")
        print("  python traffic_cash_injector.py --dir <directory>    # 批量检查目录")
        return

    if sys.argv[1] == "--dir" and len(sys.argv) >= 3:
        # Batch mode
        directory = sys.argv[2]
        print(f"📁 批量检查目录: {directory}")
        results = batch_verify_videos(directory)

        compliant_count = sum(1 for r in results if r["compliant"])
        total_count = len(results)

        print(f"\n📊 检查完成: {compliant_count}/{total_count} 个视频符合中视频伙伴计划要求")
    else:
        # Single file mode
        video_path = sys.argv[1]
        print(f"🔍 检查视频: {video_path}")

        is_compliant = verify_medium_video_compliance(video_path)

        if is_compliant:
            print("\n🎉 视频符合中视频伙伴计划要求！")
            print("   - 16:9 横屏格式")
            print("   - 时长超过 60 秒")
            print("   - 可申请平台流量分成")
        else:
            print("\n💥 视频不符合中视频伙伴计划要求")
            print("   请确保视频是：")
            print("   - 16:9 横屏分辨率 (如 1920x1080)")
            print("   - 时长严格大于 60 秒")


if __name__ == "__main__":
    main()