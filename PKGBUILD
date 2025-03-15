pkgname=hdsleep
pkgver=0.2.0
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
	61df88a9137447389ce40066587d0e76c417660b45f3ad7995df3b41ad6dce60
	d3b519cbbdd006f5fbc98cd024a86d51141de5c71b46310bfe8dd4886526df24
	a2b37c5bbb3f7406c510ca6274025d08109c017a0fea2e8ad354ecf31d0ceaa8
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
