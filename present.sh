#!/bin/bash

# BSD 2-Clause License
# 
# Copyright (c) 2021, Richard Snider
# All rights reserved.
# 
# Redistribution and use in source and binary forms, with or without
# modification, are permitted provided that the following conditions are met:
# 
# 1. Redistributions of source code must retain the above copyright notice, this
#    list of conditions and the following disclaimer.
# 
# 2. Redistributions in binary form must reproduce the above copyright notice,
#    this list of conditions and the following disclaimer in the documentation
#    and/or other materials provided with the distribution.
# 
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
# AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
# IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
# DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
# FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
# DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
# SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
# CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
# OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
# OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

# this is a cool program. it's a very basic slideshow presenter.
# argument one is the file to use. if no argument, it uses ./slides.txt
# use '---' to separate slides.
# type any key to continue or q to quit.

file="$1"

[ -z $1 ] && file="slides.txt"

[ -a $file ] || { echo "file not found: $file"; exit 1; }

IFS='
'

clear

QUOTE=0

function substr {
    echo "$(echo "$1" | cut -c "$2")"
}

for line in $(cat "$file"); do
    if [ "$line" = '---' ]; then
        clear
    elif [ "$(substr "$line" 1)" = '"' ]; then
        toprint=$(substr "$line" 2-)
        if [ $QUOTE = 0 ]; then
            echo
            echo -n " “$toprint"
            QUOTE=1
        elif [ "$(substr "$line" 2)" = "-" ]; then
            echo '”'
            echo "    $toprint"
            QUOTE=0
            read -rsn 1
        else
            echo
            echo -n "  $toprint"
        fi
    elif [ "$(substr "$line" 1)" = '$' ]; then
        eval "$(substr "$line" 2-)"
    elif [ -z "$line" ]; then
        echo
    else
        if [ $QUOTE = 1 ]; then
            echo "”"
        fi
        QUOTE=0
        echo
        echo "$line"
        read -rsn 1
        [ "$REPLY" = 'q' ] && exit
    fi
done
