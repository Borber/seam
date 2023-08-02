import "../css/SilderItem.css";

import toast from "solid-toast";

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
                <button class="btn-orange" onClick={() => copy(props.url)}>
                    复制
                </button>
            </div>
        </div>
    );
};

export default SilderItem;
