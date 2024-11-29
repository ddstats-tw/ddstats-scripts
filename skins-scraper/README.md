# skins-scraper

Scrapes DDNet skins from various sources ([DDNet](https://ddnet.org/skins/), [Teedata](https://teedata.net/), [TeeworldsDB](https://github.com/TeeworldsDB/skins), [skins.tee.world](https://skins.tee.world)). A dump of all skins made with these scripts can be found over at [ddstats.tw/skins/](https://ddstats.tw/skins/).

## Dependencies

> sudo dnf install python3 python3-urllib3 python3-requests ImageMagick git findutils zip

## Usage

Run `./scrape.sh`, final result will be inside of the `output/` directory. Script is meant to be ran incrementally, and will only fetch skins that haven't been previously downloaded.
