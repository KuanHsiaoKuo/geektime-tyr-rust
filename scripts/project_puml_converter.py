"""
用于将github项目整理成puml思维导图
"""
import sys
import os
import re

if __name__ == "__main__":
    target_path = '/Users/kuanhsiaokuo/Developer/spare_projects/substrate/'
    replace_pre, replace_after = \
        '/Users/kuanhsiaokuo/Developer/spare_projects/substrate/', \
        'https://github.com/paritytech/substrate/blob/master/'
    modules = ['bin', 'client', 'docker', 'docs', 'frame', 'primitives']
    project = 'substrate'
    start_line = '@startmindmap\n'
    end_line = '@endmindmap'
    valid_output = [start_line]
    save_dir = '../materials/anatomy/substrate'
    start_stars = '*'
    for module in modules:
        module_output = valid_output.copy()
        root_line = f"* {project}/{module}\n"
        module_output.append(root_line)
        tree_command = f'tree -L 10 -fi {target_path}{module}'
        # os.system(tree_command)
        output = os.popen(tree_command).readlines()
        # print(output)
        valid_flags = ['mod.rs', 'lib.rs', 'main.rs', 'build.sh', 'Cargo.toml']
        for item in output[:-2]:
            line = item.strip()
            valid_item = line.split("/")[-1]
            valid_conditions = [
                valid_item.strip() in valid_flags,
                valid_item.endswith('.md'),
                valid_item.endswith('.Dockerfile'),
                '.' not in valid_item
            ]
            if any(valid_conditions):
                level = len(line.replace(replace_pre, '').split('/'))
                stars = start_stars + '*' * level
                valid_line = line.replace(replace_pre, replace_after)
                puml_line = f"{stars} [[{valid_line} {valid_line.split('/')[-1]}]]\n"
                module_output.append(puml_line)
        module_output.append(end_line)
        with open(f"{save_dir}/{project}-{module}.puml", 'w') as f:
            f.writelines(module_output)
