#!/bin/sh

# Edited from update-proton-ge

# Stop on error
set -e

# Constants
REPO=citra-emu/citra-nightly
LATEST_RELEASE_URL=https://github.com/$REPO/releases/latest
CITRA_DIR=~/Downloads/Citra
autoInstall=false

# Find latest version tag
release_url=$(curl -Ls -o /dev/null -w %{url_effective} $LATEST_RELEASE_URL)
version=${release_url##*/}
echo Found latest version: Citra-$version

# Determine download URL and install path
filename=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep "name.*.tar.xz" | cut -d : -f 2,3 | tr -d '\", ')
download_url=https://github.com/$REPO/releases/download/$version/$filename
install_dir=$CITRA_DIR/$(echo $filename | cut -c 1-28)

# Exit if already installed
if [ -d $install_dir ]; then
	echo Already installed at: $install_dir
	exit
fi

# Functions

# (In case autoinstall if off) Asks if you want to download/install the newest release
InstallationPrompt() {
	if [ "$autoInstall" = true ]; then
		if [ ! -d "$CITRA_DIR"/Citra-"$version" ]; then
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
	if [[ ! -d "$CITRA_DIR" ]]; then mkdir -p $CITRA_DIR; fi
	curl -Lo /dev/stdout $download_url | bsdtar -xf /dev/stdin --directory $CITRA_DIR
}

# Check if there are other Citra Nightly versions installed
# If so, asks if you want to delete any
Wanttodelete() {
	read -r -p "Do you want to delete intalled versions? <y/n> " prompt
	if [[ $prompt == "y" || $prompt == "Y" || $prompt == "yes" || $prompt == "Yes" ]]; then
		DeleteCitraCheck
	fi
}

# Show which versions are installed, then asks which you want to delete
DeleteCitraCheck() {
	echo "Installed runners:"
	installed_versions=($(ls -d "$CITRA_DIR"/*/))
	for((i=0;i<${#installed_versions[@]};i++)); do
	inumber=$(("$i" + 1))
	folder=$(echo "${installed_versions[i]}" | rev | cut -d/ -f2 | rev)
	echo "$inumber. $folder"
	done
	echo ""
	echo -n "Please choose an option to remove [1-${#installed_versions[@]}]:"
	read -ra option_remove

	case "$option_remove" in
	[1-9])
	if (( $option_remove <= ${#installed_versions[@]} )); then
		remove_option=${installed_versions[$option_remove -1]}
		echo "removing $remove_option"
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
	rm -rf $remove_option
	echo "Removed $remove_option"
	installComplete=true
	Wanttodelete
}

# The first thing that runs in the actual script (ironically)
InstallationPrompt

# When installation is actually done
echo Installation complete, at: $install_dir
