from all import all_url
import json
import os
import msvcrt


def get_m3u():
    notic = ""
    m3u = "#EXTM3U\r\n"
    with open("config/config.json", "r", encoding='utf-8') as load_f:
        load_dict = json.load(load_f)
        length = len(load_dict["index"])
        index_json = load_dict["index"]
        for i in range(0, length):
            url = all_url(index_json[i]["kind"], index_json[i]["rid"])
            if url.startswith("http") and not url.endswith("重播中"):
                m3u = m3u + "#EXTINF:-1," + index_json[i]["name"] + "---" + index_json[i]["kind"] + "\r\n" + url + "\r\n"
                notic = notic + "\r\n" + index_json[i]["name"] + ": " + "开播啦q(≧▽≦q)"
            elif url.endswith("重播中"):
                url = url[0:len(url) - 3]
                m3u = m3u + "#EXTINF:-1," + index_json[i]["name"] + "---" + index_json[i][
                    "kind"] + "\r\n" + url + "\r\n"
                notic = notic + "\r\n" + index_json[i]["name"] + ": " + "重播中ヾ(≧▽≦*)o"
            else:
                notic = notic + "\r\n" + index_json[i]["name"] + ": " + "未开播/(ㄒoㄒ)/~~"
            # 清屏
            os.system('cls')
            # 显示
            pnotic = "[{}/{}]".format(i, length-1) + notic
            print(pnotic)

    with open("m3u/BORBER.m3u", "w", encoding="utf-8") as f:
        f.write(m3u)


if __name__ == '__main__':
    get_m3u()
    print("按任意键退出, 前往 m3u 目录 查看 m3u 播放目录")
    os.system('pause')
