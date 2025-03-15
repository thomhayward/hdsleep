pkgname=hdsleep
pkgver=0.2.0
pkgrel=1
arch=(any)
options=(!debug)
depends=(hdparm)
makedepends=(cargo)
source=(
	Cargo.toml
	Cargo.lock
	hdsleep.rs
)
sha256sums=(
	3bfeb3221935f5047ecd230f7e4c4ed3a1cb1c12ed8e149ca95ad52a742174cd
	b9e7b54382bb79b123f22b0e6272b580dff290dfa0294c9b4ee7a521ca8d09e4
	af582a14a331aa93666804216a56200e7828905ae936613f80ecf691ef155ffa
)

prepare() {
	export RUSTUP_TOOLCHAIN=stable
	cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
	export RUSTUP_TOOLCHAIN=stable
	export CARGO_TARGET_DIR=target
	cargo build --frozen --release
}

package() {
	install -Dm4755 -g wheel -t "$pkgdir/usr/local/bin" "target/release/$pkgname"
}
