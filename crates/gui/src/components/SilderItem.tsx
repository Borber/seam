import "../css/SilderItem.css";

import { invoke } from "@tauri-apps/api/tauri";
import toast from "solid-toast";

import { CopyIcon, PlayIcon } from "../icon/icon";
import { Resp } from "../model/Resp";

export interface SilderItemProps {
    live: string;
    rid: string;
    title: string;
    url: string;
}

const copy = async (text: string) => {
    try {
        await navigator.clipboard.writeText(text);
        toast.success("复制成功");
    } catch (err) {
        toast.error("复制失败");
    }
};

const play = async (url: string) => {
    try {
        const result = await invoke<Resp<boolean>>("play", {
            url: url,
        });
        if (result.data) {
            toast.success("播放成功");
        } else {
            toast.error("播放失败: " + result.msg);
        }
    } catch (err) {
        toast.error("播放失败: " + err);
    }
};

const SilderItem = (props: SilderItemProps) => {
    return (
        <div class="sider-item">
            <div class="sider-item-info">
                <div class="sider-item-title">{props.title}</div>
                <div class="sider-item-live-rid">
                    {props.live} ◆ {props.rid}
                </div>
            </div>
            <div class="sider-item-button">
                <button onClick={() => copy(props.url)} title="复制">
                    <CopyIcon size={15} />
                </button>
            </div>

            <div class="sider-item-button">
                <button onClick={() => play(props.url)} title="播放">
                    <PlayIcon size={15} />
                </button>
            </div>
        </div>
    );
};

export default SilderItem;
