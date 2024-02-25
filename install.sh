#!/usr/bin/env bash
set -euo pipefail
echo "building project"
cargo build --release
echo "copying binary to /usr/local/bin/node-exporter-radeontop"
sudo cp target/release/node-exporter-radeontop /usr/local/bin/node-exporter-radeontop
echo "create service file at /etc/systemd/user/node_exporter_radeontop.service"
sudo cp node_exporter_radeontop.service /etc/systemd/user/node_exporter_radeontop.service
echo "enabling on systemd"
systemctl --user enable node_exporter_radeontop
systemctl --user start node_exporter_radeontop
systemctl --user status node_exporter_radeontop
