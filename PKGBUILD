# Maintainer: awumii <awumii@protonmail.com>

pkgname=kernel-usb-workaround
pkgver=1.0.0
pkgrel=1
pkgdesc='Workaround for malfunctioning usb devices.'
url='https://github.com/awumii/kernel-usb-workaround'
source=("${pkgname}::git+${url}.git")
arch=('any')
#license=('Unlicense')
makedepends=('rust')
depends=('systemd')
sha256sums=(SKIP)

build () {
  cd ${pkgname}
  cargo build --release
}

package() {
  cd ${pkgname}
  install -Dm755 target/release/kernel-usb-workaround "${pkgdir}/usr/bin/kernel-usb-workaround"
  install -Dm644 kernel-usb-workaround.service "${pkgdir}/etc/systemd/system/kernel-usb-workaround.service"
}

post_install {
  echo '>> Run the service using: systemctl enable --now kernel-usb-workaround.service'
}

pkgver() {
    cd ${pkgname}
    git describe --long --tags | sed 's/^v//;s/\([^-]*-g\)/r\1/;s/-/./g'
}
