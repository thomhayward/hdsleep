pkgname=hdsleep
pkgver=0.2.1
pkgrel=1
arch=(x86_64)
options=(!debug)
depends=(hdparm)
makedepends=(cargo)
source=(
	Cargo.toml
	Cargo.lock
	hdsleep.rs
)
sha256sums=(
	SKIP
	SKIP
	SKIP
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
