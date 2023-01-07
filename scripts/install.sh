#!/usr/bin/env bash

{ # This ensures the entire script is downloaded.
	wget https://raw.githubusercontent.com/TiagoCavalcante/qr/main/scripts/qr.tar.gz
	tar -xvf qr.tar.gz
	chmod +x qr
	sudo cp qr /usr/local/bin/qr
	echo "qr is now installed! 🚀"
}
