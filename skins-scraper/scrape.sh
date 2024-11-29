python3 scraper.py

git clone https://github.com/TeeworldsDB/skins data/teeworldsdb
cd data/teeworldsdb
git pull
cd ../..

rm -rf output
mkdir output
mkdir output/original

# Prefer skins in this order, because of duplicates
cp -n data/ddnet/*.png output/original/
cp -n data/teedata/*.png output/original/
cp -n data/teeworld/*.png output/original/
cp -n data/teeworldsdb/06/*.png output/original/
cp -n data/other/*.png output/original/

# Remove any skins that don't have a alpha channel
cd ./output/original
identify -format '%A - %f\n' *.png 2>/dev/null | grep 'Undefined' | cut -c 13- | tr '\n' '\0' | xargs -0 rm

# Resize all skins to 512x256
find . ../ -maxdepth 1 -regex ".*\.png" -type f -printf "%f\n" | sort | uniq -u | tr '\n' '\0' | xargs -n 2 -P 6 -0 -I {} magick "{}" -resize 512x256\> -colorspace sRGB PNG00:"../{}"
cd ..

# Create a zip of all skins
mkdir zips/
find . -maxdepth 1 -regex ".*\.png" | zip -q -@ zips/skins.zip
find original/ -maxdepth 1 -regex ".*\.png" | zip -q -@ zips/skins-original.zip
