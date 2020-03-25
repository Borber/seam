from all import all_url
import json


def get_m3u():
    m3u = "#EXTM3U\r\n"
    with open("config/config.json", "r", encoding="utf-8") as load_f:
        load_dict = json.load(load_f)
        length = len(load_dict["index"])
        index_json = load_dict["index"]
        for i in range(0, length):
            url = all_url(index_json[i]["kind"], index_json[i]["rid"])
            if url.startswith("http"):
                m3u = m3u + "#EXTINF:-1," + index_json[i]["name"] + "---" + index_json[i]["kind"] + "\r\n" + url + "\r\n"
    with open("m3u/BORBER.m3u", "w") as f:
        f.write(m3u)


get_m3u()
