import os
import shutil
import mistune
import re
from pathlib import Path

directory: str = 'sample'
output_directory: str = 'output'
theme: str = 'default'

# Create output_directory
shutil.rmtree(output_directory, ignore_errors=True)
os.makedirs(output_directory, exist_ok=True)

# Copy static files in default to output_directory
shutil.copytree(f"{theme}/static", f"{output_directory}/static")

# Access main HTML
with open(f"{theme}/_main.html", "r") as mainfile:
    main: str = mainfile.read()

# Process partials
partial_pattern: str = r'\{\{\s*partial\s+"([^"]+)"\s*\}\}'
while re.search(partial_pattern, main):
    for match in re.findall(partial_pattern, main):
        try:
            with open(f"{theme}/partials/{match}.html", "r") as partial_file:
                partial_content: str = partial_file.read()
        except FileNotFoundError:
            print(f'Partial "{match}" not found')
            partial_content = ""
        main = main.replace(f'{{{{ partial "{match}" }}}}', partial_content)

# Access all user's files in specified directory
all_files: list[str] = [
    os.path.join(dirpath, f)
    for dirpath, _, filenames in os.walk(directory)
    for f in filenames
]

md_files: list[str] = [f for f in all_files if f.endswith('.md')]
non_md_files: list[str] = [f for f in all_files if not f.endswith('.md')]

for path in md_files:
    contents: str = Path(path).read_text()

    # Frontmatter
    fm_match = re.search(r'---\n(.*?)\n---', contents, re.DOTALL)
    frontmatter: dict[str, str] = {}
    if fm_match:
        frontmatter_content: str = fm_match.group(1)
        frontmatter = {
            match.group(1): match.group(2)
            for match in re.finditer(r'(\w+):\s*(.+)', frontmatter_content)
        }
        contents = contents.replace(fm_match.group(0), '', 1)

    # Cleaning markdown links
    contents = re.sub(r'(\[.*?\])\(([^)]+)\.md\)', r'\1(\2.html)', contents)

    markdown = mistune.create_markdown(
        plugins=['strikethrough', 'footnotes', 'table', 'url', 'task_lists', 'mark', 'insert', 'superscript', 'subscript', 'math', 'ruby']
    )
    html_contents: str = markdown(contents)
    file_contents: str = main.replace('{{content}}', html_contents)
    
    html_path: str = path.replace(directory, output_directory).replace('.md', '.html')
    os.makedirs(os.path.dirname(html_path), exist_ok=True)
    
    with open(html_path, 'w') as file:
        file.write(file_contents)

for path in non_md_files:
    dest_path: str = path.replace(directory, output_directory)
    os.makedirs(os.path.dirname(dest_path), exist_ok=True)
    shutil.copy2(path, dest_path)

