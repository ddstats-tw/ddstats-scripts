import requests
import urllib.request
import urllib.parse
import os
import re

teeworld_offset = 0

def save_skin(url, skin, service):
    save_path = f'''./data/{service}/{skin}.png'''

    if os.path.isfile(save_path):
        return
    urllib.request.urlretrieve(url, save_path)
    print(f"Downloaded '{skin}' from '{url}'")

def scrape_ddnet():
    r = requests.get(url="https://ddnet.org/skins/skin/skins.json")
    json = r.json()

    for skin in json['skins']:
        url = f'''https://ddnet.org/skins/skin/community/{urllib.parse.quote(skin['name'])}.png'''
        save_skin(url, skin['name'], "ddnet")

def scrape_teedata():
    r = requests.get(url="https://teedata.net/api/skin/read?limit=99999")
    json = r.json()

    for skin in json['result']['items']:
        url = f'''https://teedata.net/api/skin/resolve/{urllib.parse.quote(skin['name'])}'''
        save_skin(url, skin['name'], "teedata")

def scrape_teeworld(offset):
    r = requests.get(url=f"https://skins.tee.world/skinlist.php?offset={offset}")

    html = r._content.decode("utf-8")
    skins = re.findall("skin.php\\?skin=(.*?)\\.png", html)

    for skin in skins:
        url = f'''https://skins.tee.world/{urllib.parse.quote(skin)}.png'''
        save_skin(url, skin, "teeworld")

    if(len(skins) == 195):
        offset += len(skins)
        scrape_teeworld(offset)

opener = urllib.request.build_opener()
opener.addheaders = [("User-agent", "DDStats (ddstats.tw/skins/)")]
urllib.request.install_opener(opener)

os.makedirs("./data/", exist_ok=True)
os.makedirs("./data/other", exist_ok=True)
os.makedirs("./data/ddnet", exist_ok=True)
os.makedirs("./data/teedata", exist_ok=True)
os.makedirs("./data/teeworld", exist_ok=True)

scrape_ddnet()
scrape_teedata()
scrape_teeworld(teeworld_offset)
