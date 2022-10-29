#!/bin/bash

# test if there's an internet connection
if [[ "$(ping -qc 1 google.com | wc -l)" -lt 2 ]]; then
	echo "No internet connection"
	exit 1
fi

# you need run this script only once for it to create the TMPFILE
TMPFILE="$HOME/.linux-latest"

REPO=https://github.com/torvalds/linux
# gets full tag list from REPO, in ascending order, then isolates the last
# line (the latest tag)
# and isolates the string starting at the 12th character from the second word,
# which so happens
# to be where the actual tag name starts
TAG=$(git ls-remote $REPO | sort -Vk2 | tail -1 | \
	awk '{print substr ($2, 12 )}')
GARBAGE='^{}'

# checks if the tag name has that weird substring, and if so, removes it
if [[ "$TAG" == *"$GARBAGE"* ]]; then
	TAG=$(echo "$TAG" | tr -d "$GARBAGE")
fi

# in case the file holding the latest version name doesn't exist, it's created
# if the latest release also doesn't match the current tag name, it overwrites its value
# at last, if none of these apply, the script is ended
if [[ ((! -f "$TMPFILE")) || ((-f $TMPFILE && "$(cat "$TMPFILE")" != "$TAG")) ]]; then
	echo "$TAG" > "$TMPFILE"
else
	exit 2
fi

# sends a notification with the current Linux version tag, lasting for 20s and using the tux icon
notify-send -a sirkhancision -i tux "Linux version update:" -t 20000 \
    "Latest version: $TAG"
