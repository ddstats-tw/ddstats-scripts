# dnf install rust-std-static-wasm32-unknown-unknown clang

git clone https://gitlab.com/Patiga/twgpu.git ../data/twgpu
git pull
mkdir -p /var/www/map-inspect/
cp -r mapres_06 /var/www/map-inspect/
cd ../data/twgpu/map-inspect-web
cargo install wasm-pack
wasm-pack build --target web
cp index.html /var/www/map-inspect/
cp -r pkg /var/www/map-inspect/
