# MacOS

No Release yet

## When using SMB
- Disable Package Signing `printf "[default]\nsigning_required=no\n" | sudo tee /etc/nsmb.conf >/dev/null
- Disable delayed tcp ack `sudo sysctl -w net.inet.tcp.delayed_ack=0`
- Disable .DS_STORE fileson entwork share `defaults write com.apple.desktopservices DSDontWriteNetworkStores -bool TRUE`
