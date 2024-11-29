git clone https://github.com/ddnet/ddnet-maps/ ../data/ddnet-maps/
cd ../data/ddnet-maps
git pull
mkdir -p /var/www/map-inspect/maps/
find . -type f -name "*.map" -exec cp "{}" /var/www/map-inspect/maps/ \;
