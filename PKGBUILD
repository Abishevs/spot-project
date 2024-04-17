pkgname=spot-package
pkgver=1.0
pkgrel=1
pkgdesc="S.P.O.T (Software/Study Pomodoro Orginaiser & Tracker)"
arch=('x86_64')
url="https://github.com/Abishevs/spot-project"
license=('MIT')
depends=('rust')
source=("$url/download/$pkgname-$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --release
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    install -Dm755 target/release/spot-cli "$pkgdir/usr/bin/spot-cli"
    install -Dm755 target/release/spot-deamon "$pkgdir/usr/bin/spot-deamon"
    install -Dm644 spot-deamon.service "$pkgdir/usr/lib/systemd/system/spot-deamon.service"
}

post_install() {
    systemctl enable spot-deamon.service
    systemctl start spot-deamon.service
}
