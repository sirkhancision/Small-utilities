#!/bin/bash

# Edited from update-proton-ge

# Stop on error
set -e

# Constants
REPO="citra-emu/citra-nightly"
LATEST_RELEASE_URL="https://github.com/$REPO/releases/latest"
CITRA_DIR="$HOME/Downloads/Citra"
AUTOINSTALL=false

# Find latest version tag
RELEASE_URL=$(curl -Ls -o /dev/null -w %"{url_effective}" $LATEST_RELEASE_URL)
VERSION=${RELEASE_URL##*/}
echo Found latest version: Citra-"$VERSION"

# Determine download URL and install path
FILENAME=$(curl -s https://api.github.com/repos/$REPO/releases/latest | \
    perl -nle 'print for /^[[:blank:]]+"name":[[:blank:]]"(citra-linux-.+\.tar\.xz)",$/')
VERSION_NAME=$(perl -nle 'print for /(^citra-linux-.+)\.tar\.xz$/' <<< "$FILENAME")
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$FILENAME"
INSTALL_DIR="$CITRA_DIR/$VERSION_NAME"

# Exit if already installed
if [ -d "$INSTALL_DIR" ]; then
	echo Already installed at: "$INSTALL_DIR"
fi

# Functions

# (In case autoinstall if off) Asks if you want to download/install the newest release
InstallationPrompt() {
	if [ "$AUTOINSTALL" = true ]; then
		if [ ! -d "$CITRA_DIR"/Citra-"$VERSION" ]; then
			InstallCitraNightly
		fi
	else
		read -r -p "Do you want to try to download and (re)install this release? <y/n> " prompt
		if [[ $prompt == "y" || $prompt == "Y" || $prompt == "yes" || $prompt == "Yes" ]]; then
			InstallCitraNightly
			Wanttodelete
		else
			echo "Operation canceled"
            Wanttodelete
		fi
	fi
}

# If Citra directory doesn't exist under Downloads, it is created
# Use curl and bsdtar to download and extract release to Citra directory
InstallCitraNightly() {
	if [[ ! -d "$CITRA_DIR" ]]; then
        mkdir -p "$CITRA_DIR";
    fi
	curl -Lo /dev/stdout "$DOWNLOAD_URL" | bsdtar -xf /dev/stdin --directory "$CITRA_DIR"
}

# Check if there are other Citra Nightly versions installed
# If so, asks if you want to delete any
Wanttodelete() {
	read -r -p "Do you want to delete any installed versions? <y/n> " prompt
	if [[ $prompt == "y" || $prompt == "Y" || $prompt == "yes" || $prompt == "Yes" ]]; then
		DeleteCitraCheck
	fi
}

# Show which versions are installed, then asks which you want to delete
DeleteCitraCheck() {
	echo "Installed runners:"
    mapfile -t INSTALLED_VERSIONS < <(ls -d "$CITRA_DIR"/*/)
	for((i=0;i<${#INSTALLED_VERSIONS[@]};i++)); do
        INUMBER=$(("$i" + 1))
        FOLDER=$(echo "${INSTALLED_VERSIONS[i]}" | rev | cut -d/ -f2 | rev)

        if [ "$FOLDER" == "$VERSION_NAME" ]; then
            echo "$INUMBER. $FOLDER [newest]"
        else
            echo "$INUMBER. $FOLDER"
        fi
	done
	echo ""
	echo -n "Please choose an option to remove [1-${#INSTALLED_VERSIONS[@]}]:"
	read -ra OPTION_REMOVE

	case "$OPTION_REMOVE" in
        [1-9])
            if (( OPTION_REMOVE <= ${#INSTALLED_VERSIONS[@]} )); then
                REMOVE_OPTION=${INSTALLED_VERSIONS[$OPTION_REMOVE-1]}
                echo "removing $REMOVE_OPTION"
                DeleteCitraPrompt
            else
                echo "That is not a valid option"
            fi
            ;;
        *)
            echo "Not a valid option"
            ;;
	esac
}

# Confirmation for version deletion
DeleteCitraPrompt() {
	read -r -p "Do you really want to permanently delete this version? <y/n> " prompt
	if [[ $prompt == "y" || $prompt == "Y" || $prompt == "yes" || $prompt == "Yes" ]]; then
		DeleteCitra
	else
		echo "Operation canceled"
		Wanttodelete
	fi
}

# Actually deleting the damn thing
DeleteCitra() {
	rm -rf "$REMOVE_OPTION"
	echo "Removed $REMOVE_OPTION"
	Wanttodelete
}

# The first thing that runs in the actual script (ironically)
InstallationPrompt

# When installation is actually done
echo Installation complete, at: "$INSTALL_DIR"

exit 0
