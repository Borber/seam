import "./App.css";

import { invoke } from "@tauri-apps/api/tauri";
import { createSignal, For, onMount } from "solid-js";
import toast, { Toaster } from "solid-toast";

import logo from "./assets/logo.png";
import Bar from "./components/Bar";
import SilderItem, { SilderItemProps } from "./components/SilderItem";
import allLives from "./model/Live";

interface Node {
    rid: string;
    title: string;
    urls: Url[];
}

interface Url {
    format: string;
    url: string;
}

interface Resp<T> {
    code: number;
    msg: string;
    data: T;
}

const App = () => {
    const [repos, setRepos] = createSignal<SilderItemProps[]>([]);
    const [live, setLive] = createSignal<string>("bili");
    const [rid, setRid] = createSignal<string>("");

    onMount(async () => {
        // 全局取消右键菜单;
        // document.oncontextmenu = (event) => {
        //     event.preventDefault();
        // };

        // 避免窗口闪烁, 等待500ms再显示窗口
        // 这个该死的bug什么时候才能修
        setTimeout(() => {
            setupWindow();
        }, 500);
    });

    const setupWindow = async () => {
        const appWindow = (await import("@tauri-apps/api/window")).appWindow;
        appWindow.show();
    };

    // 向侧边栏顶部添加一个新的item
    const addSiderItem = (node: Node) => {
        console.log(node);
        const newRepos = [
            {
                live: live(),
                rid: rid(),
                title: node.title,
                url: node.urls[0].url,
            },
            ...repos(),
        ];
        setRepos(newRepos);
    };

    const onFuck = async () => {
        if (rid() == "") {
            toast.error("房间号不能为空");
        } else {
            const result = await invoke<Resp<Node>>("url", {
                live: live(),
                rid: rid(),
            });

            if (result.code == 0) {
                toast.success("获取成功");
                addSiderItem(result.data);
            } else {
                toast.error(result.msg);
            }
        }
    };

    return (
        <>
            <div class="container  not-draggable">
                <div class="sider">
                    <For each={repos()}>
                        {(item) => <SilderItem {...item} />}
                    </For>
                </div>
                <div data-tauri-drag-region class="loader">
                    <Bar maximize={false} />
                    <div data-tauri-drag-region class="release">
                        <img src={logo} alt="Logo" width={"100px"} />
                        <div class="name">Seam</div>
                        <div class="version">0.1.0</div>
                    </div>

                    <input
                        placeholder="房间号"
                        class="rid-input"
                        onInput={(event) => setRid(event.target.value)}
                    />
                    <button onClick={async () => onFuck()}>获取</button>

                    <div class="live-item-container">
                        <For each={allLives()}>
                            {(item) => (
                                <button
                                    classList={{
                                        "is-checked": live() === item.cmd,
                                    }}
                                    class="live-item"
                                    onClick={() => setLive(item.cmd)}>
                                    {item.name}
                                </button>
                            )}
                        </For>
                    </div>
                </div>
                <Toaster
                    position="bottom-center"
                    gutter={8}
                    toastOptions={{
                        // Define default options that each toast will inherit. Will be overwritten by individual toast options
                        className: "",
                        duration: 5000,
                        style: {
                            background: "#363636",
                            color: "#fff",
                        },
                    }}
                />
            </div>
        </>
    );
};

export default App;
