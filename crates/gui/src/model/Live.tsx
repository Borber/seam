export interface LiveItem {
    name: string;
    cmd: string;
}

const allLives = (): LiveItem[] => {
    const lives: LiveItem[] = [
        {
            name: "B站",
            cmd: "bili",
        },
        {
            name: "斗鱼",
            cmd: "douyu",
        },
        {
            name: "抖音",
            cmd: "douyin",
        },
        {
            name: "虎牙",
            cmd: "huya",
        },
        {
            name: "快手",
            cmd: "ks",
        },
        {
            name: "CC",
            cmd: "cc",
        },
        {
            name: "花椒",
            cmd: "huajiao",
        },
        {
            name: "艺气山",
            cmd: "yqs",
        },
        {
            name: "棉花糖",
            cmd: "mht",
        },
        {
            name: "KK",
            cmd: "kk",
        },
        {
            name: "千帆",
            cmd: "qf",
        },
        {
            name: "Now",
            cmd: "now",
        },
        {
            name: "映客",
            cmd: "inke",
        },
        {
            name: "afreeca",
            cmd: "afreeca",
        },
        {
            name: "pandalive",
            cmd: "panda",
        },
        {
            name: "flex",
            cmd: "flex",
        },
        {
            name: "wink",
            cmd: "wink",
        },
    ]
    return lives
}

export default allLives
