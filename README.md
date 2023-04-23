# kernel-usb-workaround
A little Rust program that looks for a malfunctioning usb bus and disables it.  
It may be required to set `kernel.dmesg_restrict = 0`
```
[   38.916765] usb 1-4: device descriptor read/64, error -71
[   44.039820] usb 1-4: device descriptor read/64, error -71
[   44.269711] usb 1-4: new high-speed USB device number 8 using xhci_hcd
[   49.526425] usb 1-4: device descriptor read/64, error -71
[   54.653099] usb 1-4: device descriptor read/64, error -71
[   54.759803] usb usb1-port4: attempt power cycle
[   55.163200] usb 1-4: new high-speed USB device number 9 using xhci_hcd
[   59.951114] usb 1-4: device descriptor read/8, error -71
[   60.075184] usb 1-4: device descriptor read/8, error -71
[   60.509927] usb 1-4: new high-speed USB device number 10 using xhci_hcd
[   65.297876] usb 1-4: device descriptor read/8, error -71
[   65.422072] usb 1-4: device descriptor read/8, error -71
[   65.526793] usb usb1-port4: unable to enumerate USB device
[   65.527009] usb 1-3: USB disconnect, device number 2
```
Temporary solution until i find the root cause in the Linux kernel.  
This issue is caused by faulty hardware, but the kernel does not handle this well, causing increased power usage and refusing to shutdown.  
If anyone knows how to properly patch the kernel instead please email me.  
