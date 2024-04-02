#!/bin/sh

set -x

BIN=broadview-backend
REMOTE=ubuntu@147.135.37.165
RELEASE_DIR="~/broadview-backend/releases/$GITHUB_SHA"

ssh $REMOTE "mkdir -p $RELEASE_DIR"
scp release.zip $REMOTE:$RELEASE_DIR/release.zip

ssh $REMOTE /bin/bash << EOF
	set -x
	cd $RELEASE_DIR
	unzip -u release.zip
	rm release.zip

	cd ~/broadview-backend
	if test -f "$BIN"; then
		mv $BIN $BIN-old
	fi
	cp $RELEASE_DIR/$BIN ./$BIN

	systemctl restart broadview-backend
EOF
