# Building
Run ```vagrant up``` to get a dev envrionment.

Log into vagrant via ```vagrant ssh```.

In Vagrant ```cd /shared```, where vagrant mounted the actual project.

Build using cross ```cross build --target=armv7-unknown-linux-musleabihf```

The resulting file can be copied to your OpenWrt (Linksys WRT1900ACS-EU in my case).


# uhttpd

```
config 'uhttpd' 'qr'
        list listen_http '0.0.0.0:8888'
        list listen_http '[::]:8888'
        option home '/root/web'
```
/etc/init.d/uhttpd restart

/target/armv7-unknown-linux-musleabihf/debug/wifi_qr