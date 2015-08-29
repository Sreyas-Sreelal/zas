#!/usr/bin/env bash

set -e

VERSION="0.3.0-alpha"
PLATFORM="Darwin"

ZAS_ROOT="$HOME/Library/Application Support/Zas"

ARCHIVE_URL="https://github.com/juanibiapina/zas/releases/download/v$VERSION%2B$PLATFORM/zas-v${VERSION}.${PLATFORM}.tar.gz"

ZASD_PLIST_PATH="$HOME/Library/LaunchAgents/com.zas.zasd.plist"
FIREWALL_PLIST_PATH="/Library/LaunchDaemons/com.zas.firewall.plist"

mkdir -p "$ZAS_ROOT"
cd "$ZAS_ROOT"

echo "Downloading Zas version $VERSION"
curl -sL "${ARCHIVE_URL}" | tar xzf -

ln -sf "zas-v${VERSION}+${PLATFORM}" current

m4 --define ZAS_BINARY="$ZAS_ROOT/current/bin/zas" "$ZAS_ROOT/current/resources/com.zas.zasd.plist.template" > "${ZASD_PLIST_PATH}"

launchctl bootstrap gui/"$UID" "${ZASD_PLIST_PATH}" 2>/dev/null
launchctl enable gui/"$UID"/com.zas.zasd 2>/dev/null
launchctl kickstart -k gui/"$UID"/com.zas.zasd 2>/dev/null

sudo cp -f current/resources/dev-resolver /etc/resolver/dev

# set up port forwarding

sudo cp current/resources/com.zas.firewall.plist "${FIREWALL_PLIST_PATH}"

sudo launchctl bootstrap system "${FIREWALL_PLIST_PATH}" 2>/dev/null
sudo launchctl enable system/com.zas.firewall 2>/dev/null
sudo launchctl kickstart -k system/com.zas.firewall 2>/dev/null
