"""
これは　わたし用のプログラムだぜ☆つ（＾～＾）！
"""
import os
import shutil

source = 'C:/Users/むずでょ/source/repos/piston-pixel-art-drawing-tool'
destination = 'C:/Users/むずでょ/Documents/GitHub/piston-pixel-art-drawing-tool'


def go():
    print('Trace   | Remove.')
    remove_destination_dir('/.vscode')
    remove_destination_dir('/@not-runtime')
    remove_destination_dir('/piston-pixel-art')
    remove_destination_file('/.gitignore')
    remove_destination_file('/copy-to-git.py')
    remove_destination_file('/LICENSE')
    remove_destination_file('/README.md')

    print('Trace   | Copy.')
    copy_dir('/.vscode')
    copy_dir('/@not-runtime')
    copy_dir('/piston-pixel-art',
             ignore=shutil.ignore_patterns('.git', 'target'))
    copy_file('/.gitignore')
    copy_file('/copy-to-git.py')
    copy_file('/LICENSE')
    copy_file('/README.md')
    print('Trace   | Finished.')


def remove_destination_dir(child_path: str):
    path = f'{destination}{child_path}'
    if os.path.isdir(path):
        shutil.rmtree(path)


def remove_destination_file(child_path: str):
    path = f'{destination}{child_path}'
    if os.path.isfile(path):
        os.remove(path)


def copy_dir(child_path, ignore=None):
    shutil.copytree(f'{source}{child_path}',
                    f'{destination}{child_path}', ignore=ignore)


def copy_file(child_path):
    shutil.copy2(f'{source}{child_path}', f'{destination}{child_path}')


go()