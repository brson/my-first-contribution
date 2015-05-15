#!/bin/sh

set -u -e

git_dir="$1"
out_dir="$2"

authors="$(git -C "$git_dir" log --format=%an | sort | uniq)"

mkdir -p "$out_dir"
rm "$out_dir"/*

echo "$authors" | while read author; do
    commit="$(git -C "$git_dir" log --author="$author" --format=%h | tail -n1)"
    if [ -z "$commit" ]; then
	echo "no commit for $author"
    else
	format="<div>%an</div>%n<div><span>%ad</span><span>%h</span></div>%n<div>%s</div>"
	git -C "$git_dir" log "$commit"  -n1 --format="$format" --date=short > "$out_dir/$author"
	echo "<div>" >> "$out_dir/$author"
	echo "<code>" >> "$out_dir/$author"
	git -C "$git_dir" log "$commit"  -n1 -p --format="" >> "$out_dir/$author"
	echo "</code>" >> "$out_dir/$author"
	echo "</div>" >> "$out_dir/$author"
    fi
done
