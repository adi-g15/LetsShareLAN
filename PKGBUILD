# Maintainer: Aditya Gupta <me dot adityag15 at gmail dot com>
pkgname=letssharelan-git
pkgver=r14.2689bb6
pkgrel=1
pkgdesc="Automates logging in to the NITP LAN, with randomly chosen credentials"
arch=('x86_64')
url="https://github.com/adi-g15/LetsShareLAN"
license=('GPL3')
depends=()
makedepends=('gcc' 'git' 'cargo')
provides=("lsl")
source=('letssharelan::git+https://github.com/adi-g15/LetsShareLAN')
md5sums=('SKIP')

pkgver() {
	cd "$srcdir/${pkgname%-git}"

	printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

build() {
	cd "$srcdir/${pkgname%-git}"
        cargo build --release
}

package() {
	cd "$srcdir/${pkgname%-git}"
        cargo install --root "$pkgdir"/usr --path . --no-track
}
