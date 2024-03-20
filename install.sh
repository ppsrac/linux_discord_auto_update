# Update privileges
sudo chmod 755 ./discord_auto_update
sudo cp discord_auto_update /usr/bin/discord-auto-update/discord_auto_update

# Path: /etc/systemd/system/discord_auto_update.service
echo "Please enter your sudo password: "
read pw

service_script="[Unit]
Description=discord auto update
After=network.target

[Service]
Type=simple
ExecStart=/usr/bin/discord-auto-update/discord_auto_update $pw

[Install]
WantedBy=multi-user.target"

echo "$service_script" >> discord_auto_update.service
sudo mv discord_auto_update.service /etc/systemd/system/discord_auto_update.service

sudo systemctl enable discord_auto_update.service
sudo systemctl start discord_auto_update.service

sudo systemctl status discord_auto_update.service