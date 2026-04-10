#!/usr/bin/env python3
"""
将现有 SKILL.md 转换为模板格式。

转换规则：
1. 将工具表格转换为 ::: tools 占位符
2. 将包含 lx 命令的 bash 代码块转换为 ::: example 占位符
3. 保留其他内容不变

用法:
    python scripts/convert-skills-to-template.py skills/lx-search/SKILL.md
    python scripts/convert-skills-to-template.py --all  # 转换所有
    python scripts/convert-skills-to-template.py --dry-run skills/lx-search/SKILL.md
"""

import re
import sys
from pathlib import Path


def parse_tool_table(lines: list[str], start_idx: int) -> tuple[dict[str, list[tuple[str, str]]], int]:
    """
    解析工具表格，返回 {namespace: [(tool, desc), ...]} 和结束行索引
    """
    tools_by_ns: dict[str, list[tuple[str, str]]] = {}
    i = start_idx
    
    # 跳过表头
    # | 命令 | 说明 | 参考 |
    # |------|------|------|
    if i < len(lines) and lines[i].startswith('|'):
        i += 1  # 表头
    if i < len(lines) and lines[i].startswith('|---'):
        i += 1  # 分隔线
    
    # 解析表格行
    while i < len(lines):
        line = lines[i].strip()
        if not line.startswith('|'):
            break
        
        # | `lx namespace tool` | 说明 | 参考 |
        match = re.match(r'\|\s*`lx\s+(\S+)\s+(\S+)`\s*\|\s*([^|]+)', line)
        if match:
            ns, tool, desc = match.groups()
            desc = desc.strip()
            # 移除末尾的参考链接部分
            desc = re.sub(r'\s*\|\s*\[.*?\]\(.*?\)\s*\|?\s*$', '', desc)
            desc = re.sub(r'\s*\|\s*无参数\s*\|?\s*$', '', desc)
            desc = desc.strip()
            
            if ns not in tools_by_ns:
                tools_by_ns[ns] = []
            tools_by_ns[ns].append((tool, desc))
        else:
            # 可能是特殊格式，如 `lx sh --exec "<cmd>"`
            match2 = re.match(r'\|\s*`lx\s+(\S+)(\s+--\S+.*)?`\s*\|\s*([^|]+)', line)
            if match2:
                ns = match2.group(1)
                desc = match2.group(3).strip()
                desc = re.sub(r'\s*\|.*$', '', desc).strip()
                # 这种是带参数的示例，跳过
                pass
        
        i += 1
    
    return tools_by_ns, i


def generate_tools_placeholder(ns: str, tools: list[tuple[str, str]]) -> str:
    """生成 ::: tools 占位符"""
    lines = [f"::: tools {ns}"]
    for tool, desc in tools:
        if desc:
            lines.append(f"{tool}: {desc}")
        else:
            lines.append(tool)
    lines.append(":::")
    return "\n".join(lines)


def parse_bash_codeblock(lines: list[str], start_idx: int) -> tuple[str | None, int]:
    """
    解析 bash 代码块，返回 (代码块内容, 结束行索引)
    如果代码块包含 lx 命令，返回内容；否则返回 None
    """
    i = start_idx + 1  # 跳过 ```bash
    code_lines = []
    
    while i < len(lines):
        line = lines[i]
        if line.strip() == '```':
            i += 1
            break
        code_lines.append(line)
        i += 1
    
    code = '\n'.join(code_lines)
    
    # 检查是否包含 lx 命令
    if re.search(r'^lx\s+\w+', code, re.MULTILINE):
        return code, i
    
    return None, i


def convert_skill(content: str) -> str:
    """转换 SKILL.md 内容为模板格式"""
    lines = content.split('\n')
    output_lines = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        
        # 检测表头：| 命令 | 说明 |
        if re.match(r'\|\s*命令\s*\|', line):
            # 找到分隔线
            if i + 1 < len(lines) and lines[i + 1].startswith('|---'):
                # 解析工具表格
                tools_by_ns, end_idx = parse_tool_table(lines, i)
                
                if tools_by_ns:
                    # 生成占位符
                    for ns, tools in tools_by_ns.items():
                        placeholder = generate_tools_placeholder(ns, tools)
                        output_lines.append(placeholder)
                        output_lines.append("")
                    i = end_idx
                    continue
        
        # 检测 bash 代码块
        if line.strip() == '```bash':
            code, end_idx = parse_bash_codeblock(lines, i)
            if code is not None:
                # 转换为 ::: example 占位符
                output_lines.append("::: example")
                output_lines.append(code)
                output_lines.append(":::")
                i = end_idx
                continue
        
        output_lines.append(line)
        i += 1
    
    return '\n'.join(output_lines)


def convert_file(path: Path, dry_run: bool = False) -> None:
    """转换单个文件"""
    content = path.read_text()
    converted = convert_skill(content)
    
    if dry_run:
        print(f"=== {path} ===")
        print(converted)
        print()
    else:
        # 备份原文件
        backup_path = path.with_suffix('.md.bak')
        path.rename(backup_path)
        path.write_text(converted)
        print(f"✓ Converted: {path} (backup: {backup_path})")


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)
    
    dry_run = '--dry-run' in sys.argv
    
    if '--all' in sys.argv:
        skills_dir = Path('skills')
        for skill_dir in skills_dir.iterdir():
            if skill_dir.is_dir():
                skill_file = skill_dir / 'SKILL.md'
                if skill_file.exists():
                    convert_file(skill_file, dry_run)
    else:
        for arg in sys.argv[1:]:
            if arg.startswith('--'):
                continue
            path = Path(arg)
            if path.exists():
                convert_file(path, dry_run)
            else:
                print(f"File not found: {path}", file=sys.stderr)


if __name__ == '__main__':
    main()
