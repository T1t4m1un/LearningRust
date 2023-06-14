#!/bin/bash

# 函数：递归删除目录下的可执行文件
delete_executables() {
    local dir=$1
    
    # 遍历目录中的文件和目录
    for file in "$dir"/*; do
        if [[ -f "$file" && $(file -b "$file") =~ "executable" && "$file" != "./clean.sh" ]]; then
            # 如果是可执行文件，则删除
            echo "删除文件: $file"
            rm "$file"
        elif [ -d "$file" ]; then
            # 如果是目录，则递归删除该目录下的可执行文件
            delete_executables "$file"
        fi
    done
}

# 指定要删除可执行文件的目录
directory="."

# 调用函数删除目录下的可执行文件
delete_executables "$directory"
