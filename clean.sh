#!/bin/bash

# 递归删除目录下的可执行文件
delete_executables() {
    local dir=$1

    # 遍历目录中的文件和目录
    for file in "$dir"/*; do
        if [[ -f "$file" && $(file -b "$file") =~ "ELF" && "$file" != "./clean.sh" ]]; then
            # 如果是可执行文件，则删除
            echo "删除文件: $file"
            rm "$file"
        elif [ -d "$file" ]; then
            # 如果是目录，则递归删除该目录下的可执行文件
            delete_executables "$file"
        fi
    done
}

# 若没有参数指定进行清除的目录
if [ $# -eq 0 ]; then
  directory="."
else
  directory="$1"
fi

delete_executables "$directory"
