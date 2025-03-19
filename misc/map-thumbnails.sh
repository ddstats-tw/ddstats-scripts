mkdir -p ../data/thumbnails/
cd ../data/thumbnails/
curl https://ddnet.org/releases/maps.json | jq -r '.[]|[.name, .thumbnail] | @tsv' |
  while IFS=$'\t' read -r name thumbnail; do
    if [ ! -f "$name".png ]; then
        wget -O "$name".png "$thumbnail"
    fi
done

\cp -rf ../thumbnails/ /var/www/
