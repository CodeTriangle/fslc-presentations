#!/bin/bash

# this is a cool program. it's a very basic slideshow presenter.
# argument one is the file to use. if no argument, it uses ./slides.txt

file="$1"

[ -z $1 ] && file="slides.txt"

[ -a $file ] || { echo "file not found: $file"; exit 1; }

IFS='
'

clear

for line in $(cat "$file"); do
    case "$line" in
        ('---')
            clear;;
        (*)
            echo
            echo "$line"
            read -rsn 1;;
    esac
done
