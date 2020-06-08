#!/bin/bash

ARCH=$(uname -m)
BASE="https://d7.wtf/pd"

command -v yum >/dev/null 2>&1 || { echo "your OS is not supported, you have to manually install pull-daemon"; exit 1; }
ver=$(rpm -q --queryformat '%{VERSION}' centos-release)

echo "installing pull-daemon to /opt/pull-daemon, please wait"
if [ "$ver" == "7" ] || [ "$ver" == "8" ]; then
	systemctl stop pull-daemon >/dev/null 2>&1
	mkdir -p /opt/pull-daemon/update-repos.d
	curl -sL -o /opt/pull-daemon/pd "$BASE/pd" || exit 2
	curl -sL -o /etc/systemd/system/pull-daemon.service "$BASE/systemd/pull-daemon.service" || exit 3
	curl -sL -o /opt/pull-daemon/update-repos "$BASE/update-repos" || exit 4
	curl -sL -o /opt/pull-daemon/update-repos.d/repo1.example "$BASE/update-repos.d/repo1.example" || exit 5
	chmod +x /opt/pull-daemon/pd
	chmod +x /opt/pull-daemon/update-repos
	useradd -r pd >/dev/null 2>&1
	chown -R pd /opt/pull-daemon
	systemctl daemon-reload
	systemctl enable pull-daemon
	systemctl restart pull-daemon
	read -p "add rule to firewalld (y/n): " -n 1 firewall
	echo
	if [ "$firewall" == "y" ]; then
		firewall-cmd --zone=public --add-port=8888/tcp --permanent
		firewall-cmd --reload
	fi
else
	echo "your CentOS version is not supported"
	exit 1
fi

echo "--- DONE! ---"
