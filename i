#!/bin/bash

ARCH=$(uname -m)

command -v yum >/dev/null 2>&1 || { echo "your OS is not supported, you have to manually install pull-daemon"; exit 1; }
ver=$(rpm -q --queryformat '%{VERSION}' centos-release)

echo "installing pull-daemon to /srv/pull-daemon, please wait"
if [ "$ver" == "7" ] || [ "$ver" == "8" ]; then
	systemctl stop pull-daemon >/dev/null 2>&1
	mkdir -p /srv/pull-daemon/update-repos.d
	curl -s -o /srv/pull-daemon/pd "http://home-nadym.ru/pd/pd" || exit 2
	curl -s -o /etc/systemd/system/pull-daemon.service "http://home-nadym.ru/pd/systemd/pull-daemon.service" || exit 3
	curl -s -o /srv/pull-daemon/update-repos "http://home-nadym.ru/pd/update-repos" || exit 4
	curl -s -o /srv/pull-daemon/update-repos.d/repo1.example "http://home-nadym.ru/pd/update-repos.d/repo1.example" || exit 5
	chmod +x /srv/pull-daemon/pd
	chmod +x /srv/pull-daemon/update-repos
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
