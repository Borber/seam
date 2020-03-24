from bilibili import get_real_url as bilibili
from chushou import get_real_url as chushou
from douyin import get_real_url as douyin
from douyu import get_real_url as douyu
from egame import get_real_url as egame
from huajiao import get_real_url as huajiao
from huomao import get_real_url as huomao
from huya import get_real_url as huya
from iqiyi import get_real_url as iqiyi
from kuaishou import get_real_url as kuaishou
from kugou import get_real_url as kugou
from longzhu import get_real_url as longzhu
from now import get_real_url as now
from pps import get_real_url as pps
from v6cn import get_real_url as v6cn
from wangyicc import get_real_url as wangyicc
from xigua import get_real_url as xigua
from yingke import get_real_url as yingke
from yizhibo import get_real_url as yizhibo
from yy import get_real_url as yy
from zhanqi import get_real_url as zhanqi


def all_url(kind, rid):
    if kind == "bilibili":
        return bilibili(rid)
    if kind == "chushou":
        return chushou(rid)
    if kind == "douyin":
        return douyin(rid)
    if kind == "douyu":
        return douyu(rid)
    if kind == "egame":
        return egame(rid)
    if kind == "huajiao":
        return huajiao(rid)
    if kind == "huomao":
        return huomao(rid)
    if kind == "huya":
        return huya(rid)
    if kind == "iqiyi":
        return iqiyi(rid)
    if kind == "kuaishou":
        return kuaishou(rid)
    if kind == "kugou":
        return kugou(rid)
    if kind == "longzhu":
        return longzhu(rid)
    if kind == "now":
        return now(rid)
    if kind == "pps":
        return pps(rid)
    if kind == "v6cn":
        return v6cn(rid)
    if kind == "wangyicc":
        return wangyicc(rid)
    if kind == "xigua":
        return xigua(rid)
    if kind == "yingke":
        return yingke(rid)
    if kind == "yizhibo":
        return yizhibo(rid)
    if kind == "yy":
        return yy(rid)
    if kind == "zhanqi":
        return zhanqi(rid)
